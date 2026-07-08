---
description: W3 — Sentence segmentation, state history, and FacesProfile baseline tracking
---

# W3: Multi-Sentence + FacesProfile

## Objective

Add multi-sentence text handling (segment input, detect per sentence,
aggregate results) and implement `FacesProfile` for tracking user baseline
and state history over time.

## Prerequisites

- W2 complete (scored detection with `DetectionResult`)
- `detect.rs` has `detect_scored()` returning `DetectionResult`

## Steps

1. **Implement sentence segmentation** in new `segment.rs` module:
   - Split on `.`, `!`, `?`, `;`, newlines
   - Trim whitespace, discard empty segments
   - Return `Vec<&str>` of sentence slices
   - Handle abbreviations minimally (don't split on "Mr." "Dr." etc. — keep simple)

2. **Implement multi-sentence detection** in `detect.rs`:
   - `detect_multi(text: &str) -> Vec<DetectionResult>` — per-sentence results
   - `detect_aggregate(text: &str) -> DetectionResult` — weighted aggregate
   - Aggregation: later sentences weighted higher (recency bias)
   - Confidence: weighted average across sentences, boosted if multiple sentences agree

3. **Define `FacesProfile` struct** in new `profile.rs` module:
   - `baseline: FacesState` — the user's typical state (VAAM-set or neutral)
   - `history: Vec<HistoryEntry>` — ring buffer of recent states (max 64)
   - `decay_rate: f32` — how fast baseline reverts to neutral (default 0.01)
   - `baseline_confidence: f32` — how well-established the baseline is

4. **Define `HistoryEntry` struct**:
   - `state: FacesState`
   - `timestamp: u64` — simple counter or epoch seconds (no chrono dep)
   - `confidence: f32` — from DetectionResult
   - `source: DetectionMethod` — how this state was detected

5. **Implement `FacesProfile` methods**:
   - `new()` — neutral baseline, empty history
   - `with_baseline(state: FacesState)` — set initial baseline (VAAM bridge)
   - `observe(&mut self, result: &DetectionResult)` — add to history, update baseline
   - `current_baseline() -> FacesState` — computed baseline from history + decay
   - `deviation(&self, state: &FacesState) -> TransitionVector` — how far from baseline
   - `stability() -> f32` — how consistent recent states are (low variance = stable)

6. **Baseline update algorithm**:
   - Exponential moving average: `baseline = lerp(baseline, observed, decay_rate)`
   - If confidence is high, increase decay_rate temporarily (learn faster)
   - If confidence is low, ignore (don't update baseline from noise)
   - Baseline confidence increases with number of observations

7. **History ring buffer**:
   - Fixed-size array (64 entries) with head index
   - No heap allocation — use array + count, not Vec
   - `oldest()` and `newest()` accessors
   - `recent(n: usize) -> &[HistoryEntry]` — last n entries

## Testing

- Sentence segmentation: simple splits, multiple delimiters, empty input, no delimiters
- Multi-sentence detection: each sentence gets its own DetectionResult
- Aggregation: recency bias, agreement boost, single-sentence fallback
- FacesProfile: baseline starts neutral, updates with observations, decay works
- History: ring buffer wraps correctly, oldest/newest/recent return right entries
- Stability: consistent states → high stability, varied states → low stability
- Deviation: returns correct TransitionVector from baseline to observed state

## Completion Criteria

- `segment.rs` module with sentence splitting
- `detect_multi()` and `detect_aggregate()` functions
- `profile.rs` module with `FacesProfile` and `HistoryEntry`
- Ring buffer with no heap allocation
- All tests pass (target: 170+ tests)
- Zero dependencies maintained
- PROGRESS.md updated
