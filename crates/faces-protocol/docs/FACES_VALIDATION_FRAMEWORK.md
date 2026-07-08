# FACES Validation & ML Evaluation Framework

> Date: 2026-07-02  
> Author: Cascade + Joshua Atkinson  
> Status: Active — companion to FACES_GAP_ANALYSIS.md, FACES_TECHNICAL_INTENT.md, FACES_PYTHAGOREAN_RESEARCH.md
> Strategic positioning: FACES is the TCP/IP of emotive AI — protocol first, physical AI as proof.  
> Question: How do we quantify utility and cost savings for FACES, and how do we improve the system for real ML data engineering with complexity and measurability?

---

## 1. The Core Problem

FACES claims to be useful. It claims to save tokens. It claims to convey emotion. **How do we KNOW?**

In ML engineering, we know things through five methods:

1. **Ground truth** — labeled data that represents reality
2. **Metrics** — quantitative measures against ground truth
3. **Baselines** — comparison points to prove we're actually better
4. **Ablation** — removing components to see what contributes
5. **Human evaluation** — does it actually help humans?

FACES currently has none of these. We have 105 tests that verify *structural* correctness (encoding round-trips, glyph rendering). We have zero tests that verify *semantic* correctness (does this text actually mean this FACES state?).

This document defines the framework to close that gap.

---

## 2. Quantifying Utility

### 2.1 Does the Protocol Itself Convey Emotion? (Pareidolia Validation)

Before testing the detector, test the protocol. If humans can't read FACES faces, the detector doesn't matter.

**Test: Pareidolia Comprehension Study**

