//! Evaluation runner — runs detection on labeled examples and builds reports.
//!
//! For each labeled example, runs `detect_scored()` and compares the
//! predicted FACES state against the expected state, building per-dimension
//! confusion matrices and computing overall metrics.

use crate::detect::{detect_scored, Congruence};
use super::loader::LabeledExample;
use super::metrics::OverallMetrics;

/// Full evaluation report.
#[derive(Debug, Clone)]
pub struct EvalReport {
    /// Number of examples evaluated.
    pub examples_evaluated: usize,
    /// Per-dimension and overall metrics.
    pub metrics: OverallMetrics,
    /// Mean inference latency in microseconds (0 if not measured).
    pub mean_latency_us: u64,
    /// Per-dimension accuracy breakdown.
    pub aura_accuracy: f32,
    pub container_accuracy: f32,
    pub focus_accuracy: f32,
    pub action_accuracy: f32,
}

impl EvalReport {
    /// Human-readable text summary.
    pub fn to_summary(&self) -> String {
        let m = &self.metrics;
        format!(
            "═══ FACES Evaluation Report ═══\n\
             Examples: {}\n\
                             \n\
             ┌───────────┬───────────┬───────────┬───────────┐\n\
             │ Dimension │ Precision │   Recall  │    F1     │\n\
             ├───────────┼───────────┼───────────┼───────────┤\n\
             │ Aura      │  {:.4}   │  {:.4}   │  {:.4}   │\n\
             │ Container │  {:.4}   │  {:.4}   │  {:.4}   │\n\
             │ Focus     │  {:.4}   │  {:.4}   │  {:.4}   │\n\
             │ Action    │  {:.4}   │  {:.4}   │  {:.4}   │\n\
             ├───────────┼───────────┼───────────┼───────────┤\n\
             │ MACRO     │  {:.4}   │  {:.4}   │  {:.4}   │\n\
             └───────────┴───────────┴───────────┴───────────┘\n\
                             \n\
             Overall Accuracy:    {:.4}\n\
             Congruence Accuracy: {:.4}\n\
             Mean Latency:        {} μs",
            self.examples_evaluated,
            m.aura.precision(), m.aura.recall(), m.aura.f1(),
            m.container.precision(), m.container.recall(), m.container.f1(),
            m.focus.precision(), m.focus.recall(), m.focus.f1(),
            m.action.precision(), m.action.recall(), m.action.f1(),
            m.macro_precision(), m.macro_recall(), m.macro_f1(),
            m.overall_accuracy(),
            m.congruence_accuracy(),
            self.mean_latency_us,
        )
    }

    /// JSON output for tooling (manual formatting, zero-dep).
    pub fn to_json(&self) -> String {
        let m = &self.metrics;
        format!(
            r#"{{"examples":{},"aura":{{"p":{:.4},"r":{:.4},"f1":{:.4}}},"container":{{"p":{:.4},"r":{:.4},"f1":{:.4}}},"focus":{{"p":{:.4},"r":{:.4},"f1":{:.4}}},"action":{{"p":{:.4},"r":{:.4},"f1":{:.4}}},"macro":{{"p":{:.4},"r":{:.4},"f1":{:.4}}},"overall_accuracy":{:.4},"congruence_accuracy":{:.4},"mean_latency_us":{}}}"#,
            self.examples_evaluated,
            m.aura.precision(), m.aura.recall(), m.aura.f1(),
            m.container.precision(), m.container.recall(), m.container.f1(),
            m.focus.precision(), m.focus.recall(), m.focus.f1(),
            m.action.precision(), m.action.recall(), m.action.f1(),
            m.macro_precision(), m.macro_recall(), m.macro_f1(),
            m.overall_accuracy(),
            m.congruence_accuracy(),
            self.mean_latency_us,
        )
    }
}

/// Run evaluation on a set of labeled examples.
///
/// For each example, runs `detect_scored()` and compares the predicted
/// state against the expected state per-dimension.
pub fn run_evaluation(examples: &[LabeledExample]) -> EvalReport {
    let mut metrics = OverallMetrics::default();
    let mut total_latency = 0u64;

    for ex in examples {
        let start = std::time::Instant::now();
        let result = detect_scored(&ex.text);
        let elapsed = start.elapsed().as_micros() as u64;
        total_latency += elapsed;

        // Compare per-dimension
        metrics.aura.record(
            result.state.aura.index() as u32,
            ex.expected_state.aura.index() as u32,
        );
        metrics.container.record(
            result.state.container as u8 as u32,
            ex.expected_state.container as u8 as u32,
        );
        metrics.focus.record(
            result.state.focus as u8 as u32,
            ex.expected_state.focus as u8 as u32,
        );
        metrics.action.record(
            result.state.action as u8 as u32,
            ex.expected_state.action as u8 as u32,
        );

        // Compare congruence
        metrics.congruence_total += 1;
        if result.congruence == ex.expected_congruence {
            metrics.congruence_correct += 1;
        }
    }

    let mean_latency = if examples.is_empty() { 0 } else { total_latency / examples.len() as u64 };

    EvalReport {
        examples_evaluated: examples.len(),
        aura_accuracy: metrics.aura.accuracy(),
        container_accuracy: metrics.container.accuracy(),
        focus_accuracy: metrics.focus.accuracy(),
        action_accuracy: metrics.action.accuracy(),
        metrics,
        mean_latency_us: mean_latency,
    }
}

/// Run evaluation with a sample dataset embedded in the binary.
pub fn run_evaluation_embedded() -> EvalReport {
    let examples = super::loader::load_from_str(SAMPLE_DATA);
    run_evaluation(&examples)
}

/// Embedded sample dataset — 50+ labeled examples.
pub const SAMPLE_DATA: &str = include_str!("../../eval/data/sample.jsonl");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Aura, Container, Focus, Action, FacesState};
    use crate::detect::Congruence;

    #[test]
    fn test_run_evaluation_empty() {
        let report = run_evaluation(&[]);
        assert_eq!(report.examples_evaluated, 0);
        assert_eq!(report.mean_latency_us, 0);
    }

    #[test]
    fn test_run_evaluation_single() {
        let examples = vec![LabeledExample {
            text: "I am so happy and joyful".to_string(),
            expected_state: FacesState::new(
                Aura::from_index(44),
                Container::Neutral,
                Focus::Neutral,
                Action::Playful,
            ),
            expected_congruence: Congruence::Congruent,
            expected_confidence: 0.8,
        }];
        let report = run_evaluation(&examples);
        assert_eq!(report.examples_evaluated, 1);
        assert!(report.mean_latency_us > 0);
    }

    #[test]
    fn test_report_summary_non_empty() {
        let report = run_evaluation_embedded();
        let summary = report.to_summary();
        assert!(summary.contains("FACES Evaluation Report"));
        assert!(summary.contains("Examples:"));
    }

    #[test]
    fn test_report_json_parseable() {
        let report = run_evaluation_embedded();
        let json = report.to_json();
        assert!(json.starts_with('{'));
        assert!(json.ends_with('}'));
        assert!(json.contains("\"aura\""));
        assert!(json.contains("\"macro\""));
    }

    #[test]
    fn test_embedded_dataset_loads() {
        let examples = super::super::loader::load_from_str(SAMPLE_DATA);
        assert!(examples.len() >= 50, "Sample dataset should have 50+ examples, got {}", examples.len());
    }
}
