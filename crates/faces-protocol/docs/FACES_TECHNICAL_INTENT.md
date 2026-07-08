# FACES Technical & ML Systems Intent — Physical AI Engineering Management

> Date: 2026-07-02  
> Author: Cascade + Joshua Atkinson  
> Status: Research phase — protocol-first, physical AI as proof  
> Companion to: FACES_GAP_ANALYSIS.md, FACES_VALIDATION_FRAMEWORK.md, FACES_PYTHAGOREAN_RESEARCH.md

---

## 0. The Thesis of This Document

**FACES is the TCP/IP of emotive AI — a low-level 32-bit emotional state protocol that any system can use.**

Physical AI — robots, XR environments, embedded systems — is the *proof* that the protocol works, not the *product*. The product is the protocol itself and the reference implementation (Trinity). The value compounds at the protocol layer through adoption, not at the application layer through deployment scale.

This document defines the technical intent, ML systems engineering, and management framework for building FACES as a protocol first, with physical AI as the convergence proof. It is written during the research phase, before integration, so that every code decision forward is grounded in this strategic positioning.

**Strategic positioning:**
- **Open-source (the protocol):** `faces-protocol` crate, FACES specification, labeling guide, evaluation harness
- **Product (the integration):** VAAM→FACES bridge, FACES-Embed trained model, Trinity pipeline, XR rendering, fleet dashboard
- **Partner for (the proof):** AMD (NPU hardware), Purdue (annotators + study), future robot manufacturer (physical demo)

The five FACES documents form a complete picture:

| Document | Question It Answers |
|----------|-------------------|
| FACES_GAP_ANALYSIS.md | What's missing in the code? |
| FACES_VALIDATION_FRAMEWORK.md | How do we know it works? |
| **This document** | **What is it for, and how do we engineer it for physical AI?** |
| FACES_PYTHAGOREAN_RESEARCH.md | Can Pythagorean ratios improve transition metrics, grounded in neuroscience? |

---

## 1. What Physical AI Needs That FACES Provides

### 1.1 The Bandwidth Problem

Physical AI systems — robots, XR companions, autonomous tools — face a fundamental bandwidth problem. The emotional state of an AI agent must be communicated to humans and other agents in real-time, across constrained channels:

| Channel | Bandwidth | Latency Constraint | FACES Fit |
|---------|-----------|-------------------|-----------|
| CAN bus (robotics) | 1 Mbps | <1ms | 4 bytes = 32 bits = 0.03ms |
| I2C (embedded) | 100 Kbps | <10ms | 4 bytes = 0.32ms |
| BLE (wearables) | 1 Mbps | <5ms | 4 bytes = 0.03ms |
| WiFi (XR streaming) | 100+ Mbps | <20ms | 4 bytes = negligible |
| WiFi (phone → desktop) | 100+ Mbps | <20ms | 4 bytes = negligible |
| MCP/IPC (Trinity) | Local | <1ms | 4 bytes = instant |
| Hardware register | GHz | <1µs | 4 bytes = 1 register read |

Traditional emotive signaling — verbose text descriptions, 3D blendshape parameters, facial animation vectors — requires kilobytes per frame. FACES requires **4 bytes per state update**. This is a 100-1000× bandwidth reduction that makes real-time emotional state transmission feasible on constrained physical channels.

### 1.2 The Compute Problem

Physical AI systems are compute-constrained. A robot's GPU is busy with navigation, vision, and planning. Its NPU may be small. Its CPU may be a microcontroller.

| Emotion Rendering Approach | Compute Cost | Hardware Requirement | FACES Alternative |
|---------------------------|-------------|---------------------|------------------|
| 3D blendshape animation | 5-15W GPU | Dedicated GPU | 0W — character lookup table |
| Neural facial rendering | 10-30W GPU | High-end GPU + VRAM | 0W — 5 ASCII characters |
| Verbose LLM emotion text | 30-50 tokens | LLM inference per turn | 1-4 tokens — hex string |
| FACES-Embed (NPU) | ~5W, 1ms | NPU (XDNA 2) | Same NPU, parallel with GPU |
| Keyword detection (CPU) | ~0W, <1ms | Any CPU | Already implemented |

FACES renders on **any display** (LED matrix, e-ink, segmented LCD, terminal, XR panel) with **zero GPU cost**. The emotional state is encoded in 4 bytes, rendered as 5 characters, and the human's pre-cognitive pareidolia does the rest. No neural rendering pipeline. No blendshape interpolation. No VRAM allocation.

### 1.3 The Determinism Problem

Physical AI systems require **deterministic behavior** for safety certification. A robot's emotional state must not be ambiguous — if the state says "urgent," every receiver must interpret it identically.

| Approach | Determinism | Safety Certifiable? |
|----------|------------|-------------------|
| LLM-generated emotion text | Non-deterministic (temperature > 0) | No — same input, different output |
| Neural facial rendering | Non-deterministic (sampling) | No — visual output varies |
| FACES 4-byte protocol | **Fully deterministic** | **Yes — same bytes, same face, same meaning** |
| FACES-Embed (NPU) | Deterministic at inference | Yes — fixed weights, fixed output |

