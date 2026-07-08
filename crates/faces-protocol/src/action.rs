// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/action.rs
// PURPOSE:     Action shapes (Byte 3) — mouth/expression state
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE ACTION (Byte 3)
//
// The Action defines the communicative intent or mechanical output
// state of the FACES primitive. It is the "mouth" — the expression
// below the eyes. In linguistic terms, the Action maps to the VERB
// (action/predicate): kinetic readiness, dynamic output, and
// expressiveness, such as speaking, withholding, or playing.
//
// FACS MAPPING
//
// The Action byte maps to FACS Action Units related to the mouth:
//   AU 12 (Lip Corner Puller) → Playful (~)
//   AU 15 (Lip Corner Depressor) → Thoughtful (-)
//   AU 22 (Lip Funneler) → Withheld (_)
//   AU 27 (Mouth Stretch) → Assertive (v)
//   Relaxed/uncertain mouth → Hesitant (.)
//
// COMMITTEE FRAMEWORK MAPPING
//
// The Action maps to the "Action Channel" (Will) from the Committee
// framework, and to "Vocabulary" (V) in the VAAM model. It executes
// kinetic expression, verbalization, real-world manifestation, and
// direct output.
//
// SOMATIC PUNCTUATION
//
// FACES is defined not as a decorative icon but as a functional
// punctuation mark. The Action byte is the "punctuation" — it is
// appended to standard text to entirely alter its prosodic reading:
//
//   "I'm fine (oo_)"  — neutral, withheld, stoic
//   "I'm fine {^^~}"  — fluid, happy, playful (genuine)
//   "I'm fine [><v]"  — rigid, intense, assertive (defensive)
//   "I'm fine {.~.}"  — fluid, distant, playful (sarcastic)
//
// This structural addition changes the receiver's interpretation of
// the preceding semantic content, serving as a universal emotional
// metadata layer that survives cross-lingual translation.
//
// ═══════════════════════════════════════════════════════════════════════════════

// ── Action Enum ──────────────────────────────────────────────────────────────

/// The 5 Action shapes of the FACES protocol (Byte 3).
///
/// Each variant represents a communicative intent/mouth state, mapped
/// to FACS Action Units, a single ASCII character, and a semantic meaning.
///
/// # Variants
///
/// | Value | Variant    | Glyph | FACS AU              | Meaning                          |
/// |-------|------------|-------|----------------------|----------------------------------|
/// | 0     | Withheld   | `_`   | AU 22 (Lip Funneler) | Silence, stoicism, data collection|
/// | 1     | Assertive  | `v`   | AU 27 (Mouth Stretch)| Directional input, command       |
/// | 2     | Playful    | `~`   | AU 12 (Lip Puller)   | Irony, creativity, non-critical  |
/// | 3     | Thoughtful | `-`   | AU 15 (Lip Depressor)| Processing, evaluation, concern  |
/// | 4     | Hesitant   | `.`   | Relaxed/uncertain    | Low confidence, uncertainty      |
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Action {
    /// Value 0 — `_` — Silence, data collection, or stoicism.
    ///
    /// The withheld action. Represents an agent that is not outputting —
    /// listening, collecting data, or maintaining stoic silence. Maps to
    /// FACS AU 22 (Lip Funneler): the pursed, closed mouth of withheld
    /// expression.
    ///
    /// **When to use:** Listening mode, data collection, stoic observation,
    /// default/idle state, processing without output.
    Withheld = 0,

    /// Value 1 — `v` — Directional input, command, or confidence.
    ///
    /// The assertive action. Represents an agent making a definitive
    /// statement — giving a command, asserting a direction, or expressing
    /// confidence. Maps to FACS AU 27 (Mouth Stretch): the open, directed
    /// mouth of assertive speech.
    ///
    /// **When to use:** Commands, assertions, confident statements,
    /// directional guidance, authoritative output.
    Assertive = 1,

    /// Value 2 — `~` — Irony, creativity, or non-critical interaction.
    ///
    /// The playful action. Represents an agent in a playful or creative
    /// mode — irony, humor, non-critical interaction. Maps to FACS
    /// AU 12 (Lip Corner Puller): the asymmetric, wry mouth of playful
    /// expression.
    ///
    /// **When to use:** Humor, irony, creative suggestions, non-critical
    /// interaction, Socratic playfulness.
    Playful = 2,

    /// Value 3 — `-` — Processing, evaluation, or mild concern.
    ///
    /// The thoughtful action. Represents an agent in a processing or
    /// evaluative mode — thinking through something, expressing mild
    /// concern, or carefully considering. Maps to FACS AU 15 (Lip
    /// Corner Depressor): the tight, thoughtful mouth of careful
    /// consideration.
    ///
    /// **When to use:** Evaluation, analysis, mild concern, careful
    /// consideration, Socratic questioning, reflective depth.
    Thoughtful = 3,

    /// Value 4 — `.` — Low confidence, uncertainty, or error-checking.
    ///
    /// The hesitant action. Represents an agent expressing low confidence
    /// or uncertainty — hedging, error-checking, or expressing doubt.
    /// Maps to a relaxed/uncertain mouth posture with no specific FACS
    /// AU activation.
    ///
    /// **When to use:** Uncertainty, low confidence, error-checking,
    /// hedging, requests for clarification, vulnerability.
    Hesitant = 4,
}

