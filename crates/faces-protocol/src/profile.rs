// ═══════════════════════════════════════════════════════════════════════════════
// FACES PROTOCOL — faces-protocol
// FILE:        src/profile.rs
// PURPOSE:     FacesProfile — user baseline tracking and state history
// ═══════════════════════════════════════════════════════════════════════════════
//
// FACES PROFILE — TRACKING THE USER OVER TIME
//
// While FACES detection (`detect.rs`) responds to what was said *right now*,
// FacesProfile tracks the user's *typical* emotional state over time. This
// is the baseline that VAAM (Vocabulary, Autonomy, Acquisition, Mastery)
// would set from long-term observation, and that the Consent Gate can
// override.
//
// The architecture is layered:
//   Layer 1: VAAM Baseline (zero-compute) — VaamProfile → FacesProfile
//   Layer 2: Text Detection (detect.rs)   — current message → override
//   Layer 3: FACES-Embed (NPU, future)    — neural classification
//   Layer 4: Consent Gate (user override) — always supreme
//
// FacesProfile implements Layer 1 and the history tracking that supports
// Layer 2+ integration. It does NOT depend on VAAM types — it takes plain
// f32/u8 inputs, preserving the zero-dependency constraint.
//
// RING BUFFER — NO HEAP ALLOCATION
//
// The history uses a fixed-size array (64 entries) with a head index and
// count. When the buffer is full, new entries overwrite the oldest. This
// is suitable for NPU and embedded targets where heap allocation is
// undesirable.
//
// BASELINE UPDATE — EXPONENTIAL MOVING AVERAGE
//
// The baseline is updated using an EMA:
//   baseline = lerp(baseline, observed, effective_decay)
//
// Where effective_decay is adjusted by confidence:
//   - High confidence → learn faster (higher decay)
//   - Low confidence → ignore (don't update from noise)
//   - Baseline confidence increases with observation count
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::detect::{DetectionMethod, DetectionResult};
use crate::protocol::FacesState;
use crate::transition::TransitionVector;

/// Maximum number of history entries stored in the ring buffer.
pub const HISTORY_CAPACITY: usize = 64;

/// A single entry in the FACES state history.
///
/// Records a FACES state along with when it was observed, how confident
/// the detection was, and what method produced it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HistoryEntry {
    /// The observed FACES state.
    pub state: FacesState,
    /// Timestamp — a simple monotonically increasing counter (no chrono dep).
    pub timestamp: u64,
    /// Detection confidence (0.0 to 1.0).
    pub confidence: f32,
    /// How this state was detected.
    pub source: DetectionMethod,
}

/// User baseline and state history tracker.
///
/// Maintains a ring buffer of recent FACES states and computes a
/// baseline (exponential moving average) that represents the user's
/// typical emotional state. The baseline can be set externally (e.g.,
/// from VAAM) or learned from observations.
///
/// # Example
///
/// ```
/// use faces_protocol::profile::FacesProfile;
/// use faces_protocol::detect::detect_scored;
///
/// let mut profile = FacesProfile::new();
///
/// // Observe some text
/// let result = detect_scored("Critical error! System crash!");
/// profile.observe(&result);
///
/// // The baseline shifts toward the observed state
/// let baseline = profile.current_baseline();
/// assert!(profile.observation_count() == 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FacesProfile {
    /// The user's typical FACES state (EMA of observations).
    baseline: FacesState,
    /// Ring buffer of recent state observations.
    history: [HistoryEntry; HISTORY_CAPACITY],
    /// Index where the next entry will be written.
    head: usize,
    /// Number of entries currently stored (caps at HISTORY_CAPACITY).
    count: usize,
    /// Base rate at which baseline moves toward observations.
    decay_rate: f32,
    /// How well-established the baseline is (0.0 = new, 1.0 = confident).
    baseline_confidence: f32,
    /// Total number of observations ever recorded.
    total_observations: u64,
    /// Monotonic timestamp counter (incremented per observe()).
    clock: u64,
}

impl FacesProfile {
    /// Create a new profile with a neutral baseline and empty history.
    pub fn new() -> Self {
        Self {
            baseline: FacesState::neutral(),
            history: [HistoryEntry {
                state: FacesState::neutral(),
                timestamp: 0,
                confidence: 0.0,
                source: DetectionMethod::Keyword,
            }; HISTORY_CAPACITY],
            head: 0,
            count: 0,
            decay_rate: 0.01,
            baseline_confidence: 0.0,
            total_observations: 0,
            clock: 0,
        }
    }

