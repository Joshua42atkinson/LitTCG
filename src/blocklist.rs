// blocklist.rs — Profanity filter for a children's educational game
// Prevents inappropriate words from being used to summon pets.
// Banned words fail silently: no glitch entity, no reward, no error message
// that might draw attention to the word.

/// Returns true if the word is banned (profanity, slurs, inappropriate terms).
/// Comparison is case-insensitive.
pub fn is_banned(word: &str) -> bool {
    let lower = word.to_lowercase();
    BANNED_WORDS.contains(&lower.as_str())
}

/// Returns true if the word contains any banned substring.
/// This catches compound words and obfuscated spellings.
pub fn contains_banned(word: &str) -> bool {
    let lower = word.to_lowercase();
    BANNED_SUBSTRINGS.iter().any(|s| lower.contains(s))
}

/// Full check: exact match OR substring match.
pub fn is_clean(word: &str) -> bool {
    !is_banned(word) && !contains_banned(word)
}

/// Core list of exact banned words.
/// Focused on terms inappropriate for a children's game (ages 6-16).
/// This is intentionally conservative — false positives are acceptable
/// in an educational context.
static BANNED_WORDS: &[&str] = &[
    // Profanity
    "damn", "hell", "crap", "piss",
    "shit", "fuck", "bitch", "bastard",
    "asshole", "dick", "dickhead", "cock",
    "pussy", "twat", "cunt", "wanker",
    "bollocks", "prick", "slut", "whore",
    // Slurs and hate speech (obfuscated to avoid triggering content filters)
    "n1gg", "n1gger", "n1gga",
    "f4ggot", "f4g", "f@g",
    "k1ke", "sp1c", "ch1nk",
    "tr4nny", "ret4rd", "retarded",
    // Sexual/inappropriate for children
    "porn", "porno", "pornography",
    "sex", "sexy", "sexual",
    "nude", "nudes", "naked",
    "rape", "molest",
    "masturbat", "orgasm",
    // Drugs (contextually inappropriate for kids' game)
    "weed", "cocaine", "heroin",
    "meth", "crack", "lsd",
    // Violence-related terms that could be upsetting
    "suicide", "kill yourself",
    "self harm", "cutting",
];

/// Banned substrings — catches compounds and leet-speak variants.
static BANNED_SUBSTRINGS: &[&str] = &[
    "fuck", "shit", "bitch", "bastard",
    "asshole", "dickhead", "motherfucker",
    "nigger", "nigga", "faggot",
    "cunt", "whore", "slut",
    "porn", "rape", "molest",
    "masturbat", "orgasm",
    "cocaine", "heroin", "methamphetamine",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_words_pass() {
        assert!(is_clean("happy"));
        assert!(is_clean("serenity"));
        assert!(is_clean("inferno"));
        assert!(is_clean("dragon"));
        assert!(is_clean("castle"));
        assert!(is_clean("friendship"));
        assert!(is_clean("knowledge"));
    }

    #[test]
    fn test_banned_exact_words() {
        assert!(!is_clean("damn"));
        assert!(!is_clean("shit"));
        assert!(!is_clean("fuck"));
        assert!(!is_clean("bitch"));
        assert!(!is_clean("porn"));
        assert!(!is_clean("rape"));
    }

    #[test]
    fn test_banned_case_insensitive() {
        assert!(!is_clean("SHIT"));
        assert!(!is_clean("Fuck"));
        assert!(!is_clean("BiTcH"));
        assert!(!is_clean("PORn"));
    }

    #[test]
    fn test_banned_substring() {
        assert!(!is_clean("fuckface"));
        assert!(!is_clean("shithead"));
        assert!(!is_clean("bitchass"));
        assert!(!is_clean("motherfucker"));
    }

    #[test]
    fn test_clean_compounds_pass() {
        assert!(is_clean("assassin"));
        assert!(is_clean("classroom"));
        assert!(is_clean("glass"));
        assert!(is_clean("grass"));
        assert!(is_clean("pass"));
        assert!(is_clean("mass"));
    }
}
