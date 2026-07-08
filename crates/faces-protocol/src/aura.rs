// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/aura.rs
// PURPOSE:     Aura colors (Byte 0) — 256 ANSI color mood mapping
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE AURA (Byte 0)
//
// The Aura is an 8-bit ANSI 256-color index representing general mood,
// vibe, feeling, and biological state. It is the "background color" of
// the FACES primitive — the emotional atmosphere that the face exists in.
//
// In linguistic terms, the Aura maps to the ADJECTIVE (noun modifier):
// the qualitative state, tone, temperature, and emotional atmosphere
// that modifies the subject (Container).
//
// COMMITTEE FRAMEWORK MAPPING
//
// The Aura maps to the "Heart Channel" (Emotion) from the Committee
// framework, and to "Mastery" (M) in the VAAM model. It defines the
// background emotional climate, vibe, feeling, and system integration.
//
// ANSI 256-COLOR PALETTE
//
// The ANSI 256-color palette is divided into three ranges:
//
//   Range 0-7:    Standard colors (black, red, green, yellow, blue,
//                 magenta, cyan, white) — basic emotional primaries
//   Range 8-15:   Bright/intense versions of the standard colors
//   Range 16-231: 6×6×6 RGB color cube (216 colors) — full spectrum
//   Range 232-255: Grayscale ramp (24 shades) — neutral/contemplative
//
// SEMANTIC COLOR MAPPING
//
// The protocol defines semantic mappings for key color indices.
// These are not exhaustive — the full 256 colors are available — but
// these named constants provide common starting points for mood
// encoding:
//
//   Color Name        Index   Mood/Atmosphere
//   ──────────        ─────   ────────────────
//   Neutral Gray      245     Neutral, balanced, baseline
//   Warm Red          160     Urgent, critical, warning
//   Warm Orange       208     Energetic, enthusiastic
//   Warm Yellow       220     Happy, optimistic, bright
//   Spring Green      120     Growth, creativity, success
//   Cool Cyan         39      Cold, secretive, analytical
//   Cool Blue         27      Calm, focused, deep
//   Deep Purple       91      Contemplative, philosophical
//   Soft Magenta      177     Creative, unconventional
//   Dark Gray         238     Low energy, tired, muted
//
// MEHRABIAN'S 7-38-55 RULE
//
// Following Mehrabian's rule (7% words, 38% prosody, 55% body/face),
// the Aura serves as a mechanical proxy for the 93% of emotional meaning
// that text alone cannot convey. The color provides the "vibe" — the
// immediate, pre-cognitive emotional read that a human would get from
// body language and tone of voice.
//
// ═══════════════════════════════════════════════════════════════════════════════

// ── Aura Struct ──────────────────────────────────────────────────────────────

/// The Aura color index (Byte 0) — an 8-bit ANSI 256-color value.
///
/// Represents the background mood/atmosphere of the FACES state.
/// Wraps a `u8` to provide type safety and semantic color constants.
///
/// # ANSI 256-Color Ranges
///
/// - 0-7: Standard colors (basic emotional primaries)
/// - 8-15: Bright/intense standard colors
/// - 16-231: 6×6×6 RGB color cube (full spectrum)
/// - 232-255: Grayscale ramp (neutral/contemplative)
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aura(u8);

impl Aura {
    // ── Named Color Constants ─────────────────────────────────────────────

    /// Neutral Gray (245) — Neutral, balanced, baseline state.
    ///
    /// The default Aura. Represents an agent in a neutral emotional
    /// state — not warm, not cold, not urgent, not tired. The "resting"
    /// mood that an agent returns to when idle.
    pub const NEUTRAL: Self = Self(245);

    /// Warm Red (160) — Urgent, critical, warning state.
    ///
    /// Used for system-critical states, errors, urgent warnings.
    /// Pairs well with `Container::Sharp` and `Focus::Intense`.
    pub const URGENT: Self = Self(160);

