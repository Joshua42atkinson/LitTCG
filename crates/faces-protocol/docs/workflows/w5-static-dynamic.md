---
description: W5 — Two-phase state management: Static Lock and Dynamic Interaction in transition.rs
---

# W5: Static Lock / Dynamic Interaction

## Objective

Implement two-phase temporal state management in `transition.rs`. The Static
Lock phase holds a committed state (no changes). The Dynamic Interaction
phase allows live state changes with optional keystroke friction — small
delays or resistance that model the effort of changing one's mind.

## Prerequisites

- W4 complete (Consent Gate with lock/unlock/commit)
- Transition layer solid (W1: magnitude, harmonic_distance, lerp, decay)

## Steps

1. **Define `Phase` enum** in `transition.rs`:
   - `StaticLock` — committed state, frozen, no transitions
   - `DynamicInteraction` — live state, transitions allowed with friction

2. **Define `PhaseManager` struct**:
   - `phase: Phase` — current phase
   - `committed_state: FacesState` — the locked-in state
   - `current_state: FacesState` — live state (may drift from committed)
   - `friction: f32` — 0.0 = no resistance, 1.0 = maximum resistance
   - `tick_count: u16` — ticks since phase entered
   - `min_lock_ticks: u16` — minimum ticks before unlock allowed (default 10)

3. **Implement phase transitions**:
   - `enter_static_lock(state: FacesState)` — commit a state, freeze
   - `enter_dynamic_interaction(friction: f32)` — unfreeze, allow changes
   - `force_unlock()` — emergency unlock (ignores min_lock_ticks)
   - `tick()` — increment tick counter, auto-transition if conditions met

4. **Implement keystroke friction**:
   - In DynamicInteraction, each state change is resisted by `friction`
   - `apply_change(proposed: &FacesState) -> ChangeResult`:
     - If friction > 0.8: reject change (return `ChangeResult::Rejected`)
     - If friction 0.3-0.8: partial change (lerp toward proposed by (1-friction))
     - If friction < 0.3: accept change fully
   - Friction decays over time: `friction *= 0.95` per tick (warming up)

5. **Define `ChangeResult` enum**:
   - `Accepted(FacesState)` — change applied fully
   - `Partial(FacesState)` — change applied partially (lerped)
   - `Rejected` — change refused (friction too high)

6. **Implement drift detection**:
   - `drift_magnitude() -> u16` — how far current has drifted from committed
   - `drift_harmonic() -> f32` — harmonic distance from committed
   - `is_drift_acceptable() -> bool` — drift < threshold (default magnitude 10)

7. **Implement recommit**:
   - `recommit() -> FacesState` — current state becomes new committed state
   - Only valid in DynamicInteraction phase
   - Returns to StaticLock after recommit

8. **Integration with Consent Gate**:
   - `from_gate(gate: &ConsentGate) -> PhaseManager` — sync phase from gate state
   - StaticLock ↔ gate.Locked, DynamicInteraction ↔ gate.Unlocked

## Testing

- Phase transitions: StaticLock → DynamicInteraction → StaticLock cycle
- Force unlock ignores min_lock_ticks
- Tick counter increments, auto-transition triggers
- Friction: high friction rejects, medium partial-accepts, low fully accepts
- Friction decay over time
- Drift detection: magnitude and harmonic correct
- Recommit: current becomes committed, returns to StaticLock
- Partial change: lerp factor correct based on friction
- Consent gate integration: phases sync correctly

## Completion Criteria

- `Phase` enum and `PhaseManager` struct in `transition.rs`
- StaticLock / DynamicInteraction two-phase model
- Keystroke friction with three outcomes (Accept/Partial/Reject)
- Friction decay, drift detection, recommit
- Consent Gate integration
- All tests pass (target: 210+ tests)
- Zero dependencies maintained
- PROGRESS.md updated
- **Gate 1 review: Joshua reviews full crate (W1-W5 complete)**