    /// Create a profile with a pre-set baseline (e.g., from VAAM).
    ///
    /// This is how the VAAM→FACES bridge sets the initial baseline
    /// without depending on VAAM types — the caller converts VAAM
    /// metrics to a FacesState externally.
    pub fn with_baseline(state: FacesState) -> Self {
        let mut profile = Self::new();
        profile.baseline = state;
        profile.baseline_confidence = 0.5;
        profile
    }

    /// Set the decay rate (how fast baseline adapts to new observations).
    ///
    /// Lower values = slower adaptation (more stable baseline).
    /// Higher values = faster adaptation (more responsive baseline).
    /// Default is 0.01 (very slow adaptation).
    pub fn set_decay_rate(&mut self, rate: f32) {
        self.decay_rate = rate.clamp(0.0, 1.0);
    }

    /// Set the baseline state directly (e.g., from VAAM or user override).
    ///
    /// This is the external setter for the baseline, complementing
    /// `with_baseline()` which sets it at construction time.
    pub fn set_baseline(&mut self, state: FacesState) {
        self.baseline = state;
        self.baseline_confidence = 0.5;
    }

    /// Observe a detection result and update the profile.
    ///
    /// Adds the result to history and updates the baseline via EMA.
    /// Low-confidence observations (< 0.1) are recorded but do not
    /// update the baseline (noise filtering).
    pub fn observe(&mut self, result: &DetectionResult) {
        self.clock += 1;
        self.total_observations += 1;

        let entry = HistoryEntry {
            state: result.state,
            timestamp: self.clock,
            confidence: result.overall_confidence,
            source: result.method,
        };

        self.history[self.head] = entry;
        self.head = (self.head + 1) % HISTORY_CAPACITY;
        if self.count < HISTORY_CAPACITY {
            self.count += 1;
        }

        if result.overall_confidence >= 0.1 {
            let effective_decay = self.decay_rate * (0.5 + result.overall_confidence * 0.5);
            self.baseline = lerp_state(&self.baseline, &result.state, effective_decay);

            self.baseline_confidence = (self.baseline_confidence + 0.01).min(1.0);
        }
    }

    /// Get the current computed baseline state.
    pub fn current_baseline(&self) -> FacesState {
        self.baseline
    }

    /// Get the baseline confidence (how well-established it is).
    pub fn baseline_confidence(&self) -> f32 {
        self.baseline_confidence
    }

    /// Get the total number of observations ever recorded.
    pub fn observation_count(&self) -> u64 {
        self.total_observations
    }

    /// Get the number of entries currently in the history buffer.
    pub fn history_len(&self) -> usize {
        self.count
    }

    /// Get the oldest entry in the history, or None if empty.
    pub fn oldest(&self) -> Option<&HistoryEntry> {
        if self.count == 0 {
            return None;
        }
        if self.count < HISTORY_CAPACITY {
            Some(&self.history[0])
        } else {
            Some(&self.history[self.head])
        }
    }

    /// Get the newest entry in the history, or None if empty.
    pub fn newest(&self) -> Option<&HistoryEntry> {
        if self.count == 0 {
            return None;
        }
        let newest_idx = if self.head == 0 {
            HISTORY_CAPACITY - 1
        } else {
            self.head - 1
        };
        Some(&self.history[newest_idx])
    }

    /// Get the last `n` entries, ordered oldest to newest.
    ///
    /// Returns fewer than `n` if the buffer doesn't have enough entries.
    pub fn recent(&self, n: usize) -> Vec<&HistoryEntry> {
        if self.count == 0 || n == 0 {
            return Vec::new();
        }

        let take = n.min(self.count);
        let mut result = Vec::with_capacity(take);

        let start = if self.count < HISTORY_CAPACITY {
            self.count.saturating_sub(take)
        } else {
            (self.head + HISTORY_CAPACITY - take) % HISTORY_CAPACITY
        };

        for i in 0..take {
            let idx = (start + i) % HISTORY_CAPACITY;
            result.push(&self.history[idx]);
        }

        result
    }

    /// Calculate how far a given state deviates from the baseline.
    ///
    /// Returns a `TransitionVector` representing the emotional distance
    /// from the user's typical state to the given state.
    pub fn deviation(&self, state: &FacesState) -> TransitionVector {
        TransitionVector::between(&self.baseline, state)
    }

