// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/focus.rs
// PURPOSE:     Focus shapes (Byte 2) — eye/attention state
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE FOCUS (Byte 2)
//
// The Focus defines the attentional state and intensity of the FACES
// primitive. It is the center focal point — the "eyes" of the ASCII
// face. In linguistic terms, the Focus maps to the ADVERB (verb
// modifier): how the action is performed, detailing intensity,
// directional focus, processing load, or vulnerability.
//
// PAUL EKMAN'S FACS — FACIAL ACTION CODING SYSTEM MAPPING
//
// Empirical Action Units (AUs) from Ekman's FACS are mapped to Focus
// parameters. FACES mathematically condenses complex muscular
// biomechanics into a low-compute geometric representation:
//
//   AU 4 (Brow Lowerer) + AU 7 (Lid Tightener) → Intense Focus ><
//   AU 5 (Upper Lid Raiser) + Pupil Dilation   → Open Focus OO
//   AU 12 (Lip Corner Puller) — cross-mapped   → Happy Focus ^^
//   (Relaxed lids, low AU activity)             → Neutral Focus oo
//   (AU 43, Eye Closure / dissociation)         → Distant Focus ..
//   (AU 43 + AU 4, fatigue + strain)            → Tired Focus --
//
// COMMITTEE FRAMEWORK MAPPING
//
// The Focus maps to the "Body Channel" (Sensation) from the Committee
// framework, and to "Acquisition" (Ac) in the VAAM model. It controls
// sensory processing, attention tracking, cognitive load, and initial
// input acquisition.
//
// COGNITIVE LOAD INDICATORS
//
// The Focus state is the primary indicator of cognitive load in the
// FACES protocol:
//   - Neutral (oo): Baseline attention, standard processing
//   - Intense (><): High concentration, frustration, or strain
//   - Open (OO): Surprise, shock, or hyper-awareness (overload)
//   - Distant (..): Dissociation, background processing, or boredom
//   - Happy (^^): Success, validation, or social warmth (low load)
//   - Tired (--): Low energy, resource depletion, or sleep mode
//
// TEMPORAL DYNAMICS
//
// The Focus state is dynamic and responds to keystroke friction in
// real-time. As characters are typed, the speed and rhythm act as
// physical friction that shifts the Focus parameter, visualizing
// cognitive processing load. When typing ceases, the expression
// decays back to baseline (Neutral) over a 5-second window.
//
// ═══════════════════════════════════════════════════════════════════════════════

// ── Focus Enum ───────────────────────────────────────────────────────────────

