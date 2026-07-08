// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/protocol.rs
// PURPOSE:     The 4-byte FACES state — encoding, decoding, serialization
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE 4-BYTE PAYLOAD STRUCTURE
//
// The FACES state is encapsulated in a 32-bit (4-byte) structure, allowing
// for extreme portability and integration into standard network packets,
// MCP messages, telemetry logs, and hardware registers.
//
//   ┌────────┬──────────┬─────────┬─────────┐
//   │ Byte 0 │ Byte 1   │ Byte 2  │ Byte 3  │
//   │ Aura   │ Container│ Focus   │ Action  │
//   │ 8-bit  │ 5 values │ 6 values│ 5 values│
//   │ 0-255  │ 0-4      │ 0-5     │ 0-4     │
//   └────────┴──────────┴─────────┴─────────┘
//
//   Total unique states: 256 × 5 × 6 × 5 = 38,400
//
// BYTE DETAILS:
//
//   Byte 0 — Aura (8-bit, 0-255):
//     ANSI 256-color index representing general mood, vibe, feeling,
//     and biological state. Maps to the "Heart Channel" (Emotion) in
//     the Committee framework and to "Mastery" (M) in VAAM.
//     Linguistic mapping: The Adjective (noun modifier) — qualitative
//     state, tone, temperature, emotional atmosphere.
//
//   Byte 1 — Container (5 values, 0-4):
//     Defines the physical boundary and temperament of the primitive.
//     The head shape — the outermost geometric bracket. Maps to the
//     "Mind Channel" (Logic) in the Committee framework and to
//     "Autonomy" (A) in VAAM. Also maps to Mian Xiang Five Elements.
//     Linguistic mapping: The Noun (subject) — entity, boundary,
//     structure, physical identity of the agent.
//
//   Byte 2 — Focus (6 values, 0-5):
//     Defines the attentional state and intensity of the primitive.
//     The eyes — the center focal point. Maps to the "Body Channel"
//     (Sensation) in the Committee framework and to "Acquisition" (Ac)
//     in VAAM. Also maps to FACS Action Units.
//     Linguistic mapping: The Adverb (verb modifier) — how the action
//     is performed, intensity, directional focus, processing load.
//
//   Byte 3 — Action (5 values, 0-4):
//     Defines the communicative intent or mechanical output state.
//     The mouth — the expression below the eyes. Maps to the "Action
//     Channel" (Will) in the Committee framework and to "Vocabulary"
//     (V) in VAAM.
//     Linguistic mapping: The Verb (action/predicate) — kinetic
//     readiness, dynamic output, expressiveness.
//
// THE 5-CHARACTER MATRIX
//
// The protocol maps the 4-byte model to exactly 5 monospace terminal
// characters, typically wrapped in an 8-bit ANSI escape sequence for
// colorization. The string length remains constant regardless of
// emotional complexity:
//
//   [Container-Left][Focus-Left][Action][Focus-Right][Container-Right]
//
//   Example: (oo_)  — Neutral container, neutral focus, withheld action
//            {^^~}  — Fluid container, happy focus, playful action
//            [<><v] — Sharp container, intense focus, assertive action
//
// ENCODING
//
// For network transmission and storage, the state is encoded as 4 raw
// bytes. The Aura byte uses the full 0-255 range. Container, Focus,
// and Action bytes use modular arithmetic (% 5, % 6, % 5 respectively)
// so any byte value maps to a valid state — this is by design, allowing
// the protocol to accept arbitrary byte streams without validation errors.
//
// For human-readable display, the state is encoded as an 8-character
// uppercase hex string: AACCFFAA (Aura, Container, Focus, Action).
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::action::Action;
use crate::aura::Aura;
use crate::container::Container;
use crate::focus::Focus;

// ── FacesState ───────────────────────────────────────────────────────────────