    /// Calculate stability — how consistent recent states are.
    ///
    /// Returns a value from 0.0 (highly volatile) to 1.0 (perfectly stable).
    /// Computed as 1.0 minus normalized average transition magnitude
    /// across recent history entries.
    pub fn stability(&self) -> f32 {
        if self.count < 2 {
            return 1.0;
        }

        let recent = self.recent(self.count.min(16));
        let mut total_magnitude: u32 = 0;
        let mut transitions = 0u32;

        for i in 1..recent.len() {
            let tv = TransitionVector::between(&recent[i - 1].state, &recent[i].state);
            total_magnitude += tv.magnitude() as u32;
            transitions += 1;
        }

        if transitions == 0 {
            return 1.0;
        }

        let avg_magnitude = total_magnitude as f32 / transitions as f32;
        let normalized = (avg_magnitude / 50.0).min(1.0);
        1.0 - normalized
    }
}

impl Default for FacesProfile {
    fn default() -> Self {
        Self::new()
    }
}

/// Linear interpolation between two FacesStates.
///
/// This is a local version that doesn't require importing the full
/// transition::lerp (which uses circular interpolation for enums).
/// For the baseline EMA, simple linear interpolation on the byte
/// values is sufficient and faster.
fn lerp_state(from: &FacesState, to: &FacesState, t: f32) -> FacesState {
    let t = t.clamp(0.0, 1.0);

    let aura_from = from.aura.index() as f32;
    let aura_to = to.aura.index() as f32;
    let aura_new = (aura_from + (aura_to - aura_from) * t).round() as u8;

    let container_from = from.container as u8 as f32;
    let container_to = to.container as u8 as f32;
    let container_new = (container_from + (container_to - container_from) * t).round() as u8;

    let focus_from = from.focus as u8 as f32;
    let focus_to = to.focus as u8 as f32;
    let focus_new = (focus_from + (focus_to - focus_from) * t).round() as u8;

    let action_from = from.action as u8 as f32;
    let action_to = to.action as u8 as f32;
    let action_new = (action_from + (action_to - action_from) * t).round() as u8;

    FacesState::from_bytes([aura_new, container_new, focus_new, action_new])
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::aura::Aura;
    use crate::container::Container;
    use crate::focus::Focus;

    fn make_result(state: FacesState, confidence: f32) -> DetectionResult {
        DetectionResult {
            state,
            aura_confidence: confidence,
            container_confidence: confidence,
            focus_confidence: confidence,
            action_confidence: confidence,
            overall_confidence: confidence,
            congruence: crate::detect::Congruence::Congruent,
            method: DetectionMethod::Keyword,
        }
    }

    #[test]
    fn test_new_profile_neutral_baseline() {
        let profile = FacesProfile::new();
        assert_eq!(profile.current_baseline(), FacesState::neutral());
        assert_eq!(profile.observation_count(), 0);
        assert_eq!(profile.history_len(), 0);
        assert_eq!(profile.baseline_confidence(), 0.0);
    }

    #[test]
    fn test_with_baseline() {
        let state = FacesState::new(
            Aura::CREATIVE,
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let profile = FacesProfile::with_baseline(state);
        assert_eq!(profile.current_baseline(), state);
        assert!(profile.baseline_confidence() > 0.0);
    }

    #[test]
    fn test_observe_adds_to_history() {
        let mut profile = FacesProfile::new();
        let state = FacesState::new(
            Aura::URGENT,
            Container::Sharp,
            Focus::Intense,
            Action::Assertive,
        );
        let result = make_result(state, 0.8);

        profile.observe(&result);

        assert_eq!(profile.history_len(), 1);
        assert_eq!(profile.observation_count(), 1);
        assert_eq!(profile.newest().unwrap().state, state);
    }

    #[test]
    fn test_observe_updates_baseline() {
        let mut profile = FacesProfile::new();
        let initial_baseline = profile.current_baseline();

        let state = FacesState::new(
            Aura::URGENT,
            Container::Sharp,
            Focus::Intense,
            Action::Assertive,
        );
        let result = make_result(state, 0.9);

        profile.observe(&result);

        let new_baseline = profile.current_baseline();
        assert_ne!(new_baseline, initial_baseline, "Baseline should shift toward observation");
    }

    #[test]
    fn test_low_confidence_does_not_update_baseline() {
        let mut profile = FacesProfile::new();
        let initial_baseline = profile.current_baseline();

        let state = FacesState::new(
            Aura::URGENT,
            Container::Sharp,
            Focus::Intense,
            Action::Assertive,
        );
        let result = make_result(state, 0.05);

        profile.observe(&result);

        assert_eq!(
            profile.current_baseline(),
            initial_baseline,
            "Low confidence should not update baseline"
        );
        assert_eq!(profile.history_len(), 1, "But should still be recorded in history");
    }

    #[test]
    fn test_ring_buffer_wraps() {
        let mut profile = FacesProfile::new();

        for i in 0..(HISTORY_CAPACITY + 10) {
            let state = FacesState::new(
                Aura::from_index((i % 256) as u8),
                Container::Neutral,
                Focus::Neutral,
                Action::Withheld,
            );
            let result = make_result(state, 0.5);
            profile.observe(&result);
        }

        assert_eq!(profile.history_len(), HISTORY_CAPACITY);
        assert_eq!(profile.observation_count(), (HISTORY_CAPACITY + 10) as u64);
    }

    #[test]
    fn test_oldest_newest() {
        let mut profile = FacesProfile::new();

        let state1 = FacesState::new(Aura::NEUTRAL, Container::Neutral, Focus::Neutral, Action::Withheld);
        let state2 = FacesState::new(Aura::CREATIVE, Container::Fluid, Focus::Open, Action::Playful);

        profile.observe(&make_result(state1, 0.5));
        profile.observe(&make_result(state2, 0.5));

        assert_eq!(profile.oldest().unwrap().state, state1);
        assert_eq!(profile.newest().unwrap().state, state2);
    }

    #[test]
    fn test_oldest_newest_empty() {
        let profile = FacesProfile::new();
        assert!(profile.oldest().is_none());
        assert!(profile.newest().is_none());
    }

    #[test]
    fn test_oldest_after_wrap() {
        let mut profile = FacesProfile::new();

        for i in 0..(HISTORY_CAPACITY + 5) {
            let state = FacesState::new(
                Aura::from_index((i % 256) as u8),
                Container::Neutral,
                Focus::Neutral,
                Action::Withheld,
            );
            profile.observe(&make_result(state, 0.5));
        }

        let oldest = profile.oldest().unwrap();
        let newest = profile.newest().unwrap();

        assert!(newest.timestamp > oldest.timestamp);
    }

    #[test]
    fn test_recent_returns_correct_entries() {
        let mut profile = FacesProfile::new();

        for i in 0..5 {
            let state = FacesState::new(
                Aura::from_index((200 + i) as u8),
                Container::Neutral,
                Focus::Neutral,
                Action::Withheld,
            );
            profile.observe(&make_result(state, 0.5));
        }

        let recent = profile.recent(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].state.aura.index(), 202);
        assert_eq!(recent[1].state.aura.index(), 203);
        assert_eq!(recent[2].state.aura.index(), 204);
    }

    #[test]
    fn test_recent_more_than_available() {
        let mut profile = FacesProfile::new();
        profile.observe(&make_result(FacesState::neutral(), 0.5));

        let recent = profile.recent(10);
        assert_eq!(recent.len(), 1);
    }

    #[test]
    fn test_recent_empty() {
        let profile = FacesProfile::new();
        let recent = profile.recent(5);
        assert!(recent.is_empty());
    }

    #[test]
    fn test_recent_zero() {
        let mut profile = FacesProfile::new();
        profile.observe(&make_result(FacesState::neutral(), 0.5));

        let recent = profile.recent(0);
        assert!(recent.is_empty());
    }

    #[test]
    fn test_recent_after_wrap() {
        let mut profile = FacesProfile::new();

        for i in 0..(HISTORY_CAPACITY + 3) {
            let state = FacesState::new(
                Aura::from_index((i % 256) as u8),
                Container::Neutral,
                Focus::Neutral,
                Action::Withheld,
            );
            profile.observe(&make_result(state, 0.5));
        }

        let recent = profile.recent(5);
        assert_eq!(recent.len(), 5);
        for i in 0..4 {
            assert!(
                recent[i + 1].timestamp > recent[i].timestamp,
                "Recent entries should be ordered oldest to newest"
            );
        }
    }

    #[test]
    fn test_deviation_from_baseline() {
        let baseline = FacesState::new(
            Aura::NEUTRAL,
            Container::Neutral,
            Focus::Neutral,
            Action::Withheld,
        );
        let profile = FacesProfile::with_baseline(baseline);

        let observed = FacesState::new(
            Aura::URGENT,
            Container::Sharp,
            Focus::Intense,
            Action::Assertive,
        );

        let dev = profile.deviation(&observed);
        assert!(dev.magnitude() > 0, "Deviation from neutral to urgent should be non-zero");
    }

    #[test]
    fn test_deviation_zero_for_identical() {
        let state = FacesState::new(
            Aura::CREATIVE,
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );
        let profile = FacesProfile::with_baseline(state);

        let dev = profile.deviation(&state);
        assert_eq!(dev.magnitude(), 0);
    }

    #[test]
    fn test_stability_consistent_states() {
        let mut profile = FacesProfile::new();

        let state = FacesState::new(
            Aura::CREATIVE,
            Container::Fluid,
            Focus::Happy,
            Action::Playful,
        );

        for _ in 0..10 {
            profile.observe(&make_result(state, 0.8));
        }

        let stability = profile.stability();
        assert!(
            stability > 0.99,
            "Consistent states should yield high stability, got {}",
            stability
        );
    }

    #[test]
    fn test_stability_varied_states() {
        let mut profile = FacesProfile::new();

        let states = [
            FacesState::new(Aura::URGENT, Container::Sharp, Focus::Intense, Action::Assertive),
            FacesState::new(Aura::CREATIVE, Container::Fluid, Focus::Open, Action::Playful),
            FacesState::new(Aura::CALM, Container::Rigid, Focus::Distant, Action::Thoughtful),
            FacesState::new(Aura::TIRED, Container::Defensive, Focus::Tired, Action::Hesitant),
            FacesState::new(Aura::ENERGETIC, Container::Neutral, Focus::Intense, Action::Assertive),
        ];

        for s in &states {
            profile.observe(&make_result(*s, 0.8));
        }

        let stability = profile.stability();
        assert!(
            stability < 0.9,
            "Varied states should yield lower stability, got {}",
            stability
        );
    }

    #[test]
    fn test_stability_single_entry() {
        let mut profile = FacesProfile::new();
        profile.observe(&make_result(FacesState::neutral(), 0.5));

        assert_eq!(profile.stability(), 1.0, "Single entry should be perfectly stable");
    }

    #[test]
    fn test_stability_empty() {
        let profile = FacesProfile::new();
        assert_eq!(profile.stability(), 1.0, "Empty profile should be perfectly stable");
    }

    #[test]
    fn test_set_decay_rate() {
        let mut profile = FacesProfile::new();
        profile.set_decay_rate(0.5);
        profile.set_decay_rate(2.0);
        assert!(profile.decay_rate <= 1.0, "Decay rate should be clamped to [0,1]");
    }

    #[test]
    fn test_baseline_confidence_grows() {
        let mut profile = FacesProfile::new();
        assert_eq!(profile.baseline_confidence(), 0.0);

        let state = FacesState::new(Aura::CREATIVE, Container::Fluid, Focus::Happy, Action::Playful);
        for _ in 0..10 {
            profile.observe(&make_result(state, 0.8));
        }

        assert!(
            profile.baseline_confidence() > 0.05,
            "Baseline confidence should grow with observations"
        );
    }

    #[test]
    fn test_high_confidence_learns_faster() {
        let mut profile_slow = FacesProfile::new();
        profile_slow.set_decay_rate(0.01);

        let mut profile_fast = FacesProfile::new();
        profile_fast.set_decay_rate(0.01);

        let target = FacesState::new(
            Aura::URGENT,
            Container::Sharp,
            Focus::Intense,
            Action::Assertive,
        );

        profile_slow.observe(&make_result(target, 0.1));
        profile_fast.observe(&make_result(target, 1.0));

        let slow_dev = profile_slow.deviation(&target).magnitude();
        let fast_dev = profile_fast.deviation(&target).magnitude();

        assert!(
            fast_dev < slow_dev,
            "High confidence should move baseline more (fast_dev={}, slow_dev={})",
            fast_dev,
            slow_dev
        );
    }

    #[test]
    fn test_default_trait() {
        let profile = FacesProfile::default();
        assert_eq!(profile.current_baseline(), FacesState::neutral());
    }
}