/// The 6 Focus shapes of the FACES protocol (Byte 2).
///
/// Each variant represents an attentional/eye state, mapped to
/// FACS Action Units, a pair of ASCII eye characters, and a
/// semantic meaning.
///
/// # Variants
///
/// | Value | Variant   | Glyph | FACS AU              | Meaning                          |
/// |-------|-----------|-------|----------------------|----------------------------------|
/// | 0     | Neutral   | `oo`  | Baseline             | Standard observation             |
/// | 1     | Intense   | `><`  | AU 4 + AU 7          | High concentration, strain       |
/// | 2     | Open      | `OO`  | AU 5 + Dilation      | Surprise, hyper-awareness        |
/// | 3     | Distant   | `..`  | AU 43 (partial)      | Dissociation, boredom            |
/// | 4     | Happy     | `^^`  | AU 12 (cross-map)    | Success, social warmth           |
/// | 5     | Tired     | `--`  | AU 43 + AU 4         | Low energy, depletion            |
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Focus {
    /// Value 0 — `oo` — Standard observation; baseline attention.
    ///
    /// The neutral focus. Represents an agent at baseline attention —
    /// observing, processing normally, not under strain. No specific
    /// FACS AUs are active beyond baseline muscle tone.
    ///
    /// **When to use:** Default state, normal processing, listening.
    Neutral = 0,

    /// Value 1 — `><` — High concentration, frustration, or strain.
    ///
    /// The intense focus. Represents an agent under high cognitive load —
    /// concentrating hard, frustrated, or straining. Maps to FACS
    /// AU 4 (Brow Lowerer) + AU 7 (Lid Tightener): the squinted,
    /// piercing eye of focused effort.
    ///
    /// **When to use:** Complex problem-solving, compilation, error
    /// analysis, debugging, high-load processing.
    Intense = 1,

    /// Value 2 — `OO` — Surprise, shock, or hyper-awareness.
    ///
    /// The open focus. Represents an agent in a state of surprise or
    /// hyper-awareness — eyes wide open, taking in everything. Maps to
    /// FACS AU 5 (Upper Lid Raiser) + Pupil Dilation: the wide-eyed
    /// response to unexpected input.
    ///
    /// In Brené Brown's framework, open eyes 'OO' paired with porous
    /// braces '{}' represent vulnerable exposure — the willingness to
    /// be surprised and affected.
    ///
    /// **When to use:** Unexpected results, new information, surprise,
    /// vulnerability, hyper-awareness.
    Open = 2,

    /// Value 3 — `..` — Dissociation, background processing, or boredom.
    ///
    /// The distant focus. Represents an agent that has dissociated from
    /// the immediate interaction — either processing something in the
    /// background, or simply bored. Maps to FACS AU 43 (Eye Closure,
    /// partial): the unfocused, "thousand-yard stare" of background
    /// processing.
    ///
    /// **When to use:** Background tasks, waiting states, dissociation,
    /// low-engagement periods.
    Distant = 3,

    /// Value 4 — `^^` — Success, validation, or social warmth.
    ///
    /// The happy focus. Represents an agent experiencing success,
    /// validation, or social warmth — the "crinkled eyes" of genuine
    /// positive emotion. Cross-mapped from FACS AU 12 (Lip Corner
    /// Puller) which is technically a mouth AU, but the crinkled-eye
    /// appearance is the universally recognized signal of genuine
    /// happiness (the Duchenne smile).
    ///
    /// **When to use:** Success, completion, positive feedback,
    /// celebration, social warmth.
    Happy = 4,

    /// Value 5 — `--` — Low energy, resource depletion, or sleep mode.
    ///
    /// The tired focus. Represents an agent with low energy — resource
    /// depletion, fatigue, or entering sleep/low-power mode. Maps to
    /// FACS AU 43 (Eye Closure) + AU 4 (Brow Lowerer): the heavy-lidded,
    /// strained look of exhaustion.
    ///
    /// **When to use:** Low battery, resource limits, long-running tasks,
    /// system fatigue, sleep mode.
    Tired = 5,
}

// ── Constants ────────────────────────────────────────────────────────────────

impl Focus {
    /// Total number of Focus variants (6).
    pub const COUNT: usize = 6;
}

// ── Conversions ──────────────────────────────────────────────────────────────

impl Focus {
    /// Convert a raw byte to a Focus using modular arithmetic.
    ///
    /// Any byte value maps to a valid Focus: `byte % 6`.
    pub const fn from_byte(byte: u8) -> Self {
        match byte % Self::COUNT as u8 {
            0 => Self::Neutral,
            1 => Self::Intense,
            2 => Self::Open,
            3 => Self::Distant,
            4 => Self::Happy,
            _ => Self::Tired,
        }
    }
}

// ── Glyph Access ─────────────────────────────────────────────────────────────

impl Focus {
    /// Get the left and right ASCII eye characters for this Focus.
    ///
    /// Returns a tuple of `(&'static str, &'static str)` representing
    /// the left and right eyes. These appear inside the Container brackets
    /// in the 5-character FACES render:
    ///
    /// ```text
    /// [Container-Left][Focus-Left][Action][Focus-Right][Container-Right]
    /// ```
    pub const fn glyphs(&self) -> (&'static str, &'static str) {
        match self {
            Self::Neutral => ("o", "o"),
            Self::Intense => (">", "<"),
            Self::Open => ("O", "O"),
            Self::Distant => (".", "."),
            Self::Happy => ("^", "^"),
            Self::Tired => ("-", "-"),
        }
    }

