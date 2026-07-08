//! JSONL loader — parses labeled examples from JSONL format.
//!
//! Zero-dependency: uses manual string parsing instead of serde.
//! Each line is a JSON object with fields:
//!   {"text": "...", "aura": N, "container": N, "focus": N, "action": N,
//!    "congruence": "Congruent|Incongruent|Neutral", "confidence": 0.X}

use crate::{Aura, Container, Focus, Action, FacesState};
use crate::detect::{Congruence, DetectionResult, DetectionMethod};

/// A labeled example for evaluation.
#[derive(Debug, Clone)]
pub struct LabeledExample {
    /// The input text to detect from.
    pub text: String,
    /// The expected FACES state.
    pub expected_state: FacesState,
    /// Expected congruence.
    pub expected_congruence: Congruence,
    /// Expected overall confidence (0.0–1.0).
    pub expected_confidence: f32,
}

/// Load labeled examples from a JSONL file on disk.
pub fn load_jsonl(path: &str) -> Vec<LabeledExample> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    load_from_str(&content)
}

/// Load labeled examples from a JSONL string.
/// Skips malformed lines gracefully.
pub fn load_from_str(jsonl: &str) -> Vec<LabeledExample> {
    let mut examples = Vec::new();
    for (lineno, line) in jsonl.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_line(line) {
            Ok(ex) => examples.push(ex),
            Err(e) => {
                // Skip malformed line, log to stderr
                eprintln!("eval: skipping line {} — {}", lineno + 1, e);
            }
        }
    }
    examples
}

fn parse_line(line: &str) -> Result<LabeledExample, String> {
    let text = extract_string(line, "text")
        .ok_or_else(|| "missing 'text' field".to_string())?;

    let aura_idx = extract_u32(line, "aura")
        .ok_or_else(|| "missing 'aura' field".to_string())?;
    let container_idx = extract_u32(line, "container")
        .ok_or_else(|| "missing 'container' field".to_string())?;
    let focus_idx = extract_u32(line, "focus")
        .ok_or_else(|| "missing 'focus' field".to_string())?;
    let action_idx = extract_u32(line, "action")
        .ok_or_else(|| "missing 'action' field".to_string())?;

    let congruence_str = extract_string(line, "congruence")
        .unwrap_or_else(|| "Neutral".to_string());
    let congruence = match congruence_str.as_str() {
        "Congruent" | "congruent" => Congruence::Congruent,
        "Incongruent" | "incongruent" => Congruence::Incongruent,
        _ => Congruence::Neutral,
    };

    let confidence = extract_f32(line, "confidence").unwrap_or(0.5);

    let state = FacesState::new(
        Aura::from_index(aura_idx as u8),
        Container::from_byte(container_idx as u8),
        Focus::from_byte(focus_idx as u8),
        Action::from_byte(action_idx as u8),
    );

    Ok(LabeledExample {
        text,
        expected_state: state,
        expected_congruence: congruence,
        expected_confidence: confidence,
    })
}

fn extract_string(line: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let start = line.find(&pattern)?;
    let rest = &line[start + pattern.len()..];
    // Find the colon
    let colon = rest.find(':')?;
    let after_colon = rest[colon + 1..].trim_start();
    // Find opening quote
    let q_start = after_colon.find('"')?;
    let after_q = &after_colon[q_start + 1..];
    // Find closing quote (handle escaped quotes)
    let mut end = 0;
    let mut chars = after_q.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            chars.next();
            end += 2;
            continue;
        }
        if c == '"' {
            break;
        }
        end += c.len_utf8();
    }
    Some(after_q[..end].replace("\\\"", "\"").replace("\\n", "\n"))
}

fn extract_u32(line: &str, key: &str) -> Option<u32> {
    let pattern = format!("\"{}\"", key);
    let start = line.find(&pattern)?;
    let rest = &line[start + pattern.len()..];
    let colon = rest.find(':')?;
    let after_colon = rest[colon + 1..].trim_start();
    let num_str: String = after_colon.chars().take_while(|c| c.is_ascii_digit()).collect();
    num_str.parse().ok()
}

fn extract_f32(line: &str, key: &str) -> Option<f32> {
    let pattern = format!("\"{}\"", key);
    let start = line.find(&pattern)?;
    let rest = &line[start + pattern.len()..];
    let colon = rest.find(':')?;
    let after_colon = rest[colon + 1..].trim_start();
    let num_str: String = after_colon.chars().take_while(|c| {
        c.is_ascii_digit() || *c == '.' || *c == '-'
    }).collect();
    num_str.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_str_basic() {
        let jsonl = r#"{"text": "I am happy", "aura": 44, "container": 0, "focus": 0, "action": 2, "congruence": "Congruent", "confidence": 0.8}"#;
        let examples = load_from_str(jsonl);
        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].text, "I am happy");
        assert_eq!(examples[0].expected_state.aura, Aura::from_index(44));
        assert_eq!(examples[0].expected_congruence, Congruence::Congruent);
    }

    #[test]
    fn test_load_from_str_multiple_lines() {
        let jsonl = "\
{\"text\": \"hello\", \"aura\": 0, \"container\": 0, \"focus\": 0, \"action\": 0, \"congruence\": \"Neutral\", \"confidence\": 0.3}\n\
{\"text\": \"angry\", \"aura\": 1, \"container\": 1, \"focus\": 3, \"action\": 3, \"congruence\": \"Congruent\", \"confidence\": 0.9}";
        let examples = load_from_str(jsonl);
        assert_eq!(examples.len(), 2);
    }

    #[test]
    fn test_skip_malformed() {
        let jsonl = "\
{\"text\": \"good\", \"aura\": 0, \"container\": 0, \"focus\": 0, \"action\": 0}\n\
this is not json\n\
{\"text\": \"ok\", \"aura\": 0, \"container\": 0, \"focus\": 0, \"action\": 0, \"congruence\": \"Neutral\", \"confidence\": 0.5}";
        let examples = load_from_str(jsonl);
        assert_eq!(examples.len(), 2);
    }

    #[test]
    fn test_skip_empty_and_comments() {
        let jsonl = "\n\n# comment\n{\"text\": \"test\", \"aura\": 0, \"container\": 0, \"focus\": 0, \"action\": 0, \"congruence\": \"Neutral\", \"confidence\": 0.5}";
        let examples = load_from_str(jsonl);
        assert_eq!(examples.len(), 1);
    }

    #[test]
    fn test_empty_input() {
        let examples = load_from_str("");
        assert!(examples.is_empty());
    }

    #[test]
    fn test_escaped_quotes_in_text() {
        let jsonl = r#"{"text": "say \"hi\"", "aura": 0, "container": 0, "focus": 0, "action": 0, "congruence": "Neutral", "confidence": 0.5}"#;
        let examples = load_from_str(jsonl);
        assert_eq!(examples.len(), 1);
        assert_eq!(examples[0].text, r#"say "hi""#);
    }
}
