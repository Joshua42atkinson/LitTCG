// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/render.rs
// PURPOSE:     ASCII renderer — converts FacesState to terminal output
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE 5-CHARACTER MATRIX
//
// The FACES protocol maps the 4-byte state to exactly 5 monospace
// terminal characters, wrapped in an 8-bit ANSI escape sequence for
// colorization. The string length remains constant regardless of
// emotional complexity.
//
// RENDER FORMAT:
//
//   \x1b[38;5;{AURA}m{CONTAINER_LEFT}{FOCUS_LEFT}{ACTION}{FOCUS_RIGHT}{CONTAINER_RIGHT}\x1b[0m
//
// CHARACTER POSITIONS:
//
//   Position:  0              1            2          3            4
//              Container-L    Focus-L      Action     Focus-R      Container-R
//
//   Example:   (              o            _          o            )
//              ↑              ↑            ↑          ↑            ↑
//              Neutral ()     Neutral (oo) Withheld   Neutral      Neutral
//
// COGNITIVE PAREIDOLIA
//
// The human brain is biologically wired for pareidolia — perceiving
// faces in simple patterns. The 5-character matrix triggers this
// response at a primal level:
//
//   (o_o)  — a neutral face staring at you
//   {^^~}  — a happy, playful face
//   [><v]  — an intense, assertive face
//   |.._|  — a dissociated, withdrawn face
//   <>--<> — a sharp, tired, aggressive face
//
// By using minimal ASCII geometry, the protocol bypasses the Uncanny
// Valley entirely. Simple geometry triggers empathy and recognition
// at a subconscious level, requiring negligible VRAM compared to
// traditional skeletal animation.
//
// ZERO ALLOCATION
//
// The renderer is designed for real-time use in terminal output loops,
// telemetry streams, and MCP message payloads. String allocation is
// minimal — the output is a single `String` of predictable length
// (~25 bytes including ANSI codes).
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::aura::Aura;
use crate::protocol::FacesState;

/// Render a `FacesState` as a 5-character ASCII face with ANSI color.
///
/// This is the primary render function. It produces a string like:
///
/// ```text
/// \x1b[38;5;245m(o_o)\x1b[0m
/// ```
///
/// The output consists of:
/// 1. ANSI 256-color foreground escape sequence (from Aura byte)
/// 2. Container left bracket
/// 3. Focus left eye
/// 4. Action mouth
/// 5. Focus right eye
/// 6. Container right bracket
/// 7. ANSI reset escape sequence
///
/// # Example
///
/// ```
/// use faces_protocol::FacesState;
/// use faces_protocol::Aura;
/// use faces_protocol::Container;
/// use faces_protocol::Focus;
/// use faces_protocol::Action;
///
/// let state = FacesState::new(
///     Aura::CREATIVE,
///     Container::Fluid,
///     Focus::Happy,
///     Action::Playful,
/// );
///
/// let rendered = state.render();
/// // rendered == "\x1b[38;5;120m{^^~}\x1b[0m"
/// ```
pub fn render_state(state: &FacesState) -> String {
    let (container_l, container_r) = state.container.glyphs();
    let (focus_l, focus_r) = state.focus.glyphs();
    let action = state.action.glyph();

    format!(
        "{}{}{}{}{}{}",
        state.aura.ansi_fg(),
        container_l,
        focus_l,
        action,
        focus_r,
        container_r,
    ) + Aura::ansi_reset()
}

/// Render a `FacesState` as a 5-character ASCII face WITHOUT ANSI color.
///
/// This produces the raw 5-character string without any escape codes,
/// useful for:
/// - Log files where ANSI codes would be noise
/// - JSON/CSV output where escape codes would break parsing
/// - Testing and comparison
/// - Environments that don't support ANSI (some web contexts)
///
/// # Example
///
/// ```
/// use faces_protocol::FacesState;
///
/// let state = FacesState::default();
/// let plain = faces_protocol::render::render_plain(&state);
/// assert_eq!(plain, "(o_o)");
/// ```
pub fn render_plain(state: &FacesState) -> String {
    let (container_l, container_r) = state.container.glyphs();
    let (focus_l, focus_r) = state.focus.glyphs();
    let action = state.action.glyph();

    format!("{}{}{}{}{}", container_l, focus_l, action, focus_r, container_r)
}

/// Render a `FacesState` as a multi-line "big" face for terminal display.
///
/// This produces a 3-line ASCII art representation that is more visually
/// prominent than the single-line 5-character matrix. Useful for:
/// - Terminal UI headers
/// - Demo displays
/// - Debug output where the face needs to be clearly visible
///
/// # Example output
///
/// ```text
///  ┌─────┐
///  │ o o │   ← Focus (eyes)
///  │  _  │   ← Action (mouth)
///  └─────┘
/// ```
///
/// The brackets change based on the Container shape, and the eyes/mouth
/// change based on Focus and Action respectively.
pub fn render_big(state: &FacesState) -> String {
    let (container_l, container_r) = state.container.glyphs();
    let (focus_l, focus_r) = state.focus.glyphs();
    let action = state.action.glyph();

    // Use the container glyphs as the outer frame
    // For a more visual display, we use a 3-line format
    format!(
        "  {cl}═════{cr}\n  {cl} {fl} {fr} {cr}   ← Focus ({focus_desc})\n  {cl}  {action}  {cr}   ← Action ({action_desc})\n  {cl}═════{cr}\n  Aura: {aura_desc}",
        cl = container_l,
        cr = container_r,
        fl = focus_l,
        fr = focus_r,
        action = action,
        focus_desc = state.focus.describe(),
        action_desc = state.action.describe(),
        aura_desc = state.aura.describe(),
    )
}