FACES is a **deterministic protocol**, not a generative system. Given the same 4 bytes, every receiver renders the same face and interprets the same emotional state. This is critical for:
- **Safety certification** (ISO 13849, IEC 61508) — deterministic behavior is required
- **Audit trails** — emotional state history is 4 bytes per turn, trivially loggable
- **Reproducibility** — debugging requires deterministic state reproduction
- **Inter-agent communication** — robots must agree on what "urgent" means

### 1.4 The Latency Problem

In physical AI, latency is safety. A robot that takes 500ms to communicate "I am stopping" may collide before the human reacts.

| Signal Type | Generation Time | Human Perception Time | Total |
|-------------|----------------|----------------------|-------|
| LLM emotion text (30-50 tokens) | 500-2000ms | 300-500ms (reading) | 800-2500ms |
| FACES keyword detection | <1ms | 100-200ms (pareidolia) | 101-201ms |
| FACES-Embed (NPU) | <1ms | 100-200ms (pareidolia) | 101-201ms |
| FACES register read | <1µs | 100-200ms (pareidolia) | ~200ms |

FACES moves the bottleneck from **generation** (LLM text) to **perception** (human pareidolia). The AI communicates its state in <1ms. The human perceives it in ~200ms. Total emotional communication latency: ~200ms, which is within human reaction time for safety-critical scenarios.

---

## 2. The Physical AI System Architecture

### 2.1 The Three-Layer FACES Stack for Physical AI

```
┌─────────────────────────────────────────────────────┐
│ LAYER 3: ACTUATION (Physical Output)                │
│                                                     │
│  FACES state → Physical behavior                    │
│  • Aura → LED color, ambient lighting, XR env color │
│  • Container → Robot posture, boundary distance     │
│  • Focus → Movement speed, attention direction      │
│  • Action → Voice volume, movement intent           │
│                                                     │
│  Three-device targets:                              │
│  • Phone (Pixel 10 Pro XL): ASCII face display,    │
│    Socratic dialogue, ADDIECRAPEYE phase tracker    │
│  • XR (XREAL Aura): SpatialPanel with FACES face,  │
│    Orbiter with state info, 3D avatar expression,   │
│    ARCore spatial anchoring, EYE phase UI           │
│  • Robot: LED matrix, speakers, motors              │
│                                                     │
│  Hardware: Phone screen, XR display, LED matrix,    │
│  speakers, motors                                   │
├─────────────────────────────────────────────────────┤
│ LAYER 2: DETECTION (Emotional State Computation)    │
│                                                     │
│  Input → FACES state                                │
│  • Text → FACES (detect.rs or FACES-Embed)          │
│  • Voice prosody → FACES (FACES-Embed on NPU)       │
│  • Sensor readings → FACES (rule-based mapping)     │
│  • User override → FACES (Consent Gate)             │
│                                                     │
│  Hardware: NPU (XDNA 2), CPU, microphone, camera    │
├─────────────────────────────────────────────────────┤
│ LAYER 1: PROTOCOL (State Representation)            │
│                                                     │
│  4 bytes = 32 bits = 38,400 states                  │
│  • Aura (8-bit) + Container (5) + Focus (6) + Action(5) │
│  • Hardware register compatible (single u32)        │
│  • Deterministic, zero-allocation, zero-dependency  │
│  • Cross-platform: terminal, LED, XR, robot, fleet  │
│                                                     │
│  Hardware: Any (protocol is hardware-agnostic)      │
└─────────────────────────────────────────────────────┘
```

### 2.2 The Strix Halo Physical AI Configuration

```
┌─────────────────────────────────────────────────────────────┐
│ STRIX HALO (Base Station / Edge Compute)                    │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ GPU         │  │ NPU         │  │ CPU         │         │
│  │ RDNA 3.5    │  │ XDNA 2      │  │ Zen 5       │         │
│  │             │  │             │  │             │         │
│  │ LLM inference│ │ FACES-Embed  │  │ Rust        │         │
│  │ 59.39 TFLOPS │ │ 50 TOPS     │  │ orchestration│        │
│  │ FP16        │  │ INT8/MXFP4  │  │             │         │
│  │             │  │             │  │ detect.rs   │         │
│  │ Navigation  │  │ Emotion     │  │ Consent Gate│         │
│  │ Vision      │  │ detection   │  │ Telemetry   │         │
│  │ Planning    │  │ Sensor fusion│ │ VAAM bridge │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │                 │
│         └────────────────┼────────────────┘                 │
│                          │                                   │
│                    ┌─────┴─────┐                             │
│                    │ FACES     │  4 bytes per state update   │
│                    │ State     │  32 bits, 1 register        │
│                    │ [A][C][F][V]│                          │
│                    └─────┬─────┘                             │
│                          │                                   │
└──────────────────────────┼───────────────────────────────────┘
                           │
                    ┌──────┴──────┐
│                    │ Transport   │
                    │ CAN/I2C/BLE │
                    │ WiFi6E/WS   │
                    │ Register    │
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           │               │               │
    ┌──────┴──────┐ ┌─────┴─────┐ ┌───────┴───────┐
    │ ROBOT       │ │ XR CLIENT │ │ FLEET MONITOR │
    │             │ │           │ │               │
    │ LED matrix  │ │ XREAL Aura│ │ Terminal      │
    │ display     │ │ ComposeXR │ │ dashboard     │
    │             │ │           │ │               │
    │ Motor       │ │ SpatialPnl│ │ 20 robots:    │
    │ behavior    │ │ Orbiter   │ │ (o_o) R01     │
    │             │ │ GlTF avatar│ │ [><v] R03 ←!  │
    │ Speaker     │ │ ARCore    │ │ |.._| R04     │
    │ volume      │ │ anchors   │ │               │
    └─────────────┘ └───────────┘ └───────────────┘

    ┌─────────────┐
    │ PHONE       │
    │ Pixel 10 XL │
    │             │
    │ ASCII face  │
    │ Socratic UI │
    │ Phase track │
    │ ADK + Nano  │
    └─────────────┘
```

