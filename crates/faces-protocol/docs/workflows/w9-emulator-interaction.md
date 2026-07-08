---
description: W9 — Emulator interaction features: Consent Gate UI, VAAM baseline simulation, transition visualization
---

# W9: Emulator Interaction

## Objective

Extend the emulator with interactive Consent Gate UI, VAAM baseline
simulation, and transition visualization. This makes the emulator a full
demonstration environment for the FACES protocol's interactive features.

## Prerequisites

- W8 complete (emulator core with terminal UI, render modes, input handling)
- W4 complete (Consent Gate)
- W5 complete (Phase Manager)

## Steps

1. **Consent Gate UI panel**:
   - New UI panel showing gate state: Locked / Unlocked / Committed
   - Visual indicator: `[LOCKED]`, `[UNLOCKED]`, `[COMMITTED]`
   - Show nudge count: `nudges: 2/3`
   - Show proposed state vs locked state side-by-side
   - Show violation magnitude and harmonic distance if proposed differs
   - Commands: `:gate lock`, `:gate unlock`, `:gate commit`, `:gate propose <hex>`

2. **Nudge visualization**:
   - When nudge triggers, display nudge message in a highlighted panel
   - Color-code: Suggest (cyan), Warn (yellow), Refuse (red)
   - Show nudge escalation: `→ suggest → warn → refuse`
   - Auto-lock after refuse is visually indicated

3. **VAAM baseline simulation**:
   - `VaamSimulator` struct: generates synthetic VAAM profiles
   - `VaamProfile` simplified: vocabulary_level, autonomy_style, acquisition_mode, mastery_domain
   - Map VAAM profile to FACES baseline using the isomorphic mapping:
     - Vocabulary → Action
     - Autonomy → Container
     - Acquisition → Focus
     - Mastery → Aura
   - `set_baseline_from_vaam(profile: &VaamProfile)` — set FacesProfile baseline
   - Commands: `:vaam set <profile>`, `:vaam random`, `:vaam clear`

4. **Transition visualization**:
   - Animated transition display when state changes
   - Show from-state → to-state with arrow: `{oo_} → {^^~}`
   - Show magnitude bar: `████░░░░░░ (4)`
   - Show harmonic distance bar: `██░░░░░░░░ (1.93)`
   - Show ratio complexity per dimension: `C:0.83 F:0.00 A:0.00`
   - Color the transition by volatility (green=minor, yellow=moderate, red=major)

5. **Phase manager UI**:
   - Show current phase: `[STATIC LOCK]` or `[DYNAMIC INTERACTION]`
   - Show friction level: `friction: ████░░░░░░ (0.4)`
   - Show drift from committed: `drift: mag=2, harmonic=0.83`
   - Commands: `:phase static`, `:phase dynamic`, `:phase friction <0.0-1.0>`, `:phase recommit`

6. **Interactive demo scenarios**:
   - `:demo emergency` — simulate emergency detection → safety preset
   - `:demo creative` — simulate brainstorm session → fluid container, creative aura
   - `:demo consent` — walk through consent gate cycle (lock → propose → nudge → commit)
   - `:demo vaam` — set VAAM baseline, show detection overriding it
   - `:demo transitions` — show series of transitions with visualization

7. **History browser**:
   - `:history` command shows state history with timestamps
   - Navigate with arrow keys
   - Show transition between each history entry
   - Export history as JSONL: `:history export <path>`

## Testing

- Consent Gate UI: panel renders correct gate state
- Nudge visualization: correct color and message for each level
- VAAM simulator: profile maps to correct FACES baseline
- Transition visualization: bars and labels correct for known transitions
- Phase UI: correct phase display, friction bar, drift values
- Demo scenarios: each scenario runs without panic, produces expected states
- History browser: entries displayed correctly, export produces valid JSONL

## Completion Criteria

- Consent Gate UI panel with state, nudge count, violation display
- Nudge visualization with color-coded messages
- VAAM baseline simulator with isomorphic mapping
- Transition visualization with magnitude/harmonic bars
- Phase manager UI with friction and drift display
- 5 interactive demo scenarios
- History browser with JSONL export
- All tests pass (target: 270+ tests)
- Zero dependencies maintained in default build
- PROGRESS.md updated
