// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/lib.rs
// PURPOSE:     Crate root — public API for the FACES Protocol
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE FACES PROTOCOL — Fact Align Computational Emotive System
//
// FACES is a deterministic, low-compute emotive signaling system that
// compresses complex psychological states into a 4-byte (32-bit) payload.
// It serves as a mechanical bridge for intent alignment between humans
// and AI agents, enabling real-time emotive feedback that is both
// machine-readable and human-intuitive.
//
// THE TRIPLE REFLECTION — FACES IS THE "IMAGE" REFLECTION
//
// Trinity's cognitive framework uses three reflections (mirrors) that
// together produce full-spectrum understanding before any work begins:
//
//   1. FACES (Image)      — Set and setting. Emotional context. THIS CRATE.
//   2. LitRPG (Narrative)  — Perspective engineering. Quest scaffolding.
//   3. Socratic (Depth)    — Reflective questioning. Vision extraction.
//
// FACES establishes the emotional and atmospheric context of the work.
// The AI understands *how it feels* to be in this work, not just what
// the work is about. This is the "set and setting" — the background
// emotional climate that colors all interaction.
//
// THE 4-BYTE PAYLOAD
//
//   Byte 0 — Aura      (8-bit):  256 ANSI color indices → mood/atmosphere
//   Byte 1 — Container (5-val):  Head shape → cognitive boundary/temperament
//   Byte 2 — Focus     (6-val):  Eyes → attentional state/intensity
//   Byte 3 — Action    (5-val):  Mouth → communicative intent/output
//
//   Total unique states: 256 × 5 × 6 × 5 = 38,400
//
// WHY ASCII GEOMETRY?
//
// The human brain is biologically wired for pareidolia — the tendency to
// perceive faces in simple patterns. FACES uses minimal ASCII geometry
// (a container, two eyes, and a mouth) to trigger primal empathy and
// recognition at a subconscious level. By avoiding complex 3D blendshapes
// and high-fidelity rendering, the protocol bypasses the Uncanny Valley
// entirely while requiring negligible compute (zero VRAM, CPU negligible).
//
// TOKEN ECONOMICS
//
// Traditional LLM emotive signaling uses "thought blocks" or descriptive
// tags (e.g., *The AI looks at you with slight exhaustion*), consuming
// 30-50 tokens per turn. FACES uses 1-4 tokens (a single hex string or
// 5-character ASCII block). In a 50-turn conversation, FACES saves
// ~1,500-2,500 tokens, reducing context pressure and inference cost.
//
// HARDWARE TARGET
//
// On AMD Strix Halo:
//   - NPU (XDNA 2, 50 TOPS): Runs FACES-Embed (~66M param DistilBERT
//     encoder-only model, ONNX format) for text-to-FACES classification
//   - GPU (RDNA 3.5, 59.39 FP16 TFLOPS): Runs LLM inference in parallel
//   - The two workloads are independent and heterogeneous — emotive AI
//     on NPU, language generation on GPU — not a prefill/decode split
//
// On Android XR (future):
//   - Mobile NPU (Tensor/Snapdragon): Could run FACES-Embed for local
//     emotion detection from voice prosody
//   - FACES state streamed from Strix Halo base station via MCP/IPC
//   - FACES state drives XR environment selection (set and setting)
//
// ═══════════════════════════════════════════════════════════════════════════════

#![warn(missing_docs)]
#![doc = "FACES Protocol — Fact Align Computational Emotive System for Trinity ID AI OS"]

// ── Module Declarations ─────────────────────────────────────────────────────

/// The 4-byte protocol state — `FacesState` struct, encoding, decoding.
pub mod protocol;

/// Container shapes (Byte 1) — 5 head boundary geometries mapped to
/// Mian Xiang Five Elements and cognitive boundary states.
pub mod container;

/// Focus shapes (Byte 2) — 6 eye/attention states mapped to FACS
/// Action Units and cognitive load indicators.
pub mod focus;

/// Action shapes (Byte 3) — 5 mouth/expression states mapped to
/// communicative intent and kinetic output readiness.
pub mod action;