/// Render a `FacesState` as a telemetry log line.
///
/// Produces a single-line format suitable for log files and
/// real-time monitoring:
///
/// ```text
/// [FACES] 5D024002 {^^~} | Aura: spring green (120) | Container: fluid | Focus: happy | Action: playful
/// ```
///
/// This format allows developers to visually scan agent health in
/// terminal output. A "loop trap" or compilation error can be
/// instantly identified by a red, sharp, intense face appearing
/// in the telemetry stream.
pub fn render_telemetry(state: &FacesState) -> String {
    format!(
        "[FACES] {} {} | {} | {} | {} | {}",
        state.to_hex(),
        render_plain(state),
        state.aura.describe(),
        state.container.describe(),
        state.focus.describe(),
        state.action.describe(),
    )
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::aura::Aura;
    use crate::container::Container;
    use crate::focus::Focus;

    #[test]
    fn test_render_plain_neutral() {
        let state = FacesState::default();
        let plain = render_plain(&state);
        assert_eq!(plain, "(o_o)");
    }

    #[test]
    fn test_render_plain_fluid_happy_playful() {
        let state = FacesState::new(
            Aura::CREATIVE,
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let plain = render_plain(&state);
        assert_eq!(plain, "{^~^}");
    }

    #[test]
    fn test_render_plain_sharp_intense_assertive() {
        let state = FacesState::new(
            Aura::URGENT,
            Container::Sharp,
            Focus::Intense,
            Action::Assertive,
        );
        let plain = render_plain(&state);
        assert_eq!(plain, "<>v<>");
    }

    #[test]
    fn test_render_plain_defensive_distant_withheld() {
        let state = FacesState::new(
            Aura::TIRED,
            Container::Defensive,
            Focus::Distant,
            Action::Withheld,
        );
        let plain = render_plain(&state);
        assert_eq!(plain, "|._.|");
    }

    #[test]
    fn test_render_with_color_contains_ansi() {
        let state = FacesState::default();
        let rendered = render_state(&state);
        assert!(rendered.contains("\x1b[38;5;"));
        assert!(rendered.contains("\x1b[0m"));
    }

    #[test]
    fn test_render_with_color_contains_face() {
        let state = FacesState::new(
            Aura::CREATIVE,
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let rendered = render_state(&state);
        assert!(rendered.contains("{^~^}"));
    }

    #[test]
    fn test_render_big_has_three_lines() {
        let state = FacesState::default();
        let big = render_big(&state);
        let lines = big.lines().count();
        assert!(lines >= 4, "render_big should have at least 4 lines, got {}", lines);
    }

    #[test]
    fn test_render_telemetry_contains_hex_and_face() {
        let state = FacesState::default();
        let telemetry = render_telemetry(&state);
        assert!(telemetry.contains("[FACES]"));
        assert!(telemetry.contains("(o_o)"));
        assert!(telemetry.contains("F5")); // hex of default state
    }

    #[test]
    fn test_render_plain_all_containers() {
        // Verify each container renders its correct brackets
        let test_cases = [
            (Container::Neutral, "(o_o)"),
            (Container::Rigid, "[o_o]"),
            (Container::Fluid, "{o_o}"),
            (Container::Defensive, "|o_o|"),
            (Container::Sharp, "<o_o>"),
        ];

        for (container, expected) in test_cases {
            let state = FacesState::new(
                Aura::NEUTRAL,
                container,
                Focus::Neutral,
                Action::Withheld,
            );
            assert_eq!(render_plain(&state), expected);
        }
    }

    #[test]
    fn test_render_plain_all_focuses() {
        let test_cases = [
            (Focus::Neutral, "(o_o)"),
            (Focus::Intense, "(>_<)"),
            (Focus::Open, "(O_O)"),
            (Focus::Distant, "(._.)"),
            (Focus::Happy, "(^_^)"),
            (Focus::Tired, "(-_-)"),
        ];

        for (focus, expected) in test_cases {
            let state = FacesState::new(
                Aura::NEUTRAL,
                Container::Neutral,
                focus,
                Action::Withheld,
            );
            assert_eq!(render_plain(&state), expected);
        }
    }

    #[test]
    fn test_render_plain_all_actions() {
        let test_cases = [
            (Action::Withheld, "(o_o)"),
            (Action::Assertive, "(ovo)"),
            (Action::Playful, "(o~o)"),
            (Action::Thoughtful, "(o-o)"),
            (Action::Hesitant, "(o.o)"),
        ];

        for (action, expected) in test_cases {
            let state = FacesState::new(
                Aura::NEUTRAL,
                Container::Neutral,
                Focus::Neutral,
                action,
            );
            assert_eq!(render_plain(&state), expected);
        }
    }
}
