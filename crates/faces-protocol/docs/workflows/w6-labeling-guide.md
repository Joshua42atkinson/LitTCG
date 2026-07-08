---
description: W6 — Ground truth labeling guide defining what text maps to what FACES state, with harmonic transition annotations
---

# W6: Labeling Guide

## Objective

Create the ground truth labeling guide that defines exactly what text maps to
what FACES state. This is the reference document for human annotators, the
evaluation harness (W7), and future FACES-Embed training data. Includes
harmonic transition annotations from the Pythagorean research.

## Prerequisites

- W1-W5 complete (Gate 1 passed — crate reviewed)
- FACES_PYTHAGOREAN_RESEARCH.md exists with ratio mappings
- FACES_VALIDATION_FRAMEWORK.md exists with labeling guide structure

## Steps

1. **Create `docs/active/FACES_LABELING_GUIDE.md`**:
   - This is the ground truth document, not code
   - Defines rules for mapping text to FACES states
   - Includes examples, edge cases, and inter-rater agreement guidelines

2. **Section: Aura labeling rules**:
   - 10 named Auras with text triggers for each
   - Neutral (245): default, no emotional signal
   - Urgent (160): errors, critical, danger, deadline, failure
   - Energetic (208): enthusiasm, excitement, fast-paced, urgent-but-positive
   - Happy (220): success, completion, gratitude, joy, achievement
   - Creative (120): explore, brainstorm, what-if, imagine, novel
   - Analytical (39): data, analysis, metrics, logical, systematic
   - Calm (27): peaceful, stable, steady, patient, resting
   - Contemplative (91): reflect, consider, philosophical, deep, meaning
   - Unconventional (177): weird, strange, unexpected, creative-odd
   - Tired (238): exhausted, depleted, slow, long-running, fatigued
   - Unnamed indices: map to nearest named Aura by hue

3. **Section: Container labeling rules**:
   - Neutral (): default, open, receptive, listening
   - Rigid ([]): formal, protocol, rules, structured, logical
   - Fluid ({}): creative, adaptive, brainstorm, flexible
   - Defensive (||): security, caution, protected, defensive, risk-averse
   - Sharp (<>): aggressive, urgent, critical, high-priority, attack

4. **Section: Focus labeling rules**:
   - Neutral (oo): default, normal attention, baseline
   - Intense (><): critical, focused, strain, concentration, urgent
   - Open (OO): surprise, new, unexpected, hyper-aware, discovery
   - Distant (..): waiting, bored, dissociated, background, idle
   - Happy (^^): success, social warmth, positive, gratitude
   - Tired (--): exhausted, depleted, low energy, sleep, fatigue

5. **Section: Action labeling rules**:
   - Withheld (_): silence, listening, data collection, stoic, idle
   - Assertive (v): command, must, directive, imperative, direction
   - Playful (~): irony, creativity, joke, non-critical, what-if
   - Thoughtful (-): consider, reflect, evaluate, processing, concern
   - Hesitant (.): maybe, might, error, uncertain, low-confidence

6. **Section: Congruence rules**:
   - Define which dimension combinations are congruent
   - Example: Urgent + Sharp + Intense + Assertive = congruent (all urgent)
   - Example: Happy + Sharp + Intense + Assertive = incongruent (happy but aggressive?)
   - Provide 20+ labeled examples with congruence assessment

7. **Section: Harmonic transition annotations**:
   - Map common text transitions to harmonic_distance values
   - "error → success" = high harmonic distance (resolution)
   - "neutral → neutral" = zero harmonic distance (stable)
   - "calm → urgent" = high harmonic distance (alarm)
   - "creative → analytical" = moderate harmonic distance (mode switch)
   - Include expected ratio_complexity for each dimension change

8. **Section: Edge cases and ambiguous text**:
   - Sarcasm: "Great, another error" → Happy aura is wrong, Urgent is right
   - Mixed signals: "I'm happy but concerned" → two states, pick dominant
   - Negation: "not urgent" → reduce Urgent confidence
   - Context-dependent: "critical" can be Urgent or Analytical depending on context
   - Provide 10+ edge case examples with correct labels

9. **Section: Inter-rater agreement guidelines**:
   - Two annotators label same text independently
   - Cohen's kappa target: ≥ 0.7 for acceptable agreement
   - Disagreements resolved by third annotator
   - Document common disagreement sources

10. **Section: Labeling format**:
    - JSONL format: `{"text": "...", "aura": 245, "container": 0, "focus": 0, "action": 0, "congruence": "neutral", "confidence": 0.8}`
    - This format is consumed by the W7 evaluation harness

## Testing

- No code tests — this is a documentation deliverable
- Review checklist:
  - All 10 named Auras have at least 5 text examples each
  - All 5 Containers have at least 5 text examples each
  - All 6 Focus variants have at least 5 text examples each
  - All 5 Actions have at least 5 text examples each
  - 20+ congruence examples
  - 10+ edge case examples
  - 10+ harmonic transition annotations
  - JSONL format documented and consistent

## Completion Criteria

- `docs/active/FACES_LABELING_GUIDE.md` created
- All dimensions covered with examples
- Congruence rules defined
- Harmonic transition annotations included
- Edge cases documented
- Inter-rater agreement guidelines written
- JSONL labeling format specified
- **Gate 2: Joshua approves labeling guide before W7**
