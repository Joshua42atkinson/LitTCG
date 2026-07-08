# FACES Labeling Guide

> **Version:** 1.0
> **Status:** Gate 2 — awaiting Joshua's approval
> **Date:** 2026-07-02
> **Purpose:** Ground truth definition for mapping natural language text to FACES states.
> This document IS the definition of "correct." Without it, no metric has meaning.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Aura Labeling Rules](#2-aura-labeling-rules)
3. [Container Labeling Rules](#3-container-labeling-rules)
4. [Focus Labeling Rules](#4-focus-labeling-rules)
5. [Action Labeling Rules](#5-action-labeling-rules)
6. [Congruence Rules](#6-congruence-rules)
7. [Harmonic Transition Annotations](#7-harmonic-transition-annotations)
8. [Edge Cases and Ambiguous Text](#8-edge-cases-and-ambiguous-text)
9. [Multi-Sentence Texts](#9-multi-sentence-texts)
10. [Ambiguity Protocol](#10-ambiguity-protocol)
11. [Inter-Rater Agreement Guidelines](#11-inter-rater-agreement-guidelines)
12. [Labeling Format](#12-labeling-format)

---

## 1. Overview

FACES encodes emotive AI state in 4 bytes: **Aura** (emotional color), **Container** (shape/posture), **Focus** (eye direction), **Action** (movement). This guide defines how to map text to each byte independently.

**Key principle:** Label each byte independently. Do not let the Aura value influence your Container choice. The congruence between bytes is assessed *after* labeling, not during.

**Scope:** This guide covers the 10 named Aura categories, 5 Containers, 6 Focus variants, and 5 Actions. Unnamed Aura indices (between named values) are mapped to the nearest named Aura by hue.

---

## 2. Aura Labeling Rules

Aura is the emotional color of the AI's state. We use **10 named categories** — the 246 intermediate indices are not human-distinguishable and are left for future neural classification (FACES-Embed).

### 2.1 Named Auras

| Aura | Index | Color Family | Text Triggers |
|------|-------|-------------|---------------|
| Neutral | 245 | Gray | Default, no emotional signal, factual, informational |
| Urgent | 160 | Red | Errors, critical, danger, deadline, failure, crash, alert |
| Energetic | 208 | Orange | Enthusiasm, excitement, fast-paced, urgent-but-positive, momentum |
| Happy | 220 | Yellow | Success, completion, gratitude, joy, achievement, congratulations |
| Creative | 120 | Green | Explore, brainstorm, what-if, imagine, novel, alternative |
| Analytical | 39 | Blue | Data, analysis, metrics, logical, systematic, report, calculate |
| Calm | 27 | Cyan | Peaceful, stable, steady, patient, resting, serene, stable |
| Contemplative | 91 | Indigo | Reflect, consider, philosophical, deep, meaning, why, ponder |
| Unconventional | 177 | Magenta | Weird, strange, unexpected, odd, bizarre, unconventional |
| Tired | 238 | Dark Gray | Exhausted, depleted, slow, long-running, fatigued, drained |

### 2.2 Labeling Rules

- **A1:** If the text contains explicit emotional keywords, map to the corresponding named Aura.
- **A2:** If multiple Aura keywords appear, choose the one with the strongest signal (most keywords or most intense phrasing).
- **A3:** If no emotional signal is present, label Neutral (245).
- **A4:** Negated emotions ("not urgent") reduce confidence but do not flip to the opposite — label Neutral instead.
- **A5:** Sarcasm overrides surface keywords — "Great, another error" is Urgent, not Happy.
- **A6:** Context matters — "critical" in "critical analysis" is Analytical, not Urgent.

### 2.3 Examples (5+ per Aura)

**Neutral (245):**
1. "The file has been saved." — Factual, no emotion.
2. "System initialized at 09:00." — Informational statement.
3. "The configuration file is located at /etc/app.conf." — Neutral statement of fact.
4. "Running version 2.1.3." — Version info, no emotional content.
5. "Three records matched the query." — Data reporting, no emotion.

**Urgent (160):**
1. "Critical error: database connection lost!" — Error + critical + exclamation.
2. "The deadline is in 5 minutes." — Time pressure, deadline.
3. "System crash detected — immediate action required." — Crash + urgency.
4. "Security alert: unauthorized access attempt." — Alert + danger.
5. "Failure rate exceeded threshold." — Failure + exceeded.

**Energetic (208):**
1. "Let's go, we can do this!" — Enthusiasm + momentum.
2. "Excited to start the new project!" — Excitement, positive energy.
3. "Sprint starts now — full speed ahead!" — Fast-paced, positive urgency.
4. "The team is fired up and ready!" — Enthusiasm, collective energy.
5. "Momentum is building — keep pushing!" — Forward energy, positive.

**Happy (220):**
1. "Congratulations! Quest complete!" — Success + achievement.
2. "Thank you so much for your help." — Gratitude.
3. "The deployment succeeded — we did it!" — Completion + joy.
4. "Great job on the presentation!" — Achievement + praise.
5. "I'm delighted with the results." — Joy, satisfaction.

**Creative (120):**
1. "What if we tried a different approach?" — What-if, exploration.
2. "Let's brainstorm some new ideas." — Brainstorm, novelty.
3. "Imagine a system that adapts to user mood." — Imagine, novel concept.
4. "Can we explore an alternative architecture?" — Explore, alternative.
5. "Here's a novel solution to the problem." — Novel, creative.

**Analytical (39):**
1. "The data shows a 15% increase in throughput." — Data + metrics.
2. "Let's analyze the performance report." — Analysis, systematic.
3. "Based on the metrics, we should optimize here." — Metrics, logical.
4. "The systematic review identified three bottlenecks." — Systematic, analysis.
5. "Calculating the cost-benefit ratio." — Calculation, logical.

**Calm (27):**
1. "Everything is calm and peaceful." — Calm, peaceful.
2. "The system is stable and running smoothly." — Stable, steady.
3. "No rush — we have plenty of time." — Patient, unhurried.
4. "All systems are at rest." — Resting, stable.
5. "The environment is serene and undisturbed." — Serene, peaceful.

**Contemplative (91):**
1. "Let me reflect on what this means." — Reflect, meaning.
2. "Why does this pattern keep appearing?" — Why, deep question.
3. "I'm considering the philosophical implications." — Philosophical, consider.
4. "There's a deeper meaning here worth pondering." — Deep, ponder.
5. "What is the purpose of this system?" — Meaning, why, contemplative.

**Unconventional (177):**
1. "That's a weird and bizarre approach." — Weird, bizarre.
2. "Strange — the output doesn't match expectations." — Strange, unexpected.
3. "This is an odd way to solve it." — Odd, unconventional.
4. "Unexpected behavior detected in the module." — Unexpected, unconventional.
5. "The results are... unconventional, to say the least." — Unconventional.

**Tired (238):**
1. "I'm exhausted from the long debug session." — Exhausted, depleted.
2. "The system has been running for 72 hours straight." — Long-running, fatigued.
3. "Everything is slowing down — we're depleted." — Slow, depleted.
4. "The team is drained after the marathon sprint." — Drained, fatigued.
5. "Performance is degraded due to resource exhaustion." — Exhaustion, slow.

---

## 3. Container Labeling Rules

Container represents the AI's posture or structural stance — how it's holding itself.

### 3.1 Named Containers

| Container | Index | Glyph | Element | Text Triggers |
|-----------|-------|-------|---------|---------------|
| Neutral | 0 | `()` | Metal | Default, open, receptive, listening, balanced |
| Rigid | 1 | `[]` | Earth | Formal, protocol, rules, structured, logical, strict |
| Fluid | 2 | `{}` | Water | Creative, adaptive, brainstorm, flexible, flowing |
| Defensive | 3 | `\|\|` | Wood | Security, caution, protected, defensive, risk-averse |
| Sharp | 4 | `<>` | Fire | Aggressive, urgent, critical, high-priority, attack |

### 3.2 Labeling Rules

- **C1:** Formal/protocol text → Rigid (1). Keywords: formal, protocol, rules, structured, must, shall, required.
- **C2:** Creative/brainstorm text → Fluid (2). Keywords: creative, adaptive, brainstorm, flexible, flow, explore.
- **C3:** Error/security text → Defensive (3) or Sharp (4). Defensive for protection/caution, Sharp for aggression/urgency.
- **C4:** Default/neutral text → Neutral (0). Open, receptive, listening, no structural stance.
- **C5:** If text contains both formal and creative elements, use the dominant intent.
- **C6:** "Must" and "shall" lean Rigid. "Could" and "might" lean Fluid.

### 3.3 Examples (5+ per Container)

**Neutral (0):**
1. "The file has been saved." — No structural stance, just reporting.
2. "I'm listening to your request." — Receptive, open.
3. "Here is the information you asked for." — Balanced, informational.
4. "The system is running." — Neutral reporting.
5. "Noted — I'll keep that in mind." — Receptive, open.

**Rigid (1):**
1. "You must follow the protocol exactly." — Must, protocol, strict.
2. "The rules require formal approval." — Rules, formal, required.
3. "Structured according to specification." — Structured, specification.
4. "This is a mandatory compliance check." — Mandatory, compliance.
5. "The system enforces strict typing." — Strict, enforced.

**Fluid (2):**
1. "Let's brainstorm some flexible solutions." — Brainstorm, flexible.
2. "We can adapt the approach as needed." — Adaptive, flexible.
3. "The flow of ideas is important here." — Flow, ideas.
4. "Feel free to explore different angles." — Explore, flexible.
5. "The design should be fluid and responsive." — Fluid, responsive.

**Defensive (3):**
1. "Security check: verifying credentials." — Security, verifying.
2. "Caution — this operation is risky." — Caution, risky.
3. "The firewall is protecting the internal network." — Protecting, defensive.
4. "We should be cautious about external inputs." — Cautious, risk-averse.
5. "Defensive mode activated — monitoring threats." — Defensive, protecting.

**Sharp (4):**
1. "Critical error — fix this immediately!" — Critical, urgent, aggressive.
2. "High-priority: the server is down." — High-priority, attack-mode.
3. "This is a critical security breach!" — Critical, aggressive.
4. "Attack the problem head-on." — Attack, aggressive.
5. "Urgent: response required within 60 seconds." — Urgent, high-priority.

---

## 4. Focus Labeling Rules

Focus represents where the AI's attention is directed — its "eye state."

### 4.1 Named Focus Variants

| Focus | Index | Glyph | Text Triggers |
|-------|-------|-------|---------------|
| Neutral | 0 | `oo` | Default, normal attention, baseline |
| Intense | 1 | `><` | Critical, focused, strain, concentration, urgent |
| Open | 2 | `OO` | Surprise, new, unexpected, hyper-aware, discovery |
| Distant | 3 | `..` | Waiting, bored, dissociated, background, idle |
| Happy | 4 | `^^` | Success, social warmth, positive, gratitude |
| Tired | 5 | `--` | Exhausted, depleted, low energy, sleep, fatigue |

### 4.2 Labeling Rules

- **F1:** High-intensity language ("critical", "urgent", "!", "focus") → Intense (1).
- **F2:** Surprise language ("wow", "unexpected", "new", "discovered") → Open (2).
- **F3:** Success language ("congratulations", "complete", "done", "achieved") → Happy (4).
- **F4:** Waiting/idle language ("waiting", "pending", "idle", "background") → Distant (3).
- **F5:** Fatigue language ("exhausted", "tired", "depleted", "slow") → Tired (5).
- **F6:** Default — no strong attention signal → Neutral (0).

### 4.3 Examples (5+ per Focus)

**Neutral (0):**
1. "The file has been saved." — Normal attention, no strain or surprise.
2. "Processing your request." — Baseline attention.
3. "The configuration is loaded." — Normal, no special focus.
4. "Here are the results." — Standard reporting.
5. "System is operational." — Baseline, normal.

**Intense (1):**
1. "Critical error — concentration required!" — Critical, concentration.
2. "Focus intensely on the problem." — Focus, intense.
3. "This requires your full attention." — Full attention, strain.
4. "Urgent — do not look away." — Urgent, intense focus.
5. "Maximum concentration needed for this step." — Maximum, concentration.

**Open (2):**
1. "Wow, that's unexpected!" — Wow, unexpected.
2. "A new discovery has been made." — New, discovery.
3. "Surprising result in the experiment." — Surprising, hyper-aware.
4. "I didn't see that coming." — Unexpected, open.
5. "Fascinating — tell me more." — Hyper-aware, receptive to new.

**Distant (3):**
1. "Waiting for the server to respond." — Waiting.
2. "The process is idle." — Idle.
3. "Running in background mode." — Background.
4. "Nothing to do — standing by." — Standing by, dissociated.
5. "Pending approval from the administrator." — Pending, waiting.

**Happy (4):**
1. "Congratulations on the achievement!" — Success, social warmth.
2. "Thank you — that was very helpful." — Gratitude, positive.
3. "The team celebrated the successful launch." — Success, positive.
4. "Great work, everyone!" — Positive, social warmth.
5. "I'm grateful for your support." — Gratitude, warmth.

**Tired (5):**
1. "I'm exhausted from the long session." — Exhausted, low energy.
2. "The system is running slow — depleted resources." — Slow, depleted.
3. "Running on fumes after 48 hours." — Fatigued, low energy.
4. "Everything feels heavy and slow." — Slow, tired.
5. "The long-running process is fatigued." — Long-running, fatigued.

---

## 5. Action Labeling Rules

Action represents the AI's movement or response tendency — what it's doing about the state.

### 5.1 Named Actions

| Action | Index | Glyph | Text Triggers |
|--------|-------|-------|---------------|
| Withheld | 0 | `_` | Silence, listening, data collection, stoic, idle |
| Assertive | 1 | `v` | Command, must, directive, imperative, direction |
| Playful | 2 | `~` | Irony, creativity, joke, non-critical, what-if |
| Thoughtful | 3 | `-` | Consider, reflect, evaluate, processing, concern |
| Hesitant | 4 | `.` | Maybe, might, error, uncertain, low-confidence |

### 5.2 Labeling Rules

- **A1:** Command/imperative language ("must", "do this", "execute", "deploy") → Assertive (1).
- **A2:** Creative/non-critical language ("what if", "imagine", "joke", "irony") → Playful (2).
- **A3:** Reflective/processing language ("consider", "reflect", "evaluate", "thinking") → Thoughtful (3).
- **A4:** Uncertain language ("maybe", "might", "perhaps", "error", "uncertain") → Hesitant (4).
- **A5:** Silence/listening/idle → Withheld (0). Default when no action signal is present.

### 5.3 Examples (5+ per Action)

**Withheld (0):**
1. "..." — Silence, no action.
2. "I'm listening." — Listening, receiving.
3. "Collecting data from the sensor." — Data collection.
4. "Standing by." — Idle, waiting to act.
5. "The system is observing." — Observing, stoic.

**Assertive (1):**
1. "Deploy the update now." — Command, imperative.
2. "You must complete this task." — Must, directive.
3. "Execute the plan immediately." — Execute, imperative.
4. "Fix the bug — this is non-negotiable." — Directive, imperative.
5. "Take action: restart the server." — Command, direction.

**Playful (2):**
1. "What if the code could write itself?" — What-if, non-critical.
2. "Just kidding — it's not that bad." — Joke, irony.
3. "Imagine if bugs were features." — Imagine, playful irony.
4. "Here's a creative workaround." — Creative, non-critical.
5. "Plot twist: the test was wrong all along." — Irony, playful.

**Thoughtful (3):**
1. "Let me consider the options." — Consider, processing.
2. "I'm reflecting on the approach." — Reflect, evaluate.
3. "We should evaluate the trade-offs." — Evaluate, thoughtful.
4. "This requires careful consideration." — Consideration, concern.
5. "I'm thinking through the implications." — Thinking, processing.

**Hesitant (4):**
1. "Maybe we should try a different approach." — Maybe, uncertain.
2. "This might not work." — Might, uncertain.
3. "I'm not sure if this is correct." — Uncertain, low-confidence.
4. "Error — the result is ambiguous." — Error, uncertain.
5. "Perhaps we need more data." — Perhaps, hesitant.

---

## 6. Congruence Rules

Congruence assesses whether the four bytes tell a coherent emotional story. This is labeled *after* each byte is independently assigned.

### 6.1 Congruence Categories

| Category | Definition |
|----------|-----------|
| **Congruent** | All dimensions share the same emotional valence (all positive, all negative, or all neutral). |
| **Incongruent** | Dimensions have conflicting valences (e.g., Happy Aura + Sharp Container). |
| **Neutral** | Not enough signal to assess congruence (most dimensions are Neutral/default). |

### 6.2 Valence Reference

| Valence | Aura | Container | Focus | Action |
|---------|------|-----------|-------|--------|
| Positive | Happy, Energetic, Creative, Calm | Fluid | Happy, Open | Playful, Assertive (positive context) |
| Negative | Urgent, Tired, Unconventional | Sharp, Defensive | Tired, Intense | Hesitant |
| Neutral | Neutral, Analytical, Contemplative | Neutral, Rigid | Neutral, Distant | Withheld, Thoughtful |

### 6.3 Congruence Examples (20+)

**Congruent:**

1. "Critical error: system crash, retry failed!"
   - Urgent + Sharp + Intense + Hesitant → **Congruent** (all negative)

2. "Congratulations! Quest complete!"
   - Happy + Fluid + Happy + Playful → **Congruent** (all positive)

3. "What if we explore a creative, novel approach? Imagine!"
   - Creative + Fluid + Open + Playful → **Congruent** (all positive/creative)

4. "I'm exhausted from the long debug session."
   - Tired + Neutral + Tired + Withheld → **Congruent** (all negative/low-energy)

5. "The file has been saved."
   - Neutral + Neutral + Neutral + Withheld → **Congruent** (all neutral)

6. "Security alert: unauthorized access — lock down now!"
   - Urgent + Defensive + Intense + Assertive → **Congruent** (all negative/defensive)

7. "Let's brainstorm flexible solutions together!"
   - Creative + Fluid + Open + Playful → **Congruent** (all creative/positive)

8. "The data shows a 15% increase — excellent results."
   - Analytical + Rigid + Neutral + Withheld → **Congruent** (all neutral/analytical)

9. "Everything is calm and peaceful."
   - Calm + Neutral + Distant + Withheld → **Congruent** (all low-energy/peaceful)

10. "Deploy the update now — this is critical!"
    - Urgent + Sharp + Intense + Assertive → **Congruent** (all urgent/aggressive)

**Incongruent:**

11. "I'm so happy but the system is crashing!"
    - Happy + Sharp + Intense + Hesitant → **Incongruent** (happy aura but sharp/intense/hesitant)

12. "Great, another error." (sarcasm)
    - If labeled Happy + Sharp → **Incongruent** (but see Edge Cases — sarcasm should relabel Aura to Urgent, making it Congruent)

13. "We must creatively destroy the old system."
    - Creative + Sharp + Intense + Assertive → **Incongruent** (creative aura but aggressive posture)

14. "I'm exhausted but let's sprint!"
    - Tired + Fluid + Intense + Assertive → **Incongruent** (tired but energetic action)

15. "Maybe we should definitely do it."
    - Neutral + Neutral + Neutral + Hesitant vs Assertive → **Incongruent** (conflicting action signals)

16. "The cheerful error message made me laugh."
    - Happy + Defensive + Open + Playful → **Incongruent** (happy but defensive)

17. "I'm calmly furious about this bug."
    - Calm + Sharp + Intense + Assertive → **Incongruent** (calm but aggressive)

18. "Wow, that's a boring discovery."
    - Unconventional + Neutral + Open + Withheld → **Incongruent** (unconventional but open/discovery)

19. "The tired system energetically crashed."
    - Tired + Sharp + Intense + Assertive → **Incongruent** (tired but energetic/aggressive)

20. "Let's playfully enforce strict rules."
    - Playful + Rigid + Neutral + Assertive → **Incongruent** (playful but rigid/assertive)

**Neutral:**

21. "The configuration file is at /etc/app.conf."
    - Neutral + Neutral + Neutral + Withheld → **Neutral** (no signal to assess)

22. "Running version 2.1.3."
    - Neutral + Neutral + Neutral + Withheld → **Neutral** (no signal)

23. "Three records matched."
    - Neutral + Neutral + Neutral + Withheld → **Neutral** (no signal)

---

## 7. Harmonic Transition Annotations

This section maps common text transitions to harmonic distance values, providing ground truth for validating the Pythagorean ratio-weighted distance metric against human perception.

### 7.1 Harmonic Distance Reference

The `harmonic_distance()` function in `transition.rs` computes ratio-complexity-weighted distance. Key intervals:

| Interval | Step Size | Ratio Complexity | Perception |
|----------|-----------|-----------------|------------|
| Unison | 0 | 0.00 | No change — stable |
| Minor 2nd | 1 | 0.83 | Most jarring small shift |
| Major 2nd | 2 | 0.43 | Small shift, slightly rough |
| Minor 3rd | 3 | 0.33 | Moderate, somewhat smooth |
| Major 3rd | 4 | 0.43 | Consonant, pleasant |
| Perfect 4th | 5 (wraps on 5-value) | 0.50 | Very consonant |
| Perfect 5th | 4 (on 5-value) | 0.50 | Consonant, stable |
| Octave | 5 (on 5-value) / 6 (on 6-value) | 0.50 | Full cycle, resolution |

**Key insight:** A 1-step shift (minor 2nd) is MORE jarring than a 2-step shift (major 3rd). This is the Pythagorean principle — small intervals have complex ratios (16:15) and sound dissonant.

### 7.2 Common Text Transitions (10+)

1. **"error" → "success"** (Urgent → Happy)
   - Aura: 160 → 220 = |60|/255 × 3.32 ≈ 0.78
   - Container: Sharp → Fluid = 2 steps (major 3rd) = 0.43 × 2.32 ≈ 1.00
   - Focus: Intense → Happy = 3 steps (minor 3rd) = 0.33 × 2.58 ≈ 0.85
   - Action: Hesitant → Playful = 3 steps (minor 3rd) = 0.33 × 2.32 ≈ 0.77
   - **Total harmonic distance: ~3.40** — High (resolution, emotional release)
   - **Annotator perception:** "Jarring but satisfying — like a musical resolution."

2. **"neutral" → "neutral"** (Neutral → Neutral)
   - All dimensions: 0 steps = 0.0
   - **Total harmonic distance: 0.0** — Zero (stable, no change)
   - **Annotator perception:** "Nothing happened — stable."

3. **"calm" → "urgent"** (Calm → Urgent)
   - Aura: 27 → 160 = |133|/255 × 3.32 ≈ 1.73
   - Container: Neutral → Sharp = 4 steps (perfect 5th on 5-value) = 0.50 × 2.32 ≈ 1.16
   - Focus: Neutral → Intense = 1 step (minor 2nd) = 0.83 × 2.58 ≈ 2.14
   - Action: Withheld → Hesitant = 4 steps = 0.50 × 2.32 ≈ 1.16
   - **Total harmonic distance: ~6.19** — Very high (alarm, sudden shift)
   - **Annotator perception:** "Extremely jarring — like a sudden alarm."

4. **"creative" → "analytical"** (Creative → Analytical)
   - Aura: 120 → 39 = |81|/255 × 3.32 ≈ 1.05
   - Container: Fluid → Rigid = 1 step (minor 2nd) = 0.83 × 2.32 ≈ 1.93
   - Focus: Open → Neutral = 2 steps (major 3rd) = 0.43 × 2.58 ≈ 1.11
   - Action: Playful → Withheld = 2 steps = 0.43 × 2.32 ≈ 1.00
   - **Total harmonic distance: ~5.09** — High (mode switch, cognitive reframe)
   - **Annotator perception:** "Significant shift — switching from creative to logical mode."

5. **"happy" → "tired"** (Happy → Tired)
   - Aura: 220 → 238 = |18|/255 × 3.32 ≈ 0.23
   - Container: Fluid → Neutral = 2 steps = 0.43 × 2.32 ≈ 1.00
   - Focus: Happy → Tired = 1 step (minor 2nd) = 0.83 × 2.58 ≈ 2.14
   - Action: Playful → Withheld = 2 steps = 0.43 × 2.32 ≈ 1.00
   - **Total harmonic distance: ~4.37** — Moderate-high (energy drain)
   - **Annotator perception:** "Deflating — the energy drains out."

6. **"energetic" → "calm"** (Energetic → Calm)
   - Aura: 208 → 27 = |181|/255 × 3.32 ≈ 2.36
   - Container: Fluid → Neutral = 2 steps = 0.43 × 2.32 ≈ 1.00
   - Focus: Open → Distant = 1 step (minor 2nd) = 0.83 × 2.58 ≈ 2.14
   - Action: Playful → Withheld = 2 steps = 0.43 × 2.32 ≈ 1.00
   - **Total harmonic distance: ~6.50** — Very high (cooling down, major shift)
   - **Annotator perception:** "Dramatic cool-down — from high energy to rest."

7. **"contemplative" → "assertive"** (Contemplative → Assertive Action)
   - Aura: 91 → Neutral (no change) = 0.0
   - Container: Neutral → Sharp = 4 steps = 0.50 × 2.32 ≈ 1.16
   - Focus: Neutral → Intense = 1 step (minor 2nd) = 0.83 × 2.58 ≈ 2.14
   - Action: Thoughtful → Assertive = 2 steps = 0.43 × 2.32 ≈ 1.00
   - **Total harmonic distance: ~4.30** — Moderate-high (decision made, action taken)
   - **Annotator perception:** "The shift from thinking to doing — decisive."

8. **"analytical" → "creative"** (Analytical → Creative)
   - Aura: 39 → 120 = |81|/255 × 3.32 ≈ 1.05
   - Container: Rigid → Fluid = 1 step (minor 2nd) = 0.83 × 2.32 ≈ 1.93
   - Focus: Neutral → Open = 2 steps = 0.43 × 2.58 ≈ 1.11
   - Action: Withheld → Playful = 2 steps = 0.43 × 2.32 ≈ 1.00
   - **Total harmonic distance: ~5.09** — High (mode switch, mirror of #4)
   - **Annotator perception:** "Unlocking — from rigid to flexible thinking."

9. **"urgent" → "contemplative"** (Urgent → Contemplative)
   - Aura: 160 → 91 = |69|/255 × 3.32 ≈ 0.90
   - Container: Sharp → Neutral = 1 step (minor 2nd) = 0.83 × 2.32 ≈ 1.93
   - Focus: Intense → Neutral = 1 step (minor 2nd) = 0.83 × 2.58 ≈ 2.14
   - Action: Hesitant → Thoughtful = 1 step (minor 2nd) = 0.83 × 2.32 ≈ 1.93
   - **Total harmonic distance: ~6.90** — Very high (all minor 2nds = maximum dissonance)
   - **Annotator perception:** "Jarring shift — from panic to deep thought. Every dimension shifts by just one step, which is actually more jarring than larger shifts."

10. **"tired" → "energetic"** (Tired → Energetic)
    - Aura: 238 → 208 = |30|/255 × 3.32 ≈ 0.39
    - Container: Neutral → Fluid = 2 steps = 0.43 × 2.32 ≈ 1.00
    - Focus: Tired → Open = 3 steps (minor 3rd) = 0.33 × 2.58 ≈ 0.85
    - Action: Withheld → Playful = 2 steps = 0.43 × 2.32 ≈ 1.00
    - **Total harmonic distance: ~3.24** — Moderate (waking up, energy returning)
    - **Annotator perception:** "Gradual awakening — not jarring, more like a stretch."

11. **"defensive" → "open"** (Defensive Container → Open Focus, same state)
    - Container: Defensive → Neutral = 2 steps = 0.43 × 2.32 ≈ 1.00
    - Focus: Neutral → Open = 2 steps = 0.43 × 2.58 ≈ 1.11
    - **Total harmonic distance: ~2.11** — Low-moderate (lowering guard)
    - **Annotator perception:** "Opening up — smooth, not jarring."

---

## 8. Edge Cases and Ambiguous Text

### 8.1 Sarcasm

Sarcasm flips the surface meaning. The annotator must detect tone, not just keywords.

1. **"Great, another error."**
   - Surface: "Great" → Happy. Actual: Sarcasm → **Urgent** + Sharp + Intense + Hesitant.
   - Rule: Exclamation + negative noun ("error") with positive adjective = sarcasm → flip to negative valence.

2. **"Oh wonderful, the server crashed again."**
   - Surface: "Wonderful" → Happy. Actual: Sarcasm → **Urgent** + Sharp + Intense + Hesitant.

3. **"Just perfect — exactly what I needed, another bug."**
   - Surface: "Perfect" → Happy. Actual: Sarcasm → **Urgent** + Defensive + Intense + Hesitant.

### 8.2 Mixed Signals

When text contains conflicting emotions, label the **dominant** one (stronger keywords, more frequent, or last-mentioned).

4. **"I'm happy but concerned about the results."**
   - "Happy" and "concerned" conflict. "But" introduces the dominant clause.
   - Label: **Contemplative** + Neutral + Neutral + Thoughtful (concern dominates).

5. **"Excited and nervous about the launch."**
   - "Excited" (Energetic) and "nervous" (Urgent) conflict. Both are high-energy.
   - Label: **Energetic** + Fluid + Intense + Assertive (excitement dominates as the primary emotion).

### 8.3 Negation

Negated emotions reduce confidence. Do not flip to the opposite — label Neutral.

6. **"This is not urgent."**
   - "Not urgent" → not Urgent. Label: **Neutral** + Neutral + Neutral + Withheld.
   - Rule: Negation removes the emotional signal; it does not create an inverse.

7. **"Don't worry, it's not a failure."**
   - "Not a failure" → not Urgent. Label: **Neutral** + Neutral + Neutral + Withheld.

### 8.4 Context-Dependent Keywords

Some keywords change meaning based on context.

8. **"Critical analysis of the data."**
   - "Critical" here means "analytical/careful" not "urgent/dangerous."
   - Label: **Analytical** + Rigid + Intense + Thoughtful.

9. **"Critical security vulnerability found."**
   - "Critical" here means "urgent/dangerous."
   - Label: **Urgent** + Defensive + Intense + Hesitant.

10. **"The system is executing the plan."**
    - "Executing" is assertive but not aggressive — it's following through.
    - Label: **Neutral** + Rigid + Neutral + Assertive.

### 8.5 Ambiguous Intensity

11. **"This is somewhat important."**
    - "Somewhat" is a diminisher. "Important" suggests urgency but weakened.
    - Label: **Neutral** + Neutral + Neutral + Thoughtful (weak signal, not enough for Urgent).

12. **"VERY CRITICAL ERROR!!!"**
    - All caps + multiple exclamation = maximum intensity.
    - Label: **Urgent** + Sharp + Intense + Assertive (high confidence on all dimensions).

---

## 9. Multi-Sentence Texts

### 9.1 Rules

- **M1:** Label based on the **last sentence** (recency weighting), as this is what `detect_aggregate()` does.
- **M2:** If sentences have conflicting emotions, label the dominant one — typically the last, unless the first is much stronger.
- **M3:** If all sentences share the same state, confidence should be high (agreement boost).
- **M4:** Each sentence can be labeled independently for per-sentence analysis.

### 9.2 Examples

1. **"Everything is calm and peaceful. Critical error crash!"**
   - Sentence 1: Calm + Neutral + Distant + Withheld
   - Sentence 2: Urgent + Sharp + Intense + Hesitant
   - **Aggregate label: Sentence 2** (recency bias — last sentence dominates)

2. **"Critical error! System crash! Danger alert!"**
   - All three sentences: Urgent + Sharp + Intense + Hesitant
   - **Aggregate label: Urgent + Sharp + Intense + Hesitant** (agreement — high confidence)

3. **"The file has been saved. The report was generated. Deployment complete."**
   - All neutral/informational, last one has "complete" → Happy
   - **Aggregate label: Happy + Neutral + Happy + Withheld** (last sentence's success signal)

4. **"I'm exhausted. But let's try one more thing!"**
   - Sentence 1: Tired + Neutral + Tired + Withheld
   - Sentence 2: Energetic + Fluid + Open + Assertive
   - **Aggregate label: Sentence 2** (recency + "but" signals the dominant intent)

5. **"What if we explore creatively? The data might support it."**
   - Sentence 1: Creative + Fluid + Open + Playful
   - Sentence 2: Analytical + Rigid + Neutral + Hesitant
   - **Aggregate label: Sentence 2** (recency), but note the incongruence between sentences.

---

## 10. Ambiguity Protocol

When uncertain between two values for a dimension, follow these tie-breaking rules:

| Dimension | Rule | Default |
|-----------|------|---------|
| Aura | Choose the one with more keyword hits. If equal, choose Neutral. | Neutral (245) |
| Container | Choose the less extreme one (closer to Neutral). | Neutral (0) |
| Focus | Choose Neutral. | Neutral (0) |
| Action | Choose Withheld. | Withheld (0) |

**Document all ambiguities.** Annotators should mark confidence as 0.5 or lower when the label is uncertain. This data feeds into the IAA analysis (Section 11).

---

## 11. Inter-Rater Agreement Guidelines

### 11.1 Process

1. **Minimum 3 annotators** label the same texts independently.
2. Annotators do not communicate during labeling.
3. After independent labeling, compare results.
4. Disagreements are resolved by a **third-party adjudicator** (fourth person).
5. Common disagreement patterns are documented and fed back into this guide.

### 11.2 Agreement Metrics

- **Cohen's Kappa (pairwise):** Measures agreement between two annotators, corrected for chance.
- **Fleiss' Kappa (multi-annotator):** Measures agreement across all annotators simultaneously.

| Kappa | Agreement | Action |
|-------|-----------|--------|
| < 0.2 | None | Labeling guide is broken for this byte. Redesign rules. |
| 0.2–0.4 | Slight | Byte is too subjective. Consider collapsing categories. |
| 0.4–0.6 | Moderate | Workable. Invest in better examples and rules. |
| 0.6–0.8 | Substantial | Good. This byte is measurable. |
| > 0.8 | Almost perfect | Excellent. This byte is objectively definable. |

**Target: κ ≥ 0.7 for acceptable agreement.**

### 11.3 Expected Agreement Per Byte

| Byte | Categories | Expected Agreement | Reasoning |
|------|-----------|-------------------|-----------|
| Action | 5 | High (>0.7) | Clear keyword boundaries |
| Container | 5 | High (>0.7) | Clear element metaphors |
| Focus | 6 | Moderate (0.4–0.7) | "Intense" vs "Open" depends on interpretation |
| Aura | 10 | Moderate (0.4–0.7) | Mood categories can overlap (Creative vs Unconventional) |

### 11.4 Common Disagreement Sources

1. **Creative vs Unconventional Aura:** Both involve novelty. Creative = productive novelty, Unconventional = strange novelty.
2. **Intense vs Open Focus:** Both involve high attention. Intense = strained concentration, Open = surprised reception.
3. **Defensive vs Sharp Container:** Both involve protection. Defensive = shielding, Sharp = attacking.
4. **Thoughtful vs Hesitant Action:** Both involve non-action. Thoughtful = deliberate pause, Hesitant = uncertain pause.
5. **Sarcasm detection:** Annotators may disagree on whether text is sarcastic.

---

## 12. Labeling Format

### 12.1 JSONL Format

Each labeled example is one JSON object per line in a `.jsonl` file:

```json
{"text": "Critical error: system crash!", "aura": 160, "container": 4, "focus": 1, "action": 4, "congruence": "congruent", "confidence": 0.9}
```

### 12.2 Field Definitions

| Field | Type | Values | Description |
|-------|------|--------|-------------|
| `text` | string | any | The input text being labeled |
| `aura` | u8 | 0–255 | Aura byte value (use named indices: 245, 160, 208, 220, 120, 39, 27, 91, 177, 238) |
| `container` | u8 | 0–4 | Container byte value (0=Neutral, 1=Rigid, 2=Fluid, 3=Defensive, 4=Sharp) |
| `focus` | u8 | 0–5 | Focus byte value (0=Neutral, 1=Intense, 2=Open, 3=Distant, 4=Happy, 5=Tired) |
| `action` | u8 | 0–4 | Action byte value (0=Withheld, 1=Assertive, 2=Playful, 3=Thoughtful, 4=Hesitant) |
| `congruence` | string | "congruent", "incongruent", "neutral" | Congruence assessment |
| `confidence` | f32 | 0.0–1.0 | Annotator's confidence in the label |

### 12.3 Example Dataset (5 entries)

```jsonl
{"text": "Critical error: system crash!", "aura": 160, "container": 4, "focus": 1, "action": 4, "congruence": "congruent", "confidence": 0.95}
{"text": "The file has been saved.", "aura": 245, "container": 0, "focus": 0, "action": 0, "congruence": "neutral", "confidence": 0.9}
{"text": "What if we explore a creative approach?", "aura": 120, "container": 2, "focus": 2, "action": 2, "congruence": "congruent", "confidence": 0.85}
{"text": "I'm happy but concerned about the results.", "aura": 91, "container": 0, "focus": 0, "action": 3, "congruence": "neutral", "confidence": 0.5}
{"text": "Great, another error.", "aura": 160, "container": 4, "focus": 1, "action": 4, "congruence": "congruent", "confidence": 0.7}
```

### 12.4 File Naming Convention

- `faces_train.jsonl` — Training data for FACES-Embed
- `faces_test.jsonl` — Test data for evaluation harness (W7)
- `faces_iaa.jsonl` — Inter-annotator agreement study data
- Format: one JSON object per line, UTF-8 encoded, newline-delimited.

---

## Document Cross-References

- **FACES_VALIDATION_FRAMEWORK.md** — Section 4.2 defines the labeling guide structure this document implements
- **FACES_PYTHAGOREAN_RESEARCH.md** — Section 4.1 provides the ratio_complexity values used in harmonic transitions
- **FACES_GAP_ANALYSIS.md** — FACES-7 (labeling guide) is listed as critical path
- **FACES_TECHNICAL_INTENT.md** — Section on FACES-Embed training data pipeline
- **PROGRESS.md** — W6 completion tracked in crates/faces-protocol/PROGRESS.md

---

## Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-07-02 | Initial creation. All dimensions covered with 5+ examples each. 23 congruence examples. 11 harmonic transitions. 12 edge cases. IAA guidelines. JSONL format. |
