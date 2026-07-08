# FACES Gap Analysis & VAAM Integration Study

> Date: 2026-07-02  
> Author: Cascade + Joshua Atkinson  
> Status: Active — pre-integration review of `faces-protocol` crate
> Strategic positioning: FACES is the TCP/IP of emotive AI — a protocol first, physical AI as proof.
> See FACES_TECHNICAL_INTENT.md for full positioning.
> See FACES_PYTHAGOREAN_RESEARCH.md for grounded Pythagorean ratio research applied to FACES transition metrics.

---

## Part 1: Gap Analysis — What We Built vs What the Spec Requires

### 1.1 Protocol Layer (Solid)

| Component | Spec Requirement | Implementation Status | Gap |
|-----------|-----------------|----------------------|-----|
| 4-byte state struct | Aura(8) + Container(5) + Focus(6) + Action(5) = 38,400 states | ✅ Complete, type-safe, 105 tests | None |
| Byte encode/decode | Modular arithmetic, any byte stream valid | ✅ Complete | None |
| Hex serialization | 8-char uppercase hex, round-trip | ✅ Complete | None |
| ASCII render | 5-char matrix with ANSI color | ✅ Complete | None |
| Container shapes | 5 shapes, Mian Xiang mapping | ✅ Complete | None |
| Focus shapes | 6 shapes, FACS AU mapping | ✅ Complete | None |
| Action shapes | 5 shapes, FACS mouth AU mapping | ✅ Complete | None |
| Aura colors | 256 ANSI indices | ✅ Complete (10 named, 246 unnamed) | Named coverage is thin but structurally sound |

### 1.2 Detection Layer (Placeholder Quality)

| Component | Spec Requirement | Implementation Status | Gap |
|-----------|-----------------|----------------------|-----|
| Text→FACES mapping | Syntactic isomorphism (POS tagging: noun→container, adj→aura, verb→action, adverb→focus) | ❌ Flat keyword matching with `contains()` | **Major** — no POS parsing, no sentence structure analysis |
| Scoring system | "Highest-scoring category wins" | ❌ First-match-wins (if/else chain) | **Major** — no confidence scores, no weighted matching |
| Congruence detection | Support both congruent and incongruent states | ❌ Congruent only | **Moderate** — sarcasm/irony requires FACES-Embed or user override |
| Multi-sentence handling | Handle texts with shifting emotions | ❌ Whole-text scan, single result | **Moderate** — no sentence segmentation or temporal weighting |
| Aura coverage | 256 possible values | ❌ Only 10 reachable values | **Moderate** — keyword detector can't differentiate fine color gradations |

### 1.3 Transition Layer (Functional but Incomplete)

| Component | Spec Requirement | Implementation Status | Gap |
|-----------|-----------------|----------------------|-----|
| Contrastive Transition Vector | Delta between consecutive states | ✅ Complete | None |
| Magnitude/volatility | L1 norm, stability classification | ✅ Complete | None |
| Interpolation (lerp) | Linear interpolation between states | ✅ Complete (linear only) | Spec mentions cubic spline lerps as option |
| Temporal decay | Smooth return to baseline when idle | ✅ Complete | None |
| **Static Lock/Commit phase** | Freeze 4-byte payload on message finalization | ❌ Not implemented | **Critical** — without this, temporal decay resets intended expressions during reflective pauses |
| **Dynamic Interaction phase** | Real-time keystroke friction shifting Focus | ❌ Not implemented | **Critical** — spec explicitly defines this as resolving the "State-Decay Contradiction" |
| Toxic escalation prediction | Sequence of increasing magnitudes → proactive intervention | ❌ Not implemented | **Low** — future feature, needs multi-turn history |

### 1.4 Missing Components (Not Yet Built)