// ── Constants ────────────────────────────────────────────────────────────────

impl Action {
    /// Total number of Action variants (5).
    pub const COUNT: usize = 5;
}

// ── Conversions ──────────────────────────────────────────────────────────────

impl Action {
    /// Convert a raw byte to an Action using modular arithmetic.
    ///
    /// Any byte value maps to a valid Action: `byte % 5`.
    pub const fn from_byte(byte: u8) -> Self {
        match byte % Self::COUNT as u8 {
            0 => Self::Withheld,
            1 => Self::Assertive,
            2 => Self::Playful,
            3 => Self::Thoughtful,
            _ => Self::Hesitant,
        }
    }
}

// ── Glyph Access ─────────────────────────────────────────────────────────────

impl Action {
    /// Get the ASCII mouth character for this Action.
    ///
    /// This is the center character in the 5-character FACES render:
    ///
    /// ```text
    /// [Container-Left][Focus-Left][Action][Focus-Right][Container-Right]
    /// ```
    pub const fn glyph(&self) -> &'static str {
        match self {
            Self::Withheld => "_",
            Self::Assertive => "v",
            Self::Playful => "~",
            Self::Thoughtful => "-",
            Self::Hesitant => ".",
        }
    }

    /// Get the FACS Action Unit description for this Action.
    pub const fn facs_mapping(&self) -> &'static str {
        match self {
            Self::Withheld => "AU 22 (Lip Funneler)",
            Self::Assertive => "AU 27 (Mouth Stretch)",
            Self::Playful => "AU 12 (Lip Corner Puller)",
            Self::Thoughtful => "AU 15 (Lip Corner Depressor)",
            Self::Hesitant => "Relaxed/uncertain (no active AU)",
        }
    }

    /// Get a human-readable description of this Action's semantic meaning.
    pub const fn describe(&self) -> &'static str {
        match self {
            Self::Withheld => "withheld (silence/data collection/stoicism)",
            Self::Assertive => "assertive (directional input/command/confidence)",
            Self::Playful => "playful (irony/creativity/non-critical)",
            Self::Thoughtful => "thoughtful (processing/evaluation/concern)",
            Self::Hesitant => "hesitant (low confidence/uncertainty)",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_is_five() {
        assert_eq!(Action::COUNT, 5);
    }

    #[test]
    fn test_from_byte_all_variants() {
        assert_eq!(Action::from_byte(0), Action::Withheld);
        assert_eq!(Action::from_byte(1), Action::Assertive);
        assert_eq!(Action::from_byte(2), Action::Playful);
        assert_eq!(Action::from_byte(3), Action::Thoughtful);
        assert_eq!(Action::from_byte(4), Action::Hesitant);
        // Wraps around
        assert_eq!(Action::from_byte(5), Action::Withheld);
        // 255 % 5 = 0 → Withheld (not Hesitant)
        assert_eq!(Action::from_byte(255), Action::Withheld);
    }

    #[test]
    fn test_glyph() {
        assert_eq!(Action::Withheld.glyph(), "_");
        assert_eq!(Action::Assertive.glyph(), "v");
        assert_eq!(Action::Playful.glyph(), "~");
        assert_eq!(Action::Thoughtful.glyph(), "-");
        assert_eq!(Action::Hesitant.glyph(), ".");
    }

    #[test]
    fn test_facs_mapping_non_empty() {
        for variant in [
            Action::Withheld,
            Action::Assertive,
            Action::Playful,
            Action::Thoughtful,
            Action::Hesitant,
        ] {
            assert!(!variant.facs_mapping().is_empty());
        }
    }

    #[test]
    fn test_describe_non_empty() {
        for variant in [
            Action::Withheld,
            Action::Assertive,
            Action::Playful,
            Action::Thoughtful,
            Action::Hesitant,
        ] {
            assert!(!variant.describe().is_empty());
        }
    }

    #[test]
    fn test_discriminant_values() {
        assert_eq!(Action::Withheld as u8, 0);
        assert_eq!(Action::Assertive as u8, 1);
        assert_eq!(Action::Playful as u8, 2);
        assert_eq!(Action::Thoughtful as u8, 3);
        assert_eq!(Action::Hesitant as u8, 4);
    }
}
