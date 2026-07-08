// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/container.rs
// PURPOSE:     Container shapes (Byte 1) — head boundary geometry
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE CONTAINER (Byte 1)
//
// The Container defines the physical boundary and temperament of the
// FACES primitive. It is the outermost geometric bracket — the "head
// shape" that contains the eyes and mouth. In linguistic terms, the
// Container maps to the NOUN (subject): the entity, boundary, structure,
// and physical identity of the agent.
//
// MIAN XIANG (面相) — CHINESE FACE READING MAPPING
//
// The FACES protocol maps the Five Elements system of Chinese face
// reading (Mian Xiang) directly to Container geometries. This honors
// Taoist Qi and somatic boundaries, establishing FACES as a modern
// engineering translation of ancient physiognomic wisdom:
//
//   Element   Shape              Container    Value   Meaning
//   ───────   ─────              ────────    ─────   ───────
//   Metal     Oval/structured    ()           0       Open, balanced, receptive
//   Earth     Square/solid       []           1       Formal, logical, constrained
//   Water     Soft/round/curly   {}           2       Creative, adaptive, unstable
//   Wood      Long/rectangular   ||           3       Protected, cautious, defensive
//   Fire      Pointed/triangular <>           4       Aggressive, urgent, high-priority
//
// COMMITTEE FRAMEWORK MAPPING
//
// The Container maps to the "Mind Channel" (Logic) from Joshua's
// philosophical treatise 'The Great Game', and to "Autonomy" (A) in
// the VAAM pedagogical model. It governs cognitive boundaries,
// structural constraints, self-determination, and rule-based schemas.
//
// BRENNÉ BROWN — VULNERABILITY & SHAME RESILIENCE
//
// The protocol maps vulnerability directly to physical geometry:
//   - Vulnerable exposure: open eyes 'OO' + porous braces '{}' (Water)
//   - Defensiveness/shielding: rigid brackets '[]' (Earth) or
//     walled pipes '||' (Wood) or squinted eyes '><' (Intense Focus)
//   - Shame resilience and boundary-setting: structural shifts in the
//     Container shape provide a mechanical representation of emotional
//     boundaries
//
// ALAN WATTS — ZEN AND FLUIDITY
//
// FACES contrasts cognitive rigidity with a fluid ego. Drawing from
// Watts' concept of the "middle way" and the acceptance of contradictions,
// the protocol utilizes fluid curly braces '{}' (Water) for creative,
// adaptive states. This framework prioritizes continuous, flowing
// geometric transitions over static, binary state-switching.
//
// ═══════════════════════════════════════════════════════════════════════════════

// ── Container Enum ───────────────────────────────────────────────────────────

/// The 5 Container shapes of the FACES protocol (Byte 1).
///
/// Each variant represents a cognitive boundary state, mapped to
/// Mian Xiang Five Elements, a pair of ASCII bracket characters,
/// and a semantic meaning.
///
/// # Variants
///
/// | Value | Variant   | Glyph | Element | Meaning                          |
/// |-------|-----------|-------|---------|----------------------------------|
/// | 0     | Neutral   | `()`  | Metal   | Open, balanced, receptive        |
/// | 1     | Rigid     | `[]`  | Earth   | Formal, logical, constrained     |
/// | 2     | Fluid     | `{}`  | Water   | Creative, adaptive, unstable     |
/// | 3     | Defensive | `\|\|`| Wood    | Protected, cautious, high-security|
/// | 4     | Sharp     | `<>`  | Fire    | Aggressive, urgent, high-priority|
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Container {
    /// Value 0 — Metal Element — `()` — Open, balanced, and receptive state.
    ///
    /// The neutral container. Represents an agent in a default, open
    /// posture — willing to receive input, not defensive, not aggressive.
    /// Maps to the Metal element in Mian Xiang: oval/structured,
    /// representing symmetry and focus.
    ///
    /// **When to use:** Default state, listening mode, baseline interaction.
    Neutral = 0,

    /// Value 1 — Earth Element — `[]` — Formal, logical, or constrained state.
    ///
    /// The rigid container. Represents an agent in a formal or rule-bound
    /// posture — following protocols, enforcing constraints, operating
    /// within defined schemas. Maps to the Earth element in Mian Xiang:
    /// square/solid, representing stability and assertiveness.
    ///
    /// **When to use:** System prompts, guardrails, protocol enforcement,
    /// formal instruction, rule-based decision making.
    Rigid = 1,

    /// Value 2 — Water Element — `{}` — Creative, adaptive, or unstable state.
    ///
    /// The fluid container. Represents an agent in a creative or
    /// adaptive posture — generating novel ideas, flowing between
    /// concepts, open to contradiction. Maps to the Water element in
    /// Mian Xiang: soft/round/curly, representing adaptability and depth.
    ///
    /// Drawing from Alan Watts' Zen philosophy, the fluid braces represent
    /// the "middle way" — acceptance of contradictions and continuous,
    /// flowing transitions. Brené Brown maps vulnerability to this
    /// porous, open boundary state.
    ///
    /// **When to use:** Creative work, brainstorming, Socratic exploration,
    /// vulnerability, adaptive responses.
    Fluid = 2,

    /// Value 3 — Wood Element — `||` — Protected, cautious, or high-security state.
    ///
    /// The defensive container. Represents an agent in a protective
    /// posture — shielding against threats, maintaining boundaries,
    /// operating in high-security mode. Maps to the Wood element in
    /// Mian Xiang: long/rectangular, representing growth and vision
    /// (but also defensive walls).
    ///
    /// **When to use:** Error handling, security warnings, boundary
    /// enforcement, defensive responses, system protection.
    Defensive = 3,

    /// Value 4 — Fire Element — `<>` — Aggressive, urgent, or high-priority state.
    ///
    /// The sharp container. Represents an agent in an aggressive or
    /// urgent posture — high-priority alerts, critical warnings,
    /// forceful assertions. Maps to the Fire element in Mian Xiang:
    /// pointed/triangular, representing dynamic energy and passion.
    ///
    /// **When to use:** Critical errors, urgent warnings, high-priority
    /// alerts, forceful assertions, system-critical states.
    Sharp = 4,
}