| Component | Spec Section | Priority | Notes |
|-----------|-------------|----------|-------|
| **Mechanical Consent Gate** | §4 | High | User can nudge/adjust emotive states before finalizing. No implementation exists. |
| **System prompt template** | §6 | High | LLM system prompt block for FACES output. Not implemented. |
| **Agentic telemetry** | §6 | Medium | `render_telemetry()` exists but no log integration. |
| **FACES-Web Translation Layer** | §9 | Low | SVG/CSS rendering for browsers. Future XR work. |
| **FACES-Embed model** | §6.E, §6.F | Medium | ~66M param DistilBERT encoder for NPU. Requires dataset first. See FACES-16 decision. |
| **Dataset labeling guide** | §9 | **Critical** | Without this, no semantic correctness can be measured. |
| **Evaluation harness** | — | **Critical** | No way to measure detector accuracy against ground truth. |
| **Latency benchmarks** | §6.E | High | "Near-zero" and "sub-millisecond" claims are unverified. |
| **FACES Kotlin SDK** | §4.2 | High | Kotlin data class + WebSocket client for phone and XR apps. 4-byte protocol consumption on Android. |
| **FACES WebSocket streaming** | §2.2 | High | Server-side WebSocket endpoint broadcasting 4-byte FACES states to phone + XR clients. |
| **Jetpack Compose for XR rendering** | §4.2 | Medium | SpatialPanel + Orbiter + SpatialGltfModel for FACES state display in XR space. Requires XREAL dev kit. |
| **ADK + Gemini Nano agent** | §4.2 | Medium | On-device Socratic questioning agent + standalone FACES detection fallback. Testable on Pixel 10 Pro XL now. |
| **ARCore spatial anchoring** | §4.2 | Low | Anchor FACES panels to physical locations. Requires XREAL dev kit. |

### 1.5 Architectural Concerns

**Zero-dependency constraint vs integration needs:**
- `faces-protocol` is zero-dependency by design (NPU target, embedded compatibility)
- VAAM lives in `trinity-protocol` (depends on serde, chrono) and `trinity` (depends on sqlx, tokio)
- Direct dependency would break the zero-dep constraint
- Solution: FACES should define traits/interfaces that VAAM can implement externally, or integration happens at a higher layer

**No FACES profile/state persistence:**
- VAAM has `VaamProfile` with serde Serialize/Deserialize, persisted in CharacterSheet
- FACES has no equivalent — no user FACES preference profile, no history of FACES states
- The spec implies a "FACES baseline" per user but we don't track one

---

## Part 2: VAAM → FACES Integration Analysis

### 2.1 The Isomorphic Bridge

The FACES spec explicitly maps its 4 bytes to VAAM dimensions:

| FACES Byte | Linguistic | VAAM Dimension | Meaning |
|-----------|-----------|---------------|---------|
| Aura (Byte 0) | Adjective | **M**astery (M) | Qualitative state, mood, atmosphere |
| Container (Byte 1) | Noun | **A**utonomy (A) | Boundary, structure, self-determination |
| Focus (Byte 2) | Adverb | **Ac**quisition (Ac) | Intensity, attention, processing load |
| Action (Byte 3) | Verb | **V**ocabulary (V) | Kinetic output, expressiveness |

VAAM literally spells V-A-A-M. FACES bytes map to these dimensions. This is not coincidental — it's designed isomorphism.

### 2.2 What VAAM Already Tracks That FACES Needs

**VaamProfile** (`/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-protocol/src/vaam_profile.rs`):

| VAAM Field | What It Tracks | FACES Equivalent | How It Could Guide FACES |
|-----------|---------------|-----------------|------------------------|
| `circuit_affinity[4]` | User's quadrant distribution (Scope/Build/Listen/Ship) | Container + Focus | Dominant quadrant → default Container/Focus baseline |
| `circuit_usage[15]` | Which of 15 circuits the user activates | Focus | Active circuit → current Focus state (e.g., Flow = Intense, Receive = Open) |
| `word_weights` | Deliberate word choices with affinity scores | Aura | User's preferred vocabulary carries emotional tone → Aura color |
| `style.brevity` | Terse vs verbose (EMA) | Action | Terse user → assertive Action default; verbose → thoughtful default |
| `style.directness` | Direct vs exploratory (EMA) | Container | Direct user → Rigid container; exploratory → Fluid container |
| `agreements` | Explicit user-AI agreements about what matters | Consent Gate | Agreements constrain which FACES states are appropriate |
| `interactions_analyzed` | Total interaction count | Growth tracking | Novice → more positive reinforcement FACES; Expert → peer-level FACES |