/// Aura colors (Byte 0) — 256 ANSI color indices mapped to mood,
/// atmosphere, and biological state.
pub mod aura;

/// ASCII renderer — converts `FacesState` to terminal-displayable
/// string with ANSI color codes. Zero-allocation.
pub mod render;

/// Text-to-FACES detection — maps natural language text to `FacesState`,
/// replacing Trinity's legacy `detect_emotion()` keyword matcher with
/// the full 38,400-state protocol.
pub mod detect;

/// Contrastive Transition Vector — calculates emotional velocity
/// between consecutive FACES states and provides interpolation
/// for smooth morphing between discrete coordinate states.
pub mod transition;

/// Sentence segmentation — splits text into sentences for multi-sentence
/// FACES detection. Zero-allocation, handles common abbreviations.
pub mod segment;

/// FacesProfile — user baseline tracking and state history with a
/// fixed-size ring buffer (no heap allocation). Supports VAAM bridge
/// via `with_baseline()`.
pub mod profile;

/// Mechanical Consent Gate — three-phase state machine (Locked/Unlocked/
/// Committed) that prevents unwanted emotive state changes. Provides
/// graduated nudge functions (Suggest/Warn/Refuse) and violation detection.
pub mod consent;

/// Evaluation harness — JSONL loader, per-dimension P/R/F1 metrics, latency
/// benchmarks. Only compiled with `--features eval`.
#[cfg(feature = "eval")]
pub mod eval;

// ── Re-exports ──────────────────────────────────────────────────────────────

/// The primary FACES state struct. Re-exported at crate root for convenience.
pub use protocol::FacesState;

/// The 5 Container shape variants.
pub use container::Container;

/// The 6 Focus shape variants.
pub use focus::Focus;

/// The 5 Action shape variants.
pub use action::Action;

/// Aura color index (0-255).
pub use aura::Aura;

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS — see individual modules for unit tests.
// Integration tests live in the module files to keep everything co-located
// with the code it tests, following the Trinity "living textbook" convention
// where code is heavily commented for educational readability.
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Verify the total state space is exactly 38,400.
    /// This is the protocol's core mathematical claim:
    /// 256 Aura × 5 Container × 6 Focus × 5 Action = 38,400.
    #[test]
    fn test_total_state_space() {
        let total = 256 * Container::COUNT * Focus::COUNT * Action::COUNT;
        assert_eq!(total, 38_400, "FACES protocol must have exactly 38,400 states");
    }

    /// Verify a round-trip: create state → encode to bytes → decode back.
    #[test]
    fn test_roundtrip_encode_decode() {
        let original = FacesState::new(
            Aura::from_index(245),
            Container::Fluid,
            Focus::Open,
            Action::Playful,
        );
        let bytes = original.to_bytes();
        let decoded = FacesState::from_bytes(bytes);
        assert_eq!(original, decoded);
    }

    /// Verify the default state is neutral/baseline.
    #[test]
    fn test_default_state() {
        let state = FacesState::default();
        assert_eq!(state.container, Container::Neutral);
        assert_eq!(state.focus, Focus::Neutral);
        assert_eq!(state.action, Action::Withheld);
    }

    /// Verify rendering produces a non-empty string with ANSI codes.
    #[test]
    fn test_render_non_empty() {
        let state = FacesState::default();
        let rendered = state.render();
        assert!(!rendered.is_empty());
        assert!(rendered.contains("\x1b["), "Render should contain ANSI escape codes");
    }

    /// Verify hex encoding round-trips correctly.
    #[test]
    fn test_hex_roundtrip() {
        let state = FacesState::new(
            Aura::from_index(39),
            Container::Sharp,
            Focus::Intense,
            Action::Hesitant,
        );
        let hex = state.to_hex();
        assert_eq!(hex.len(), 8, "Hex string should be 8 characters (4 bytes)");
        let decoded = FacesState::from_hex(&hex).unwrap();
        assert_eq!(state, decoded);
    }
}