// ── Constants ────────────────────────────────────────────────────────────────

impl Container {
    /// Total number of Container variants (5).
    pub const COUNT: usize = 5;
}

// ── Conversions ──────────────────────────────────────────────────────────────

impl Container {
    /// Convert a raw byte to a Container using modular arithmetic.
    ///
    /// Any byte value maps to a valid Container: `byte % 5`.
    /// This ensures that arbitrary byte streams (from network packets,
    /// telemetry logs, or hardware registers) always produce valid states.
    pub const fn from_byte(byte: u8) -> Self {
        match byte % Self::COUNT as u8 {
            0 => Self::Neutral,
            1 => Self::Rigid,
            2 => Self::Fluid,
            3 => Self::Defensive,
            _ => Self::Sharp,
        }
    }
}

// ── Glyph Access ─────────────────────────────────────────────────────────────

impl Container {
    /// Get the left and right ASCII bracket characters for this Container.
    ///
    /// Returns a tuple of `(&'static str, &'static str)` representing
    /// the opening and closing brackets. These are the outermost
    /// characters in the 5-character FACES render:
    ///
    /// ```text
    /// [Container-Left][Focus-Left][Action][Focus-Right][Container-Right]
    /// ```
    pub const fn glyphs(&self) -> (&'static str, &'static str) {
        match self {
            Self::Neutral => ("(", ")"),
            Self::Rigid => ("[", "]"),
            Self::Fluid => ("{", "}"),
            Self::Defensive => ("|", "|"),
            Self::Sharp => ("<", ">"),
        }
    }

    /// Get the Mian Xiang element name for this Container.
    ///
    /// Returns the Chinese face reading (面相) Five Elements mapping:
    /// Metal, Earth, Water, Wood, or Fire.
    pub const fn element(&self) -> &'static str {
        match self {
            Self::Neutral => "Metal",
            Self::Rigid => "Earth",
            Self::Fluid => "Water",
            Self::Defensive => "Wood",
            Self::Sharp => "Fire",
        }
    }

    /// Get a human-readable description of this Container's semantic meaning.
    ///
    /// Used for debugging, logging, accessibility (screen readers),
    /// and the `FacesState::describe()` method.
    pub const fn describe(&self) -> &'static str {
        match self {
            Self::Neutral => "neutral (open/balanced/receptive)",
            Self::Rigid => "rigid (formal/logical/constrained)",
            Self::Fluid => "fluid (creative/adaptive/unstable)",
            Self::Defensive => "defensive (protected/cautious/high-security)",
            Self::Sharp => "sharp (aggressive/urgent/high-priority)",
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
        assert_eq!(Container::COUNT, 5);
    }

    #[test]
    fn test_from_byte_neutral() {
        assert_eq!(Container::from_byte(0), Container::Neutral);
        assert_eq!(Container::from_byte(5), Container::Neutral);
        assert_eq!(Container::from_byte(250), Container::Neutral);
    }

    #[test]
    fn test_from_byte_rigid() {
        assert_eq!(Container::from_byte(1), Container::Rigid);
        assert_eq!(Container::from_byte(6), Container::Rigid);
        assert_eq!(Container::from_byte(251), Container::Rigid);
    }

    #[test]
    fn test_from_byte_fluid() {
        assert_eq!(Container::from_byte(2), Container::Fluid);
        assert_eq!(Container::from_byte(7), Container::Fluid);
    }

    #[test]
    fn test_from_byte_defensive() {
        assert_eq!(Container::from_byte(3), Container::Defensive);
        assert_eq!(Container::from_byte(8), Container::Defensive);
    }

    #[test]
    fn test_from_byte_sharp() {
        assert_eq!(Container::from_byte(4), Container::Sharp);
        assert_eq!(Container::from_byte(9), Container::Sharp);
        // 255 % 5 = 0 → Neutral (not Sharp)
        assert_eq!(Container::from_byte(255), Container::Neutral);
    }

    #[test]
    fn test_glyphs() {
        assert_eq!(Container::Neutral.glyphs(), ("(", ")"));
        assert_eq!(Container::Rigid.glyphs(), ("[", "]"));
        assert_eq!(Container::Fluid.glyphs(), ("{", "}"));
        assert_eq!(Container::Defensive.glyphs(), ("|", "|"));
        assert_eq!(Container::Sharp.glyphs(), ("<", ">"));
    }

    #[test]
    fn test_element_mapping() {
        assert_eq!(Container::Neutral.element(), "Metal");
        assert_eq!(Container::Rigid.element(), "Earth");
        assert_eq!(Container::Fluid.element(), "Water");
        assert_eq!(Container::Defensive.element(), "Wood");
        assert_eq!(Container::Sharp.element(), "Fire");
    }

    #[test]
    fn test_describe_non_empty() {
        for variant in [
            Container::Neutral,
            Container::Rigid,
            Container::Fluid,
            Container::Defensive,
            Container::Sharp,
        ] {
            assert!(!variant.describe().is_empty());
        }
    }

    #[test]
    fn test_discriminant_values() {
        assert_eq!(Container::Neutral as u8, 0);
        assert_eq!(Container::Rigid as u8, 1);
        assert_eq!(Container::Fluid as u8, 2);
        assert_eq!(Container::Defensive as u8, 3);
        assert_eq!(Container::Sharp as u8, 4);
    }
}
