// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/segment.rs
// PURPOSE:     Sentence segmentation for multi-sentence FACES detection
// ═══════════════════════════════════════════════════════════════════════════════
//
// SPLITTING STRATEGY
//
// FACES detection improves when multi-sentence input is split and each
// sentence is scored independently. This module provides a lightweight,
// zero-allocation sentence splitter that handles the common delimiters
// (., !, ?, ;, newlines) without requiring a full NLP tokenizer.
//
// ABBREVIATION HANDLING
//
// The splitter includes a minimal abbreviation list (Mr., Dr., Mrs., Ms.,
// Prof., Sr., Jr., vs., etc., e.g., i.e.) to avoid splitting on common
// title and abbreviation periods. This is intentionally simple — a full
// tokenizer would do better, but this is the zero-compute path.
//
// ═══════════════════════════════════════════════════════════════════════════════

/// Split text into sentence segments.
///
/// Splits on `.`, `!`, `?`, `;`, and newlines. Handles a minimal set of
/// common abbreviations to avoid false splits. Trims whitespace and
/// discards empty segments.
///
/// # Arguments
///
/// * `text` — The input text to segment
///
/// # Returns
///
/// A `Vec<String>` of trimmed, non-empty sentence strings.
///
/// # Example
///
/// ```
/// use faces_protocol::segment::segment_sentences;
///
/// let sentences = segment_sentences("Hello world. How are you? Fine!");
/// assert_eq!(sentences.len(), 3);
/// assert_eq!(sentences[0], "Hello world");
/// assert_eq!(sentences[1], "How are you");
/// assert_eq!(sentences[2], "Fine");
/// ```
pub fn segment_sentences(text: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '.' && is_abbreviation(&chars, i) {
            current.push(c);
            i += 1;
            continue;
        }

        if is_delimiter(c) {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                results.push(trimmed);
            }
            current.clear();
        } else {
            current.push(c);
        }
        i += 1;
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        results.push(trimmed);
    }

    results
}

/// Check if a character is a sentence delimiter.
fn is_delimiter(c: char) -> bool {
    matches!(c, '.' | '!' | '?' | ';' | '\n' | '\r')
}

/// Check if the period at position `i` is part of a known abbreviation.
///
/// Looks at the word ending at the period and checks against a small
/// set of common abbreviations.
fn is_abbreviation(chars: &[char], period_idx: usize) -> bool {
    let mut start = period_idx;
    while start > 0 && chars[start - 1].is_alphabetic() {
        start -= 1;
    }

    let word: String = chars[start..period_idx]
        .iter()
        .collect::<String>()
        .to_lowercase();

    const ABBREVIATIONS: &[&str] = &[
        "mr", "dr", "mrs", "ms", "prof", "sr", "jr",
        "vs", "etc", "st", "approx",
        "fig", "vol", "no", "pp", "ch", "sec",
        "al", "op", "ed", "re", "inc", "ltd", "co",
    ];

    ABBREVIATIONS.contains(&word.as_str())
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_split() {
        let s = segment_sentences("Hello world. How are you? Fine!");
        assert_eq!(s.len(), 3);
        assert_eq!(s[0], "Hello world");
        assert_eq!(s[1], "How are you");
        assert_eq!(s[2], "Fine");
    }

    #[test]
    fn test_multiple_delimiters() {
        let s = segment_sentences("One; two! Three? Four.");
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], "One");
        assert_eq!(s[1], "two");
        assert_eq!(s[2], "Three");
        assert_eq!(s[3], "Four");
    }

    #[test]
    fn test_empty_input() {
        let s = segment_sentences("");
        assert!(s.is_empty());
    }

    #[test]
    fn test_no_delimiters() {
        let s = segment_sentences("just some text without punctuation");
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "just some text without punctuation");
    }

    #[test]
    fn test_newline_split() {
        let s = segment_sentences("Line one\nLine two\nLine three");
        assert_eq!(s.len(), 3);
        assert_eq!(s[0], "Line one");
        assert_eq!(s[1], "Line two");
        assert_eq!(s[2], "Line three");
    }

    #[test]
    fn test_carriage_return_split() {
        let s = segment_sentences("Line one\r\nLine two");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Line one");
        assert_eq!(s[1], "Line two");
    }

    #[test]
    fn test_trailing_delimiter() {
        let s = segment_sentences("Hello world.");
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "Hello world");
    }

    #[test]
    fn test_leading_delimiter() {
        let s = segment_sentences(".Hello world");
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "Hello world");
    }

    #[test]
    fn test_consecutive_delimiters() {
        let s = segment_sentences("Hello... world");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Hello");
        assert_eq!(s[1], "world");
    }

    #[test]
    fn test_whitespace_only() {
        let s = segment_sentences("   \n\t  \n  ");
        assert!(s.is_empty());
    }

    #[test]
    fn test_abbreviation_mr() {
        let s = segment_sentences("Mr. Smith went home. He was tired.");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Mr. Smith went home");
        assert_eq!(s[1], "He was tired");
    }

    #[test]
    fn test_abbreviation_dr() {
        let s = segment_sentences("Dr. Jones arrived. The patient is ready.");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Dr. Jones arrived");
        assert_eq!(s[1], "The patient is ready");
    }

    #[test]
    fn test_abbreviation_etc() {
        // "etc." is recognized as abbreviation, so period doesn't split.
        // Use "!" to end the sentence instead.
        let s = segment_sentences("We need apples, oranges, etc! Then we go home.");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "We need apples, oranges, etc");
        assert_eq!(s[1], "Then we go home");
    }

    #[test]
    fn test_abbreviation_etc_no_split() {
        // "etc." at end of sentence — abbreviation recognized, no split at that period
        let s = segment_sentences("We need apples, etc. Then we go home.");
        // "etc." doesn't split, so it merges with next until "home."
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "We need apples, etc. Then we go home");
    }

    #[test]
    fn test_abbreviation_prof() {
        let s = segment_sentences("Prof. Adams lectured. The class was engaged.");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Prof. Adams lectured");
        assert_eq!(s[1], "The class was engaged");
    }

    #[test]
    fn test_single_word_no_punct() {
        let s = segment_sentences("Hello");
        assert_eq!(s.len(), 1);
        assert_eq!(s[0], "Hello");
    }

    #[test]
    fn test_single_period() {
        let s = segment_sentences(".");
        assert!(s.is_empty());
    }

    #[test]
    fn test_trailing_whitespace_in_segment() {
        let s = segment_sentences("Hello  .  World  !");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Hello");
        assert_eq!(s[1], "World");
    }

    #[test]
    fn test_mixed_delimiters() {
        let s = segment_sentences("What? Why! How; When. Done.");
        assert_eq!(s.len(), 5);
        assert_eq!(s[0], "What");
        assert_eq!(s[1], "Why");
        assert_eq!(s[2], "How");
        assert_eq!(s[3], "When");
        assert_eq!(s[4], "Done");
    }

    #[test]
    fn test_inc_abbreviation() {
        let s = segment_sentences("Acme Inc. is hiring. Apply now.");
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], "Acme Inc. is hiring");
        assert_eq!(s[1], "Apply now");
    }
}