### 2.3 Heterogeneous Compute Budget

The critical insight for engineering management: **FACES does not compete with the LLM for compute.** It runs on a different processor.

| Workload | Processor | Power | Latency | Memory |
|----------|-----------|-------|---------|--------|
| LLM inference (7B-70B) | GPU (RDNA 3.5) | 55-120W | 20-50ms/token | 4-40GB VRAM |
| FACES-Embed (~66M) | NPU (XDNA 2) | 5-15W | <1ms | ~33MB INT8 |
| FACES keyword detection | CPU (Zen 5) | <1W | <1ms | <1MB |
| FACES rendering | Any display | ~0W | <1ms | 5 bytes |
| FACES detection (edge) | Gemini Nano (phone/XR) | 2-5W | <5ms | ~4GB on-device |

**The NPU is idle during LLM inference.** This is the key architectural argument: FACES uses compute that would otherwise be wasted. The NPU doesn't slow down the GPU. The GPU doesn't slow down the NPU. They are independent silicon on the same APU.

This is what makes the AMD pitch work: "Trinity is the only software that uses both the GPU and NPU simultaneously for different AI workloads." FACES on NPU is not a cost — it's utilization of an otherwise-idle resource.

---

## 3. FACES-Embed ML Systems Engineering

### 3.1 Model Architecture Specification

The spec describes two parameter scales: "5M to 50M" (micro-encoder) and "~66M" (DistilBERT standard). These serve different deployment scenarios:

| Variant | Parameters | Memory (INT8) | Target Hardware | Use Case |
|---------|-----------|---------------|----------------|----------|
| FACES-Embed Micro | 5-50M | 5-50MB | Microcontroller, mobile NPU | Edge robots, wearables, ESP32-class |
| FACES-Embed Standard | ~66M (DistilBERT) | ~33MB | Strix Halo NPU, desktop | Trinity base station, XR streaming |
| FACES-Embed Teacher | 7B+ | 3.5GB+ | Cloud or Strix Halo GPU | Dataset generation, distillation |

**Recommendation for Trinity v1:** Build FACES-Embed Standard using DistilBERT-base (~66M params) for Strix Halo. The spec mentions 350M as an upper bound, but DistilBERT is proven, ONNX-exportable, and 5× smaller — making it a better fit for NPU deployment. Use keyword detection as the fallback for any device without an NPU. The micro variant is a future optimization for embedded deployment.

### 3.2 Multi-Task Classification Architecture

```
                    Input Text
                        │
                ┌───────┴───────┐
                │  Encoder      │
                │  (BERT-style  │
                │   transformer) │
                │  ~66M params  │
                └───────┬───────┘
                        │
                 Pooled Hidden State
                        │
          ┌─────────┬───┴───┬─────────┐
          │         │       │         │
     ┌────┴────┐ ┌──┴──┐ ┌──┴──┐ ┌───┴───┐
     │ Head 0  │ │Head1│ │Head2│ │Head 3 │
     │ Aura    │ │Cont │ │Focus│ │Action │
     │         │ │     │ │     │ │       │
     │ Option A│ │ 5-  │ │ 6-  │ │ 5-    │
     │ 10-way  │ │ way │ │ way │ │ way   │
     │ softmax │ │soft │ │soft │ │softmax│
     └────┬────┘ │max  │ │max  │ └───┬───┘
          │      └──┬──┘ └──┬──┘     │
          │         │       │        │
     ┌────┴────┐    │       │   ┌────┴────┐
     │Loss 0   │    │       │   │Loss 3   │
     │CE (10)  │    │       │   │CE (5)   │
     └─────────┘    │       │   └─────────┘
               ┌────┴──┐ ┌──┴────┐
               │Loss 1 │ │Loss 2 │
               │CE (5) │ │CE (6) │
               └───────┘ └───────┘

    L_total = w₀·L_aura + w₁·L_container + w₂·L_focus + w₃·L_action
```

**Architecture decisions to lock down before training:**