- Show participants ASCII FACES faces with no text context
- Ask them to rate each on three dimensions (Russell's Circumplex Model):
  - **Valence** (negative ↔ positive): 1-7 scale
  - **Arousal** (calm ↔ activated): 1-7 scale
  - **Dominance** (submissive ↔ dominant): 1-7 scale
- Compare human ratings to the spec's intended semantic mappings

**Pass criteria:**
- Container shapes produce distinct dominance ratings (e.g., `<>` rated more dominant than `{}`)
- Focus shapes produce distinct arousal ratings (e.g., `OO` rated higher arousal than `..`)
- Action shapes produce distinct valence ratings (e.g., `~` rated more positive than `.`)
- Statistical significance: p < 0.05 on Kruskal-Wallis test per dimension

**Sample size:** Minimum 30 participants × 150 face geometries (5×6×5, Aura held constant at neutral gray)

**What this tells us:** Whether the ASCII geometry actually triggers pareidolia as the spec claims, or whether it's just theoretical.

### 2.2 Does the Detector Produce Correct States? (Inter-Annotator Agreement)

Before building FACES-Embed, we need to know: **can humans agree on what FACES state a text maps to?**

If humans can't agree, no model can be "correct" — the task is subjective, not objective.

**Test: Inter-Annotator Agreement (IAA)**

- 3+ annotators independently label 200+ texts
- Each annotator assigns all 4 bytes (Aura, Container, Focus, Action)
- Calculate **Cohen's Kappa** (pairwise) and **Fleiss' Kappa** (multi-annotator) per byte

**Interpretation per byte:**

| Kappa | Agreement | Action |
|-------|-----------|--------|
| < 0.2 | None | Labeling guide is broken for this byte. Redesign. |
| 0.2–0.4 | Slight | Byte is too subjective. Consider collapsing categories. |
| 0.4–0.6 | Moderate | Workable. Invest in better labeling guide. |
| 0.6–0.8 | Substantial | Good. This byte is measurable. |
| > 0.8 | Almost perfect | Excellent. This byte is objectively definable. |

**Expected outcome:**
- Container (5 shapes): likely high agreement — "formal" vs "creative" is relatively objective
- Action (5 shapes): likely moderate — "playful" vs "thoughtful" can be subjective
- Focus (6 shapes): likely moderate — "intense" vs "open" depends on interpretation
- Aura (256 values): likely LOW agreement — color 120 vs 121 is not human-distinguishable

**If Aura has low agreement:** This confirms the gap analysis finding. Aura should be treated as coarse classification (10 named moods) or regression, not 256-way classification. The labeling guide should define Aura in terms of mood categories, not exact color indices.

### 2.3 Does FACES Improve Task Outcomes? (A/B Testing)

The ultimate utility test: does adding FACES to Trinity improve user outcomes?

**Test: Controlled A/B Experiment**

- **Group A**: Trinity with FACES (state displayed, VAAM-informed baseline, detection active)
- **Group B**: Trinity without FACES (current behavior, `detect_emotion()` legacy)
- **Metrics:**

| Metric | Type | Measurement |
|--------|------|-------------|
| Task completion rate | Outcome | % of sessions where user reaches their goal |
| Turns to resolution | Efficiency | Number of conversation turns to complete task |
| User satisfaction | Subjective | Post-session survey (1-7 Likert scale) |
| Emotional comprehension | Outcome | "Did you understand the AI's tone?" (1-7) |
| Context window pressure | Technical | Tokens consumed by emotive signaling per conversation |
| Cost per conversation | Economic | Estimated inference cost (tokens × price/token) |

**Sample size:** Minimum 50 sessions per group for statistical power (α=0.05, power=0.8)

**What this tells us:** Whether FACES is actually useful in practice, or just theoretically elegant.

---

## 3. Quantifying Cost Savings

### 3.1 Token Economics

The spec claims FACES uses 1-4 tokens vs 30-50 for verbose emotive descriptions. This is testable.

**Test: Token Counting on Real Conversations**

```
Method:
1. Run 100 real Trinity conversations (with user consent or synthetic)
2. For each turn, calculate:
   - Verbose approach: "The AI feels [emotion] and is [cognitive state]" 
     → count tokens with the target LLM's tokenizer
   - FACES approach: "F5231" or "(o_o)"
     → count tokens with the same tokenizer
3. Also count the system prompt overhead for FACES protocol explanation
4. Calculate net savings per turn and per conversation
```

**Metrics:**

| Metric | Formula | Expected |
|--------|---------|----------|
| Tokens per emotive signal (verbose) | Count of descriptive text tokens | 30-50 |
| Tokens per emotive signal (FACES) | Count of hex/ASCII tokens | 1-4 |
| System prompt overhead | FACES protocol explanation tokens | ~100 (one-time) |
| Net savings per turn | verbose_tokens - faces_tokens | ~25-45 |
| Break-even turn | overhead / net_savings_per_turn | ~2-4 turns |
| 50-turn conversation savings | 50 × net_savings - overhead | ~1,150-2,250 tokens |

**Critical nuance:** The system prompt overhead is one-time per conversation. The per-turn savings compound. But if the FACES system prompt is too long, it eats into the context window that could be used for actual work. The protocol explanation must be minimal.

### 3.2 Compute Economics

**On Strix Halo (NPU + GPU):**

| Component | Hardware | Cost | Savings |
|-----------|----------|------|---------|
| FACES-Embed inference | NPU (XDNA 2) | ~1ms, negligible power | Replaces nothing — new cost |
| Reduced LLM context | GPU (RDNA 3.5) | — | Fewer tokens = less KV cache = faster generation |
| Keyword detection (current) | CPU | ~0 | Free, but less accurate |

**The key question:** Does the NPU cost of FACES-Embed exceed the GPU savings from shorter context?

```
GPU savings per turn:
  - Reduced context = reduced prefill compute
  - Prefill is O(n²) for attention, so saving 40 tokens from a 4000-token context
    saves ~40×4000 = 160K attention operations per layer
  - At ~$0.50/Mtok inference cost, saving 40 tokens/turn × 50 turns = 2000 tokens
    = ~$0.001 per conversation (marginal, but compounds at scale)

NPU cost per turn:
  - FACES-Embed: ~1ms at ~5W = ~0.0014 Wh per inference
  - 50 turns = 0.07 Wh per conversation
  - At ~$0.12/kWh = ~$0.000008 per conversation (negligible)
```

**Conclusion:** NPU cost is negligible. GPU savings are small per conversation but meaningful at scale. The real value is **context window pressure** — in long conversations, 2000 saved tokens might be the difference between the LLM remembering the user's original goal or forgetting it.

### 3.3 Cognitive Economics

The hardest to measure, but the most aligned with Trinity's "imagination amplification" thesis.

**Test: Reading Speed & Comprehension**

- Show participants AI responses with FACES annotation vs verbose emotion description
- Measure:
  - Time to parse the emotional context (eye-tracking or self-paced reading)
  - Comprehension of the AI's tone (quiz after reading)
  - Cognitive load (NASA-TLX questionnaire)

**Hypothesis:** `(o_o)` is parsed faster than "The AI is in a neutral, receptive state, observing without judgment" because pareidolia is pre-cognitive (subconscious, fast) while language processing is cognitive (conscious, slow).

**If confirmed:** This is the strongest argument for FACES — not token savings, but **cognitive bandwidth savings** for the human. The human reads emotion in parallel with content, not serially.

---

## 4. ML Data Engineering — Complexity with Measurability

### 4.1 The Fundamental Insight: Don't Evaluate FACES as One Problem

The 38,400-state space is intimidating but misleading. FACES is not a 38,400-way classification problem. It's **four independent classification problems**:

| Byte | Problem Type | Classes | Difficulty | Measurability |
|------|-------------|---------|------------|---------------|
| Container | Multi-class classification | 5 | Easy | High — categories are semantically distinct |
| Focus | Multi-class classification | 6 | Moderate | High — categories are semantically distinct |
| Action | Multi-class classification | 5 | Moderate | High — categories are semantically distinct |
| Aura | Classification OR regression | 256 or 10 or 8 | Hard | Low — fine color distinctions are subjective |

**Evaluate each byte independently.** Report Container F1, Focus F1, Action F1, and Aura MAE (or coarse Aura accuracy). Never report a single "FACES accuracy" number — it hides where the system succeeds and fails.

### 4.2 The Labeling Guide as Ground Truth Definition

The labeling guide (FACES-7) is not just documentation — it IS the ground truth definition. It defines what "correct" means. Without it, no metric has meaning.

**Structure of the labeling guide:**

```
Section 1: Container Labeling Rules
  - Rule C1: Formal/protocol text → Rigid (Earth)
  - Rule C2: Creative/brainstorm text → Fluid (Water)
  - Rule C3: Error/security text → Defensive (Wood) or Sharp (Fire)
  - Rule C4: Default/neutral text → Neutral (Metal)
  - Rule C5: If text contains both formal and creative elements, use the dominant intent
  - Edge cases: [10+ examples with reasoning]

Section 2: Focus Labeling Rules
  - Rule F1: High-intensity language ("critical", "urgent", "!") → Intense
  - Rule F2: Surprise language ("wow", "unexpected") → Open
  - Rule F3: Success language ("congratulations", "complete") → Happy
  - Rule F4: Waiting/idle language → Distant
  - Rule F5: Fatigue language → Tired
  - Rule F6: Default → Neutral
  - Edge cases: [10+ examples with reasoning]

Section 3: Action Labeling Rules
  [Similar structure with A1-A5 + edge cases]

Section 4: Aura Labeling Rules
  - Use 10 named mood categories, not 256 raw indices
  - Map each category to a representative color index
  - If text doesn't clearly map to a named mood, use Neutral Gray (245)
  - Edge cases: [10+ examples]

Section 5: Multi-Sentence Texts
  - Label based on the LAST sentence (recency weighting)
  - If sentences have conflicting emotions, label the dominant one
  - Provide 10+ multi-sentence examples with reasoning

Section 6: Ambiguity Protocol
  - If unsure between two Container values, choose the less extreme one
  - If unsure between two Focus values, choose Neutral
  - If unsure between two Action values, choose Withheld
  - Document the ambiguity for IAA analysis
```

### 4.3 Aura: The Hard Problem

Aura is the most complex byte because it's 256-valued but semantically continuous. Three approaches, each with different measurability:

**Option A: 10-Way Classification (Recommended for v1)**
- Use the 10 named Aura constants (Neutral, Urgent, Energetic, Happy, Creative, Analytical, Calm, Contemplative, Unconventional, Tired)
- 10-way softmax, Cross-Entropy loss
- Measurable: accuracy, F1, confusion matrix — all standard
- Loses 246 values, but those 246 values are not human-distinguishable anyway
- **This is what the labeling guide should define**

**Option B: 8-Way Color Family Classification**
- Group 256 colors into 8 families: Red, Orange, Yellow, Green, Cyan, Blue, Purple, Gray
- 8-way softmax
- More granular than Option A, less subjective than 256-way
- Useful if the application needs color variation within mood

**Option C: Two-Stage (Mood + Offset)**
- Stage 1: 10-way mood classification (same as Option A)
- Stage 2: Within each mood, regression to predict exact color index
- Stage 2 is only trained on data where the mood is known
- Allows fine color variation while keeping measurability

**Recommendation:** Start with Option A (10-way). It's measurable, human-agreeable, and sufficient for v1. Expand to Option C only if the application demands fine color control (e.g., XR environments where color gradient matters).

### 4.4 Class Imbalance — The Silent Killer

Most text is neutral. If 80% of Trinity conversations produce Neutral Container, a detector that always predicts Neutral gets 80% accuracy. This is useless.

**Required metrics (not just accuracy):**

| Metric | Formula | Why It Matters |
|--------|---------|---------------|
| **Precision** (per class) | TP / (TP + FP) | When detector says "Playful", is it actually playful? |
| **Recall** (per class) | TP / (TP + FN) | How many actually-playful texts does the detector find? |
| **F1** (per class) | 2 × P × R / (P + R) | Harmonic mean — penalizes imbalance |
| **Macro-F1** | Average of F1 across all classes | Treats all classes equally — punishes always-neutral |
| **Weighted-F1** | F1 weighted by class frequency | Reflects real-world distribution |
| **Confusion Matrix** | Full matrix | Shows exactly which states get confused |

**Target for keyword baseline:**
- Macro-F1 > 0.40 (better than random for 5-6 classes)
- No class has F1 = 0 (every state must be detectable)
- Neutral class precision > 0.80 (when it says neutral, it's neutral)

### 4.5 Information-Theoretic Evaluation

Not all bytes carry equal information. We should measure the **entropy** of each byte in the dataset:

```
H(Container) = -Σ p(ci) × log2(p(ci))
  - If H = 0: all texts are the same Container. The byte carries no information.
  - If H = log2(5) = 2.32: perfectly uniform distribution. Maximum information.

H(Focus) = -Σ p(fi) × log2(p(fi))
  - Max = log2(6) = 2.58 bits

H(Action) = -Σ p(ai) × log2(p(ai))
  - Max = log2(5) = 2.32 bits
```

**What this tells us:**
- If Container entropy is 0.5 bits (mostly Neutral), investing in better Container detection yields diminishing returns
- If Focus entropy is 2.0 bits (highly variable), Focus detection is where investment pays off
- This guides **loss weighting** in FACES-Embed: bytes with higher entropy get higher loss weights

### 4.6 Calibration — Confidence Must Mean Something

When the detector says "80% confident this is Playful", it should be right ~80% of the time. If it's right only 50% of the time, the confidence is meaningless and the Consent Gate can't trust it.

**Test: Reliability Diagram**

- Bin predictions by confidence (0-10%, 10-20%, ..., 90-100%)
- For each bin, calculate actual accuracy
- Plot predicted confidence vs actual accuracy
- Perfect calibration: diagonal line
- **Expected Calibration Error (ECE):** weighted average of |confidence - accuracy| per bin

**Why this matters for FACES:**
- The Consent Gate uses confidence to decide whether to auto-accept or prompt the user
- Low-confidence detections (<60%) should trigger user review
- High-confidence detections (>90%) can be auto-applied
- If confidence is uncalibrated, the gate either over-prompts (annoying) or under-prompts (wrong states)

### 4.7 The Dataset Pipeline

```
Phase 0: Labeling Guide (FACES-7)
  ↓ Defines ground truth
Phase 1: IAA Study (100 texts × 3 annotators)
  ↓ Validates labeling guide
  ↓ If κ < 0.4 for any byte → revise guide, re-annotate
Phase 2: Human-Labeled Seed Set (500 texts)
  ↓ High-quality ground truth for evaluation
Phase 3: Teacher Model Labeling (97B LLM, 100K texts)
  ↓ Validates teacher against seed set
  ↓ If teacher accuracy < IAA → revise teacher prompt
Phase 4: Train/Val/Test Split (80/10/10, stratified)
  ↓ Stratified by Container, Focus, Action classes
Phase 5: Train FACES-Embed (multi-task, 4 heads)
  ↓ Loss weighted by entropy per byte
  ↓ Early stopping on validation Macro-F1
Phase 6: Evaluate on Test Set
  ↓ Per-byte F1, confusion matrices, calibration
  ↓ Compare to keyword baseline
  ↓ If FACES-Embed doesn't beat baseline by >10% Macro-F1 → not worth deploying
```

### 4.8 Ablation Studies — What Actually Matters?

Once FACES is integrated, test which components contribute:

| Ablation | Remove | Measure |
|----------|--------|---------|
| No FACES | Entire system | Baseline task outcomes |
| FACES render only | Detection, just display neutral state | Does just showing a face help? |
| Keyword detection | Remove FACES-Embed | Is the neural model worth it? |
| No Aura | Set Aura to constant neutral | Does color matter, or just geometry? |
| No Container | Set Container to constant neutral | Does head shape matter? |
| No Focus | Set Focus to constant neutral | Do eyes matter? |
| No Action | Set Action to constant withheld | Does mouth matter? |
| No VAAM baseline | Remove VAAM-informed defaults | Does user profiling help? |
| No Consent Gate | Remove user override | Does user control matter? |

**Expected findings:**
- Geometry (Container + Focus + Action) likely matters more than color (Aura)
- Focus (eyes) likely matters most for pareidolia — it's the most primal signal
- VAAM baseline likely matters less per-turn but more over time (consistency)
- Consent Gate likely matters most for user trust and adoption

---

## 5. Metrics Summary — The FACES Scorecard

### 5.1 Protocol Validation (before any ML)

| Metric | Method | Target | Status |
|--------|--------|--------|--------|
| Pareidolia comprehension | Human rating study | p<0.05 on K-W test | Not started |
| Inter-annotator agreement | Cohen's Kappa (3+ annotators) | κ > 0.5 per byte | Not started |
| Cross-cultural validity | Replicate study with diverse participants | Consistent ratings | Not started |

### 5.2 Detection Quality (keyword baseline → FACES-Embed)

| Metric | Method | Keyword Target | FACES-Embed Target |
|--------|--------|---------------|-------------------|
| Container Macro-F1 | Test set evaluation | > 0.40 | > 0.70 |
| Focus Macro-F1 | Test set evaluation | > 0.35 | > 0.65 |
| Action Macro-F1 | Test set evaluation | > 0.40 | > 0.70 |
| Aura (10-way) accuracy | Test set evaluation | > 0.30 | > 0.60 |
| Exact match (all 4 bytes) | Test set evaluation | > 0.15 | > 0.45 |
| Calibration (ECE) | Reliability diagram | < 0.20 | < 0.10 |
| Latency (p99) | Benchmark on target hardware | < 1ms CPU | < 1ms NPU |

### 5.3 System Utility (A/B testing in Trinity)

| Metric | Method | FACES Must Beat Baseline By |
|--------|--------|----------------------------|
| Task completion rate | A/B test | > 5% improvement |
| Turns to resolution | A/B test | > 10% reduction |
| User satisfaction | Post-session survey | > 0.5 Likert point |
| Token cost per conversation | Token counting | > 20% reduction |
| Context window headroom | Token counting | > 15% more available tokens |

### 5.4 Cost Savings

| Metric | Method | Target |
|--------|--------|--------|
| Tokens saved per turn | Tokenizer comparison | > 25 tokens |
| Break-even turn | Overhead / per-turn savings | < 5 turns |
| 50-turn conversation savings | Cumulative | > 1,000 tokens |
| NPU power cost per inference | Power measurement | < 0.01 Wh |
| GPU time saved per conversation | Inference timing | Measurable (>0) |

---

## 6. The Honest Assessment

### What We Can Measure Now (with code changes)
- Latency of `detect_faces()` and `render_state()` — just needs benchmarks
- Token counts — just needs a tokenizer and sample conversations
- State space coverage — how many of 38,400 states does the keyword detector actually produce?

### What We Can Measure Soon (with labeling guide + dataset)
- Per-byte accuracy, F1, confusion matrices — needs 500 labeled examples
- Inter-annotator agreement — needs 3 annotators × 100 texts
- Keyword baseline performance — needs the above two

### What We Can Measure Later (with human studies)
- Pareidolia comprehension — needs IRB-approved human study
- Cognitive load reduction — needs eye-tracking or NASA-TLX
- A/B task outcomes — needs deployed Trinity with real users
- XR rendering validation — does FACES face display correctly in SpatialPanel on XREAL Aura?
- Edge detection accuracy — Gemini Nano FACES detection vs FACES-Embed on NPU vs keyword baseline
- WebSocket transport latency — Strix Halo → phone/XR round-trip for 4-byte state

### What We Might Never Be Able to Measure
- Whether FACES enables "imagination amplification" — this is a thesis, not a metric
- Whether the Mian Xiang mapping is "correct" — it's a design choice, not a factual claim
- Whether 38,400 states is the "right" number — it's a protocol design decision

**The job of ML engineering is not to prove the thesis. It's to ensure the system we build actually does what we claim it does, measurably and repeatably.**

---

## 7. Revised Priority Order

Given this framework, the priority order is now organized into the 10-workflow execution plan (see FACES_GAP_ANALYSIS.md Part 4 and FACES_TECHNICAL_INTENT.md §5.6):

**Session 1-2: Protocol + Detection (W1-W5)**
1. **W1: Protocol hardening** — to_u32/from_u32, to_rgb, safety presets, serde feature flag
2. **W2: Scored detection** — confidence-scored DetectionResult, not first-match-wins
3. **W3: Multi-sentence + FacesProfile** — sentence segmentation, state history, baseline
4. **W4: Consent Gate** — lock/unlock/commit state machine, user override
5. **W5: Static/Dynamic phases** — two-phase temporal model, decay management
   → ★ Gate 1: Joshua reviews the full crate

**Session 3: Measurement (W6-W7)**
6. **W6: Labeling guide** — ground truth document defining what text maps to what FACES state
   → ★ Gate 2: Joshua approves labeling guide
7. **W7: Eval harness + benchmarks** — JSONL loader, per-byte F1, confusion matrices, latency

**Session 4-6: Emulator (W8-W10)**
8. **W8: Emulator core** — terminal UI, real-time detection, all render modes
   → ★ Gate 3: Joshua reviews emulator (5 min)
9. **W9: Emulator interaction** — Consent Gate UI, VAAM baseline sim, transition visualization
10. **W10: Emulator fleet + physical sim** — multi-agent, behavior mapping, LED matrix sim

**Post-emulator (future sessions):**
- Token counting study (needs real Trinity conversations)
- FACES-9: Initial dataset (500 examples, needs labeling guide + annotators)
- FACES-11-14: VAAM integration prep
- FACES-17-18: ML engineering (needs hardware + dataset)
- Three-device integration: FACES WebSocket streaming, FACES Kotlin SDK, ADK Socratic agent
- XR rendering validation: SpatialPanel FACES display on XREAL Aura
- Edge detection benchmark: Gemini Nano vs FACES-Embed vs keyword baseline

**The labeling guide (W6) remains job zero for measurement.** Everything before it is engineering. Everything after it can be measured.