/// The complete FACES protocol state — a 4-byte emotive payload.
///
/// This struct is the primary type of the FACES protocol. It encodes
/// a complete emotional/intent state in 32 bits, representing:
/// - **Aura**: Background mood/atmosphere (256 colors)
/// - **Container**: Cognitive boundary/temperament (5 shapes)
/// - **Focus**: Attentional state/intensity (6 eye shapes)
/// - **Action**: Communicative intent/output (5 mouth shapes)
///
/// Total unique states: 256 × 5 × 6 × 5 = **38,400**
///
/// # Example
///
/// ```
/// use faces_protocol::FacesState;
/// use faces_protocol::Container;
/// use faces_protocol::Focus;
/// use faces_protocol::Action;
/// use faces_protocol::Aura;
///
/// // Create a "playful, creative, happy" state
/// let state = FacesState::new(
///     Aura::from_index(93), // bright green — growth/creativity
///     Container::Fluid,     // {} — creative, adaptive
///     Focus::Happy,         // ^^ — success, warmth
///     Action::Playful,      // ~  — irony, creativity
/// );
///
/// // Render as ASCII with ANSI color
/// let rendered = state.render(); // "{^^~}" in green
///
/// // Encode as hex for transmission
/// let hex = state.to_hex(); // "5D0242 02" (8 chars)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FacesState {
    /// Byte 0 — Aura: 8-bit ANSI color index (0-255).
    /// Represents mood, atmosphere, biological state.
    /// Maps to: Heart Channel (Emotion), Mastery (M), Adjective.
    pub aura: Aura,

    /// Byte 1 — Container: head shape boundary (5 variants).
    /// Represents cognitive boundary, temperament, self-determination.
    /// Maps to: Mind Channel (Logic), Autonomy (A), Noun, Mian Xiang Elements.
    pub container: Container,

    /// Byte 2 — Focus: eye/attention state (6 variants).
    /// Represents sensory processing, attention, cognitive load.
    /// Maps to: Body Channel (Sensation), Acquisition (Ac), Adverb, FACS AUs.
    pub focus: Focus,

    /// Byte 3 — Action: mouth/expression state (5 variants).
    /// Represents communicative intent, kinetic output, verbalization.
    /// Maps to: Action Channel (Will), Vocabulary (V), Verb.
    pub action: Action,
}

impl FacesState {
    /// Create a new FACES state from explicit component values.
    ///
    /// This is the primary constructor. Each component is a typed enum
    /// (except Aura, which is an 8-bit index wrapper) ensuring that
    /// only valid protocol states can be constructed at the type level.
    pub const fn new(aura: Aura, container: Container, focus: Focus, action: Action) -> Self {
        Self {
            aura,
            container,
            focus,
            action,
        }
    }