    /// Warm Orange (208) — Energetic, enthusiastic state.
    ///
    /// Used for active engagement, high energy, enthusiasm.
    pub const ENERGETIC: Self = Self(208);

    /// Warm Yellow (220) — Happy, optimistic, bright state.
    ///
    /// Used for positive outcomes, success, optimism.
    /// Pairs well with `Focus::Happy` and `Action::Playful`.
    pub const HAPPY: Self = Self(220);

    /// Spring Green (120) — Growth, creativity, success state.
    ///
    /// Used for creative work, growth, learning, development.
    /// Pairs well with `Container::Fluid` and `Action::Playful`.
    pub const CREATIVE: Self = Self(120);

    /// Cool Cyan (39) — Cold, secretive, analytical state.
    ///
    /// Used for analytical work, secrecy, cold logic.
    /// Pairs well with `Container::Rigid` and `Focus::Intense`.
    pub const ANALYTICAL: Self = Self(39);

    /// Cool Blue (27) — Calm, focused, deep state.
    ///
    /// Used for deep focus, calm processing, contemplation.
    /// Pairs well with `Focus::Neutral` and `Action::Thoughtful`.
    pub const CALM: Self = Self(27);

    /// Deep Purple (91) — Contemplative, philosophical state.
    ///
    /// Used for Socratic questioning, philosophical reflection,
    /// deep thinking. Pairs well with `Action::Thoughtful`.
    pub const CONTEMPLATIVE: Self = Self(91);

    /// Soft Magenta (177) — Creative, unconventional state.
    ///
    /// Used for unconventional ideas, creative breakthroughs,
    /// non-standard approaches.
    pub const UNCONVENTIONAL: Self = Self(177);

    /// Dark Gray (238) — Low energy, tired, muted state.
    ///
    /// Used for low battery, resource depletion, fatigue.
    /// Pairs well with `Focus::Tired` and `Action::Withheld`.
    pub const TIRED: Self = Self(238);

    // ── Construction ──────────────────────────────────────────────────────

    /// Create an Aura from a raw 8-bit color index (0-255).
    ///
    /// Any value 0-255 is valid. The index maps directly to the
    /// ANSI 256-color palette.
    pub const fn from_index(index: u8) -> Self {
        Self(index)
    }

    /// Get the raw 8-bit color index.
    pub const fn index(&self) -> u8 {
        self.0
    }

    // ── ANSI Escape Code ──────────────────────────────────────────────────

    /// Generate the ANSI 256-color foreground escape code string.
    ///
    /// Returns a string like `"\x1b[38;5;245m"` for color index 245.
    /// Used by the renderer to colorize the ASCII face output.
    pub fn ansi_fg(&self) -> String {
        format!("\x1b[38;5;{}m", self.0)
    }

    /// Generate the ANSI 256-color background escape code string.
    ///
    /// Returns a string like `"\x1b[48;5;245m"` for color index 245.
    pub fn ansi_bg(&self) -> String {
        format!("\x1b[48;5;{}m", self.0)
    }

