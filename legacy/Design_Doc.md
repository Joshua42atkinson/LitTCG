# Communication Class — Master Design Document

> **Communication Class** is a pet-based gamified educational tool for English.
>
> Everything else is negotiable. This document is the single source of truth.

---

## Document Status

| Field | Value |
|-------|-------|
| **Project Name** | Communication Class |
| **Engine** | Bevy 0.18.1 ECS |
| **Target** | Desktop → Web (WASM) → Android XR |
| **Audience** | Homeschool market, ages 6-16 |
| **Status** | Alpha — Engine Complete, Building Product Surface |
| **Date** | July 2026 |

---

## Table of Contents

1. [Core Statement](#1-core-statement)
2. [What We Already Have (Engine Status)](#2-what-we-already-have-engine-status)
3. [What We Cut](#3-what-we-cut)
4. [System Architecture](#4-system-architecture)
5. [Pet System](#5-pet-system)
6. [FACES Integration](#6-faces-integration)
7. [Game Loop](#7-game-loop)
8. [Curriculum & Data](#8-curriculum--data)
9. [Input Systems](#9-input-systems)
10. [Save System](#10-save-system)
11. [Build & Deployment](#11-build--deployment)
12. [English Skills Covered](#12-english-skills-covered)
13. [Implementation Phases](#13-implementation-phases)
14. [Open Questions](#14-open-questions)
15. [Monetization Model](#15-monetization-model)
16. [Demo Scope](#16-demo-scope)
17. [Shipping Checklist](#17-shipping-checklist)

---

## 1. Core Statement

Communication Class is a pet-based gamified educational tool for English.

The child does not learn what a word means. The child builds a pet from a word and watches it come alive. The learning is the playing. The playing is the learning.

One game. One pet system. One save file. Ship to Web and Desktop.

---

## 2. What We Already Have (Engine Status)

The engine has been successfully ported into a unified workspace under `/communication-class/`. 

### 2.1 The Core Engine
- **14 source files** compile cleanly with a passing integration test suite (6/6 pass).
- **5 embedded JSON databases** (~3.3MB) handle curriculum data (Etymology, Lore, Quests, Synonyms, Words).
- **Full Game Loop**: Loading → Collecting → Constructing → Playing → Questing → Battling → Reviewing.
- **Pet Spawning Pipeline**: Translates words into `FacesState`, mapping grammatical rules to 3D procedural meshes (wings, orbital rings, glTF fallback).
- **Quest System**: Mad-Lib templates filling slots with pets, granting XP and evolution points.
- **Battle System**: Synonym/antonym matching for turn-based combat.
- **Chat System**: Integration with Kokoro TTS sidecar and FACES-driven pet dialogues.

### 2.2 Reused Components
- **FACES Protocol**: Fully integrated from `faces-protocol`. Maps grammatical structure to the 4-byte emotional protocol (Aura/Container/Focus/Action).
- **Voix Vive / trinity-daydream concepts**: Hand tracking and XR spatial UI shells are scaffolded but inactive for the desktop/WASM targets.

---

## 3. What We Cut

| System | Why | Replace With |
|--------|-----|-------------|
| Three separate games (Trivium) | Can't ship 3 games | One game, pet type evolves with difficulty |
| Symbol / ARCANA | Redundant with FACES | FACES Container/Focus/Action |
| SynergyLinks / Wǔ Xíng | Over-engineered, not fun | Simple synonym/antonym matching |
| Sled Vector DB | Embedded vector DB, way too heavy | JSON save file |
| Phase (ADDIECRAPEYE) | 12-phase progression | Day_Dream GameState (6 states) |
| Autopoietic code mutation | Self-modifying code sandbox | Cut entirely |
| DAG Curriculum graph | Over-engineered word graph | Simple word list with grade levels |

---

## 4. System Architecture

```
Communication Class (single Bevy app)
│
├── main.rs & components.rs
│   ├── GameState machine: Loading → Playing → Questing → Battling → Reviewing
│   ├── ECS Components: SpellBook, CharacterSheet, FacesState, Channel, etc.
│
├── database.rs & assets/
│   ├── Embedded JSON schema loader for quests, synonyms, etymology
│
├── render.rs
│   ├── AAA+ procedural pet rendering, FACES morphs, particle effects
│
├── letter.rs
│   ├── Spatial letter collection and word constructor UI
│
├── quest.rs
│   ├── Mad-Lib quest engine and CurriculumManager
│
├── battle.rs
│   ├── Turn-based synonym/antonym card combat system
│
├── chat.rs
│   ├── FACES pet dialogue and Kokoro TTS integration
│
├── save.rs
│   ├── JSON save/load persistence
│
└── input.rs / hand_tracking.rs / spatial_ui.rs
    ├── Touch, mouse, keyboard, and placeholder XR hand tracking inputs
```

---

## 5. Pet System

### 5.1 Pet Anatomy

Every pet is a Bevy ECS entity with:

- **FacesState** — 4-byte emotive state driving face mesh + material color
- **WordCard** — the word this pet was built from
- **Channel** — Mind/Heart/Body/Action
- **Stage** — Hero/Outlaw/EdgeLord/BestSelf (mastery tier)
- **PetStats** — Logos (attack), Pathos (health), Ethos (defense), Speed (intellect)
- **Element** — Fire/Water/Earth/Air/Shadow/Light/Normal (from etymology root)
- **Role** — Tank/Striker/Support/Caster/Healer/etc (from suffix)
- **Transform** — 3D position

### 5.2 Pet Creation Pipeline

```
Player collects letters
    ↓
Player arranges letters into word (word constructor UI)
    ↓
Validate word against WordDatabase
    ↓
Analyze word: split root + suffix (EtymologyDB)
    ↓
Determine element from root & role from suffix
    ↓
Calculate stats from psycholinguistic data (Concreteness, etc)
    ↓
Generate FacesState from word properties
    ↓
Spawn 3D pet entity with all components
```

---

## 6. FACES Integration

### 6.1 FACES = Parts of Speech

The FACES protocol maps to English grammar:

| FACES Byte | Grammar Role | What It Represents |
|-----------|-------------|-------------------|
| Aura (256) | Adjective | Mood, atmosphere, qualitative state |
| Container (5) | Noun | Entity, boundary, physical identity |
| Focus (6) | Adverb | How the action is performed, intensity |
| Action (5) | Verb | Kinetic output, communicative intent |

When a child builds a pet from a word, the word's grammatical properties directly determine the pet's face.

---

## 7. Game Loop

```
COLLECT letters (explore 3D space, pick up letter crystals)
    ↓
BUILD words (arrange letters in word constructor, validate)
    ↓
SPAWN pets (word → etymology → element/role → 3D pet with FACES)
    ↓
QUEST (Mad-Lib: fill noun/verb/adj slots with your pets)
    ↓
BATTLE (synonym/antonym matching — your pet vs wild pet)
    ↓
REWARDS (XP, evolution, new words unlocked, mastery upgrades)
    ↓
SAVE (local JSON, COPPA compliant)
    ↓
REPEAT (spiral curriculum — same words at higher difficulty)
```

---

## 8. Curriculum & Data

Data has been successfully ported into embedded JSON databases.

- **EtymologyDB**: 24 roots, 30+ suffixes
- **WordDatabase**: Words with psycholinguistic metrics
- **SynonymDatabase**: Synonym/antonym relationships
- **QuestData**: 60+ Mad-Lib quest templates
- **LoreDB**: 12 NPCs

---

## 9. Input Systems

- **Touch / Mouse**: Swipe right (Yes), Swipe left (No), Swipe down (Dig Deeper), Tap (Select).
- **Keyboard**: Arrow keys/WASD (Swipe directions), 1-5 (Slot selection), Escape (Back).
- **Hand Tracking**: ASL stub built for XR mode.

---

## 10. Save System

- **Engine:** Built-in `serde_json` serialization to local disk. `moonshine-save` was cut.
- **Saves:** SpellBook, CharacterSheet, StudentTrail.
- **Constraints:** Local-first, no cloud, COPPA compliant.

---

## 11. Build & Deployment

- **Desktop**: Development and primary target.
- **Web (WASM)**: Primary demo distribution (itch.io). Built with `trunk build --release`.
- **Android XR**: Future target behind the `xr` feature flag.

---

## 12. English Skills Covered

| Skill | Mechanic | Source Code |
|-------|----------|------------|
| Spelling | Collect letters, arrange into words | `letter.rs` |
| Vocabulary | Each word becomes a collectible pet | `components.rs` |
| Etymology | Root analysis determines pet element/stats | `database.rs` |
| Parts of speech | FACES Container/Focus/Action = noun/adverb/verb | `faces-protocol` |
| Synonyms/antonyms | Battle mechanic — match word relationships | `battle.rs` |
| Sentence structure | Mad-Lib quests (fill noun/verb/adj slots) | `quest.rs` |

---

## 13. Implementation Phases

**Phases 1-8 (Engine Core)**: DONE. The 14 source files and 5 databases are integrated and compile with passing tests.

### Phase 9: Clean Up Code
- Fix compiler warnings, dead types, and unused fields.
- Ensure 100% stable `cargo check` and `cargo test`.
- Separate WASM-incompatible dependencies like `reqwest::blocking` via feature flags.

### Phase 10: Product Surface (UI & Flow)
- Build a persistent HUD (`hud.rs`).
- Build visual letter collection & spelling feedback.
- Build Main Menu (`menu.rs`).
- Build 3-step Tutorial (`tutorial.rs`).
- Add polish to pet renderer (screen shakes, particles).

### Phase 11: Web Demo (WASM)
- Verify `trunk serve` and build processes.
- Implement Paywall/Demo limitations (e.g. max 10 words).
- Prepare itch.io page and demo wrapper.

### Phase 12: Revenue Infrastructure
- Scaffold Parent Dashboard (a separate web app to view `save.json`).

---

## 14. Open Questions (Answered)

- **Pet mesh:** Procedural Bevy meshes + glTF fallbacks (Done).
- **Word database format:** Embedded JSON using `include_str!` (Done).
- **Quest system:** Kept NPC archetypes and implemented Mad-Lib system (Done).
- **Battle system:** Turn-based matching (Done).
- **Save system:** `serde_json` (Done).
- **TTS Engine:** Kokoro TTS sidecar, disabled for WASM target (Done).

---

## 15. Monetization Model

**Hybrid Freemium Strategy:**
1. **Free Web Demo**: Hosted on itch.io (WASM). Includes 10 starter words, 1 NPC quest chain, 1 battle. Used as top-of-funnel marketing for homeschool parents.
2. **Paid Desktop/Mobile Version**: $9.99 for full word database, all NPCs, all quests.
3. **Expansion Packs**: $4.99 themed word lists (e.g., SAT Prep, Science Vocab).
4. **Parent Dashboard**: Future $7.99/mo SaaS offering that provides analytics and reporting based on the child's save file.

---

## 16. Demo Scope

For the itch.io WASM release, the game is restricted to:
- 10 total words in the dictionary.
- 1 NPC Quest Chain.
- 1 Battle encounter.
- No local saving (progress resets).
- Shows "Get Full Version" prompt after significant play.

---

## 17. Shipping Checklist

- [ ] Zero compiler warnings.
- [ ] Passing integration tests.
- [ ] Main menu loads on launch.
- [ ] Tutorial plays for first-time users.
- [ ] Player can spell words and see visual feedback.
- [ ] HUD displays required information.
- [ ] WASM build runs in browser without crashing.
- [ ] Demo limitations apply correctly.
- [ ] Itch.io page copy and assets ready.
