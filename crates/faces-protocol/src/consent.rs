// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/consent.rs
// PURPOSE:     Mechanical Consent Gate — prevents unwanted emotive state changes
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE MECHANICAL CONSENT GATE
//
// FACES detection produces emotive states automatically. But the user is
// always supreme — they must be able to prevent, review, and override any
// state change. The Consent Gate is the mechanical enforcement of that
// supremacy.
//
// THREE PHASES
//
//   1. LOCKED     — The current FACES state is frozen. No changes accepted.
//                   This is the default resting state. The gate protects
//                   the user's emotional context from unwanted drift.
//
//   2. UNLOCKED   — Changes are allowed but not yet finalized. A proposed
//                   state can be reviewed, nudged, or rejected. The user
//                   (or an agent acting on their behalf) has opened the gate
//                   for negotiation.
//
//   3. COMMITTED  — A proposed state has been finalized. The gate locks
//                   around the new state, returning to protection mode.
//                   This is the "I accept this change" action.
//
// NUDGE FUNCTIONS — GRADUATED PRESSURE
//
// When a state change is proposed that violates the locked state, the gate
// doesn't silently accept or reject — it nudges. This mirrors the Socratic
// method: gentle questioning before hard refusal.
//
//   Nudge 0: Suggest  — "Consider whether this state change is needed."
//   Nudge 1-2: Warn   — "This has been proposed N times. Are you sure?"
//   Nudge 3+: Refuse  — "State change refused. Gate locked."
//
// After refusal, the gate auto-locks and refuses further proposals until
// explicitly unlocked.
//
// AUTO-LOCK TIMER
//
// If the gate is unlocked but no commit happens within `auto_lock_after`
// ticks, it auto-locks. This prevents an unlocked gate from being left
// open indefinitely — a safety mechanism for interactive sessions.
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::profile::FacesProfile;
use crate::protocol::FacesState;
use crate::transition::TransitionVector;

// ── GateState ────────────────────────────────────────────────────────────────

/// The three-phase state of the Consent Gate.
///
/// Represents where the gate is in its lock/unlock/commit cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    /// State is frozen — no changes accepted. Default resting phase.
    Locked,
    /// Changes allowed — a proposal may be set, reviewed, and committed.
    Unlocked,
    /// State finalized — proposal has been committed, gate re-locks.
    Committed,
}

// ── NudgeResult ──────────────────────────────────────────────────────────────

/// Result of a nudge — graduated pressure on the user to reconsider.
///
/// Each level increases in firmness. After `Refuse`, the gate auto-locks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NudgeResult {
    /// Gentle suggestion (nudge count 0).
    Suggest(String),
    /// Moderate warning (nudge count 1 to max_nudges-1).
    Warn(String),
    /// Firm refusal (nudge count >= max_nudges). Gate auto-locks.
    Refuse(String),
}

// ── ConsentGate ──────────────────────────────────────────────────────────────

/// Mechanical Consent Gate for FACES state changes.
///
/// Prevents unwanted emotive state changes by requiring explicit unlock,
/// proposal, and commit. Provides graduated nudge functions when violations
/// are attempted.
///
/// # Example
///
/// ```
/// use faces_protocol::consent::{ConsentGate, GateState};
/// use faces_protocol::FacesState;
///
/// let mut gate = ConsentGate::new(FacesState::neutral());
/// assert_eq!(gate.gate_state(), GateState::Locked);
///
/// gate.unlock();
/// assert_eq!(gate.gate_state(), GateState::Unlocked);
///
/// gate.propose(FacesState::default());
/// assert!(gate.has_proposal());
///
/// gate.commit();
/// assert_eq!(gate.gate_state(), GateState::Committed);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ConsentGate {
    /// Current gate phase (Locked, Unlocked, Committed).
    state: GateState,
    /// The FACES state being protected.
    locked_state: FacesState,
    /// Pending state change (None if no proposal).
    proposed_state: Option<FacesState>,
    /// How many times the user has been nudged about the current proposal.
    nudge_count: u8,
    /// Threshold before hard refusal (default 3).
    max_nudges: u8,
    /// Ticks remaining before auto-lock (only counts when Unlocked).
    auto_lock_timer: u16,
    /// Initial auto-lock timeout value.
    auto_lock_after: u16,
}