    /// Create a FACES state from 4 raw bytes.
    ///
    /// Uses modular arithmetic to map any byte value to a valid state:
    /// - Aura: byte 0 used directly (0-255)
    /// - Container: byte 1 % 5 (0-4)
    /// - Focus: byte 2 % 6 (0-5)
    /// - Action: byte 3 % 5 (0-4)
    ///
    /// This means any 4-byte sequence is a valid FACES state — by design,
    /// allowing the protocol to accept arbitrary byte streams without
    /// validation errors. This is critical for telemetry logging and
    /// hardware register compatibility.
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self {
            aura: Aura::from_index(bytes[0]),
            container: Container::from_byte(bytes[1]),
            focus: Focus::from_byte(bytes[2]),
            action: Action::from_byte(bytes[3]),
        }
    }

    /// Encode the FACES state as 4 raw bytes.
    ///
    /// The inverse of `from_bytes`. The Aura byte is the direct index.
    /// Container, Focus, and Action bytes are the enum discriminant values.
    pub const fn to_bytes(&self) -> [u8; 4] {
        [
            self.aura.index(),
            self.container as u8,
            self.focus as u8,
            self.action as u8,
        ]
    }

    /// Pack the FACES state into a single `u32` for hardware register access.
    ///
    /// Layout: `[Aura (8) | Container (8) | Focus (8) | Action (8)]`
    /// in big-endian byte order (Aura is most-significant byte).
    ///
    /// This enables direct read/write to a 32-bit hardware register on
    /// NPU targets, CAN bus frames, or any system that expects a single
    /// u32 word. The packing is trivial (4 bytes to 1 u32) but having it
    /// as a named method makes the intent explicit at call sites.
    ///
    /// # Example
    ///
    /// ```
    /// use faces_protocol::FacesState;
    ///
    /// let state = FacesState::neutral();
    /// let packed = state.to_u32();
    /// let unpacked = FacesState::from_u32(packed);
    /// assert_eq!(state, unpacked);
    /// ```
    pub const fn to_u32(&self) -> u32 {
        let bytes = self.to_bytes();
        ((bytes[0] as u32) << 24)
            | ((bytes[1] as u32) << 16)
            | ((bytes[2] as u32) << 8)
            | (bytes[3] as u32)
    }

    /// Unpack a `u32` into a FACES state.
    ///
    /// The inverse of `to_u32`. Extracts 4 bytes from the u32 in
    /// big-endian order and passes them through `from_bytes`, which
    /// applies modular arithmetic to ensure validity.
    ///
    /// Any `u32` value is a valid FACES state by design, allowing
    /// the protocol to accept arbitrary register values without
    /// validation errors.
    pub const fn from_u32(packed: u32) -> Self {
        Self::from_bytes([
            (packed >> 24) as u8,
            (packed >> 16) as u8,
            (packed >> 8) as u8,
            packed as u8,
        ])
    }

    /// Encode the FACES state as an 8-character uppercase hex string.
    ///
    /// Format: `AACCFFAA` where:
    /// - `AA` = Aura byte (2 hex chars)
    /// - `CC` = Container byte (2 hex chars)
    /// - `FF` = Focus byte (2 hex chars)
    /// - `AA` = Action byte (2 hex chars)
    ///
    /// This format is used for:
    /// - Network packet payloads
    /// - Telemetry log entries
    /// - System prompt LLM output (compact, 8 chars)
    /// - Debug display
    pub fn to_hex(&self) -> String {
        let bytes = self.to_bytes();
        format!(
            "{:02X}{:02X}{:02X}{:02X}",
            bytes[0], bytes[1], bytes[2], bytes[3]
        )
    }

    /// Decode a FACES state from an 8-character hex string.
    ///
    /// Accepts both uppercase and lowercase hex. Returns `Err` if the
    /// string is not exactly 8 hex characters.
    ///
    /// # Example
    ///
    /// ```
    /// use faces_protocol::FacesState;
    ///
    /// let state = FacesState::from_hex("5D020402").unwrap();
    /// let hex = state.to_hex();
    /// assert_eq!(hex, "5D020402");
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        if hex.len() != 8 {
            return Err("Hex string must be exactly 8 characters");
        }

        let bytes = [0, 2, 4, 6]
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| "Invalid hex characters"));

        let bytes: [u8; 4] = match bytes {
            [Ok(a), Ok(b), Ok(c), Ok(d)] => [a, b, c, d],
            _ => return Err("Invalid hex characters"),
        };

        Ok(Self::from_bytes(bytes))
    }

    /// Render the FACES state as a 5-character ASCII string with ANSI color.
    ///
    /// This delegates to the `render` module, which produces the
    /// terminal-displayable string:
    ///
    /// ```text
    /// [Container-Left][Focus-Left][Action][Focus-Right][Container-Right]
    /// ```
    ///
    /// Wrapped in an ANSI 256-color escape sequence using the Aura byte.
    /// The output string length is constant regardless of emotional complexity.
    pub fn render(&self) -> String {
        crate::render::render_state(self)
    }

    /// Create a neutral/baseline FACES state.
    ///
    /// The default state is:
    /// - Aura: 245 (light gray — neutral, balanced)
    /// - Container: Neutral () — open, balanced, receptive
    /// - Focus: Neutral (oo) — standard observation, baseline attention
    /// - Action: Withheld (_) — silence, data collection, stoicism
    ///
    /// This is the "resting face" of the protocol — the state an agent
    /// returns to when idle or when temporal decay relaxes the expression
    /// back to baseline.
    pub const fn neutral() -> Self {
        Self::new(
            Aura::NEUTRAL,
            Container::Neutral,
            Focus::Neutral,
            Action::Withheld,
        )
    }

    // ── Safety Presets ───────────────────────────────────────────────────
    //
    // These are advisory states for physical AI systems. They encode
    // emotional/intent context that safety systems can use to modulate
    // behavior. They are NOT safety overrides — a safety system that
    // detects an obstacle must always stop regardless of FACES state.
    //
    // The mapping is:
    //   EMERGENCY       → maximum urgency, sharp boundary, intense focus,
    //                     assertive action. Signals "stop everything,
    //                     critical situation."
    //   LOW_POWER       → tired aura, neutral boundary, tired focus,
    //                     withheld action. Signals "reduced capacity,
    //                     minimal output, conserve resources."
    //   HUMAN_PROXIMITY → calm aura, fluid boundary, open focus,
    //                     thoughtful action. Signals "human nearby,
    //                     be gentle, receptive, non-threatening."

    /// Emergency preset — maximum urgency, sharp focus, assertive action.
    ///
    /// Use when a safety-critical situation is detected. The sharp
    /// container and intense focus signal "all attention on the
    /// critical issue." The assertive action signals "act now."
    /// The urgent red aura signals alarm.
    ///
    /// **Advisory only** — safety systems must override FACES state
    /// when physical safety is at stake.
    pub const EMERGENCY: Self = Self::new(
        Aura::URGENT,
        Container::Sharp,
        Focus::Intense,
        Action::Assertive,
    );

    /// Low power preset — reduced capacity, minimal output.
    ///
    /// Use when battery is low, compute is throttled, or the system
    /// is in a resource-conservation mode. The tired aura and tired
    /// focus signal "low energy." The withheld action signals
    /// "minimal output." The neutral container signals "no strong
    /// boundary assertion."
    pub const LOW_POWER: Self = Self::new(
        Aura::TIRED,
        Container::Neutral,
        Focus::Tired,
        Action::Withheld,
    );

    /// Human proximity preset — gentle, receptive, non-threatening.
    ///
    /// Use when a human is detected nearby. The calm aura signals
    /// "no alarm." The fluid container signals "open, adaptable."
    /// The open focus signals "receptive attention." The thoughtful
    /// action signals "considered, careful output."
    pub const HUMAN_PROXIMITY: Self = Self::new(
        Aura::CALM,
        Container::Fluid,
        Focus::Open,
        Action::Thoughtful,
    );
}

