//! Metrics — confusion matrices and P/R/F1 per dimension.
//!
//! Tracks true positives, false positives, and false negatives for each
//! FACES dimension (Aura, Container, Focus, Action) and computes
//! precision, recall, F1, and accuracy.

/// Confusion matrix for a single dimension.
#[derive(Debug, Clone, Default)]
pub struct ConfusionMatrix {
    /// True positives — predicted matches expected.
    pub tp: u32,
    /// False positives — predicted a value, but wrong.
    pub fp: u32,
    /// False negatives — expected a value, but missed.
    pub fn_: u32,
    /// True negatives — correctly predicted "no signal" / default.
    pub tn: u32,
    /// Total samples evaluated.
    pub total: u32,
}

impl ConfusionMatrix {
    /// Record a prediction vs expected result.
    pub fn record(&mut self, predicted: u32, expected: u32) {
        self.total += 1;
        if predicted == expected {
            self.tp += 1;
        } else {
            self.fp += 1;
            self.fn_ += 1;
        }
    }

    /// Precision: TP / (TP + FP)
    pub fn precision(&self) -> f32 {
        let denom = self.tp + self.fp;
        if denom == 0 { 0.0 } else { self.tp as f32 / denom as f32 }
    }

    /// Recall: TP / (TP + FN)
    pub fn recall(&self) -> f32 {
        let denom = self.tp + self.fn_;
        if denom == 0 { 0.0 } else { self.tp as f32 / denom as f32 }
    }

    /// F1: 2 * P * R / (P + R)
    pub fn f1(&self) -> f32 {
        let p = self.precision();
        let r = self.recall();
        let denom = p + r;
        if denom == 0.0 { 0.0 } else { 2.0 * p * r / denom }
    }

    /// Accuracy: correct / total
    pub fn accuracy(&self) -> f32 {
        if self.total == 0 { 0.0 } else { self.tp as f32 / self.total as f32 }
    }
}

/// Overall metrics across all dimensions.
#[derive(Debug, Clone)]
pub struct OverallMetrics {
    /// Per-dimension confusion matrices.
    pub aura: ConfusionMatrix,
    pub container: ConfusionMatrix,
    pub focus: ConfusionMatrix,
    pub action: ConfusionMatrix,
    /// Congruence detection accuracy.
    pub congruence_correct: u32,
    pub congruence_total: u32,
}

impl Default for OverallMetrics {
    fn default() -> Self {
        Self {
            aura: ConfusionMatrix::default(),
            container: ConfusionMatrix::default(),
            focus: ConfusionMatrix::default(),
            action: ConfusionMatrix::default(),
            congruence_correct: 0,
            congruence_total: 0,
        }
    }
}

impl OverallMetrics {
    /// Macro-averaged precision across all dimensions.
    pub fn macro_precision(&self) -> f32 {
        (self.aura.precision() + self.container.precision()
            + self.focus.precision() + self.action.precision()) / 4.0
    }

    /// Macro-averaged recall across all dimensions.
    pub fn macro_recall(&self) -> f32 {
        (self.aura.recall() + self.container.recall()
            + self.focus.recall() + self.action.recall()) / 4.0
    }

    /// Macro-averaged F1 across all dimensions.
    pub fn macro_f1(&self) -> f32 {
        (self.aura.f1() + self.container.f1()
            + self.focus.f1() + self.action.f1()) / 4.0
    }

    /// Overall accuracy across all dimensions.
    pub fn overall_accuracy(&self) -> f32 {
        let total_correct = self.aura.tp + self.container.tp
            + self.focus.tp + self.action.tp;
        let total = self.aura.total + self.container.total
            + self.focus.total + self.action.total;
        if total == 0 { 0.0 } else { total_correct as f32 / total as f32 }
    }

    /// Congruence detection accuracy.
    pub fn congruence_accuracy(&self) -> f32 {
        if self.congruence_total == 0 { 0.0 }
        else { self.congruence_correct as f32 / self.congruence_total as f32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_prediction() {
        let mut cm = ConfusionMatrix::default();
        cm.record(1, 1);
        cm.record(2, 2);
        cm.record(3, 3);
        assert_eq!(cm.precision(), 1.0);
        assert_eq!(cm.recall(), 1.0);
        assert_eq!(cm.f1(), 1.0);
        assert_eq!(cm.accuracy(), 1.0);
    }

    #[test]
    fn test_all_wrong() {
        let mut cm = ConfusionMatrix::default();
        cm.record(1, 2);
        cm.record(3, 4);
        assert_eq!(cm.precision(), 0.0);
        assert_eq!(cm.recall(), 0.0);
        assert_eq!(cm.f1(), 0.0);
        assert_eq!(cm.accuracy(), 0.0);
    }

    #[test]
    fn test_half_right() {
        let mut cm = ConfusionMatrix::default();
        cm.record(1, 1);
        cm.record(2, 3);
        cm.record(3, 3);
        cm.record(4, 5);
        assert!((cm.precision() - 0.5).abs() < 0.001);
        assert!((cm.recall() - 0.5).abs() < 0.001);
        assert!((cm.f1() - 0.5).abs() < 0.001);
        assert!((cm.accuracy() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_empty_matrix() {
        let cm = ConfusionMatrix::default();
        assert_eq!(cm.precision(), 0.0);
        assert_eq!(cm.recall(), 0.0);
        assert_eq!(cm.f1(), 0.0);
        assert_eq!(cm.accuracy(), 0.0);
    }

    #[test]
    fn test_overall_metrics() {
        let mut om = OverallMetrics::default();
        om.aura.record(44, 44);
        om.aura.record(10, 10);
        om.container.record(0, 0);
        om.container.record(1, 2);
        om.focus.record(0, 0);
        om.action.record(2, 2);
        om.congruence_correct = 3;
        om.congruence_total = 4;

        assert!(om.macro_precision() > 0.0);
        assert!(om.macro_recall() > 0.0);
        assert!(om.macro_f1() > 0.0);
        assert!((om.congruence_accuracy() - 0.75).abs() < 0.001);
    }
}