impl ConsentGate {
    /// Create a new ConsentGate locked around the given state.
    pub fn new(state: FacesState) -> Self {
        Self {
            state: GateState::Locked,
            locked_state: state,
            proposed_state: None,
            nudge_count: 0,
            max_nudges: 3,
            auto_lock_timer: 100,
            auto_lock_after: 100,
        }
    }

    /// Create a ConsentGate locked to a FacesProfile's baseline.
    ///
    /// This is the primary integration point between the profile (which
    /// tracks the user's typical state) and the gate (which protects it).
    pub fn from_profile(profile: &FacesProfile) -> Self {
        Self::new(profile.current_baseline())
    }

    /// Get the current gate phase.
    pub fn gate_state(&self) -> GateState {
        self.state
    }

    /// Get the locked (protected) state.
    pub fn locked_state(&self) -> FacesState {
        self.locked_state
    }

    /// Get the proposed state, if any.
    pub fn proposed_state(&self) -> Option<FacesState> {
        self.proposed_state
    }

    /// Check if there is a pending proposal.
    pub fn has_proposal(&self) -> bool {
        self.proposed_state.is_some()
    }

    /// Get the current nudge count.
    pub fn nudge_count(&self) -> u8 {
        self.nudge_count
    }

    /// Set the max nudges threshold.
    pub fn set_max_nudges(&mut self, max: u8) {
        self.max_nudges = max.max(1);
    }

    /// Set the auto-lock timeout (in ticks).
    pub fn set_auto_lock_after(&mut self, ticks: u16) {
        self.auto_lock_after = ticks;
        self.auto_lock_timer = ticks;
    }

    // ── State Machine Transitions ───────────────────────────────────────────

    /// Lock the gate. Discards any pending proposal.
    ///
    /// Valid from any state. Returns to `Locked` phase.
    pub fn lock(&mut self) {
        self.proposed_state = None;
        self.nudge_count = 0;
        self.state = GateState::Locked;
    }

    /// Unlock the gate. Allows proposals to be made.
    ///
    /// Valid from any state. Clears any existing proposal and resets nudge count.
    /// Resets the auto-lock timer.
    pub fn unlock(&mut self) {
        self.proposed_state = None;
        self.nudge_count = 0;
        self.auto_lock_timer = self.auto_lock_after;
        self.state = GateState::Unlocked;
    }

    /// Propose a new FACES state.
    ///
    /// Only valid when the gate is `Unlocked`. If the gate is `Locked` or
    /// `Committed`, the proposal is rejected and a nudge is generated instead.
    ///
    /// Returns `true` if the proposal was accepted, `false` if rejected.
    pub fn propose(&mut self, state: FacesState) -> bool {
        match self.state {
            GateState::Unlocked => {
                self.proposed_state = Some(state);
                true
            }
            GateState::Locked | GateState::Committed => false,
        }
    }

    /// Commit the proposed state.
    ///
    /// Only valid when the gate is `Unlocked` and a proposal exists.
    /// The proposed state becomes the new locked state, and the gate
    /// transitions to `Committed`.
    ///
    /// Returns `true` if the commit succeeded, `false` if conditions
    /// were not met.
    pub fn commit(&mut self) -> bool {
        if self.state != GateState::Unlocked {
            return false;
        }
        if let Some(proposed) = self.proposed_state {
            self.locked_state = proposed;
            self.proposed_state = None;
            self.nudge_count = 0;
            self.state = GateState::Committed;
            true
        } else {
            false
        }
    }