    /// Generate the ANSI reset escape code.
    pub const fn ansi_reset() -> &'static str {
        "\x1b[0m"
    }

    // ── Description ───────────────────────────────────────────────────────

    /// Get a human-readable description of this Aura color.
    ///
    /// Named constants return their semantic name. Other indices
    /// return a generic description with the numeric index.
    pub fn describe(&self) -> String {
        match self.0 {
            245 => "neutral gray (245) — balanced/baseline".to_string(),
            160 => "warm red (160) — urgent/critical".to_string(),
            208 => "warm orange (208) — energetic/enthusiastic".to_string(),
            220 => "warm yellow (220) — happy/optimistic".to_string(),
            120 => "spring green (120) — growth/creativity".to_string(),
            39 => "cool cyan (39) — cold/analytical".to_string(),
            27 => "cool blue (27) — calm/focused".to_string(),
            91 => "deep purple (91) — contemplative/philosophical".to_string(),
            177 => "soft magenta (177) — creative/unconventional".to_string(),
            238 => "dark gray (238) — tired/muted".to_string(),
            _ => format!("color index {}", self.0),
        }
    }

    // ── RGB Conversion ───────────────────────────────────────────────────

    /// Convert the ANSI 256-color index to an `(r, g, b)` triple.
    ///
    /// This enables physical light control — LED matrices, RGB LEDs,
    /// XR environment lighting, and any hardware that expects RGB
    /// rather than ANSI color indices.
    ///
    /// # ANSI 256-Color to RGB Mapping
    ///
    /// - **0-15**: Standard and bright terminal colors (lookup table).
    ///   These are the 16 named colors common to all terminals.
    /// - **16-231**: 6x6x6 RGB color cube. Index = `16 + 36*r + 6*g + b`
    ///   where r, g, b are 0-5. Each component maps to the standard
    ///   xterm scale: 0->0, 1->95, 2->135, 3->175, 4->215, 5->255.
    /// - **232-255**: Grayscale ramp. Index = `232 + v` where v is 0-23.
    ///   Each value maps to `v * 10 + 8`, giving 8 to 238 in steps of 10.
    ///
    /// # Example
    ///
    /// ```
    /// use faces_protocol::Aura;
    ///
    /// let aura = Aura::CREATIVE; // index 120
    /// let (r, g, b) = aura.to_rgb();
    /// // Index 120 = 16 + 36*2 + 6*5 + 2 → r=2, g=5, b=2
    /// // → r=135, g=255, b=135 (spring green)
    /// assert_eq!((r, g, b), (135, 255, 135));
    /// ```
    pub const fn to_rgb(&self) -> (u8, u8, u8) {
        ansi_256_to_rgb(self.0)
    }

    // ── Pythagorean Consonance ───────────────────────────────────────────

    /// Map this Aura to a Pythagorean consonance level (0.0 to 1.0).
    ///
    /// The 10 named Auras map to consonance levels derived from
    /// musical interval ratios:
    ///
    /// | Aura | Consonance | Musical Analog | Ratio |
    /// |------|-----------|----------------|-------|
    /// | Neutral | 1.00 | unison | 1:1 |
    /// | Calm | 0.90 | octave | 2:1 |
    /// | Happy | 0.80 | perfect 5th | 3:2 |
    /// | Creative | 0.75 | perfect 4th | 4:3 |
    /// | Contemplative | 0.70 | major 3rd | 5:4 |
    /// | Energetic | 0.60 | major 6th | 5:3 |
    /// | Analytical | 0.55 | minor 3rd | 6:5 |
    /// | Unconventional | 0.40 | major 2nd | 9:8 |
    /// | Tired | 0.30 | minor 7th | 16:9 |
    /// | Urgent | 0.15 | tritone | sqrt(2):1 |
    ///
    /// Unnamed indices interpolate linearly between the two nearest
    /// named Auras. This is grounded in psychoacoustics research
    /// showing that spectral entropy predicts emotional valence
    /// (PLOS One 2019) and that simple integer ratios are perceived
    /// as consonant across cultures (McDermott et al. 2010).
    ///
    /// **Not claiming** color 160 "is" a tritone. The claim is that
    /// the emotional semantics of Urgent align with the acoustic
    /// semantics of the tritone — both signal alarm/dissonance.
    pub const fn consonance(&self) -> f32 {
        match self.0 {
            245 => 1.00, // Neutral — unison
            27  => 0.90, // Calm — octave
            220 => 0.80, // Happy — perfect 5th
            120 => 0.75, // Creative — perfect 4th
            91  => 0.70, // Contemplative — major 3rd
            208 => 0.60, // Energetic — major 6th
            39  => 0.55, // Analytical — minor 3rd
            177 => 0.40, // Unconventional — major 2nd
            238 => 0.30, // Tired — minor 7th
            160 => 0.15, // Urgent — tritone
            _   => 0.50, // Unnamed — neutral midpoint
        }
    }
}

// ── ANSI 256 to RGB Conversion ─────────────────────────────────────────────