**Sacred Circuitry** (`/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-protocol/src/sacred_circuitry.rs`):

| Circuit Quadrant | Cognitive Phase | Natural FACES State | Rationale |
|-----------------|----------------|-------------------|-----------|
| **Scope** (Center→Prepare) | Defining the problem | Container: Neutral, Focus: Neutral, Action: Thoughtful | Open, receptive, evaluating |
| **Build** (Express→Flow) | Producing work | Container: Fluid, Focus: Intense, Action: Assertive | Creative, concentrated, producing |
| **Listen** (Receive→Realize) | Processing feedback | Container: Neutral, Focus: Open, Action: Withheld | Receiving, attentive, silent |
| **Ship** (Act→Manifest) | Delivering output | Container: Sharp, Focus: Intense, Action: Assertive | Urgent, focused, executing |

**CommunicationStyle** — the EMA-based style tracker could directly seed FACES defaults:

```
brevity > 0.6 + directness > 0.6  →  Assertive, terse baseline (Sharp container, Assertive action)
brevity < 0.4 + directness < 0.4  →  Exploratory, verbose baseline (Fluid container, Thoughtful action)
brevity > 0.6 + directness < 0.4  →  Terse questions (Neutral container, Hesitant action)
brevity < 0.4 + directness > 0.6  →  Detailed commands (Rigid container, Assertive action)
```

### 2.3 The Complication: Direction Mismatch

**VAAM tracks the USER.** It builds a profile of how the user communicates — their word choices, their style, their attention patterns.

**FACES expresses the AI.** It encodes the AI's emotive state — its mood, its cognitive posture, its communicative intent.

VAAM→FACES means: **the user's communication patterns influence the AI's emotional presentation.** This is powerful but raises questions:

1. **Is this mirroring or manipulation?** If the user is terse and aggressive, should the AI mirror that (Sharp container, Assertive action)? Or should the AI counterbalance (Fluid container, Thoughtful action)? The spec doesn't answer this.

2. **Whose FACES state is it?** The spec describes FACES as the AI's emotive state. But if VAAM determines it from user patterns, it's really a *reflected* user state, not an independent AI state. This could be intentional — the AI's emotion mirrors the user's cognitive posture — but it should be a conscious design decision.

3. **Growth vs stasis.** VAAM tracks growth (vocabulary mastery, style evolution). If FACES is locked to VAAM, the AI's emotional range narrows to what the user's profile predicts. The spec's 38,400 states become a much smaller reachable set. FACES-Embed (the neural model) should be able to produce states *outside* the user's VAAM-predicted range — surprise, contradiction, concern — that keyword detection and VAAM mirroring cannot.

### 2.4 Proposed Architecture: VAAM-Informed, Not VAAM-Determined

The resolution is a **layered approach**:

```
Layer 1: VAAM Baseline (zero-compute, instant)
  - VaamProfile → default FACES state for this user
  - CommunicationStyle → default Container/Action
  - Active Circuit → current Focus
  - This is the "resting face" the AI returns to between turns

Layer 2: Text Detection (our detect.rs, near-zero compute)
  - Current message text → FACES state override
  - Modifies the VAAM baseline based on what was actually said
  - This is the "reactive face" that responds to content

Layer 3: FACES-Embed (NPU, sub-millisecond, future)
  - Neural classification of text → FACES state
  - Can produce states outside VAAM-predicted range
  - This is the "intelligent face" that understands subtext

Layer 4: Consent Gate (user override, always available)
  - User can nudge any byte before finalizing
  - Overrides all automated layers
  - This is the "authored face" that the user controls
```

**VAAM sets the baseline. Detection adjusts from baseline. FACES-Embed replaces detection when available. The Consent Gate is always supreme.**

### 2.5 What This Means for the Code