    /// Tick the auto-lock timer.
    ///
    /// Only counts down when the gate is `Unlocked`. When the timer
    /// reaches zero, the gate auto-locks (discarding any proposal).
    ///
    /// Returns `true` if the gate auto-locked on this tick.
    pub fn auto_lock_tick(&mut self) -> bool {
        if self.state != GateState::Unlocked {
            return false;
        }
        if self.auto_lock_timer > 0 {
            self.auto_lock_timer -= 1;
        }
        if self.auto_lock_timer == 0 {
            self.lock();
            return true;
        }
        false
    }

    // ── Nudge Functions ─────────────────────────────────────────────────────

    /// Generate a gentle suggestion nudge.
    pub fn nudge_suggest(&self) -> String {
        "Consider whether this state change is needed.".to_string()
    }

    /// Generate a moderate warning nudge.
    pub fn nudge_warn(&self) -> String {
        format!(
            "This state change has been proposed {} time(s). Are you sure?",
            self.nudge_count
        )
    }

    /// Generate a firm refusal nudge.
    pub fn nudge_refuse(&self) -> String {
        format!(
            "State change refused. {} nudges exceeded. Gate locked.",
            self.max_nudges
        )
    }

    /// Auto-select a nudge level based on the current nudge count.
    ///
    /// - Nudge count 0: `Suggest`
    /// - Nudge count 1 to max_nudges-1: `Warn`
    /// - Nudge count >= max_nudges: `Refuse` (gate auto-locks)
    ///
    /// Increments the nudge counter. After `Refuse`, the gate is locked.
    pub fn nudge(&mut self) -> NudgeResult {
        let result = if self.nudge_count == 0 {
            NudgeResult::Suggest(self.nudge_suggest())
        } else if self.nudge_count < self.max_nudges {
            NudgeResult::Warn(self.nudge_warn())
        } else {
            NudgeResult::Refuse(self.nudge_refuse())
        };

        self.nudge_count = self.nudge_count.saturating_add(1);

        if matches!(result, NudgeResult::Refuse(_)) {
            self.lock();
        }

        result
    }

    // ── Violation Detection ─────────────────────────────────────────────────

    /// Check if a proposed state violates the locked state.
    ///
    /// Returns `true` if the proposed state differs from the locked state.
    pub fn is_violation(&self, proposed: &FacesState) -> bool {
        proposed != &self.locked_state
    }

    /// Calculate the L1 magnitude of a violation.
    ///
    /// Returns the transition vector magnitude between the locked state
    /// and the proposed state. Zero means no violation.
    pub fn violation_magnitude(&self, proposed: &FacesState) -> u16 {
        TransitionVector::between(&self.locked_state, proposed).magnitude()
    }

    /// Calculate the harmonic distance of a violation.
    ///
    /// Uses Pythagorean ratio-weighted distance, which accounts for
    /// the fact that a 1-step shift (minor 2nd) is more jarring than
    /// a 2-step shift (major 3rd).
    pub fn violation_harmonic(&self, proposed: &FacesState) -> f32 {
        TransitionVector::between(&self.locked_state, proposed).harmonic_distance()
    }

    /// Calculate how far the proposed state deviates from the baseline.
    ///
    /// Requires a FacesProfile to compute the deviation. Returns the
    /// transition vector from the profile's baseline to the proposed state.
    pub fn deviation_from_baseline(&self, profile: &FacesProfile) -> TransitionVector {
        if let Some(proposed) = self.proposed_state {
            profile.deviation(&proposed)
        } else {
            profile.deviation(&self.locked_state)
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::aura::Aura;
    use crate::container::Container;
    use crate::focus::Focus;

    fn urgent_state() -> FacesState {
        FacesState::new(Aura::URGENT, Container::Sharp, Focus::Intense, Action::Assertive)
    }

    fn creative_state() -> FacesState {
        FacesState::new(Aura::CREATIVE, Container::Fluid, Focus::Open, Action::Playful)
    }

    // ── GateState Enum Tests ────────────────────────────────────────────────

    #[test]
    fn test_gate_state_variants() {
        let locked = GateState::Locked;
        let unlocked = GateState::Unlocked;
        let committed = GateState::Committed;
        assert_ne!(locked, unlocked);
        assert_ne!(unlocked, committed);
        assert_ne!(locked, committed);
    }

    // ── Construction Tests ──────────────────────────────────────────────────

    #[test]
    fn test_new_gate_locked() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert_eq!(gate.gate_state(), GateState::Locked);
        assert_eq!(gate.locked_state(), FacesState::neutral());
        assert!(!gate.has_proposal());
        assert_eq!(gate.nudge_count(), 0);
    }

    #[test]
    fn test_from_profile() {
        let mut profile = FacesProfile::new();
        profile.set_baseline(creative_state());

        let gate = ConsentGate::from_profile(&profile);
        assert_eq!(gate.locked_state(), profile.current_baseline());
        assert_eq!(gate.gate_state(), GateState::Locked);
    }

    // ── State Machine: Lock ─────────────────────────────────────────────────

    #[test]
    fn test_lock_from_locked() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.lock();
        assert_eq!(gate.gate_state(), GateState::Locked);
    }