// ── Default Trait ────────────────────────────────────────────────────────────

impl Default for FacesState {
    /// The default FACES state is the neutral/baseline state.
    /// See [`FacesState::neutral`] for details.
    fn default() -> Self {
        Self::neutral()
    }
}

// ── Display Trait ────────────────────────────────────────────────────────────

impl std::fmt::Display for FacesState {
    /// Display the FACES state as a hex string.
    /// This enables `format!("{}", state)` to produce the 8-char hex.
    /// For ASCII rendering with color, use `state.render()` instead.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

// ── Serialization (manual, zero-dependency) ──────────────────────────────────

impl FacesState {
    /// Serialize to a JSON-compatible string representation.
    ///
    /// Returns a string like `"5D024002"` (the hex encoding).
    /// This is compatible with `serde_json` if serde is added later,
    /// but works standalone for logging and config files.
    pub fn to_json_string(&self) -> String {
        format!("\"{}\"", self.to_hex())
    }

    /// Get a human-readable description of the state.
    ///
    /// Useful for debugging, logging, and accessibility (screen readers).
    /// Returns a string like: "Aura: spring green (120), Container: fluid
    /// (creative/adaptive), Focus: happy (success/warmth), Action: playful
    /// (irony/creativity)"
    pub fn describe(&self) -> String {
        format!(
            "Aura: {}, Container: {}, Focus: {}, Action: {}",
            self.aura.describe(),
            self.container.describe(),
            self.focus.describe(),
            self.action.describe(),
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neutral_state() {
        let state = FacesState::neutral();
        assert_eq!(state.aura, Aura::NEUTRAL);
        assert_eq!(state.container, Container::Neutral);
        assert_eq!(state.focus, Focus::Neutral);
        assert_eq!(state.action, Action::Withheld);
    }

    #[test]
    fn test_default_equals_neutral() {
        assert_eq!(FacesState::default(), FacesState::neutral());
    }

    #[test]
    fn test_bytes_roundtrip() {
        let original = FacesState::new(
            Aura::from_index(93),
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let bytes = original.to_bytes();
        let decoded = FacesState::from_bytes(bytes);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_hex_roundtrip() {
        let original = FacesState::new(
            Aura::from_index(39),
            Container::Sharp,
            Focus::Intense,
            Action::Hesitant,
        );
        let hex = original.to_hex();
        assert_eq!(hex.len(), 8);
        let decoded = FacesState::from_hex(&hex).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_hex_lowercase_accepted() {
        // from_hex normalizes raw bytes to canonical enum discriminants.
        // 0x40 (64) for Focus → 64 % 6 = 4 = Happy → canonical byte 0x04
        let state = FacesState::from_hex("5d024002").unwrap();
        assert_eq!(state.to_hex(), "5D020402");
    }

    #[test]
    fn test_hex_invalid_length() {
        assert!(FacesState::from_hex("5D02").is_err());
        assert!(FacesState::from_hex("5D02400200").is_err());
    }

    #[test]
    fn test_hex_invalid_chars() {
        assert!(FacesState::from_hex("GG024002").is_err());
    }

    #[test]
    fn test_display_shows_hex() {
        let state = FacesState::neutral();
        let display = format!("{}", state);
        assert_eq!(display, state.to_hex());
    }

    #[test]
    fn test_describe_non_empty() {
        let state = FacesState::new(
            Aura::from_index(93),
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let desc = state.describe();
        assert!(desc.contains("fluid"));
        assert!(desc.contains("happy"));
        assert!(desc.contains("playful"));
    }

    #[test]
    fn test_modular_arithmetic_all_bytes_valid() {
        // Any byte value should produce a valid state (no panics)
        for b in 0..=255u8 {
            let _ = FacesState::from_bytes([b, b, b, b]);
        }
    }

    #[test]
    fn test_to_json_string() {
        let state = FacesState::neutral();
        let json = state.to_json_string();
        assert!(json.starts_with('"'));
        assert!(json.ends_with('"'));
        assert_eq!(json.len(), 10); // 8 hex chars + 2 quotes
    }

    #[test]
    fn test_u32_roundtrip() {
        let original = FacesState::new(
            Aura::from_index(93),
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let packed = original.to_u32();
        let unpacked = FacesState::from_u32(packed);
        assert_eq!(original, unpacked);
    }

    #[test]
    fn test_u32_neutral() {
        let state = FacesState::neutral();
        let packed = state.to_u32();
        // Aura=245(0xF5), Container=0, Focus=0, Action=0
        assert_eq!(packed, 0xF500_0000);
        let unpacked = FacesState::from_u32(packed);
        assert_eq!(state, unpacked);
    }

    #[test]
    fn test_u32_all_bytes() {
        let state = FacesState::new(
            Aura::from_index(0xAB),
            Container::Sharp,   // 4
            Focus::Tired,       // 5
            Action::Hesitant,   // 4
        );
        let packed = state.to_u32();
        // Aura=0xAB, Container=4, Focus=5, Action=4
        assert_eq!(packed, 0xAB04_0504);
        let unpacked = FacesState::from_u32(packed);
        assert_eq!(state, unpacked);
    }

    #[test]
    fn test_u32_arbitrary_value_valid() {
        // Any u32 should produce a valid state (no panics)
        for v in 0..=0xFFFFu32 {
            let _ = FacesState::from_u32(v);
        }
        // Test some larger values
        let _ = FacesState::from_u32(0xFFFF_FFFF);
        let _ = FacesState::from_u32(0x0000_0000);
        let _ = FacesState::from_u32(0xDEAD_BEEF);
    }

    #[test]
    fn test_u32_matches_bytes() {
        let state = FacesState::new(
            Aura::from_index(160),
            Container::Rigid,
            Focus::Intense,
            Action::Assertive,
        );
        let bytes = state.to_bytes();
        let packed = state.to_u32();
        // to_u32 should be equivalent to manual packing
        assert_eq!(packed, ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | bytes[3] as u32);
    }

    #[test]
    fn test_safety_preset_emergency() {
        let state = FacesState::EMERGENCY;
        assert_eq!(state.aura, Aura::URGENT);
        assert_eq!(state.container, Container::Sharp);
        assert_eq!(state.focus, Focus::Intense);
        assert_eq!(state.action, Action::Assertive);
    }

    #[test]
    fn test_safety_preset_low_power() {
        let state = FacesState::LOW_POWER;
        assert_eq!(state.aura, Aura::TIRED);
        assert_eq!(state.container, Container::Neutral);
        assert_eq!(state.focus, Focus::Tired);
        assert_eq!(state.action, Action::Withheld);
    }

    #[test]
    fn test_safety_preset_human_proximity() {
        let state = FacesState::HUMAN_PROXIMITY;
        assert_eq!(state.aura, Aura::CALM);
        assert_eq!(state.container, Container::Fluid);
        assert_eq!(state.focus, Focus::Open);
        assert_eq!(state.action, Action::Thoughtful);
    }

    #[test]
    fn test_safety_presets_distinct() {
        // All three presets should be distinct from each other and from neutral
        assert_ne!(FacesState::EMERGENCY, FacesState::LOW_POWER);
        assert_ne!(FacesState::EMERGENCY, FacesState::HUMAN_PROXIMITY);
        assert_ne!(FacesState::LOW_POWER, FacesState::HUMAN_PROXIMITY);
        assert_ne!(FacesState::EMERGENCY, FacesState::neutral());
        assert_ne!(FacesState::LOW_POWER, FacesState::neutral());
        assert_ne!(FacesState::HUMAN_PROXIMITY, FacesState::neutral());
    }
}