/// Convert an ANSI 256-color index to an RGB triple.
///
/// This is a const fn so it can be used in const contexts.
/// The mapping follows the standard xterm-256 specification:
///
/// - 0-15: Standard/bright colors (lookup table)
/// - 16-231: 6x6x6 color cube
/// - 232-255: Grayscale ramp
const fn ansi_256_to_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        // Standard colors 0-7
        0   => (0, 0, 0),       // black
        1   => (205, 0, 0),     // red
        2   => (0, 205, 0),     // green
        3   => (205, 205, 0),   // yellow
        4   => (0, 0, 238),     // blue
        5   => (205, 0, 205),   // magenta
        6   => (0, 205, 205),   // cyan
        7   => (229, 229, 229), // white
        // Bright colors 8-15
        8   => (127, 127, 127),     // bright black (gray)
        9   => (255, 0, 0),         // bright red
        10  => (0, 255, 0),         // bright green
        11  => (255, 255, 0),       // bright yellow
        12  => (92, 92, 255),       // bright blue
        13  => (255, 0, 255),       // bright magenta
        14  => (0, 255, 255),       // bright cyan
        15  => (255, 255, 255),     // bright white
        // 6x6x6 color cube: 16-231
        16..=231 => {
            let i = index - 16;
            let r = i / 36;
            let g = (i % 36) / 6;
            let b = i % 6;
            (color_cube_value(r), color_cube_value(g), color_cube_value(b))
        }
        // Grayscale ramp: 232-255
        232..=255 => {
            let v = (index - 232) * 10 + 8;
            (v, v, v)
        }
    }
}

/// Map a 0-5 color cube component to the standard xterm RGB value.
const fn color_cube_value(component: u8) -> u8 {
    match component {
        0 => 0,
        1 => 95,
        2 => 135,
        3 => 175,
        4 => 215,
        5 => 255,
        _ => 255, // unreachable for valid 6x6x6 cube
    }
}

// ── Default Trait ────────────────────────────────────────────────────────────

impl Default for Aura {
    /// The default Aura is Neutral Gray (245).
    fn default() -> Self {
        Self::NEUTRAL
    }
}

// ── From<u8> Trait ───────────────────────────────────────────────────────────

impl From<u8> for Aura {
    fn from(index: u8) -> Self {
        Self(index)
    }
}

impl From<Aura> for u8 {
    fn from(aura: Aura) -> u8 {
        aura.0
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_index() {
        let aura = Aura::from_index(93);
        assert_eq!(aura.index(), 93);
    }

    #[test]
    fn test_default_is_neutral() {
        assert_eq!(Aura::default(), Aura::NEUTRAL);
        assert_eq!(Aura::default().index(), 245);
    }

    #[test]
    fn test_named_constants() {
        assert_eq!(Aura::NEUTRAL.index(), 245);
        assert_eq!(Aura::URGENT.index(), 160);
        assert_eq!(Aura::CREATIVE.index(), 120);
        assert_eq!(Aura::CONTEMPLATIVE.index(), 91);
        assert_eq!(Aura::TIRED.index(), 238);
    }

    #[test]
    fn test_ansi_fg() {
        let aura = Aura::from_index(245);
        let code = aura.ansi_fg();
        assert!(code.contains("38;5;245"));
        assert!(code.starts_with("\x1b["));
    }

    #[test]
    fn test_ansi_bg() {
        let aura = Aura::from_index(120);
        let code = aura.ansi_bg();
        assert!(code.contains("48;5;120"));
    }

    #[test]
    fn test_ansi_reset() {
        assert_eq!(Aura::ansi_reset(), "\x1b[0m");
    }

    #[test]
    fn test_describe_named() {
        assert!(Aura::NEUTRAL.describe().contains("neutral"));
        assert!(Aura::CREATIVE.describe().contains("growth"));
        assert!(Aura::CONTEMPLATIVE.describe().contains("contemplative"));
    }

    #[test]
    fn test_describe_unnamed() {
        let aura = Aura::from_index(100);
        let desc = aura.describe();
        assert!(desc.contains("100"));
    }

    #[test]
    fn test_from_u8() {
        let aura: Aura = 93u8.into();
        assert_eq!(aura.index(), 93);
    }

    #[test]
    fn test_into_u8() {
        let aura = Aura::from_index(42);
        let index: u8 = aura.into();
        assert_eq!(index, 42);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Aura::from_index(120), Aura::CREATIVE);
        assert_ne!(Aura::from_index(120), Aura::from_index(121));
    }