| Decision | Options | Recommendation | Rationale |
|----------|---------|---------------|-----------|
| Encoder type | BERT, DistilBERT, TinyBERT, LSTM | DistilBERT-base | 66M params, proven, ONNX-exportable, fits NPU |
| Total parameter budget | 5M, 50M, 350M | ~66M (DistilBERT + 4 heads) | Spec says 350M but DistilBERT is proven and smaller |
| Aura head | 256-way, 10-way, regression | 10-way softmax | Humans can't agree on 256 colors; 10 moods are measurable |
| Loss weights | Equal, entropy-weighted, learned | Entropy-weighted | Bytes with higher information get higher loss weight |
| Tokenizer | WordPiece, BPE, SentencePiece | WordPiece (DistilBERT native) | Compatibility with pretrained weights |
| Max sequence length | 128, 256, 512 | 256 | Covers most Trinity messages without padding waste |
| Quantization | FP32, FP16, INT8, MXFP4 | INT8 for NPU, FP16 for fine-tuning | NPU target is INT8; train in FP16 |

### 3.3 Training Pipeline

```
Phase 0: Labeling Guide (FACES-7)
  │  Defines ground truth for all 4 bytes
  ↓
Phase 1: Seed Dataset (500 human-labeled examples)
  │  3+ annotators, IAA study (Cohen's Kappa > 0.5)
  ↓
Phase 2: Teacher Model Labeling (100K examples)
  │  7B+ LLM with FACES labeling system prompt
  │  Validate teacher accuracy against seed set
  │  Target: teacher agreement with human labels > 80%
  ↓
Phase 3: Data Preparation
  │  JSONL format: {"text": "...", "aura": N, "container": N, "focus": N, "action": N}
  │  Train/Val/Test split: 80/10/10
  │  Stratified by Container, Focus, Action (ensure all classes represented)
  │  Class balance report: identify underrepresented states
  ↓
Phase 4: Pretraining (optional)
  │  Start from DistilBERT-base-uncased pretrained weights
  │  Don't train from scratch — leverage existing language understanding
  ↓
Phase 5: Fine-tuning (Multi-Task)
  │  Freeze encoder for 1 epoch (train heads only)
  │  Then unfreeze encoder, train end-to-end for 5-10 epochs
  │  Early stopping on validation Macro-F1
  │  Learning rate: 2e-5 (encoder), 1e-3 (heads)
  │  Batch size: 32 (or gradient accumulation)
  ↓
Phase 6: Evaluation
  │  Per-byte F1, confusion matrices, calibration (ECE)
  │  Compare to keyword baseline
  │  If FACES-Embed doesn't beat baseline by >10% Macro-F1 → not worth deploying
  ↓
Phase 7: Export & Deployment
  │  PyTorch → ONNX (opset 17)
  │  ONNX → VitisAI/Lemonade (XDNA 2 compilation)
  │  INT8 quantization (post-training)
  │  Latency measurement on Strix Halo NPU
  │  Target: <1ms inference, <175MB memory
```

### 3.4 The Dataset Cold Start Problem — Detailed Strategy

The spec identifies this as the primary bottleneck. Here is the engineering plan:

**Teacher Model System Prompt:**
```
You are a FACES protocol annotation expert. Your task is to label text with 
a 4-byte FACES emotional state. For each input text, output a JSON object:

{
  "aura": <integer 0-255>,     // ANSI color index (mood)
  "container": <integer 0-4>,  // 0=Neutral(), 1=Rigid[], 2=Fluid{}, 3=Defensive||, 4=Sharp<>
  "focus": <integer 0-5>,      // 0=Neutral(oo), 1=Intense(><), 2=Open(OO), 3=Distant(..), 4=Happy(^^), 5=Tired(--)
  "action": <integer 0-4>,     // 0=Withheld(_), 1=Assertive(v), 2=Playful(~), 3=Thoughtful(-), 4=Hesitant(.)
  "confidence": <0.0-1.0>,     // Your confidence in this labeling
  "reasoning": "<one sentence explaining your choice>"
}

Rules:
- Container: 0=Neutral(open), 1=Rigid(formal), 2=Fluid(creative), 3=Defensive(protected), 4=Sharp(urgent)
- Focus: 0=Neutral, 1=Intense(concentrating), 2=Open(surprised), 3=Distant(dissociated), 4=Happy(success), 5=Tired(depleted)
- Action: 0=Withheld(silent), 1=Assertive(commanding), 2=Playful(ironic), 3=Thoughtful(evaluating), 4=Hesitant(uncertain)
- Aura: Use these 10 moods: 245=Neutral, 160=Urgent, 208=Energetic, 220=Happy, 120=Creative, 39=Analytical, 27=Calm, 91=Contemplative, 177=Unconventional, 238=Tired
- For multi-sentence text, label based on the dominant emotional tone
- If text is ambiguous, choose the less extreme state
```