**In `faces-protocol` (zero-dependency crate):**
- Add a `FacesBaseline` struct that holds default Aura/Container/Focus/Action values
- Add a `FacesProfile` struct that tracks FACES state history, user overrides, and baseline evolution
- These are plain Rust structs with no external deps — serde can be optional via feature flag

**In `trinity-protocol` or a new bridge crate:**
- Implement `VaamProfile → FacesBaseline` conversion
- Map `CommunicationStyle` to Container/Action defaults
- Map `CircuitQuadrant` to Container/Focus defaults
- Map `word_weights` to Aura color tendencies

**In `trinity` (the server):**
- Wire the VAAM Bridge to produce FACES states alongside vocabulary results
- The `BridgeResult` gains a `faces_state: FacesState` field
- The Conductor's system prompt includes the FACES state

### 2.6 What VAAM Cannot Do for FACES

| FACES Need | VAAM Capability | Gap |
|-----------|----------------|-----|
| Per-message emotion detection | Tracks patterns over time, not per-message | Needs detect.rs or FACES-Embed |
| Sarcasm/irony detection | No incongruence detection | Needs FACES-Embed |
| Fine Aura color selection (256 values) | Word weights are semantic, not color-mapped | Needs explicit mapping or neural model |
| Transition velocity between turns | No temporal state sequence tracking | Needs FACES history (transition.rs) |
| Consent Gate user overrides | Agreements are topic-level, not emotion-level | Needs FACES-specific Consent Gate |

VAAM provides the **baseline** and the **growth trajectory**. It does not provide per-message detection, incongruence, or fine-grained Aura selection.

---

## Part 3: TODO List — FACES Before Integration

### Phase 1: Close Critical Code Gaps (Pure Rust, zero-dep)

- [ ] **FACES-1**: Implement `FacesProfile` struct — user's FACES baseline, state history, override log
- [ ] **FACES-2**: Implement Static Lock / Dynamic Interaction phases in `transition.rs`
  - Dynamic: Focus shifts with keystroke friction (needs input event hook)
  - Static: Freeze 4-byte payload on message commit
- [ ] **FACES-3**: Implement Mechanical Consent Gate
  - `nudge_aura(delta: i16)`, `nudge_container(direction: i8)`, etc.
  - `lock()` / `unlock()` / `commit()` state machine
  - User can adjust any byte before finalization
- [ ] **FACES-4**: Upgrade `detect.rs` from first-match-wins to scored matching
  - Each dimension (Action, Focus, Container, Aura) gets a confidence score
  - Highest score wins, not first match
  - Return `DetectionResult { state, confidence: [f32; 4] }`
- [ ] **FACES-5**: Add sentence segmentation for multi-sentence texts
  - Split on `.!?` boundaries
  - Detect each sentence independently
  - Aggregate with temporal weighting (last sentence has more weight)
- [ ] **FACES-6**: Add `serde` as optional feature flag (`#[cfg(feature = "serde")]`)
  - Enables Serialize/Deserialize for FacesState, FacesProfile
  - Keeps zero-dep default for NPU target
  - Enables persistence in CharacterSheet
- [ ] **FACES-6a**: Add physical AI protocol extensions
  - `to_u32()` / `from_u32()` — hardware register packing
  - `aura.to_rgb() -> (u8, u8, u8)` — physical light control
  - Safety presets: `EMERGENCY`, `LOW_POWER`, `HUMAN_PROXIMITY`
  - Behavioral parameter mapping: FACES state → speed/distance/volume (advisory, not authoritative)

### Phase 2: Evaluation Infrastructure

- [ ] **FACES-7**: Write the FACES Labeling Guide
  - Define ground truth mapping for common text types
  - Edge cases: sarcasm, mixed emotions, neutral statements
  - Multi-sentence texts: which sentence determines the state?
  - 50+ labeled examples covering all 5 containers, 6 focuses, 5 actions
- [ ] **FACES-8**: Build evaluation harness (`examples/evaluate.rs`)
  - Load JSONL dataset: `{"text": "...", "aura": N, "container": N, "focus": N, "action": N}`
  - Run `detect_faces()` on each entry
  - Report: per-byte accuracy, exact match, confusion matrices, Aura MAE
