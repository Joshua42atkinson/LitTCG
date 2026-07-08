# FACES Build Progress

> Single source of truth for faces-protocol build state.
> Updated after every workflow. Do not edit mid-workflow.

---

## Current State

- **Last workflow completed:** W6 — Labeling Guide
- **Tests passing:** 283 (+ 15 doc-tests)
- **Next workflow:** Gate 2 — Joshua approves labeling guide, then W7
- **Session date:** 2026-07-02

---

## Completed

- **Crate creation:** 8 source files (protocol.rs, container.rs, focus.rs, action.rs, aura.rs, render.rs, detect.rs, transition.rs), 105 tests, zero dependencies
- **Documentation:** 5 docs in docs/active/ (gap analysis, validation framework, technical intent, Pythagorean research, master pivot updates)
- **Research:** Pythagorean ratio connection grounded in neuroscience (Pallesen 2010, McDermott 2010, Erlich harmonic entropy, Cousineau 2014)
- **W1 — Protocol hardening:** Added to_u32/from_u32 hardware register packing, to_rgb() ANSI 256→RGB conversion, consonance() Pythagorean mapping, 3 safety presets (EMERGENCY/LOW_POWER/HUMAN_PROXIMITY), serde feature flag, harmonic_distance() + ratio_complexity() in transition.rs. 28 new tests. Zero-dep default preserved; serde is opt-in.
- **W2 — Scored detection:** Replaced first-match-wins with confidence-scored detection. Added DetectionResult struct (state + per-dimension confidence + congruence + method), Congruence enum (Congruent/Incongruent/Neutral), DetectionMethod enum (Keyword/Heuristic/Neural). Keyword tables for all 10 Auras, 5 Containers, 6 Focus, 5 Actions. Intensity modifiers (boosters/diminishers), negation filtering. detect_faces() is now a backward-compatible wrapper. 19 new tests. Also fixed stale 350M ref in detect.rs header.
- **W3 — Multi-sentence + FacesProfile:** Added segment.rs (sentence segmentation with abbreviation handling), detect_multi() and detect_aggregate() (recency-biased weighted aggregation with agreement boost), profile.rs (FacesProfile with 64-entry ring buffer, EMA baseline update, stability calculation, deviation tracking, VAAM bridge via with_baseline()). 56 new tests. Zero-dep maintained. 10 source files now.
- **W4 — Consent Gate:** Added consent.rs with GateState enum (Locked/Unlocked/Committed), ConsentGate struct, NudgeResult enum (Suggest/Warn/Refuse). Full state machine: lock/unlock/propose/commit with auto-lock timer. Graduated nudge functions with auto-escalation and auto-lock on refusal. Violation detection (is_violation, violation_magnitude, violation_harmonic). FacesProfile integration (from_profile, deviation_from_baseline). Also added set_baseline() to FacesProfile. 43 new tests. 11 source files now.
- **W5 — Static Lock / Dynamic Interaction:** Added Phase enum (StaticLock/DynamicInteraction), ChangeResult enum (Accepted/Partial/Rejected), PhaseManager struct to transition.rs. Two-phase temporal model with keystroke friction (high >0.8 rejects, medium 0.3-0.8 partial-lerps, low <0.3 accepts). Friction decays 5% per tick. Min lock ticks before unlock allowed. Force unlock for emergencies. Drift detection (magnitude, harmonic, acceptability threshold). Recommit returns to StaticLock. Consent Gate integration via from_gate(). 32 new tests. 11 source files (no new files).
- **W6 — Labeling Guide:** Created docs/active/FACES_LABELING_GUIDE.md — ground truth document defining text-to-FACES mapping. 10 named Auras with 5+ examples each (50 total), 5 Containers with 5+ examples each (25 total), 6 Focus variants with 5+ examples each (30 total), 5 Actions with 5+ examples each (25 total). 23 congruence examples (10 congruent, 10 incongruent, 3 neutral). 11 harmonic transition annotations with computed harmonic_distance values. 12 edge case examples (sarcasm, mixed signals, negation, context-dependent, ambiguous intensity). Multi-sentence labeling rules. Ambiguity protocol. IAA guidelines with Cohen's/Fleiss's Kappa targets. JSONL labeling format with 5 example entries. No code changes — documentation deliverable.

---

## Workflow Status

| Workflow | Status | Tests | Notes |
|----------|--------|-------|-------|
| W1: Protocol hardening | **DONE** | 133 | to_u32/from_u32, to_rgb, consonance, safety presets, serde feature, harmonic_distance() |
| W2: Scored detection | **DONE** | 152 | DetectionResult, confidence scores, congruence, intensity modifiers, negation |
| W3: Multi-sentence + profile | **DONE** | 208 | segment.rs, detect_multi/aggregate, profile.rs with ring buffer, EMA baseline |
| W4: Consent Gate | **DONE** | 251 | consent.rs, GateState, NudgeResult, auto-lock timer, violation detection |
| W5: Static/Dynamic phases | **DONE** | 283 | Phase, PhaseManager, ChangeResult, friction, drift detection, recommit |
| — Gate 1 — | **PASSED** | — | Joshua approved full crate (W1-W5) |
| W6: Labeling guide | **DONE** | 283 | FACES_LABELING_GUIDE.md — ground truth, all dimensions, congruence, harmonic transitions, edge cases, IAA, JSONL |
| — Gate 2 — | **PENDING** | — | Joshua approves labeling guide |
| W7: Eval harness + benchmarks | Not started | — | JSONL loader, per-byte F1, latency |
| W8: Emulator core | Not started | — | Terminal UI, state display, detection sim |
| — Gate 3 — | — | — | Joshua reviews emulator |
| W9: Emulator interaction | Not started | — | Consent Gate UI, VAAM sim, transitions |
| W10: Emulator fleet + physical | Not started | — | Multi-agent, behavior mapping, LED sim |

---

## Open Questions

- ~~Should harmonic_distance() replace or supplement L1 magnitude()?~~ **Resolved: supplement.** Both magnitude() and harmonic_distance() are available on TransitionVector.
- GitHub repo: Joshua considering a dedicated "faces" repo for the open-source protocol

---

## Notes

- FACES dimensions (5-6-5) map to pentatonic (5) and whole-tone (6) scales — Pythagorean by origin
- Pythagorean research doc adds two W1 items: harmonic_distance() and ratio_complexity() in transition.rs
- Pythagorean research doc adds one W6 item: harmonic transition annotations in labeling guide
- All stale "350M" references fixed across MASTER_PIVOT_DOCUMENT.md (9 locations), FACES_TECHNICAL_INTENT.md (5 locations), FACES_GAP_ANALYSIS.md (1 location), lib.rs (1 location) → now consistently ~66M DistilBERT
- Zero-dep constraint maintained throughout — all W1 features are pure Rust math
- serde feature is opt-in (`cargo build --features serde`); default build remains zero-dependency
- harmonic_distance() key insight: 1-step shift (minor 2nd, 16:15) is MORE jarring than 2-step (major 3rd, 5:4) — L1 distance gets this backwards
- ratio_complexity() maps FACES enum steps to Pythagorean interval ratios: unison→0.0, minor 2nd→0.83, major 3rd→0.43, perfect 4th→0.33, perfect 5th→0.50, octave→0.50
