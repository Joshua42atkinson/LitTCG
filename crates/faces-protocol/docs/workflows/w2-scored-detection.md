---
description: W2 — Upgrade detect.rs from first-match to confidence-scored detection with DetectionResult struct
---

# W2: Scored Detection

## Objective

Replace the current first-match-wins keyword detection in `detect.rs` with a
confidence-scored system that evaluates all candidates per dimension and
returns a `DetectionResult` struct with per-dimension scores.

## Prerequisites

- W1 complete (protocol layer solid, 133 tests passing)
- `detect.rs` exists with flat keyword matching (the placeholder to replace)

## Steps

1. **Define `DetectionResult` struct** in `detect.rs`:
   - `state: FacesState` — the best-guess state
   - `aura_confidence: f32` — 0.0 to 1.0
   - `container_confidence: f32`
   - `focus_confidence: f32`
   - `action_confidence: f32`
   - `overall_confidence: f32` — weighted average
   - `congruence: Congruence` — Congruent / Incongruent / Neutral
   - `method: DetectionMethod` — Keyword / Heuristic / Neural (future)

2. **Define `Congruence` enum**:
   - `Congruent` — dimensions agree (e.g., urgent aura + sharp container + intense focus + assertive action)
   - `Incongruent` — dimensions conflict (e.g., happy aura + sharp container)
   - `Neutral` — not enough signal to determine

3. **Implement scored keyword detection**:
   - For each dimension, scan ALL keyword sets, not just first match
   - Count hits per candidate, score = hits / max_hits for that dimension
   - Highest score wins; confidence = winner_score / sum(all_scores)
   - If no hits, use neutral/default with confidence 0.0

4. **Implement congruence detection**:
   - Define congruence rules: which Aura+Container+Focus+Action combos are congruent
   - Simple rule-based approach: map each dimension to valence (positive/negative/neutral)
   - If all valences agree → Congruent; if any conflict → Incongruent; else Neutral

5. **Add `detect_scored()` function**:
   - Returns `DetectionResult` instead of `FacesState`
   - Keep `detect_faces()` as a thin wrapper that returns `detect_scored().state`
   - This preserves backward compatibility with existing tests

6. **Expand keyword coverage**:
   - Add more keywords per dimension (at least 15 per category)
   - Add intensity modifiers ("very", "slightly", "extremely") that boost/lower confidence
   - Add negation handling ("not urgent", "no error") — reduce confidence for negated keywords

7. **Add `DetectionMethod` enum**:
   - `Keyword` — current keyword-based detection
   - `Heuristic` — rule-based congruence adjustment
   - `Neural` — placeholder for future FACES-Embed NPU detection

## Testing

- All existing `detect.rs` tests must still pass (backward compat)
- New tests for:
  - `DetectionResult` struct construction and field access
  - Scored detection returns correct confidence values
  - Congruence detection for congruent/incongruent/neutral cases
  - Intensity modifiers boost/lower confidence
  - Negation handling reduces confidence
  - Empty string returns neutral with 0.0 confidence
  - Multi-keyword sentences accumulate score correctly
  - `detect_faces()` still returns same results as before (wrapper test)

## Completion Criteria

- `detect_scored()` returns `DetectionResult` with per-dimension confidence
- `detect_faces()` unchanged behavior (backward compatible)
- Congruence detection identifies congruent vs incongruent states
- All tests pass (target: 150+ tests)
- Zero dependencies maintained
- PROGRESS.md updated