- [ ] **FACES-9**: Generate initial dataset (500 examples)
  - Hand-label 100 examples (the labeling guide's test set)
  - Use a teacher LLM to label 400 more
  - Human-review a sample of LLM labels for quality
- [ ] **FACES-10**: Benchmark latency
  - `detect_faces()` on 100-char, 500-char, 2000-char texts
  - `render_state()` throughput
  - `from_hex()` / `to_hex()` throughput
  - Establish baseline numbers for NPU budget planning

### Phase 3: VAAM Integration Prep

- [ ] **FACES-11**: Define `FacesBaseline` struct in `faces-protocol`
  - Default Aura, Container, Focus, Action for a specific user
  - `from_vaam_style(brevity, directness) -> FacesBaseline` (no VAAM dep, just takes f32s)
  - `from_circuit_quadrant(quadrant: u8) -> (Container, Focus)` (no VAAM dep, takes u8)
- [ ] **FACES-12**: Implement VAAM→FACES bridge in `trinity-protocol` or new crate
  - `VaamProfile::to_faces_baseline() -> FacesBaseline`
  - Map circuit_affinity → Container/Focus defaults
  - Map word_weights → Aura tendencies
  - Map style → Container/Action defaults
- [ ] **FACES-13**: Add FACES state to `BridgeResult` in `vaam_bridge.rs`
  - `faces_state: FacesState` field
  - VAAM baseline + detect.rs override = final state
- [ ] **FACES-14**: Add FACES system prompt block
  - The spec's recommended system prompt template
  - Inject current FACES state into LLM context
  - Instruct LLM to output FACES hex with each response

### Phase 4: ML Engineering (Future, needs hardware)

- [ ] **FACES-15**: ~~Decide Aura representation strategy~~ → **DECIDED: 10-way classification**
  - Use 10 named Aura constants (Neutral, Urgent, Energetic, Happy, Creative, Analytical, Calm, Contemplative, Unconventional, Tired)
  - 10-way softmax, Cross-Entropy loss
  - Humans can't agree on 256 colors; 10 moods are measurable and human-agreeable
  - See FACES_VALIDATION_FRAMEWORK.md §4.3 and FACES_TECHNICAL_INTENT.md §3.2
- [ ] **FACES-16**: ~~Design FACES-Embed architecture~~ → **DECIDED: DistilBERT-base + 4 heads**
  - Encoder: DistilBERT-base-uncased (~66M params, not 350M)
  - 4 classification heads: 10-way Aura, 5-way Container, 6-way Focus, 5-way Action
  - Multi-task loss: entropy-weighted (bytes with higher information get higher loss weight)
  - INT8 quantization for NPU, FP16 for fine-tuning
  - ONNX export (opset 17) → VitisAI/Lemonade for XDNA 2
  - Memory: ~33MB INT8 (not 175MB)
  - See FACES_TECHNICAL_INTENT.md §3.2 for full architecture spec
- [ ] **FACES-17**: Build dataset generation pipeline
  - Teacher model prompt for 100K text→FACES pairs
  - Validation strategy (human review sample)
  - Train/val/test split (80/10/10, stratified)
- [ ] **FACES-18**: NPU deployment
  - PyTorch → ONNX → VitisAI/Lemonade
  - INT8 quantization
  - Latency measurement on Strix Halo

---

## Summary Assessment

**What's solid:** Protocol layer (4-byte struct, encode/decode, render, transition math). This is the foundation and it's correctly implemented with 105 passing tests.

**What's placeholder:** Detection layer (keyword matching, not POS-based, no scoring). This proves the API works but cannot produce semantically correct results for complex text.

**What's missing:** Consent Gate, Static/Dynamic phases, FacesProfile, evaluation infrastructure, labeling guide, latency benchmarks.

**VAAM→FACES verdict:** VAAM can and should guide FACES — but as a **baseline setter**, not as the detector. VAAM tracks who the user is over time; FACES detection responds to what was said right now. The isomorphic mapping (V-A-A-M ↔ FACES bytes) is designed into the spec and should be implemented as a bridge layer. The zero-dependency constraint means the bridge lives outside `faces-protocol`, likely in `trinity-protocol` or a new integration crate.

**Critical path before integration:** FACES-7 (labeling guide) and FACES-8 (evaluation harness) are the most important items. Without them, we cannot measure whether any detection improvement actually improves accuracy. Everything else is engineering; those two are science.

---

## Part 2.5: Three-Device Architecture Gap (July 2026 Research)

### 2.5.1 What Changed

Research into Android XR (XREAL Aura), Google ADK, and Jetpack Compose for XR revealed that Trinity's architecture needs **three devices**, not two. The original spec assumed Strix Halo (base station) → XR client (output). The updated architecture adds a **phone as director**:

| Device | Role | Language | Framework | Status |
|---|---|---|---|---|
| Pixel 10 Pro XL | Director (human input) | Kotlin | ADK + Gemini Nano | Testable NOW |
| Strix Halo | Engine (AI compute) | Rust | trinity crate | Existing |
| XREAL Aura | Canvas (spatial output) | Kotlin | Jetpack Compose for XR + ARCore | Needs dev kit |

### 2.5.2 What's Missing for the Three-Device Pipeline

| Gap | Priority | Blocked By | Testable On |
|---|---|---|---|
| FACES WebSocket streaming endpoint (Rust server) | High | Nothing | Strix Halo (existing) |
| FACES Kotlin SDK (data class + WebSocket client) | High | WebSocket endpoint | Pixel 10 Pro XL (now) |
| ADK Socratic agent (on-device questioning) | Medium | ADK dependency | Pixel 10 Pro XL (now) |
| Jetpack Compose for XR FACES rendering | Medium | XREAL dev kit | XREAL Aura (when available) |
| ARCore spatial anchoring for FACES panels | Low | XREAL dev kit | XREAL Aura (when available) |

### 2.5.3 Reusable Assets from Bertrand-Masterclass

The Bertrand-Masterclass workspace contains a working Bevy + OpenXR engine targeting XREAL Aura. Key reusable assets (keeping Trinity's identity separate from Voix Vive):

- `AndroidManifest.xml` — template for XREAL Aura feature declarations
- `ipc.rs` — WebSocket broadcast pattern (warp + crossbeam + tokio)
- `spatial_ui.rs` concept → `SpatialPanel` composable
- `environment_manager.rs` concept → Compose state-based scene switching
- `hand_tracking.rs` concept → ARCore hand tracking + pinch gestures

See MASTER_PIVOT_DOCUMENT.md Appendix K.5 for full asset inventory.

---

## Part 4: Workflow Execution Plan

The FACES-1 through FACES-18 items above are the *feature list*. Below is the *execution plan* — 10 workflows that build them in dependency order, with 3 review gates.

| Workflow | FACES Items | Deliverable | Gate? |
|----------|-----------|------------|-------|
| W1: Protocol hardening | FACES-6, FACES-6a | to_u32/from_u32, to_rgb, safety presets, serde feature | No |
| W2: Scored detection | FACES-4 | DetectionResult with confidence scores | No |
| W3: Multi-sentence + profile | FACES-1, FACES-5 | Sentence segmentation, FacesProfile | No |
| W4: Consent Gate | FACES-3 | Lock/unlock/commit state machine | No |
| W5: Static/Dynamic phases | FACES-2 | Two-phase temporal model | ★ Gate 1: review crate |
| W6: Labeling guide | FACES-7 | Ground truth document | ★ Gate 2: approve guide |
| W7: Eval harness + benchmarks | FACES-8, FACES-10 | JSONL loader, per-byte F1, latency | No |
| W8: Emulator core | — | Terminal UI, state display, detection sim | ★ Gate 3: review emulator |
| W9: Emulator interaction | — | Consent Gate UI, VAAM sim, transitions | No |
| W10: Emulator fleet + physical | — | Multi-agent, behavior mapping, LED sim | No |

**Process:** PROGRESS.md in `crates/faces-protocol/` updated after every workflow. Tests must pass before next workflow starts. See FACES_TECHNICAL_INTENT.md §5.6 for full process.
