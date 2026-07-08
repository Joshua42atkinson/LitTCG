---
description: W4 — Consent Gate state machine with lock/unlock/commit phases and nudge functions
---

# W4: Consent Gate

## Objective

Implement a mechanical Consent Gate that prevents unwanted emotional state
changes. The gate has three phases: Lock (frozen state), Unlock (changes
allowed), and Commit (state finalized). Nudge functions provide graduated
pressure when the gate is violated.

## Prerequisites

- W3 complete (FacesProfile with baseline tracking)
- Protocol layer solid (W1)

## Steps

1. **Define `GateState` enum** in new `consent.rs` module:
   - `Locked` — state is frozen, no changes accepted
   - `Unlocked` — changes allowed, not yet committed
   - `Committed` — state finalized, changes locked in

2. **Define `ConsentGate` struct**:
   - `state: GateState` — current gate phase
   - `locked_state: FacesState` — the state being protected
   - `proposed_state: Option<FacesState>` — pending change (None if no proposal)
   - `nudge_count: u8` — how many times user has been nudged
   - `max_nudges: u8` — threshold before hard refusal (default 3)
   - `auto_lock_after: u16` — ticks before auto-lock (default 100)

3. **Implement state machine transitions**:
   - `lock()` — Locked → Locked (no-op), Unlocked → Locked (discard proposal), Committed → Locked
   - `unlock()` — any → Unlocked (clear proposed_state)
   - `propose(state: FacesState)` — only valid in Unlocked, sets proposed_state
   - `commit()` — Unlocked + has proposal → Committed (proposal becomes locked_state)
   - `auto_lock_tick()` — decrement timer, auto-lock when it hits 0

4. **Implement nudge functions**:
   - `nudge_suggest() -> String` — gentle: "Consider whether this state change is needed."
   - `nudge_warn() -> String` — moderate: "This state change has been proposed {n} times. Are you sure?"
   - `nudge_refuse() -> String` — firm: "State change refused. {max_nudges} nudges exceeded. Gate locked."
   - `nudge() -> NudgeResult` — auto-selects level based on nudge_count
     - 0: Suggest
     - 1-2: Warn
     - 3+: Refuse (and lock the gate)

5. **Define `NudgeResult` enum**:
   - `Suggest(String)`
   - `Warn(String)`
   - `Refuse(String)` — gate auto-locks after this

6. **Implement gate violation detection**:
   - `is_violation(proposed: &FacesState) -> bool` — true if proposed differs from locked_state
   - `violation_magnitude(proposed: &FacesState) -> u16` — L1 magnitude of violation
   - `violation_harmonic(proposed: &FacesState) -> f32` — harmonic distance of violation

7. **Consent gate + FacesProfile integration**:
   - `from_profile(profile: &FacesProfile) -> ConsentGate` — lock to profile's baseline
   - `deviation_from_baseline() -> TransitionVector` — how far proposed is from baseline

## Testing

- State machine: all transitions, invalid transitions rejected
- Lock/Unlock/Commit cycle works end-to-end
- Propose only works when Unlocked
- Commit only works when proposal exists
- Auto-lock timer counts down and triggers lock
- Nudge levels escalate correctly (suggest → warn → refuse)
- Nudge count resets on unlock
- Violation detection: same state = no violation, different state = violation
- Violation magnitude and harmonic distance correct
- Profile integration: gate locks to baseline, deviation computed correctly
- Max nudges exceeded → gate locks, refuses further proposals

## Completion Criteria

- `consent.rs` module with `ConsentGate`, `GateState`, `NudgeResult`
- Full state machine: Locked → Unlocked → Committed → Locked cycle
- Three nudge levels with auto-escalation
- Auto-lock timer
- All tests pass (target: 190+ tests)
- Zero dependencies maintained
- PROGRESS.md updated