    #[test]
    fn test_lock_from_unlocked_discards_proposal() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        assert!(gate.has_proposal());

        gate.lock();
        assert_eq!(gate.gate_state(), GateState::Locked);
        assert!(!gate.has_proposal());
        assert_eq!(gate.nudge_count(), 0);
    }

    #[test]
    fn test_lock_from_committed() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        gate.commit();
        assert_eq!(gate.gate_state(), GateState::Committed);

        gate.lock();
        assert_eq!(gate.gate_state(), GateState::Locked);
    }

    // ── State Machine: Unlock ───────────────────────────────────────────────

    #[test]
    fn test_unlock_from_locked() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        assert_eq!(gate.gate_state(), GateState::Unlocked);
        assert!(!gate.has_proposal());
        assert_eq!(gate.nudge_count(), 0);
    }

    #[test]
    fn test_unlock_from_committed() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        gate.commit();
        assert_eq!(gate.gate_state(), GateState::Committed);

        gate.unlock();
        assert_eq!(gate.gate_state(), GateState::Unlocked);
    }

    #[test]
    fn test_unlock_resets_nudge_count() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.nudge();
        gate.nudge();
        assert_eq!(gate.nudge_count(), 2);

        gate.unlock();
        assert_eq!(gate.nudge_count(), 0);
    }

    // ── State Machine: Propose ──────────────────────────────────────────────

    #[test]
    fn test_propose_when_unlocked() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        assert!(gate.propose(urgent_state()));
        assert!(gate.has_proposal());
        assert_eq!(gate.proposed_state(), Some(urgent_state()));
    }

    #[test]
    fn test_propose_when_locked_rejected() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        assert!(!gate.propose(urgent_state()));
        assert!(!gate.has_proposal());
    }

    #[test]
    fn test_propose_when_committed_rejected() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        gate.commit();

        assert!(!gate.propose(creative_state()));
    }

    // ── State Machine: Commit ───────────────────────────────────────────────

    #[test]
    fn test_commit_with_proposal() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        assert!(gate.commit());
        assert_eq!(gate.gate_state(), GateState::Committed);
        assert_eq!(gate.locked_state(), urgent_state());
        assert!(!gate.has_proposal());
    }

    #[test]
    fn test_commit_without_proposal_fails() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        assert!(!gate.commit());
        assert_eq!(gate.gate_state(), GateState::Unlocked);
    }

    #[test]
    fn test_commit_when_locked_fails() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        assert!(!gate.commit());
    }

    #[test]
    fn test_commit_resets_nudge_count() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        gate.nudge();
        gate.nudge();
        assert_eq!(gate.nudge_count(), 2);

        gate.commit();
        assert_eq!(gate.nudge_count(), 0);
    }

    // ── Full Lock/Unlock/Commit Cycle ───────────────────────────────────────

    #[test]
    fn test_full_cycle() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        assert_eq!(gate.gate_state(), GateState::Locked);

        gate.unlock();
        assert_eq!(gate.gate_state(), GateState::Unlocked);

        gate.propose(creative_state());
        assert!(gate.has_proposal());

        gate.commit();
        assert_eq!(gate.gate_state(), GateState::Committed);
        assert_eq!(gate.locked_state(), creative_state());

        gate.lock();
        assert_eq!(gate.gate_state(), GateState::Locked);
        assert_eq!(gate.locked_state(), creative_state());
    }

    // ── Auto-Lock Timer ─────────────────────────────────────────────────────

    #[test]
    fn test_auto_lock_does_not_trigger_when_locked() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        for _ in 0..200 {
            assert!(!gate.auto_lock_tick());
        }
        assert_eq!(gate.gate_state(), GateState::Locked);
    }

    #[test]
    fn test_auto_lock_triggers_after_timeout() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.set_auto_lock_after(10);
        gate.unlock();

        for i in 0..10 {
            let locked = gate.auto_lock_tick();
            if i < 9 {
                assert!(!locked, "Should not auto-lock before timeout at tick {}", i);
            } else {
                assert!(locked, "Should auto-lock at tick {}", i);
            }
        }
        assert_eq!(gate.gate_state(), GateState::Locked);
    }

    #[test]
    fn test_auto_lock_discards_proposal() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.set_auto_lock_after(5);
        gate.unlock();
        gate.propose(urgent_state());
        assert!(gate.has_proposal());

        for _ in 0..5 {
            gate.auto_lock_tick();
        }

        assert_eq!(gate.gate_state(), GateState::Locked);
        assert!(!gate.has_proposal());
    }

    #[test]
    fn test_auto_lock_resets_on_unlock() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.set_auto_lock_after(10);
        gate.unlock();

        for _ in 0..5 {
            gate.auto_lock_tick();
        }

        gate.unlock();
        // Timer should be reset — need another 10 ticks
        for _ in 0..9 {
            assert!(!gate.auto_lock_tick());
        }
        assert!(!gate.auto_lock_tick() == false); // 10th tick should lock
    }

    #[test]
    fn test_set_auto_lock_after() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.set_auto_lock_after(50);
        gate.unlock();

        for _ in 0..49 {
            assert!(!gate.auto_lock_tick());
        }
        assert!(gate.auto_lock_tick());
    }

    // ── Nudge Functions ─────────────────────────────────────────────────────

    #[test]
    fn test_nudge_suggest_message() {
        let gate = ConsentGate::new(FacesState::neutral());
        let msg = gate.nudge_suggest();
        assert!(msg.contains("Consider"));
        assert!(msg.contains("state change"));
    }

    #[test]
    fn test_nudge_warn_message() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.nudge_count = 2;
        let msg = gate.nudge_warn();
        assert!(msg.contains("2 time(s)"));
        assert!(msg.contains("Are you sure"));
    }

    #[test]
    fn test_nudge_refuse_message() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.max_nudges = 3;
        let msg = gate.nudge_refuse();
        assert!(msg.contains("refused"));
        assert!(msg.contains("3"));
    }

    #[test]
    fn test_nudge_suggest_level() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        let result = gate.nudge();
        assert!(matches!(result, NudgeResult::Suggest(_)));
        assert_eq!(gate.nudge_count(), 1);
    }

    #[test]
    fn test_nudge_warn_level() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.nudge(); // count → 1
        let result = gate.nudge(); // count → 2, should be Warn
        assert!(matches!(result, NudgeResult::Warn(_)));
    }

    #[test]
    fn test_nudge_refuse_level() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.max_nudges = 3;
        gate.nudge(); // 1 — suggest
        gate.nudge(); // 2 — warn
        gate.nudge(); // 3 — warn (nudge_count was 2, < max_nudges=3)
        let result = gate.nudge(); // 4 — refuse (nudge_count was 3, >= max_nudges=3)
        assert!(matches!(result, NudgeResult::Refuse(_)));
    }

    #[test]
    fn test_nudge_refuse_locks_gate() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.max_nudges = 2;
        gate.unlock();
        gate.nudge(); // 1 — suggest
        gate.nudge(); // 2 — warn (count was 1, < 2)
        let result = gate.nudge(); // 3 — refuse (count was 2, >= 2)
        assert!(matches!(result, NudgeResult::Refuse(_)));
        assert_eq!(gate.gate_state(), GateState::Locked);
    }

    #[test]
    fn test_nudge_count_increments() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        assert_eq!(gate.nudge_count(), 0);
        gate.nudge();
        assert_eq!(gate.nudge_count(), 1);
        gate.nudge();
        assert_eq!(gate.nudge_count(), 2);
    }

    #[test]
    fn test_set_max_nudges() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.set_max_nudges(5);
        assert_eq!(gate.max_nudges, 5);
    }

    #[test]
    fn test_set_max_nudges_minimum_one() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.set_max_nudges(0);
        assert_eq!(gate.max_nudges, 1);
    }

    // ── Violation Detection ─────────────────────────────────────────────────

    #[test]
    fn test_is_violation_same_state() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert!(!gate.is_violation(&FacesState::neutral()));
    }

    #[test]
    fn test_is_violation_different_state() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert!(gate.is_violation(&urgent_state()));
    }

    #[test]
    fn test_violation_magnitude_zero_for_same() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert_eq!(gate.violation_magnitude(&FacesState::neutral()), 0);
    }

    #[test]
    fn test_violation_magnitude_nonzero_for_different() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert!(gate.violation_magnitude(&urgent_state()) > 0);
    }

    #[test]
    fn test_violation_harmonic_zero_for_same() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert_eq!(gate.violation_harmonic(&FacesState::neutral()), 0.0);
    }

    #[test]
    fn test_violation_harmonic_nonzero_for_different() {
        let gate = ConsentGate::new(FacesState::neutral());
        assert!(gate.violation_harmonic(&urgent_state()) > 0.0);
    }

    // ── Profile Integration ─────────────────────────────────────────────────

    #[test]
    fn test_deviation_from_baseline_with_proposal() {
        let mut profile = FacesProfile::new();
        let baseline = FacesState::neutral();
        profile.set_baseline(baseline);

        let mut gate = ConsentGate::from_profile(&profile);
        gate.unlock();
        gate.propose(urgent_state());

        let dev = gate.deviation_from_baseline(&profile);
        assert!(dev.magnitude() > 0);
    }

    #[test]
    fn test_deviation_from_baseline_without_proposal() {
        let mut profile = FacesProfile::new();
        profile.set_baseline(FacesState::neutral());

        let gate = ConsentGate::from_profile(&profile);

        let dev = gate.deviation_from_baseline(&profile);
        assert_eq!(dev.magnitude(), 0);
    }

    // ── Edge Cases ──────────────────────────────────────────────────────────

    #[test]
    fn test_propose_overwrites_previous() {
        let mut gate = ConsentGate::new(FacesState::neutral());
        gate.unlock();
        gate.propose(urgent_state());
        assert_eq!(gate.proposed_state(), Some(urgent_state()));

        gate.propose(creative_state());
        assert_eq!(gate.proposed_state(), Some(creative_state()));
    }

    #[test]
    fn test_commit_does_not_change_locked_without_proposal() {
        let mut gate = ConsentGate::new(urgent_state());
        gate.unlock();
        gate.commit();
        assert_eq!(gate.locked_state(), urgent_state());
        assert_eq!(gate.gate_state(), GateState::Unlocked);
    }

    #[test]
    fn test_multiple_cycles() {
        let mut gate = ConsentGate::new(FacesState::neutral());

        // First cycle
        gate.unlock();
        gate.propose(urgent_state());
        gate.commit();
        assert_eq!(gate.locked_state(), urgent_state());

        // Second cycle
        gate.lock();
        gate.unlock();
        gate.propose(creative_state());
        gate.commit();
        assert_eq!(gate.locked_state(), creative_state());

        // Third cycle
        gate.lock();
        gate.unlock();
        gate.propose(FacesState::neutral());
        gate.commit();
        assert_eq!(gate.locked_state(), FacesState::neutral());
    }
}