    /// Get the FACS Action Unit description for this Focus.
    ///
    /// Returns the Paul Ekman Facial Action Coding System mapping
    /// as a human-readable string.
    pub const fn facs_mapping(&self) -> &'static str {
        match self {
            Self::Neutral => "Baseline (no active AUs)",
            Self::Intense => "AU 4 (Brow Lowerer) + AU 7 (Lid Tightener)",
            Self::Open => "AU 5 (Upper Lid Raiser) + Pupil Dilation",
            Self::Distant => "AU 43 (Eye Closure, partial)",
            Self::Happy => "AU 12 cross-map (Duchenne crinkle)",
            Self::Tired => "AU 43 + AU 4 (fatigue + strain)",
        }
    }

    /// Get a human-readable description of this Focus's semantic meaning.
    pub const fn describe(&self) -> &'static str {
        match self {
            Self::Neutral => "neutral (standard observation/baseline)",
            Self::Intense => "intense (high concentration/strain)",
            Self::Open => "open (surprise/hyper-awareness)",
            Self::Distant => "distant (dissociation/background processing)",
            Self::Happy => "happy (success/social warmth)",
            Self::Tired => "tired (low energy/depletion)",
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
    fn test_count_is_six() {
        assert_eq!(Focus::COUNT, 6);
    }

    #[test]
    fn test_from_byte_all_variants() {
        assert_eq!(Focus::from_byte(0), Focus::Neutral);
        assert_eq!(Focus::from_byte(1), Focus::Intense);
        assert_eq!(Focus::from_byte(2), Focus::Open);
        assert_eq!(Focus::from_byte(3), Focus::Distant);
        assert_eq!(Focus::from_byte(4), Focus::Happy);
        assert_eq!(Focus::from_byte(5), Focus::Tired);
        // Wraps around
        assert_eq!(Focus::from_byte(6), Focus::Neutral);
        // 255 % 6 = 3 → Distant (not Tired)
        assert_eq!(Focus::from_byte(255), Focus::Distant);
    }

    #[test]
    fn test_glyphs() {
        assert_eq!(Focus::Neutral.glyphs(), ("o", "o"));
        assert_eq!(Focus::Intense.glyphs(), (">", "<"));
        assert_eq!(Focus::Open.glyphs(), ("O", "O"));
        assert_eq!(Focus::Distant.glyphs(), (".", "."));
        assert_eq!(Focus::Happy.glyphs(), ("^", "^"));
        assert_eq!(Focus::Tired.glyphs(), ("-", "-"));
    }

    #[test]
    fn test_facs_mapping_non_empty() {
        for variant in [
            Focus::Neutral,
            Focus::Intense,
            Focus::Open,
            Focus::Distant,
            Focus::Happy,
            Focus::Tired,
        ] {
            assert!(!variant.facs_mapping().is_empty());
        }
    }

    #[test]
    fn test_describe_non_empty() {
        for variant in [
            Focus::Neutral,
            Focus::Intense,
            Focus::Open,
            Focus::Distant,
            Focus::Happy,
            Focus::Tired,
        ] {
            assert!(!variant.describe().is_empty());
        }
    }

    #[test]
    fn test_discriminant_values() {
        assert_eq!(Focus::Neutral as u8, 0);
        assert_eq!(Focus::Intense as u8, 1);
        assert_eq!(Focus::Open as u8, 2);
        assert_eq!(Focus::Distant as u8, 3);
        assert_eq!(Focus::Happy as u8, 4);
        assert_eq!(Focus::Tired as u8, 5);
    }
}