**Validation Strategy:**
1. Teacher labels 500 examples
2. Human reviews 50 random samples (10%)
3. Calculate teacher-human agreement (Cohen's Kappa)
4. If κ < 0.6, revise teacher prompt and re-label
5. Once κ > 0.6, teacher labels remaining 99,500 examples
6. Human reviews 100 random samples from the full set
7. Final dataset: 100K examples with >60% teacher-human agreement

**Data Quality Metrics:**

| Metric | Target | Action if Below |
|--------|--------|----------------|
| Teacher-human κ | > 0.60 | Revise teacher prompt |
| Class coverage | All 5 containers, 6 focuses, 5 actions represented | Generate synthetic examples for missing classes |
| Class balance | No class < 5% of dataset | Oversample rare classes or generate synthetic data |
| Aura coverage | All 10 moods represented | Generate examples for missing moods |
| Label noise estimate | < 10% | Filter low-confidence teacher labels (< 0.7) |

### 3.5 ONNX Export & NPU Deployment

```
PyTorch Model
    │
    ↓  torch.onnx.export()
    │
ONNX Model (opset 17)
    │
    ↓  VitisAI / Lemonade SDK
    │
Compiled NPU Model (INT8)
    │
    ↓  Load on Strix Halo
    │
Runtime: <1ms inference, <175MB memory
```

**NPU-Specific Considerations:**

| Concern | Mitigation |
|---------|-----------|
| XDNA 2 operator support | Verify all DistilBERT ops are supported; replace unsupported ops |
| INT8 quantization accuracy | Post-training quantization may lose 2-5% F1; monitor |
| Memory bandwidth | ~66M params at INT8 = ~33MB; XDNA 2 has sufficient bandwidth |
| Batch size on NPU | NPU may only support batch=1; design for single-sample inference |
| Model size vs. NPU cache | ~33MB may fit NPU on-chip memory; verify off-chip access latency if not |
| Fallback path | If NPU unavailable, fall back to CPU keyword detection (already implemented) |

---

## 4. Physical AI Integration Points

### 4.1 FACES → Physical Behavior Mapping

The 4-byte FACES state should map to physical robot behavior parameters. This is where FACES becomes actuation, not just display:

| FACES Byte | Value | Physical Parameter | Robot Behavior |
|-----------|-------|-------------------|---------------|
| Container | Neutral () | Approach distance: 1.5m | Open, approachable posture |
| Container | Rigid [] | Approach distance: 2.0m | Formal, maintains distance |
| Container | Fluid {} | Approach distance: 1.0m | Close, creative interaction |
| Container | Defensive \|\| | Approach distance: 3.0m | Keeps distance, protective |
| Container | Sharp <> | Approach distance: 0.5m | Urgent, close approach |
| Focus | Neutral oo | Movement speed: 0.5 m/s | Normal pace |
| Focus | Intense >< | Movement speed: 1.5 m/s | Fast, purposeful |
| Focus | Open OO | Movement speed: 0.0 m/s | Stopped, observing |
| Focus | Distant .. | Movement speed: 0.2 m/s | Slow, background processing |
| Focus | Happy ^^ | Movement speed: 0.8 m/s | Relaxed, confident |
| Focus | Tired -- | Movement speed: 0.3 m/s | Slow, low power |
| Action | Withheld _ | Voice volume: 0 dB | Silent |
| Action | Assertive v | Voice volume: 65 dB | Clear, commanding |
| Action | Playful ~ | Voice volume: 55 dB | Casual, varied intonation |
| Action | Thoughtful - | Voice volume: 45 dB | Quiet, measured |
| Action | Hesitant . | Voice volume: 35 dB | Soft, uncertain |
| Aura | Any | LED color: RGB(aura) | Ambient mood lighting |

**This mapping must be:**
- **Configurable** — different robots have different safe speeds and distances
- **Constrained** — safety limits override FACES-driven behavior (a Sharp container doesn't permit unsafe speed)
- **Auditable** — every behavior change logged with the FACES state that triggered it

### 4.2 FACES → XR Environment Mapping

On the Strix Halo → XREAL Aura pipeline, FACES state drives the spatial environment via Jetpack Compose for XR:

| FACES Component | XR Output | Compose for XR Implementation |
|----------------|-----------|-------------------------------|
| Aura (color) | Ambient environment lighting | `aura.to_rgb()` → SpatialPanel background color, ARCore lighting |
| Container (shape) | Avatar body posture | `SpatialGltfModel` with Container-driven pose (Fluid → relaxed, Sharp → tense) |
| Focus (eyes) | Avatar eye state | `SpatialGltfModel` with Focus-driven eye shape (Intense → narrowed, Open → wide) |
| Action (mouth) | Avatar mouth expression | `SpatialGltfModel` with Action-driven mouth (Playful → smile, Withheld → neutral) |
| Full state | Environment selection | FACES state → Compose state → environment switch (Zen Garden / Studio / Stage) |
| Full state | Spatial panel placement | `SpatialPanel` with `SubspaceModifier` positioning, `Orbiter` with state info |
| Congruence | Orbiter indicator | `Orbiter` attached to panel showing Congruent/Incongruent/Neutral |
| Confidence | Orbiter opacity | `Orbiter` alpha channel tied to detection confidence (0.0-1.0) |

**Three-device transport:** FACES states flow from Strix Halo via WebSocket to both the phone (Pixel 10 Pro XL) and XR client (XREAL Aura puck). The phone displays ASCII face + Socratic dialogue; the XR client renders spatial panels + 3D avatar. Both receive the same 4-byte state.

**Standalone fallback:** When the Strix Halo base station is offline, the phone or XR puck can run FACES detection locally via ADK + Gemini Nano. This is lower fidelity than FACES-Embed on NPU but enables basic emotion detection without the base station.

The Bertrand-Masterclass `environment_manager.rs` already implements scene switching (Zen Garden / Studio / Stage) in Bevy. The Kotlin XR client will implement the same scene state machine using Compose state. FACES state drives the scene selection — this is "set and setting" from the triple reflection, mechanically implemented.

### 4.3 FACES → Fleet Telemetry

For multi-robot or multi-agent monitoring:

```
FLEET STATUS — 2026-07-02 14:23:07
─────────────────────────────────────
Agent 01: (o_o)  [F5010000]  idle, neutral
Agent 02: {^^~}  [78020402]  creative, happy, playful
Agent 03: [><v]  [A0040101]  URGENT, intense, assertive ← ATTENTION
Agent 04: |.._|  [EE030300]  defensive, distant, withheld
Agent 05: <>--   [A0040503]  CRITICAL, sharp, tired ← BATTERY LOW
Agent 06: (o-v)  [F5000001]  neutral, thoughtful, assertive
─────────────────────────────────────────
```

**Why this works:** An operator scanning 20+ agents can identify problems in <1 second through pareidolia. The red/sharp/tired face triggers immediate attention without reading a single log line. This is the spec's "agentic telemetry" vision, implemented as a fleet dashboard.

---

## 5. Engineering Management Framework

### 5.1 Development Phases — Physical AI Aligned

| Phase | Duration | Deliverable | Physical AI Milestone |
|-------|----------|------------|----------------------|
| **P0: Foundation** | Done | Protocol crate (152 tests) | 4-byte state, encode/decode, render, scored detection |
| **P1: Measurement** | 1-2 weeks | Labeling guide, eval harness, benchmarks | Can measure semantic correctness |
| **P2: Detection** | 1-2 weeks | Multi-sentence, FacesProfile, Consent Gate | Detector produces measurable results |
| **P3: Terminal Demo** | 1 week | Split terminal, NPU+GPU, video | AMD pitch material |
| **P4: Phone Director** | 2-4 weeks | Kotlin app, ADK + Gemini Nano, Socratic agent | Phone controls Trinity, testable on Pixel 10 Pro XL |
| **P5: XR Client** | 4-8 weeks | Compose for XR, SpatialPanel, avatar, ARCore | Spatial computing demo on XREAL Aura |
| **P6: FACES-Embed** | 4-8 weeks | Dataset, training, ONNX, NPU deploy | Neural detection on NPU |
| **P7: Physical AI** | 8-16 weeks | Robot integration, fleet telemetry, safety | Physical AI deployment |

### 5.2 Risk Register

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|-----------|
| Low IAA on Aura byte | High | Medium | Use 10-way classification, not 256-way |
| FACES-Embed doesn't beat keyword baseline | Medium | High | Keyword baseline is the fallback; NPU cycles aren't wasted |
| NPU operator support gaps | Medium | High | Verify ONNX op support before training; have CPU fallback |
| Dataset quality too low | Medium | High | IAA study before scaling; human review of teacher labels |
| Pareidolia doesn't trigger cross-culturally | Low | Critical | Validation study with diverse participants |
| Class imbalance makes model useless | High | Medium | Stratified sampling, oversampling, Macro-F1 metric |
| Integration breaks zero-dep constraint | Medium | Medium | Bridge lives outside faces-protocol crate |
| Physical AI safety liability | Low | Critical | FACES is advisory, not authoritative; safety systems override |

### 5.3 Build vs. Buy vs. Partner Decisions

| Component | Build | Buy | Partner | Decision |
|-----------|-------|-----|--------|----------|
| FACES protocol crate | ✅ | — | — | Build — it's the IP |
| Keyword detector | ✅ | — | — | Build — already done |
| FACES-Embed model | ✅ | — | — | Build — domain-specific, no off-the-shelf |
| Dataset | ✅ (seed) | — | ✅ (teacher LLM) | Hybrid — human seed + LLM scaling |
| ONNX→NPU compilation | — | — | ✅ (AMD VitisAI) | Partner — AMD tooling |
| XR rendering | ✅ | — | — | Build — Bevy codebase exists |
| Robot hardware | — | ✅ | — | Buy — not building robots yet |
| Safety certification | — | — | ✅ (UL/CE) | Partner — when needed |

### 5.4 Team & Resource Requirements

| Role | When Needed | Duration | Could Be |
|------|------------|----------|----------|
| Rust engineer (Joshua) | Now | Ongoing | Primary |
| ML engineer | P6 (FACES-Embed) | 4-8 weeks | Joshua + Cascade, or collaborator |
| Data annotator | P1 (labeling) | 2 weeks | 3+ people for IAA (students?) |
| UX researcher | P1 (pareidolia study) | 2 weeks | Purdue collaborator |
| NPU/ONNX specialist | P6 (deployment) | 1-2 weeks | AMD dev relations (if partnership) |
| XR developer | P7 (XR prototype) | 4-8 weeks | Joshua (Bevy codebase exists) |

**Key insight:** The labeling and IAA study (P1) is where external help is most valuable. 3+ annotators are needed for valid Kappa scores. Purdue students could serve as annotators and study participants — this is both research and pedagogy.

### 5.5 Success Criteria Per Phase

| Phase | Success Criterion | Measurement |
|-------|------------------|-------------|
| P0 | Protocol layer correct | 105 tests passing ✅ |
| P1 | Can measure detector quality | Eval harness runs on labeled dataset, reports per-byte F1 |
| P2 | Detector beats random baseline | Macro-F1 > 0.40 for Container, Focus, Action |
| P3 | User can override any byte | Consent Gate tests pass, lock/commit verified |
| P4 | VAAM sets meaningful baseline | VAAM→FACES conversion produces non-neutral defaults for non-neutral profiles |
| P5 | AMD demo runs on Strix Halo | NPU inference <1ms, GPU t/s unaffected, 2-min video recorded |
| P6 | FACES-Embed beats keyword baseline | Macro-F1 improvement > 10% across all bytes |
| P7 | FACES drives XR environment | Environment changes with FACES state, visible in Bevy desktop |
| P8 | Robot displays FACES face | LED matrix renders face, behavior adjusts with state |

### 5.6 Process & Drift Prevention

Building 10 workflows across multiple sessions requires discipline to prevent scope creep and context drift.

**3 Review Gates:**

| Gate | When | What Joshua Reviews | Time |
|------|------|---------------------|------|
| Gate 1 | After W1-W5 | Full crate holistically — all new code, tests, API surface | 15 min |
| Gate 2 | After W6 | Labeling guide — the ground truth definition | 30 min |
| Gate 3 | After W8 | Emulator core — visual check, interaction feel | 5 min |

Between gates, Cascade executes autonomously. Joshua does not need to gate each workflow.

**PROGRESS.md:**

A `PROGRESS.md` file in `crates/faces-protocol/` is the single source of truth for build state. Updated after every workflow:

```markdown
## Current State
- Last workflow completed: W2
- Tests passing: 132
- Next workflow: W3

## Completed
- W1: Protocol hardening (to_u32, to_rgb, safety presets, serde)
- W2: Scored detection (DetectionResult with confidence scores)

## Open Questions
- None

## Notes
- Aura to_rgb uses ANSI → RGB mapping (xterm palette)
```

**Anti-Drift Rules:**

1. **One workflow at a time** — no parallel work, no half-finished features
2. **Tests are the quality gate** — `cargo test -p faces-protocol` must pass before a workflow is "done"
3. **The 4 docs are the spec** — if unsure what to build, re-read the docs, don't guess
4. **If the docs don't answer it, ask Joshua** — don't invent requirements
5. **No scope creep** — if a workflow reveals additional work, log it in PROGRESS.md as an open question, don't pursue it mid-workflow

**Session Structure:**

```
Session start:
  1. Read PROGRESS.md
  2. Read relevant doc sections for current workflow
  3. Execute workflow
  4. Run tests
  5. Update PROGRESS.md
  6. If gate workflow: notify Joshua for review

Session end:
  - Tests passing
  - PROGRESS.md updated
  - Next workflow identified
```

---

## 6. The Physical AI Roadmap — From Terminal to Robot

### 6.1 The Convergence Arc

```
2025: Trinity ID AI OS (terminal)
  │  FACES protocol, VAAM, ADDIECRAPEYE, autopoietic agents
  │  Hosted on Strix Halo, served to Purdue
  │
  ↓  + FACES crate (this work)
  │
2026 Q3: Terminal Demo (Strix Halo)
  │  FACES on NPU + LLM on GPU
  │  Split terminal: LLM output + FACES telemetry
  │  AMD pitch material
  │
  ↓  + Phone app (Pixel 10 Pro XL, ADK + Gemini Nano)
  │
2026 Q3-Q4: Phone Director (Pixel 10 Pro XL)
  │  Socratic questioning on-device via ADK + Gemini Nano
  │  ADDIECRAPEYE phase tracker
  │  FACES state display on phone screen
  │  WebSocket client → Strix Halo
  │  Testable NOW — no XR hardware needed
  │
  ↓  + XR client (XREAL Aura, Jetpack Compose for XR)
  │
2026 Q4: Spatial Computing (Strix Halo + Phone + XREAL Aura)
  │  Three-device pipeline: Phone (director) → Desktop (engine) → XR (canvas)
  │  FACES state streamed to XR client via WebSocket
  │  SpatialPanel with FACES face, Orbiter with state info
  │  SpatialGltfModel avatar with FACES-driven expression
  │  ARCore spatial anchoring for FACES panel placement
  │  EYE phase: Envision, Yoke, Evolve in spatial UI
  │  "Imagination becomes visible in space"
  │
  ↓  + Physical behavior mapping + fleet telemetry
  │
2027: Physical AI (Strix Halo + Robot)
  │  FACES state drives robot behavior
  │  LED matrix face display
  │  Fleet monitoring dashboard
  │  Safety-certified emotional communication
  │
  ↓  + Multi-agent coordination + cross-lingual deployment
  │
2027+: Fleet Physical AI
     Multiple robots, shared FACES state space
     Operator sees 20+ faces, pareidolia does triage
     Cross-cultural, cross-lingual, deterministic
```

### 6.2 What Each Phase Proves

| Phase | What It Proves | To Whom |
|-------|---------------|---------|
| Terminal demo | NPU + GPU heterogeneous compute works | AMD |
| Phone director | Socratic questioning on-device, ADK + Gemini Nano | Purdue, Google |
| XR prototype | FACES drives spatial environment via Compose for XR | Purdue, AMD, XR community |
| Physical AI | FACES is a robot communication standard | Robotics industry |
| Fleet | FACES scales to multi-agent monitoring | Enterprise, defense, logistics |

### 6.3 The Moat

FACES is not just code — it's a **protocol**. Like HTTP, TCP, or USB, its value increases with adoption. The moat is:

1. **First-mover advantage** — no other emotive AI protocol exists at this level of specification
2. **Hardware integration** — designed for NPU, not retrofitted
3. **Theoretical foundations** — Mian Xiang, FACS, Mehrabian, Watts, Brown — not invented from nothing
4. **Dataset** — the first labeled text→FACES dataset is a defensible asset
5. **Open standard** — if adopted by others, Trinity becomes the reference implementation

**The strategy:** Open-source the protocol (faces-protocol crate, zero-dep, Apache-2.0). Keep the integration layer (VAAM bridge, FACES-Embed, Trinity pipeline) as the product. The protocol is the standard; Trinity is the best implementation of it.

---

## 7. Open Research Questions

These are questions that need empirical answers before FACES can be deployed in physical AI. They are listed in priority order.

### 7.1 Critical (must answer before physical AI deployment)

1. **Does pareidolia trigger for ASCII faces cross-culturally?**
   - The spec assumes universal pareidolia, but most pareidolia research uses realistic faces, not ASCII
   - Test with participants from diverse cultural backgrounds
   - If ASCII pareidolia is culture-dependent, the protocol's universality claim is weakened

2. **What is the minimum display resolution for FACES pareidolia?**
   - 5 characters on a 1080p terminal: clear
   - 5 characters on an 8×8 LED matrix: does pareidolia still trigger?
   - 4 bytes on a single RGB LED (Aura only, no geometry): is color alone sufficient?

3. **Can FACES-Embed achieve <1ms latency on XDNA 2?**
   - Theoretical: ~66M params, INT8, batch=1 → should be <1ms
   - Practical: depends on operator support, memory bandwidth, compilation
   - Must be measured on actual hardware

4. **What happens when FACES state conflicts with safety systems?**
   - FACES says "Sharp, Intense, Assertive" (move fast, close, loud)
   - Safety system says "obstacle detected, stop"
   - Safety must always win. How is this enforced in the architecture?

### 7.2 Important (must answer before scale deployment)

5. **How does FACES state history affect user trust over time?**
   - Does consistent FACES behavior build trust?
   - Does inconsistent FACES behavior (frequent state changes) erode trust?
   - What is the optimal state change frequency?

6. **What is the FACES state distribution in real Trinity conversations?**
   - If 90% of conversations are Neutral, the 38,400 states are mostly unused
   - This informs whether the protocol is over-engineered or right-sized
   - Measure after P4 (VAAM integration) with real conversations

7. **Can FACES-Embed detect incongruence (sarcasm)?**
   - The spec calls for this but keyword detection cannot do it
   - FACES-Embed with the full encoder may detect incongruent text→state mappings
   - Test on a labeled sarcasm dataset

### 7.3 Exploratory (future research)

8. **Can FACES state be controlled via brain-computer interface (BCI)?**
   - The spec mentions single-switch accessibility
   - BCI could allow motor-impaired users to set FACES state directly
   - 4 bytes, 4 dimensions, each adjustable with one cognitive signal

9. **Can FACES states be interpolated in real-time for smooth robot animation?**
   - The transition.rs lerp function exists
   - Can it drive continuous LED matrix animation at 30fps?
   - What is the perceptual threshold for state change detection?

10. **Can FACES replace facial recognition for robot-human interaction?**
    - Instead of the robot reading the human's face (privacy concern)
    - The human reads the robot's FACES face (no privacy concern)
    - Does this asymmetry improve or degrade interaction quality?

---

## 8. Document Cross-Reference

| Document | Purpose | Key Output |
|----------|---------|-----------|
| MASTER_PIVOT_DOCUMENT.md | Trinity vision and strategy | The thesis, triple reflection, AMD pitch, spatial roadmap |
| FACES_GAP_ANALYSIS.md | Code gaps, VAAM integration, execution plan | 18-item feature list (FACES-1-18), 10-workflow execution plan (W1-W10), VAAM→FACES bridge |
| FACES_VALIDATION_FRAMEWORK.md | How to measure correctness | Metrics, labeling guide structure, ablation studies, IAA study, cost analysis |
| **This document** | **Physical AI intent, ML engineering, management** | **Protocol-first positioning, 3-layer architecture, FACES-Embed spec, risk register, 3-gate process** |

**Cross-references:**
- Gap analysis FACES-15/16 → Decisions documented here (§3.2) and in validation framework (§4.3)
- Gap analysis Part 4 workflows → Process defined here (§5.6)
- Validation framework §7 priority order → Aligned with gap analysis Part 4 workflows
- All docs → Protocol-first positioning defined here (§0)

The four documents together answer: **What is FACES?** (pivot), **What's missing?** (gap analysis), **How do we know it works?** (validation), and **What is it for and how do we build it?** (this document).

---

## 9. The One-Sentence Summary for Engineering Management

**FACES is the TCP/IP of emotive AI — a 4-byte deterministic emotional state protocol that any system can use, from terminals to XR to robots — and the engineering priority is to harden the protocol, build an emulator for testing, and establish ground truth through a labeling guide before writing any neural model code.**