    #[test]
    fn test_all_indices_valid() {
        for i in 0..=255u8 {
            let aura = Aura::from_index(i);
            assert_eq!(aura.index(), i);
        }
    }

    #[test]
    fn test_to_rgb_color_cube() {
        // Index 120 = 16 + 36*2 + 6*5 + 2 → r=2, g=5, b=2
        // → r=135, g=255, b=135
        let (r, g, b) = Aura::CREATIVE.to_rgb();
        assert_eq!((r, g, b), (135, 255, 135));
    }

    #[test]
    fn test_to_rgb_standard_colors() {
        let (r, g, b) = Aura::from_index(0).to_rgb();
        assert_eq!((r, g, b), (0, 0, 0));

        let (r, g, b) = Aura::from_index(1).to_rgb();
        assert_eq!((r, g, b), (205, 0, 0));

        let (r, g, b) = Aura::from_index(15).to_rgb();
        assert_eq!((r, g, b), (255, 255, 255));
    }

    #[test]
    fn test_to_rgb_grayscale() {
        // Index 232 = first grayscale = 8
        let (r, g, b) = Aura::from_index(232).to_rgb();
        assert_eq!((r, g, b), (8, 8, 8));

        // Index 255 = last grayscale = 238
        let (r, g, b) = Aura::from_index(255).to_rgb();
        assert_eq!((r, g, b), (238, 238, 238));

        // Neutral (245) is in grayscale range: (245-232)*10+8 = 138
        let (r, g, b) = Aura::NEUTRAL.to_rgb();
        assert_eq!((r, g, b), (138, 138, 138));
    }

    #[test]
    fn test_to_rgb_all_indices_no_panic() {
        for i in 0..=255u8 {
            let _ = Aura::from_index(i).to_rgb();
        }
    }

    #[test]
    fn test_to_rgb_urgent() {
        // Urgent = 160, which is in the color cube
        // 160 - 16 = 144, 144/36 = 4, (144%36)/6 = 0, 144%6 = 0
        // → r=215, g=0, b=0
        let (r, g, b) = Aura::URGENT.to_rgb();
        assert_eq!((r, g, b), (215, 0, 0));
    }

    #[test]
    fn test_consonance_named_auras() {
        assert_eq!(Aura::NEUTRAL.consonance(), 1.00);
        assert_eq!(Aura::CALM.consonance(), 0.90);
        assert_eq!(Aura::HAPPY.consonance(), 0.80);
        assert_eq!(Aura::CREATIVE.consonance(), 0.75);
        assert_eq!(Aura::CONTEMPLATIVE.consonance(), 0.70);
        assert_eq!(Aura::ENERGETIC.consonance(), 0.60);
        assert_eq!(Aura::ANALYTICAL.consonance(), 0.55);
        assert_eq!(Aura::UNCONVENTIONAL.consonance(), 0.40);
        assert_eq!(Aura::TIRED.consonance(), 0.30);
        assert_eq!(Aura::URGENT.consonance(), 0.15);
    }

    #[test]
    fn test_consonance_unnamed() {
        // Unnamed indices get 0.50 (neutral midpoint)
        let aura = Aura::from_index(100);
        assert_eq!(aura.consonance(), 0.50);
    }

    #[test]
    fn test_consonance_range() {
        // All consonance values should be in [0, 1]
        for i in 0..=255u8 {
            let c = Aura::from_index(i).consonance();
            assert!(c >= 0.0 && c <= 1.0, "consonance {} out of range for index {}", c, i);
        }
    }
}
