# LitTCG — Game Design Document

> **LitTCG** (*Literary Trading Card Game*) is a game where words are spells and a living Slime is their vessel.
>
> The learner does not learn what a word means. The learner plays a word into the world and watches the Slime change shape, face, and force. The learning is the playing. The playing is the learning.
>
> This document is the single source of truth. Everything else is negotiable.

---

## Document Status

| Field | Value |
|-------|-------|
| **Project Name** | LitTCG: Word Slimes MVP |
| **Engine** | Bevy 0.18.1 ECS (Rust) |
| **Target** | Web (WASM) MVP → Desktop → Android XR |
| **Audience** | High school and adult learners; ages 13+ |
| **Status** | Phases 1–7 implemented; Phase 8 (stealth assessment / FACES connotation metrics) in design |
| **Date** | July 2026 |
| **Codebase** | 38 source files, 59 lib tests + 34 integration tests passing, 9,582 words in database |
| **Recent Milestone** | NPC scenario training (subject + scenario_text) and Slime/Card XP progression landed; GDD expanded with §19 Stealth Assessment and §20 B2B commercialization |

---

## 0. Executive Summary (Read This First)

**LitTCG** is a game where players wield real English words through a shape-shifting Semantic Slime.

**The 60-second loop:**
1. A player explores a world and finds letters or scans objects.
2. They spell a real word and press Submit.
3. The word becomes a card that lives inside the Slime.
4. The Slime's face and the word's meaning combine to produce an effect in battle or quest.
5. The player uses the word to help NPCs, defeat typos, and learn grammar, synonyms, and emotional tone.

**Why it is special:** The same word changes its effect depending on the emotional face the Slime wears when it is cast. The FACES protocol turns every word into a 32-bit emotional state, so the player learns that meaning is contextual — not just definitional.

**Who it is for:** High school students and adult learners first. Ages 13+ are the primary audience because the NPC scenario system, AI-driven Socratic tutoring, and pragmatic FACES grading require adolescent-level reasoning and emotional maturity. A simplified mode for younger players may follow.

**How it is measured:** Every cast and quest completion silently generates psychometric evidence. The engine computes HD-D lexical diversity, MTLD, syntactic-complexity ratios, and CCSS ELA standard coverage, storing everything in a local `save.json` file. Educators can import this into a disconnected Institutional Dashboard to generate IEP-ready, standards-aligned reports.

**How we make money:** Free demo → $9.99 one-time full game → $4.99 expansion packs → optional $7.99/mo learner dashboard → B2B institutional site-license dashboard for districts and adult-education programs.

**Current status:** The engine is complete. The next 30 days focus on a playable 2D demo: the core combat loop, three NPC scenario quests, and a polished web build.

### 0.1 MVP Scope (2D Demo)

The 2D demo validates the core loop without 3D or XR:

| In Scope | Out of Scope |
|---|---|
| 20-word micro-deck (nouns, verbs, adjectives) | 3D pet rendering |
| 4 FACES presets (Fierce, Joyful, Calm, Angry) | Full 32-bit FACES register |
| 1v1 Thesaurus Dance battle | 3-card sentence plots |
| 3 NPC scenario quests | All 12 districts |
| Syntax + Pragmatics grading | Full 3-axis grading |
| Web (WASM) build | Desktop/XR builds |

### 0.2 Current vs. Target

| System | Code Status | GDD Target |
|---|---|---|
| Word cards with psycholinguistic stats | ✅ Implemented | ✅ Matches |
| Semantic Slime as deck/vessel | ✅ Implemented | ✅ Matches |
| 4-preset SlimeFace | ✅ Implemented | Target: full `FacesState` |
| Synonym/antonym battle math | ✅ Implemented | Target: + FACES resonance |
| Mad-Lib quest slots | ✅ Implemented | Target: + environmental FACES |
| 12 NPCs with dialogue | ✅ Implemented | Target: scenario training subjects |
| Three-axis grading | ✅ Implemented | Matches §8.3 |
| NPC scenario training | ✅ Implemented | Matches §10.4 |
| FACES connotation metrics / stealth assessment | ❌ Not implemented | Designed in §19 |
| CCSS ELA metadata alignment | ❌ Not implemented | Designed in §19.2 |
| Institutional telemetry dashboard | ❌ Not implemented | Designed in §19.3 |
| Crawl/Walk/Run AI observer | ❌ Not implemented | Designed in §17.6 |

---

### 0.3 The 2D Gray-Box Vertical Slice — "Pokémon Red for Words"

#### Why 2D First

The 2D build is not a downgrade of the XR vision. It is a **cheap, fast prototype of the same loop**. If walking around a 2D world, scanning objects, spelling words, and battling typos is not fun, then pass-through AR and ASL will not save it. The 2D slice answers one question: *is the core gameplay loop fun?*

Once the loop is fun in 2D, we port it to XR by replacing keyboard/mouse with hand tracking and colored squares with holograms. The game logic stays identical.

#### The 2D World

A single top-down explorable map built from colored rectangles and simple sprites. No 3D rendering, no tilemap crate, no external dependencies.

**What's in the world:**
- **Player Avatar** — a small controllable sprite (WASD / click-to-move).
- **Semantic Slime Companion** — follows the avatar at a short distance; shows the active FACES emotion; IS the player's deck/grimoire.
- **NPC Mentors** — the 12 Jungian archetypes standing in themed zones. Walk up and press `E` to talk. They give quests, teach etymology, and route the player into the Tutor Loop on defeat.
- **Scannable Objects** — rocks, trees, doors, signs, rivers. Walk up and press `E` to "scan" them. This is the 2D equivalent of the XR pinch-to-capture flow. Scanning yields a word card.
- **Wild Typos** — corrupted word creatures roaming the map. Touch one → transition to the Thesaurus Dance battle.
- **Districts** — 2-3 themed zones (Garden, Shadow Library, Irony Junction) to test the FACES/setting system.

**2D ↔ XR Mapping:**

| XR Action | 2D Equivalent |
|---|---|
| Look at real-world object | Walk avatar near object |
| Pinch to capture | Press `E` |
| ASL fingerspell word | Type the word or tap letters |
| Play a card | Click hand card |
| Change Slime face | Click face button |

#### The Core 2D Loop

```
Explore world
    → Scan object → harvest word card
    → Talk to NPC → get quest
    → Touch wild typo → enter battle

Spell word (Constructing)
    → Word becomes a pet card
    → Pet card is added to the Semantic Slime

Use pet cards
    → In battle: Thesaurus Dance
    → In quest: fill grammar slots
    → In bonding: feed, pet, attune

Progress
    → XP, mastery, evolution, new districts
    → Defeat → Tutor Loop with matching NPC
```

#### The Thesaurus Dance Battle (2D Combat)

Combat is a 1v1 vocabulary duel. The enemy is a Wild Typo with a word. The player builds a 1-3 card **sentence** (a "plot") and casts it.

**Sentence structure:**

```
[Adjective] + [Noun] + [Verb]
```

Example:
> **Searing Sword Strikes**

Each card contributes:
- **Adjective** — element/damage-type multiplier
- **Noun** — summon/target base effect
- **Verb** — action type (attack, defend, heal, burn, freeze)

**FACES Emotional Stance:**

Before casting, the player selects the Slime's emotional face. The face modifies the sentence's effect:

| Face | Effect |
|---|---|
| **Fierce** | +20% damage; fire verbs become blasts |
| **Joyful** | Heals player slightly; heals become group heals |
| **Calm** | No recoil from hyperbole; +block |
| **Angry** | +30% damage but take recoil |

FACES is a real choice independent of cards. The same three cards produce different results depending on the chosen emotion.

#### Literary Devices as Plot Mechanics

The sentence the player builds can trigger literary-device "metamagic." These are the combo system of the card game.

| Device | Trigger | Combat Effect |
|---|---|---|
| **Alliteration** | 2+ cards start with same letter | Echo Cast — repeat the last card's effect |
| **Oxymoron** | Adjective and noun are antonyms | Armor Piercing — ignore enemy resistance |
| **Hyperbole** | Any card has high intensity | Overcharge — 3× damage, self-damage recoil |
| **Palindrome** | Any card is a palindrome | Reflect — return part of next enemy attack |
| **Personification** | Noun + animate verb | Summon a temporary companion/tank |
| **Onomatopoeia** | Verb is a sound word | Stun — enemy skips next turn |
| **Metaphor** | Two nouns in the sentence | Transform damage into the second noun's element |

The player is not just picking a card — they are building a sentence with synergies.

#### Enemy Design

Each Wild Typo has:
- A **word** (e.g., "fire")
- A **part of speech weakness** (e.g., "weak to antonyms / verbs")
- An **element** (from etymology root)
- A **role** (from suffix)

The player reads the weakness, picks cards that counter it, and chooses a FACES emotion that amplifies the counter.

#### Quests in 2D

NPCs give **AR Bounties** reinterpreted for 2D:

> *"My Slime is hot. Find something cold in this district."*
> 
> The player walks to the ice cave sprite, scans it ("ice"), spells I-C-E, and returns with the word.

Mad-Lib quests remain:

> *"The {ADJECTIVE} {NOUN} {VERB} loudly."*
> 
> The player plays cards that match the grammar slots.

#### Success Criteria for the 2D Slice

- Player can explore a small map.
- Player can scan objects and spell words to get cards.
- Player can talk to NPCs and receive quests.
- Player can battle Wild Typos using sentence crafting + FACES + literary devices.
- The combat log explains why damage happened.
- Losing routes the player to a Tutor Loop NPC.
- The 1-minute loop is fun enough to replay.

Once this slice is fun, we port the exact same systems to XR: the avatar becomes the player, the scan button becomes a pinch, and the 2D sprites become holograms.

---

## 1. Core Vision

LitTCG is a word-spelling game where **words are cards** and a shape-shifting **Semantic Slime** is the companion that carries them all.

A player spells a word. The word validates against a database of 9,582 English words. The word's etymology determines its element (Fire, Water, Earth, Air, Shadow, Light). Its suffix determines its role (Tank, Striker, Caster, Healer, etc.). Its psycholinguistic profile — real research data on concreteness, valence, arousal, and dominance — determines its combat stats. Its dictionary definition drives a 4-byte emotional state called a FACES register. In the 2D demo, this emotional state changes the Slime's face and the word's effect. In the 3D/XR build, the Slime morphs into a creature colored by the element and shaped by the meaning.

The same word is never the same card twice, because the Slime's face changes the context. "Inferno" cast with a Fierce face is a fireball. "Inferno" cast with a Joyful face is a warming campfire. This is the game's core intellectual property — no other game treats words as a physics engine where emotional context changes the output.

### The Vessel and the Payload

At the deepest level, LitTCG separates **identity** from **ammunition**:

- **The Vessel:** The Semantic Slime — the player's persistent companion and deck. It has its own emotional state, its own FACES register, and its own "mood of the moment." It is the one creature the player truly bonds with.
- **The Payload:** The word cards inside the Slime. Each card carries a hard meaning (definition, POS, etymology, VAAM stats) and a soft meaning (intrinsic FACES emotional profile from its definition).
- **The Resonance:** When a word is played, the outcome depends on how the card's intrinsic FACES aligns with the Slime's current contextual FACES and the situation's emotional requirement. The same word can be a weapon, a shield, a heal, or a trap depending on the face the Slime wears when it is cast.

This is **semantic resonance**: the study of how meaning changes when the same word is used in different emotional contexts. The learner learns that language is not a dictionary — it is a physics engine where the same word has different effects depending on the face you put behind it.

> *In simple terms: Players spell words, and each word becomes a card inside the Slime. The Slime's face changes the card's effect. "Fire" with a Fierce face is an attack. "Fire" with a Joyful face is healing. "Calm" with a Calm face is a shield. The same word can be a weapon, a shield, or a heal — depending on the face.*

### Design Principles

- **Isomorphism** — The game mechanic IS the skill being taught. Spelling IS summoning. Synonyms IS combat. Grammar IS questing.
- **Active Imagination** — Words are not text on a page. They are living creatures with personalities. The player's imagination is the primary interface.
- **Stealth Assessment** — The game tracks what words the player uses, how they use them, and what patterns emerge. No tests. No quizzes. The play IS the assessment.
- **Local-First** — No cloud. No tracking. No accounts. Save files live on the learner's device. Privacy-first by design.

### The Tao of Fun

Our working design lens from `docs/TAO_OF_FUN_REVIEW.md`:

1. **Presence before points.** A world exists — a floating companion, a talking NPC, a changing sky, a shifting soundtrack — before any score is shown.
2. **Personality before procedure.** Every NPC, pet, and typo has a voice, a face, and a preference. The game does not explain; the world reacts.
3. **Permission before punishment.** Mistakes are discoveries. A misspelled word becomes a mutant, a lost battle becomes a tutor visit, a wrong root becomes a hint.
4. **Play before pedagogy.** The challenge can be quantitative and Common-Core aligned, but the exercise must feel like play — Montessori self-direction plus Steiner head/heart/hands balance.

---

## 2. The Board Game (Visual Analogy)

*Imagine the game as a board game on a kitchen table. This is the mental model for understanding all systems.*

### What's on the Table

**The player's side:**
- A **letter tray** — Scrabble tiles they've collected (A, B, C, etc.)
- A **spell pad** — where they arrange letters into a word and press Submit
- A **word card** — face-down, like a spell card. When flipped, it reveals a word with an element, role, and FACES expression. The card is a word the player has collected.
- A **hand** — 3-6 word cards face-up, the active words the Slime can cast
- A **Grimoire** — all their collected word cards, browsable, sortable, living inside the Slime
- A **companion** — one shape-shifting Slime standing next to them. It IS the deck; the word cards live inside it. The Slime's face changes the mood of every card it casts.

**The board:**
- **12 districts** — themed zones with different NPCs, letter fields, and difficulty
- **Quest board** — NPC request cards with color-coded slots to fill
- **Battle arena** — where corrupted words (Typos) appear as enemy pets
- **Nuisance zone** — roaming letters that chase the player

### One Round of Play

1. **Explore** — Move through a district. SemanticSlime companion follows. Collect letter crystals (curriculum-biased to grade level).
2. **Construct** — Arrange letters into a word. Press Submit.
3. **The Card Flip Moment** — A word card appears face-down. The player flips it. The card reveals a word with element colors, a FACES expression, rarity tier, and stats — and the Slime briefly shifts to show it. "Inferno" is a Rare Fire word with a fierce face. The player goes "WHOAAAA."
4. **Bond** — New word card joins the Grimoire inside the Slime. The player can pet the Slime (FACES → Happy), feed it related words, attune it (channel alignment), or set a favorite word as the active companion expression.
5. **Battle** — A wild Typo appears as a corrupted word. The player uses their SemanticSlime. First, they choose the Slime's emotional face (Fierce, Joyful, Calm, Angry). Then they play a word card. The same card does different things depending on the face. To counter, they play an antonym (high semantic distance). To attack heavily, they play a synonym (low semantic distance).
6. **Quest** — An NPC gives a Mad-Lib with color-coded grammar slots. The player plays a word card into the matching slot. SemanticSlime bonus XP applies.
7. **Reward** — XP, mastery points, new letter crystals. The word card gains mastery. At mastery thresholds, the word evolves — new visual decorations, stat boost, golden aura.
8. **Tutor Loop** — If defeated, the game routes to an appropriate NPC for targeted practice on the failed word concept. No "Game Over" screens.

> *In simple terms: Collect letters → spell a word → get a card inside the Slime → use the card in battles and quests → earn rewards → do it again with harder words. Like Pokémon, but the Slime is your companion and the cards are your creatures.*

---

## 3. VAAM — Vocabulary Acquisition Autonomous Meaning

VAAM is the design philosophy that words are not memorized — they are **experienced**. The learner does not look up a definition. The learner builds the word, watches it become a card inside the Slime, uses it in combat, feeds it to the Slime, fills quest slots with it, and watches it evolve. Meaning is acquired through the journey, not the definition.

### How VAAM Works in the Game

Each word in the database carries **psycholinguistic metadata** — real research data from academic linguistics studies:

| Metric | What It Measures | Game Effect |
|--------|-----------------|-------------|
| **Concreteness** (C) | How physical/tangible the word is (1-5) | → Attack power (Logos). "Rock" hits hard. "Freedom" hits soft. |
| **Valence** (V) | How positive/negative the word feels (1-9) | → Health/survivability (Pathos). "Joy" has high HP. "Despair" has low HP. |
| **Intensity** (A) | How exciting/calm the word is (1-9) | → Speed (turn order). "Rage" is fast. "Sleep" is slow. |
| **Dominance** (D) | How much control/power the word implies (1-9) | → Defense (Ethos). "King" defends well. "Whisper" defends poorly. |
| **Age of Acquisition** (AoA) | When learners typically acquire this word | → Grade level / curriculum placement. "Cat" is early. "Ephemeral" is graduate. |

### The VAAM Pipeline (Implemented in Code)

```
The player spells "thunder"
    → Validated against 9,582-word database
    → Psycholinguistic data loaded: C=3.2, V=5.8, A=7.2, D=5.1
    → Etymology root "ton" (sound) → Element: Air
    → Suffix "-er" → Role: Bruiser
    → Stats: Logos=64, Pathos=58, Ethos=51, Speed=72
    → FACES detection on definition → angry eyes, open mouth, intense focus
    → Rarity: Uncommon (120 pts, 1.15x multiplier)
    → Word card reveals: Air-colored, fast, aggressive, storm particle effects
    → In 3D/XR, the Slime briefly morphs into an air-colored storm creature
```

The learner never sees the numbers. They see a fast, aggressive, storm-colored card and the Slime reacting with an angry face. They learn that "thunder" is powerful, loud, and energetic — because the word IS those things.

> *In simple terms: The game uses real brain science data about words. Physical words (like "rock") make strong cards. Happy words (like "joy") make tough cards. Exciting words (like "rage") make fast cards. The learner learns what words mean by feeling what the cards do when the Slime casts them.*

### VAAM as Deck-Building Guide

The learner's word collection IS their vocabulary. Building a good deck requires understanding words:

- **Need a fast attacker?** Collect high-intensity words (Rage, Thunder, Storm).
- **Need a tank?** Collect high-dominance words (King, Fortress, Mountain).
- **Need a healer?** Collect high-valence words (Joy, Serenity, Comfort).
- **Need a defender?** Collect high-concreteness words (Wall, Stone, Shield).

The learner learns word categories through gameplay, not instruction. They develop an intuitive understanding of psycholinguistics without ever hearing the term.

### VAAM as Connotation Tracker

VAAM does not stop at "did the learner know the word?" It also tracks **how** the learner used the word — the pragmatic/emotional layer of meaning. This is the core of the FACES resonance system.

| New Metric | What It Measures | Pedagogical Signal |
|---|---|---|
| **faces_congruence_total** | Count of casts where word FACES matched Slime FACES | Learner understands word connotation |
| **faces_incongruence_total** | Count of casts where word FACES clashed with Slime FACES | Learner is experimenting with tone (also valuable) |
| **contextual_flexibility** | Variance in FACES choices across similar words | Learner can adapt word use to different moods |
| **resonant_casts** | Casts with triple FACES alignment (word + slime + environment) | Learner is mastering pragmatics |
| **dissonant_casts** | Casts with no FACES alignment | Learner needs scaffolding on connotation |

These metrics feed the Tutor Loop. If a learner repeatedly plays high-valence words with a Fierce face, the game can route them to the Poet or the Caregiver NPC for a targeted lesson on tone and emotional register.

> *In simple terms: The game also tracks whether the learner used the word in the right emotional context. "Fire" can be a warm campfire or a burning attack. The game notices if the learner is picking the right mood for the right situation, and helps them practice when they are not.*

---

## 4. The Pet System

> In this document, "pet" means the **word card / creature expression** that lives inside the Semantic Slime. The pet is not a separate persistent companion — the Slime is the companion. A pet card is the payload the Slime carries and casts.

### 4.1 Pet Anatomy

Every pet is a Bevy ECS entity with these components:

| Component | What It Is | Source |
|-----------|-----------|--------|
| **PetAvatar** | Marks entity as a pet, stores word and class | `components.rs` |
| **PetFacesState** | 4-byte FACES emotional state | `faces-protocol` crate |
| **PetStats** | Logos (attack), Pathos (health), Ethos (defense), Speed | `components.rs` |
| **Element** | Fire/Water/Earth/Air/Shadow/Light/Normal | `components.rs` |
| **Role** | Tank/Bruiser/Striker/Assassin/Caster/Support/Buffer/Healer | `components.rs` |
| **SummonClass** | SemanticSlime / GrammarGolem / RhetoricRobot | `components.rs` |
| **PetVisualState** | Idle/Alert/Battle/Happy/Sleeping | `components.rs` |

### 4.2 Pet Creation Pipeline (Implemented in `letter.rs:307`)

```
Player collects letter crystals → LetterStash
    ↓
Player arranges letters → CurrentSpelling
    ↓
Press Enter / pinch Submit button
    ↓
submit_spelling_word() validates against GameDatabase.words (9,582 entries)
    ↓
If invalid → spawn Unstable Mutant (glitch entity, magenta)
    ↓
If valid:
    → Root analysis: scan 25 etymology roots → Element + stat focus
    → Suffix analysis: scan 27 suffixes → Role
    → PetStats: logos = concreteness × 20, pathos = valence × 10,
                ethos = dominance × 10, speed = intensity × 10
    → FACES: detect_scored() → 4-byte emotional state
    → Spawn 3D entity at (0, 1.5, -2.0) with all components
    → render.rs adds: head mesh, glow core, eyes, mouth, wings, ears,
                       orbital ring, 10 aura particles, 20 burst particles
    → Transition to GameState::Playing
```

### 4.3 Pet Card → Pet Reveal (The Pokéball Moment)

**Status: Not yet implemented. #1 feature to build.**

When a word validates, instead of immediately spawning the 3D pet:

1. Spawn a **PetCard** entity — flat card, face-down, glowing border
2. The player clicks/taps/pinches the card
3. Card flips with animation (0.5s rotation)
4. Pet bursts out — 3D mesh, burst particles, FACES expression
5. Card now shows pet stats face-up (element, role, stats, rarity)
6. Pet added to PetCollection

The card is the pet's home. When battle ends, the word goes back in. The player always has the card. This gives both the collectible card AND the creature expression.

### 4.4 Pet Collection (The Grimoire / SpellBook)

**Status: Implemented as `SpellBook` in `components.rs`.**

The `SpellBook` resource inside the Slime stores all word cards the player has ever collected. Each `SpellBookEntry` stores: word, channel, mastery, times_encountered, element, role, stats, companion flag, and (target) intrinsic FACES state. The collection screen shows all word cards in a grid — sortable by element, role, rarity, mastery.

A future `PetCollection` may be added as a visual wrapper over `SpellBook` for the 3D/XR collection view, but `SpellBook` remains the canonical data source.

### 4.5 Roster (Battle Hand)

**Status: Implemented as `Deck` / `Hand` in `components.rs`.**

The Slime holds the player's active word cards in a `Hand` (typically 3-6 cards). This is the battle roster. Strategic choices:
- **RPS balance** — Don't rely only on one class (a Golem enemy will crush you)
- **Element diversity** — Different elements for different enemy weaknesses
- **Role coverage** — Need a tank, a striker, and a healer
- **Mastery level** — Mastered words are stronger, but maybe you want to level up a new one

A future "battle party" screen may be added as a visual wrapper over `Hand`, but `Hand` remains the canonical data source.

### 4.6 Pet Bonding (Partially in `chat.rs`)

- **[P] Pet the Slime** — FACES → Happy. The Slime smiles. Builds trust.
- **[F] Feed a Word** — The Slime "eats" related words. Synonyms nourish. Builds mastery of the fed word.
- **[T] Attune** — Aligns the Slime or a word card to a Channel (Mind/Heart/Body/Action).

### 4.7 Companion Follow System

**Status: Implemented in `src/core/companion.rs`.**

The persistent companion is the **Semantic Slime**, not a pet card. In 3D / XR modes the Slime spawns as a persistent `PetAvatar`/`SlimeEntity` and smoothly follows the camera, giving the world an emotional anchor. The `SpellBookEntry::companion` flag marks which word card the Slime is currently expressing (visual/auditory theme), but the Slime itself remains the companion. In `flat2d` mode the Slime is rendered as a 2D sprite to keep the gray-box clean. Future passes will add Slime reactions to pickups, NPCs, and battle outcomes.

### 4.8 Music & Somatic Soundtrack

**Status: Implemented in `src/core/music.rs`.**

The soundtrack is not a passive loop. It is a state-aware procedural layer:

- `scripts/generate_music.py` writes loop-safe WAV stems from integer harmonic stacks.
- Three stems exist: `music_menu.wav` (calm), `music_world.wav` (explore), `music_battle.wav` (tense).
- `MusicPlugin` crossfades between them as `GameState` changes and respects `GameSettings.music_volume`.
- Future passes will tie drone pitch to the companion word, add spatial audio around the companion/altar, and add reveal flourishes per element.

This is the first step toward a VoixVive-style audio-first pedagogy: every sound teaches the ear.

### 4.9 Pet Dream Layer

**Status: Not yet implemented. Designed in Roblox version.**

When a pet reaches Mastered, it gains the Dream Layer — idle state where the pet emits pseudo-poetry from its etymology. "Inferno" might whisper "from fire I rise, from ash I fall..." Cosmetic, collectible, makes pets feel alive when not in use.

> *In simple terms: Every word becomes a pet card that lives inside the Slime. You flip the card, the word's creature appears for a moment. The Slime carries all your cards. You pick 3-6 to bring to battle. You can pet the Slime, feed it words, and it follows you around. When a word is fully mastered, it dreams and whispers poetry inside the Slime.*

### 4.10 Semantic Resonance: The Card, the Slime, and the Context

Each pet card has two layers of identity:

1. **Hard Identity:** The word itself — its definition, POS, etymology, element, role, stats, and rarity. This is static. "Inferno" is always a fire word with high intensity and low concreteness.
2. **Soft Identity:** The word's intrinsic FACES state — the emotional face it wears when spawned from its definition. This is also static for a given word, but it varies wildly across words.

When a card is played, the **Slime** provides the third layer:

3. **Contextual Identity:** The Slime's current FACES state and the situation's environmental FACES requirement.

The pet card is the **payload**. The Slime is the **vessel**. The situation is the **target**. The game grades how well all three align.

| Scenario | Card (Intrinsic FACES) | Slime (Contextual FACES) | Environment (Required FACES) | Result |
|---|---|---|---|---|
| "Inferno" + Fierce vs. Fire Typo | Fierce/Sharp/Intense/Open | Fierce/Sharp/Intense/Open | Fierce/Sharp | Resonant cast: 3.0x damage, full VFX |
| "Inferno" + Calm vs. Fire Typo | Fierce/Sharp/Intense/Open | Calm/Fluid/Neutral/Withheld | Fierce/Sharp | Dissonant cast: 0.4x damage, muted VFX, combat log explains the clash |
| "Serenity" + Calm vs. Fire Typo | Calm/Fluid/Neutral/Withheld | Calm/Fluid/Neutral/Withheld | Calm/Fluid (e.g., Caregiver NPC) | Resonant defensive cast: strong shield or heal |
| "Serenity" + Fierce vs. Fire Typo | Calm/Fluid/Neutral/Withheld | Fierce/Sharp/Intense/Open | Fierce/Sharp | Oxymoron-like dissonance: unusual effect, possible recoil, teaches contrast |

This is why the same pet card can be a weapon, a shield, a blessing, or a trap. The card does not change. The Slime's face changes. The learner's understanding of context changes.

> *In simple terms: Your pet card is like a sword. The Slime is the person swinging it. A calm person swings the same sword differently than an angry person. And the situation matters too — you don't swing a sword the same way at a campfire and a dragon. The game teaches you that the same word can do different things depending on the face you use it with.*

---

## 5. FACES Protocol — How Words and the Slime Get Their Faces

The FACES protocol maps English grammar to visual appearance. It produces **38,400 unique emotional states** using zero compute — just keyword detection on the word's dictionary definition.

### The Four Bytes

| Byte | Grammar Role | Range | What It Controls |
|------|-------------|-------|-----------------|
| **Aura** (256) | Adjective | Mood, atmosphere | Pet color (ANSI-256 spectrum), emissive glow |
| **Container** (5) | Noun | Entity boundary | Head mesh: Neutral→IcoSphere, Rigid→Cuboid, Fluid→Torus, Defensive→Cylinder, Sharp→Cone |
| **Focus** (6) | Adverb | How action is performed | Eye shape: Intense→squinted, Open→wide, etc. |
| **Action** (5) | Verb | Kinetic output | Mouth shape: flat, open, curved smile, etc. |

### How It Works in Code

1. The player spells "inferno"
2. `faces_protocol::detect::detect_scored("inferno")` runs keyword detection
3. Definition contains "fire," "burn," "intense" → specific Aura/Container/Focus/Action
4. `render.rs:spawn_avatar_visuals()` reads FacesState:
   - `aura.index()` → `ansi_to_color()` → head color (deep red-orange)
   - `container` → mesh (Sharp → Cone)
   - `focus` → eye scale (Intense → squinted)
   - `action` → mouth (Aggressive → open)
5. Pet spawns with red-orange skin, cone head, angry eyes, open mouth

### FACES as Semantic Quantification

The same 4-byte FACES state is not just a face — it is a **portable emotional register** that the game can compare, combine, and grade. The byte layout is a 32-bit register:

```
[Aura: 8 bits] [Container: 8 bits] [Focus: 8 bits] [Action: 8 bits]
     256 values       5 values          6 values         5 values
```

Two FACES states can be subtracted to produce a **semantic resonance vector**. High alignment = the word's natural emotional profile matches how it is being used. High dissonance = the word is being forced into a context that fights its meaning.

There are three FACES layers in play at any moment:

| Layer | What It Represents | Source | Example |
|---|---|---|---|
| **Intrinsic FACES** | The word's natural emotional profile | `detect_scored(word)` on its definition | "fire" is Fierce/Sharp/Intense/Open |
| **Contextual FACES** | The Slime's current emotional stance | Player-selected face buttons, bond state, battle momentum | Player sets the Slime to Calm/Fluid/Open/Withheld |
| **Environmental FACES** | The mood required by the situation | NPC archetype, district theme, quest slot, enemy typo | The Poet expects Fluid/Open; the Grammarian expects Rigid/Neutral |

**Semantic Resonance** is the comparison of these three layers. When the player plays a word, the engine checks:

1. Does the word's **intrinsic** FACES match the **contextual** FACES (Slime mood)?
2. Does the word's **intrinsic** FACES match the **environmental** FACES (situation)?
3. Does the **contextual** FACES match the **environmental** FACES?

A triple match produces a **Resonant Cast** — bonus damage, bonus XP, cleaner visual flourish. A single match or no match produces a **Dissonant Cast** — reduced effect, possible recoil, or a "mutant" outcome that teaches the learner why the pairing felt wrong.

Because FACES is a 32-bit integer, this comparison is pure arithmetic: subtraction, masking, and small lookup tables. No neural network is required. The system runs at 60 FPS on a watch and scales up to a 384-dimensional embedding model on desktop when available.

### FACES + SemanticSlime

| Class | Material | Mesh |
|-------|----------|------|
| **SemanticSlime** | Metallic 0.8, rough 0.15 (glossy) | FACES container determines shape |

> *In simple terms: The game reads what the word means and builds a face. "Inferno" gets red skin, sharp cone head, angry squinted eyes, shouting mouth. "Serenity" gets soft blue skin, round head, calm wide eyes, gentle smile. 38,400 possible faces, all from word meanings.*

---

## 6. Rarity & Evolution

### 6.1 Rarity Tiers

**Status: Not yet implemented. Existed in Roblox SlimeFactory.**

| Rarity | Point Pool | Stat Multiplier | Visual | Example Words |
|--------|-----------|-----------------|--------|---------------|
| **Common** | 80 | 1.0x | Basic blob, muted color | cat, dog, run |
| **Uncommon** | 120 | 1.15x | Element color, clear face | thunder, garden |
| **Rare** | 180 | 1.35x | Decorations (spikes, droplets) | inferno, fortress |
| **Epic** | 260 | 1.6x | Particle effects, glowing eyes | ephemeral |
| **Legendary** | 380 | 2.0x | Full VFX, aura, wings | transcendence |
| **Mythic** | 550 | 2.5x | Golden aura, dream layer | antidisestablishmentarianism |

Rarity is calculated from word difficulty: high AoA, low concreteness, long words, rare roots → higher rarity. This incentivizes learning harder words.

### 6.2 Evolution (Mastery = Growth)

**Status: Mastery tracking exists. Visual evolution not yet implemented.**

| Mastery | Icon | How to Reach | Visual Change |
|---------|------|-------------|---------------|
| **Encountered** | 🔮 | Spell the word | Basic blob, neutral color |
| **Experienced** | ⚡ | Use in battle or quest | Element colors, FACES active |
| **Owned** | 🌟 | Critical hit or quest slot fill | Decorations added |
| **Mastered** | 👑 | Use across multiple contexts | Full flourish, golden aura, +10% stats, Dream Layer |

> *In simple terms: Easy words make common pets. Hard words make rare, powerful pets. "Cat" is common. "Ephemeral" is legendary. Every pet evolves as you use it — bigger, fancier, stronger. A mastered pet gets a golden aura and whispers poetry.*

---

## 7. The SemanticSlime — Sole Companion (MVP)

**Status: MVP Refactor — GrammarGolem and RhetoricRobot deprecated.**

For the Word Slimes MVP, we have simplified to a single companion class: **SemanticSlime**. This reduces complexity while maintaining the core learning loop.

### Why SemanticSlime Only

- **Simplified Onboarding** — One class means less to learn for new players (ages 13+)
- **Focused Mastery** — Players master one combat system deeply instead of three shallowly
- **Curriculum Alignment** — Semantic relationships (synonyms/antonyms) are foundational vocabulary skills
- **Technical Debt Reduction** — Removes RPS balance complexity, class-specific rendering, and fusion mechanics

### How SemanticSlime Plays

| Playstyle | Attack Type | Damage Logic |
|-----------|-------------|--------------|
| **Wand Duel** | Semantic relationship | High distance = antonym/counter (block), Low distance = synonym/heavy attack, Mid-range = normal damage |

### Current Implementation in `battle.rs`

The combat system uses semantic distance for Wand Duel mechanics:

- **Counter/Block** (Distance > 4.0): Antonym logic. `1.5 + (distance - 4.0) × 0.2` multiplier. Effective against opposing concepts.
- **Heavy Attack** (Distance < 2.0): Synonym logic. `2.0x` multiplier. Overwhelms with similar concepts.
- **Normal** (2.0-4.0): `1.0x` damage. Standard attack.

### Grimoire — The Slime's Body of Knowledge

The `Grimoire` resource represents the SemanticSlime as the player's physical inventory/deck. Words collected become part of the Slime's knowledge base, stored as a `Vec<String>` with a `max_capacity` of 50 words. In the target architecture, the Grimoire is not an external backpack — it is the Slime itself. Every word the player collects becomes part of the Slime's body. When the Slime opens to cast a spell, it pulls the word out of its own substance.

### The Slime's Face is the Deck's Voice

The SemanticSlime carries a **contextual FACES state** that persists across battles and quests. This is not the face of any single word — it is the Slime's own emotional posture, the "set and setting" it brings to every cast.

In the MVP, the player chooses from four face presets that map to regions of the full FACES space:

| Preset | FACES Region | Default Effect on Words |
|---|---|---|
| **Fierce** | Sharp/Intense/Open | Boosts damage and fire verbs; turns nouns into weapons |
| **Joyful** | Fluid/Open/Curved | Boosts healing and group effects; turns nouns into blessings |
| **Calm** | Rigid/Neutral/Withheld | Cancels recoil; boosts blocking and water/earth verbs |
| **Angry** | Sharp/Intense/Aggressive | High damage but self-recoil; high-risk/high-reward |

The full FACES state is stored as a 32-bit register, so the Slime can drift smoothly between presets. Bonding, feeding, battle momentum, and the current district can all nudge the Slime's face over time. A player who battles mostly in the Outlaw Outpost will find their Slime leaning toward Sharp/Intense. A player who quests in the Garden District will find their Slime leaning toward Fluid/Open.

This is the **avatar layer** of the game: one persistent creature whose emotional identity changes how every word behaves.

---

## 8. Combat System

### 8.1 The Core Idea: Learning IS Combat

Combat is not separate from learning. The player demonstrates vocabulary knowledge **through** the combat mechanics. Every attack is a vocabulary exercise. Every defense is a grammar check.

### 8.2 Wand Duel Combat (Implemented in `battle.rs`)

When a Wild Typo appears, it carries the psycholinguistic coordinates of its word. The player must play a word based on semantic relationship to the Typo's word.

**Damage Formula:**

```
Distance = √((ΔC)² + (ΔV)² + (ΔD)² + (ΔA)²)
```

- **Counter/Block (Distance > 4.0):** Antonym logic. `1.5 + (distance - 4.0) × 0.2` multiplier. Blocks opposing concepts.
- **Heavy Attack (Distance < 2.0):** Synonym logic. `2.0x` multiplier. Overwhelms with similar concepts.
- **Normal (2.0-4.0):** `1.0x` damage. Standard attack.

Example: Typo is "fire" (C=4.5, V=5.0, A=7.0, D=5.0). The player plays "ice" (C=4.8, V=6.5, A=2.0, D=3.0). Distance ≈ 5.6. Counter/block!

### 8.3 Three-Axis Spell Grading (Syntax / Semantics / Pragmatics)

Every spell is graded on three independent axes. The final damage/effect is a product of the three scores, so a perfect cast requires grammatical correctness, semantic appropriateness, and emotional resonance all at once.

| Axis | Question | Source | How It Modifies the Cast |
|---|---|---|---|
| **Syntax** | Is the sentence structurally valid? | POS tags, grammar slots, literary-device triggers | Invalid structure = spell fails or mutates. Valid structure = baseline multiplier. Combo triggers (oxymoron, alliteration) add metamagic. |
| **Semantics** | Does the word's hard meaning fit the target? | Synonym/antonym distance, VAAM stats, element/role | Synonym → heavy attack. Antonym → counter/block. Element weakness → bonus. High Dominance → better defense. |
| **Pragmatics** | Is the word being used with the right emotional tone? | Intrinsic FACES of the word vs. Slime's contextual FACES vs. environmental FACES | Resonance → bonus damage/XP and clean VFX. Dissonance → reduced effect, recoil, or "mutant" VFX. |

**The final multiplier is the product of the three axes:**

```
Final Multiplier = Syntax_Score × Semantic_Score × Pragmatic_Score
```

- A grammatically perfect but semantically wrong and emotionally clashing cast is weak.
- A semantically perfect but emotionally mismatched cast is mediocre.
- A cast that is correct on all three axes is a **Resonant Cast** — critical hit, full particle flourish, mastery bonus.

**Example: "Searing Sword Strikes" against a fire-weak Typo, with the Slime set to Fierce.**

- Syntax: `[Adjective] + [Noun] + [Verb]` → valid structure. `Syntax_Score = 1.0`.
- Semantics: "searing" and "fire" are semantically close (low distance) → synonym/heavy. `Semantic_Score = 2.0`.
- Pragmatics: "searing" is intrinsically Fierce/Sharp/Intense; the Slime is Fierce; the Typo is a fire enemy that expects heat. Triple resonance. `Pragmatic_Score = 1.5`.
- Final: `1.0 × 2.0 × 1.5 = 3.0x` — a resonant critical.

**Example: "Gentle Sword Strikes" with the Slime set to Fierce against the same Typo.**

- Syntax: valid. `1.0`.
- Semantics: "sword" still fits, but "gentle" is semantically distant from "searing/fire" → normal/slightly reduced. `0.8`.
- Pragmatics: "gentle" is intrinsically Calm/Fluid; the Slime is Fierce; the Typo is fire. Triple dissonance. `0.5`.
- Final: `1.0 × 0.8 × 0.5 = 0.4x` — the spell works, but it feels wrong. The combat log says: *"Gentle clashes with your fierce mood. The Typo barely notices."*

This teaches the learner that a word is not just a definition — it is a tool whose effect depends on the emotional context of its use.

### 8.4 Battle Flow

```
start_battle() → random word at player's grade level
    → BattleSession { typo_health: 50, player_health: 100, failed_word: None }
    → Pet visual state → Battle
    → UI: "WILD TYPO: [WORD]" + HP bars
    ↓
Player plays a card → play_battle_card() calculates damage
    → Effective: typo_health -= damage, mastery upgrade
    → Counter: antonym blocks Typo, damage multiplier applies
    → Synonym: heavy attack, 2.0x damage
    → Critical: screen shake + 30 burst particles
    ↓
typo_health ≤ 0 → Victory! → Mastered → Reviewing
player_health ≤ 0 → Defeat → Tutor Loop (Questing with NPC routing)
```

### 8.5 Tutor Loop — No Game Over (Implemented)

**Status: MVP Refactor — Failure routing added.**

When player health reaches 0, instead of a "Game Over" screen, the game enters the **Tutor Loop**:

1. `BattleSession.failed_word` tracks the word that caused defeat
2. `quest::route_to_tutor_npc()` maps the failed word to an appropriate NPC based on etymology (element/role)
3. `battle::start_tutor_loop()` initiates a targeted Mad-Lib quest with that NPC
4. Player practices with grade-appropriate words related to the failed concept
5. On quest completion, player returns to exploration with restored confidence

This ensures continuous learning without punitive failure states.

### 8.6 Planned: Active Learning During Combat

**Status: Not yet implemented. Inspired by Prodigy Math.**

| Action | Challenge | Reward |
|--------|-----------|--------|
| **Attack** | Type a synonym of your pet's word | Damage lands |
| **Counter** | Type an antonym of the enemy's word | Block enemy attack |
| **Critical Hit** | Identify the etymology root | 2x damage + VFX |

> *In simple terms: Fighting is learning. To attack heavily, find a word that means the same as the enemy's word (synonym). To counter/block, find a word that means the opposite (antonym). Your SemanticSlime uses semantic relationships to battle.*

---

## 9. Quest System

### 9.1 Mad-Lib Engine (Implemented in `quest.rs`)

NPCs give the player Mad-Lib style quests — sentences with blank slots that must be filled with word cards. Each slot requires a specific part of speech.

**Flow:** `start_quest()` picks an NPC quest at the player's grade level → parses `{ADJECTIVE}`, `{NOUN}`, `{VERB}` slots → the player plays word cards via `fill_slot()` → `complete_quest()` reconstructs sentence, upgrades mastery, awards XP + evolution points, checks for grade-up.

### 9.2 Color-Coded Grammar (Planned — Inspired by Colourful Semantics)

**Status: Not yet implemented. Quest slots are currently text labels.**

| Color | Part of Speech | Example Slot | Example Word |
|-------|---------------|-------------|-------------|
| Orange | WHO (noun) | "{WHO} went to the store" | dragon |
| Yellow | WHAT DOING (verb) | "The dragon {WHAT_DOING} loudly" | roared |
| Green | WHAT (noun-object) | "The dragon ate {WHAT}" | treasure |
| Blue | WHERE (location) | "The dragon flew {WHERE}" | mountains |
| Purple | HOW (adverb) | "The dragon flew {HOW}" | gracefully |

The player must play a word card whose word matches the part of speech. "Inferno" can't go in a WHO slot — it's not a person. But "dragon" can. Grammar validation through play.

### 9.3 Quest Data (60+ templates in `quest_data.json`)

**12 Archetypes, 5 quests each (60 total) + 33 NPC chain quests = 93 total quests.**

Each NPC has a 3-quest chain with increasing difficulty, plus time-of-day dialogue (Dawn/Day/Dusk/Night).

### 9.4 SemanticSlime Quest Bonus (Implemented in `quest.rs:108`)

- **SemanticSlime** → +5 XP (word consumption bonus)

> *In simple terms: NPCs give you fill-in-the-blank sentences. You put your pet cards in the blanks. But the pet's word has to match — a noun pet goes in a noun slot, a verb pet goes in a verb slot. It's like Mad-Libs with your collected pets. Your SemanticSlime gets bonus XP for consuming words in quests.*

### 9.5 Environmental FACES and Pragmatic Quest Bonus (Target Design)

Beyond matching the part of speech, advanced quest slots carry an **emotional mood tag** drawn from the FACES protocol. The slot is not just `{ADJECTIVE}` — it is `{ADJECTIVE:FIERCE}` or `{VERB:CALM}` or `{NOUN:JOYFUL}`.

When the player fills the slot, the engine checks three FACES layers:

1. **Word FACES** — the intrinsic emotional profile of the played word.
2. **Slime FACES** — the player's currently chosen companion mood.
3. **Slot FACES** — the mood tag on the quest slot.

| Match | Bonus | Example |
|---|---|---|
| Triple alignment (word + slime + slot) | +10 XP, golden sentence highlight, NPC praise | "The **fierce** dragon **roared**" with Fierce Slime vs. the Grammarian's request |
| Double alignment (word + slot, but slime differs) | +5 XP, standard completion | "The fierce dragon roared" with Calm Slime — the word fits, but the Slime mood is off |
| Single alignment (only POS matches) | Baseline XP | "The calm dragon roared" in a Fierce slot — grammatically valid, pragmatically awkward |
| No alignment | Reduced XP, NPC gives a hint about tone | "The gentle dragon whispered" in a Fierce slot — the NPC says, *"That word feels too soft for this battle."* |

This turns every quest into a mini-lesson on **register** and **tone**. The player learns that "dragon" can be a fearsome monster or a gentle companion depending on the adjectives and verbs around it, and that the Slime's face is a tool for setting the register.

> *In simple terms: Some quest blanks have a mood too. If the NPC wants a fierce sentence, you need a fierce word and a fierce Slime face. If you use a gentle word, the NPC might say, "That feels too soft." You learn that words have emotional flavor, not just grammar.*

---

## 10. World & Lore

### 10.1 The 12 Districts (Implemented in `quest.rs:162`)

| # | District | Theme | Grade |
|---|----------|-------|-------|
| 1 | Garden District | Growth, nature, beginnings | 1 |
| 2 | Outlaw Outpost | Rebellion, rule-breaking | 2 |
| 3 | Shadow Library | Mystery, hidden knowledge | 3 |
| 4 | Great Railway | Journey, connection | 4 |
| 5 | Maintenance Bay | Repair, practical work | 5 |
| 6 | Irony Junction | Contradiction, humor | 6 |
| 7 | Adjective Valley | Description, color | 7 |
| 8 | Central Station | Hub, crossroads | 8 |
| 9 | Metaphor Mountains | Figurative language | 9 |
| 10 | Logic Labyrinth | Reasoning, structure | 10 |
| 11 | Semantic Sea | Meaning, depth | 11 |
| 12 | Mastery Monolith | Final mastery | 12 |

### 10.2 The 12 NPCs (Implemented in `lore_db.json`)

| NPC | Archetype | District | Morphology | Scenario Subject | Expected FACES |
|-----|-----------|----------|------------|------------------|----------------|
| Barnaby | The Innocent | Brainy Borough | -s, -ed | Simple past / everyday verbs | Open / Soft |
| Yorick | The Everyman | Heartwood Grove | struct-, -ment, -tion | Noun formation / work vocabulary | Rigid / Neutral |
| Kael | The Hero | Action Alley | -ing, -er | Active voice / inspiring verbs | Fierce / Direct |
| Martha | The Caregiver | Whisper Winds | -ful, -ly, -ness | Tone / empathy | Gentle / Caring |
| Gribble | The Explorer | Action Alley | -able, -ible | Possibility / prediction | Curious / Open |
| Nyx | The Rebel | Whisper Winds | un-, de-, anti-, -ify | Negation / challenging rules | Sharp / Rebellious |
| Vlad | The Lover | Heartwood Grove | phil-, amat-, path- | Emotional vocabulary | Warm / Intimate |
| Pygmalion | The Creator | Whisper Winds | struct-, form-, -ify | Formative / causative verbs | Focused / Crafting |
| Chesty | The Jester | Heartwood Grove | -ish, -esque, pseudo- | Comparison / humor | Playful / Mischievous |
| Ozymandias | The Sage | Brainy Borough | vis-, vid-, cogn-, -ology | Observation / knowledge | Observant / Still |
| Zafir | The Magician | Action Alley | trans-, meta-, hyper- | Transformation | Mystical / Fluid |
| Ignis | The Ruler | Brainy Borough | -cracy, -archy, reg- | Order / governance | Authoritative / Rigid |

In the 2D demo, each FACES profile maps to the nearest 4-preset face (Fierce, Joyful, Calm, Angry). The full game uses the 32-bit `FacesState` register.

### 10.3 Day/Night Cycle (Implemented in `time_cycle.rs`)

NPCs have four dialogue pools — Dawn, Day, Dusk, Night — making the world feel alive. Sky lighting changes dynamically via `update_sky_lighting()` in `render.rs`.

> *In simple terms: The game world has 12 areas to explore, each with a different character who teaches you new word parts. The characters say different things depending on the time of day. As you learn more, you unlock harder areas.*

### 10.4 NPC Scenario Training (Daydream Integration)

The 12 NPCs are not just etymology tutors. They are **scenario trainers** who present narrative problems that require the right word in the right emotional context. This is the Daydream design principle applied to the Jungian archetypes: each NPC asks a question from their worldview, and the player answers by playing a word card + choosing a Slime face.

> **Audience note:** The scenarios are written for adolescent and adult reasoning (ages 13+). Nyx asks about challenging unfair rules, Vlad asks about emotional intimacy, and Ignis asks about systems of power. These topics are appropriate for high schoolers but would be simplified or omitted for younger players.

#### The Scenario Structure

Every NPC scenario follows the same three-part prompt:

1. **Story** — a short narrative problem the NPC faces.
2. **Mechanic** — the grammar or word part the NPC needs.
3. **Reflection** — a Socratic question the NPC asks if the player's answer is wrong or mismatched in tone.

This is the Daydream Triple Sandwich (Heart/Mind/Body) rendered as gameplay:
- **Heart** = the NPC's emotional need and expected FACES.
- **Mind** = the grammar/syntax requirement (the word part).
- **Body** = the gameplay consequence (damage, quest progress, reward).

#### Example Scenarios

| NPC | Scenario | Word Needed | FACES Check |
|---|---|---|---|
| **Barnaby** (Innocent) | "I tried to tell everyone what happened, but my story is all jumbled. Can you give me a word for something that already happened?" | past-tense verb | soft / reassuring |
| **Kael** (Hero) | "The village is losing hope. I need a word that will make them stand up and fight." | inspiring verb | fierce / direct |
| **Martha** (Caregiver) | "My friend is crying and the shadows are close. What gentle word should I offer?" | -ful adjective or -ly adverb | gentle / caring |
| **Nyx** (Rebel) | "This rule says 'no running.' That is boring and unfair. Give me a word that breaks it." | un- / anti- prefix | sharp / rebellious |
| **Ignis** (Ruler) | "We need a system where the people choose. If one person rules alone, what do we call that?" | -cracy / -archy noun | authoritative / rigid |

#### Failure as Socratic Reflection

When a player answers incorrectly, the NPC does not say "wrong." The NPC asks a deeper question, just as the Daydream reflection engine does:

- **Syntax failure:** "That word is strong, but it is not the shape I need. Can you find a word that ends in `-ful`?"
- **Semantics failure:** "That word is close, but it means something else here. What word means 'can be eaten'?"
- **Pragmatics failure:** "That word is correct, but it feels too sharp for this moment. How would Martha say it?"

This turns every failure into a micro-lesson delivered by the archetype the player is trying to help.

#### Mechanical Integration

Each NPC scenario is stored as a quest template with three new fields:

```json
{
  "npc": "Barnaby",
  "subject": "simple-past",
  "expected_faces": { "aura": "soft", "container": "open", "focus": "gentle", "action": "calm" },
  "socratic_failure": "That word tells me what might happen, but I need what already happened. Can you find the past-tense shape?"
}
```

The `expected_faces` field is checked against the Slime's current face when the word is played. A close match gives a Pragmatics bonus; a mismatch triggers the Socratic failure response.

---

## 11. Letter Collection & Nuisance System

### 11.1 Letter Crystals (Implemented in `letter.rs:24`)

Floating, rotating blue cubes with letters A-Z. Max 5 at a time. Collected by walking close (desktop) or pinching (XR). Stored in `LetterStash`.

### 11.2 Spelling (Implemented in `letter.rs:114`)

The player types letters (keyboard) or pinches holographic blocks (XR). Letters must be in stash. Backspace returns letters. Enter submits.

### 11.3 Nuisance Letters (Planned — From Roblox Version)

**Status: Not yet implemented.**

Clingy letters roam the world and chase the player. If they catch you, they cling to your letter tray. This can help (you needed an X!) or annoy (you have 5 Q's). Shake them off by spelling quickly. Rare letters (Z, X, Q) are valuable nuisances.

### 11.4 Curriculum-Biased Spawning (Implemented in `letter.rs:31`)

**Status: MVP Refactor — Implemented.**

Letters spawn biased toward forming grade-appropriate words. The system:

1. Queries the database for words matching the player's current grade level
2. Builds a letter frequency map from those grade-appropriate words
3. Converts the frequency map to a weighted letter pool
4. Spawns letters randomly from the weighted pool (fallback to A-Z if no grade words available)

This prevents frustration from getting 5 Q's and no vowels, and ensures letter availability aligns with curriculum goals.

> *In simple terms: You walk around collecting glowing letter cubes. You use them to spell words. Sometimes wild letters chase you and stick to your tray — helpful or annoying. The game gives you letters that can actually make words at your grade level.*

---

## 12. Character Progression

### 12.1 Attunement Channels (Implemented in `components.rs:110`)

Four channels track the player's linguistic style:

| Channel | Color | Emergent Class | Word Type |
|---------|-------|---------------|-----------|
| **Mind** | Green | The Oracle | Logical, structural |
| **Heart** | Orange | The Bard | Emotional, social |
| **Body** | Blue | The Cultivator | Physical, concrete |
| **Action** | Gold | The Templar | Aggressive, dynamic |

Each word use bumps the corresponding attunement by 10% of remaining distance to 1.0 (asymptotic). When dominant channel exceeds 0.2, emergent class manifests.

### 12.2 XP & Grade Progression (Implemented in `quest.rs:184`)

- XP from quests and battles
- Grade = `(total_xp / 1000) + 1`
- Grade-up unlocks next district
- Grade levels filter battle words: K-2, 3-5, 6-8, 6-9, 9-10, 10-12, 11-12, Graduate

### 12.3 Word Distribution by Grade

| Grade | Words |
|-------|-------|
| K-2 | 1,797 |
| 3-5 | 2,892 |
| 6-9 | 2,632 |
| 10-12 | 1,518 |
| Graduate | 739 |
| **Total** | **9,582** |

> *In simple terms: The game watches what kind of words you like. Smart words make you "The Oracle." Action words make you "The Templar." It's a personality test from your vocabulary. You level up by earning XP, and each level unlocks new areas.*

---

## 13. Curriculum & Data

### 13.1 Five Embedded JSON Databases (~3.3MB)

All data embedded via `include_str!`, loaded in `database.rs:270`:

| Database | File | Size | Contents |
|----------|------|------|----------|
| **Words** | `word_database.json` | 1.4MB | 9,582 words with psycholinguistic stats |
| **Synonyms** | `synonym_database.json` | 2.1MB | 9,578 entries: synonyms, antonyms, distractors |
| **Etymology** | `etymology_db.json` | 14KB | 25 roots → elements, 27 suffixes → roles |
| **Quests** | `quest_data.json` | 24KB | 93 quest templates (60 archetype + 33 NPC chains) |
| **Lore** | `lore_db.json` | 17KB | 12 NPCs with dialogue and schedules |

### 13.2 Etymology Root → Element (25 roots)

| Root | Element | Example | Root | Element | Example |
|------|---------|---------|------|---------|---------|
| Ignis | Fire | ignite | Cryo | Water | cryogenics |
| Aqua | Water | aquatic | Astr | Light | astronomy |
| Terra | Earth | terrain | Psych | Shadow | psychic |
| Aer | Air | aerial | Phot | Light | photon |
| Umbra | Shadow | umbrella | Therm | Fire | thermal |
| Lux | Light | lucid | Geo | Earth | geology |
| Chron | Air | chronic | Hydr | Water | hydrate |
| Mort | Shadow | mortal | Helio | Light | heliocentric |
| Vita | Light | vital | Nyct | Shadow | nyctophobia |
| Sci | Light | science | Cred | Earth | credit |
| Dyna | Fire | dynamic | Bio | Water | biology |
| Dict | Air | dictate | Voc | Air | vocal |
| Lumina | Light | luminous | | | |

### 13.3 Suffix → Role (27 suffixes)

| Suffix | Role | Suffix | Role | Suffix | Role |
|--------|------|--------|------|--------|------|
| -tion | Tank | -ize | Striker | -less | Striker |
| -ity | Tank | -ate | Caster | -ful | Buffer |
| -ment | Bruiser | -fy | Assassin | -ic | Caster |
| -ness | Tank | -ship | Support | -ist | Caster |
| -ance | Bruiser | -ous | Support | -logy | Support |
| -ence | Bruiser | -ive | Healer | -phobia | Assassin |
| -er | Bruiser | -able | Healer | -cracy | Tank |
| -or | Tank | -ible | Buffer | | |
| -en | Striker | -y | Support | | |
| -ish | Assassin | -al | Buffer | | |

### 13.4 Spiral Curriculum

Same words return at increasing difficulty:
- Grade 1: "cat" → Common pet, simple battle
- Grade 3: "cat" as a Typo with higher stats → the player uses synonyms
- Grade 5: "cat" in a quest requiring adjectives ("the ___ cat")
- Grade 8: "feline" (related) appears as a Legendary pet

### 13.5 Hot Reloading

Bevy's `AssetServer` watches JSON files. Teachers can edit word lists and the game updates instantly without restart.

> *In simple terms: 9,582 words, each with brain science data. 25 word roots determine what element a pet is. 27 suffixes determine what role a pet has. Teachers can edit the text files and the game updates instantly. Words come back at higher difficulty as you level up.*

---

## 14. Input Systems

### 14.1 Desktop (Implemented)

- **Keyboard**: Letter keys (spelling), Enter (submit), Backspace (remove), 1-5 (slot select), Escape (back)
- **Mouse**: Click to collect, click to select cards, drag for swipe

### 14.2 Touch (Implemented)

- Swipe right = Yes, left = No, down = Dig Deeper, tap = Select
- `SWIPE_THRESHOLD` in `input.rs` prevents accidental micro-drags

### 14.3 XR Hand Tracking (Implemented behind `xr` feature)

- **Pinch-to-select**: Thumb-index distance < threshold → `PinchEvent`
- **ASL Fingerspelling**: Hand joint tracking for signing letters
- **Gesture intensity → Intensity**: Stronger gestures = higher-intensity pets
- **Holographic letter blocks**: Float in arc, pinch to select
- **Submit button**: Floating 3D button, pinch to submit

> *In simple terms: Play with keyboard/mouse, touch screen, or VR hands. In VR, you pinch floating letters to spell and pinch a button to submit.*

---

## 15. Save System

### 15.1 Implementation (`save.rs`)

- `serde_json` serialization to local disk (`save.json`)
- Auto-save on every `GameState::Playing` transition
- Disabled in demo mode

### 15.2 What's Saved

- **CharacterSheet**: Attunement scores, emergent class, XP, active summon class
- **SpellBook**: All collected words with mastery levels and encounter counts
- **StudentTrail**: Visited words, swipe history, current word

### 15.3 Constraints

- Local-first, no cloud, no accounts
- Privacy-first — no personally identifiable information
- Save file is human-readable JSON — learners can open in any text editor

> *In simple terms: The game saves automatically. No internet needed. No account. The save file is a text file you can open and read. It shows what words you have learned.*

---

## 16. Build & Deployment

### 16.1 Three Build Targets

| Target | Command | Purpose |
|--------|---------|---------|
| **Desktop** | `cargo run --features desktop` | Development & full version |
| **Web (WASM)** | `trunk serve` | itch.io demo distribution |
| **Android XR** | `cargo ndk -t aarch64-linux-android check --features xr` | Future VR target |

### 16.2 Feature Flags

- `desktop` — Orbit camera, HDR, Bloom, SSAO
- `xr` — OpenXR, hand tracking, spatial UI
- `flat2d` — 2D-only rendering (lighter for WASM)
- `tts` — Kokoro TTS sidecar (disabled in WASM)

### 16.3 Cross-Platform Architecture

All game logic is platform-agnostic. Only rendering and input are feature-flagged. Both desktop and XR paths call the same `submit_spelling_word()` function.

> *In simple terms: The game runs on computers, web browsers, and VR headsets. Same game logic on all three. Only controls and graphics change. Web is the free demo. Desktop is the full game. VR is future.*

---

## 17. Pedagogical Foundations

### 17.1 Spelling as Casting

LitTCG takes the word "Spelling" literally. The learner is not memorizing vocabulary — they are casting spells. Each letter is a component. Each word is an incantation. The word card that appears is the spell made manifest inside the Slime.

### 17.2 Steiner's Head, Heart, and Hands

- **Head (Thinking/Semantics)** → Semantic Slime — tank that absorbs and analyzes meaning
- **Heart (Feeling/Rhetoric)** → Rhetoric Robot — support using voice and persuasion
- **Hands (Willing/Syntax)** → Grammar Golem — bruiser built through physical assembly

### 17.3 Cognitive Load Theory (Sweller)

Psycholinguistic data drives gameplay physics:
- Abstract words (low Concreteness) = heavier, slower entities = more cognitive load
- Positive, high-energy words = speed boosts via `TimeScale` = rewards deep engagement

### 17.4 Constructionism (Papert)

Spelling is physical block assembly. If grammatical validity fails, the entity doesn't spawn — consequences are procedural and immediate. The learner learns by building, not by being told.

### 17.5 Zone of Proximal Development (Vygotsky)

The `StudentTrail` tracks every choice. The curriculum spirals — words return at higher difficulty. The game bridges the gap between what the learner can do alone and what they can do with the game's scaffolding.

### 17.6 The Crawl/Walk/Run AI Watcher (Target Design)

Because FACES is a tiny 4-byte state, the game can run a small AI agent in the background that demonstrates optimal word+face combinations. This agent is not a large language model. It is a lightweight policy — a lookup table or small neural net over the 38,400-state FACES space and the current battle/quest context — small enough to run on a watch or Chromebook.

The agent provides a **Crawl → Walk → Run** scaffold based on Vygotsky's Zone of Proximal Development:

| Phase | Learner's Role | AI's Role | What the Learner Learns |
|---|---|---|---|
| **Crawl** | Watch | AI controls both word and face | The learner sees cause and effect: "When the AI chose a Fierce face with the word 'fire,' the spell succeeded. When it chose a Calm face, the spell fizzled." |
| **Walk** | Choose the word | AI chooses the face and explains why | The learner learns to match word meaning to the AI's chosen mood. The AI narrates: *"I picked Fierce because 'inferno' is already a fierce word, and the enemy is made of fire."* |
| **Run** | Choose both word and face | AI only intervenes when the learner is stuck | The learner takes full control. The AI becomes a hint system, not a driver. |

The agent is transparent. Because the FACES state is human-readable (e.g., `Fierce/Sharp/Intense/Open`), the learner can inspect the AI's reasoning. There are no hidden weights. The AI is a fellow player, not a black box.

This is particularly powerful on low-compute devices. A Fitbit-sized screen cannot show a full sentence-building interface, but it can show a single AI-driven battle and let the learner tap the face or word when the AI asks for help.

> *In simple terms: The game can have a tiny robot partner that shows you how to play. First you watch it. Then you help it pick words. Then you take over. Because the robot only uses a 4-byte mood code, you can see exactly why it made each choice.*

---

## 18. Research-Informed Design

### 18.1 Games That Informed This Design

| Game | What We Took | How We Use It |
|------|-------------|---------------|
| **Pokémon** | Pet collection, rarity tiers, type advantages | Words = pets, etymology = types, RPS = class balance |
| **Prodigy Math** | Learning during combat, not before | Synonym/antonym challenges during battle |
| **Duolingo** | Immediate feedback, spaced repetition | Invalid words spawn glitch entities; words return at higher difficulty |
| **WonderLang** | RPG vocabulary through enemies | Wild Typos carry word stats; defeating them = mastering words |
| **Roblox Slime Simulator** | Slime factory, rarity pool, decorations | Pet rarity system, visual evolution, companion follow |
| **Colourful Semantics / Ice Maze** | Color-coded grammar | Quest slots color-coded by part of speech |
| **Tamagotchi** | Pet bonding, care, growth | Pet/Feed/Attune interactions, FACES emotional response |

### 18.2 Academic Foundations

| Theory | Application |
|--------|------------|
| **Psycholinguistics** (Warriner et al.) | 9,582 words with concreteness, valence, intensity, dominance data |
| **Cognitive Load Theory** (Sweller) | Word difficulty → entity weight/speed |
| **Constructionism** (Papert) | Spelling as physical block assembly |
| **Zone of Proximal Development** (Vygotsky) | Spiral curriculum, StudentTrail tracking |
| **Procedural Rhetoric** (Bogost) | Grammar rules enforced through game processes, not text |
| **Steiner's Threefold Nature** | Head/Heart/Hands → Slime/Robot/Golem |
| **Classical Trivium** | Grammar/Logic/Rhetoric → Golem/Slime/Robot |
| **Jungian Archetypes** | 12 NPCs mapped to 12 archetypes |

### 18.3 FACES Protocol Research

The FACES protocol is original research, documented in `crates/faces-protocol/docs/`. It maps English grammar to a 4-byte emotional state using zero-compute keyword detection, producing 38,400 unique states. This is the game's core technical innovation — no other game generates visual appearance from word meaning.

> *In simple terms: We studied Pokémon, Duolingo, Prodigy, and other games to find what works. We took the best ideas and combined them with real brain science about how people learn words. The FACES system — making faces from word meanings — is our original invention. No other game does this.*

---

## 19. Engine Status — What Exists vs What Needs Building

### 19.1 What's Built and Working (38 source files, 33/33 integration tests)

| System | Status | Source File |
|--------|--------|------------|
| 5 JSON databases loaded | ✅ Done | `database.rs` |
| Word validation + pet spawning | ✅ Done | `letter.rs:307` |
| Etymology → Element + Role | ✅ Done | `letter.rs:335` |
| Psycholinguistic stat calculation | ✅ Done | `letter.rs:369` |
| FACES detection + 3D mesh | ✅ Done | `letter.rs:378`, `render.rs:206` |
| Procedural pet rendering | ✅ Done | `render.rs` (831 lines) |
| Eyes, mouth, wings, ears, rings, particles | ✅ Done | `render.rs:300-396` |
| Pet animations (idle, alert, battle, happy, sleeping) | ✅ Done | `render.rs:424` |
| Semantic distance battle system | ✅ Done | `battle.rs:17` |
| Class-specific combat (3 modes) | ✅ Done | `battle.rs:79-152` |
| Battle UI (2D + XR) | ✅ Done | `battle.rs:368-447` |
| Critical hit effects (screen shake, particles) | ✅ Done | `battle.rs:213` |
| Mad-Lib quest engine | ✅ Done | `quest.rs:19-144` |
| NPC dialogue system | ✅ Done | `quest.rs:146` |
| 12 districts + curriculum manager | ✅ Done | `quest.rs:162-218` |
| Pet bonding (Pet/Feed/Attune) | ✅ Done | `chat.rs` |
| FACES state observation | ✅ Done | `chat.rs` |
| Kokoro TTS integration | ⚠️ Feature-gated | `chat.rs` (behind `tts` flag, untested on WASM) |
| Save/load (JSON, auto-save) | ✅ Done | `save.rs` |
| HUD (XP, grade, deck counter) | ✅ Done | `hud.rs` |
| Main menu | ✅ Done | `menu.rs` |
| Tutorial system | ✅ Done | `tutorial.rs` |
| Paywall/demo limits | ✅ Done | `paywall.rs` |
| Day/night cycle | ✅ Done | `time_cycle.rs` |
| Spatial UI panels | ✅ Done | `spatial_ui.rs` |
| Spatial deck (XR) | ⚠️ Scaffolded | `spatial_deck.rs` (UI shell, no real XR interaction) |
| Altar/summoning system | ⚠️ Basic geometry | `altar.rs` (cylinder + button, no real summoning logic) |
| Dialogue UI | ✅ Done | `dialogue_ui.rs` |
| Hand tracking + pinch | ⚠️ Stub | `hand_tracking.rs` (simulated desktop positions, not real OpenXR joints) |
| ASL fingerspelling | ⚠️ Stub | `hand_tracking.rs:105` (only detects 'A' and 'L' via distance heuristic) |
| Grammar fusion system | ✅ Fixed | `hand_tracking.rs:125` (now queries `PetAvatar` instead of non-existent `Summon`) |
| Letter crystals + collection | ✅ Done | `letter.rs` |
| Keyboard spelling | ✅ Done | `letter.rs:114` |
| XR holographic spelling | ⚠️ Scaffolded | `letter.rs:188` (UI exists, input is simulated) |
| Deck/hand/discard system | ✅ Done | `deck.rs` |
| Swipe input | ✅ Done | `input.rs` |

### 19.2 What Needs Building (Priority Order)

| Priority | Feature | Description |
|----------|---------|-------------|
| **P0** | **"Arousal" → "Intensity" rename** | Workspace-wide rename of `arousal` to `intensity` in all structs, UI, and display traits. JSON key stays "A" for parsing. Safety: "Arousal" is inappropriate for an educational game UI. |
| **P0** | **Profanity blocklist** | Filter in `submit_spelling_word()` — banned words fail silently (no glitch entity, no reward). Safety: prevent players from summoning word cards from slurs/profanity. |
| **P0** | Pet Card reveal | Card flip animation before pet spawns (the Pokéball moment) |
| **P0** | Pet Collection screen | Browse all collected pets as cards, sortable |
| **P0** | Roster selection | Pick 3-6 pets for battle from collection |
| **P1** | ASL fingerspelling (full) | Expand `hand_tracking.rs:105` from 2-letter stub to full A-Z ASL recognition for Google Aura VR spelling |
| **P1** | Rarity system | Calculate and display rarity tiers with stat multipliers |
| **P1** | Visual evolution | Pet appearance changes at mastery thresholds |
| **P1** | RPS class modifier | +50%/-25% damage based on class matchup |
| **P1** | Active combat learning | Synonym/antonym challenges during battle |
| **P1** | Color-coded quest slots | Visual grammar validation |
| **P2** | ~~Companion follow system~~ | ✅ Done in `companion.rs` |
| **P2** | Nuisance letters | Roaming letters that chase player |
| **P2** | Curriculum-biased spawning | Letters biased toward grade-appropriate words |
| **P2** | Pet Dream Layer | Mastered pets whisper etymology poetry |
| **P3** | Learner Dashboard | Web app to read save.json and show analytics |

> *In simple terms: The engine core is done — spelling, word card creation, battles, quests, saving, rendering all work. But several XR systems are stubs (ASL only detects 2 letters, hand tracking is simulated, grammar fusion is broken). Two safety issues must be fixed before shipping: rename "arousal" to "intensity" and add a profanity blocklist. Then the card flip reveal, collection screen, and roster selection are the next things to build.*

---

## 19. Stealth Assessment & Institutional Telemetry

LitTCG is not only a game — it is an embedded psychometric evaluation tool. Every cast, quest completion, and Socratic interaction generates evidence that can be mapped to established literacy standards. This data is collected silently, without interrupting play or inducing test anxiety.

### 19.1 Evidence-Centered Design (ECD)

The telemetry architecture follows the three ECD models:

- **Competency Model:** CCSS ELA Grades 9–12 constructs — vocabulary acquisition, syntax, and figurative/pragmatic language comprehension.
- **Task Model:** The existing combat loop and NPC Mad-Lib quest slots. No new interactions are required.
- **Evidence Model:** Mathematical bridge connecting gameplay to standards, implemented inside `VaamMetrics` and serialized through `CharacterSheet`.

### 19.2 Psycholinguistic Metrics

The engine computes the following metrics from existing gameplay data:

| Metric | Description | Source |
|---|---|---|
| **HD-D (Lexical Diversity)** | Hypergeometric probability that a random token sample captures each word type; mitigates text-length bias. | Rolling cast/token window |
| **MTLD** | Windowed Measure of Textual Lexical Diversity; cross-checks HD-D. | Rolling cast/token window |
| **Syntactic Complexity Ratio** | Frequency of multi-card literary-device combos (alliteration, oxymoron, hyperbole) vs basic single-card casts. | Battle session logs |
| **FACES Pragmatics Axis** | Contextual flexibility score derived from resonance between word intrinsic FACES and environmental/contextual FACES. | `play_battle_card()`, `complete_quest()` |
| **Subject Mastery** | Per-language-subject success counters (e.g., simple-past, negation, tone). | Quest completion |
| **CCSS Coverage** | Count of demonstrated exposures per standard code. | Tagged words, combos, and quest outcomes |

### 19.3 CCSS ELA Standard Mapping

Existing mechanics map directly to Common Core standards:

| Systemic Mechanic | CCSS ELA Standard | Telemetry Output |
|---|---|---|
| FACES Resonance Math | L.9-10.5 — figurative language, word relationships, nuances | Contextual flexibility score (Pragmatics Axis) |
| Literary Device Combo System | L.11-12.3 — language functions in different contexts | Syntactic complexity ratio |
| Socratic Tutor Loop | L.9-10.3 — apply knowledge of language to contexts | Failure-to-success ratio after corrections |
| Mad-Lib Quest Slots | L.9-10.4 — clarify unknown/multiple-meaning words | Expected vs actual FACES precision matrix |

### 19.4 Data Privacy & Local-First Architecture

All telemetry is stored strictly in the local `save.json` file. No cloud servers, no accounts, no third-party analytics. Educators can manually import the save file into the disconnected Institutional Dashboard. This design satisfies FERPA and COPPA review hurdles by default and is a core selling point for school districts.

### 19.5 Institutional Dashboard

A local HTML/JavaScript dashboard (`dashboard/index.html`) reads `save.json` and renders:

- Per-standard coverage heatmap.
- Lexical diversity (HD-D/MTLD) trend lines.
- Syntactic complexity ratio over time.
- IEP-friendly raw-numbers table (syntax, semantics, pragmatics series).

The dashboard is optional; the game functions fully without it.

---

## 20. Commercialization

**Hybrid Freemium + B2B Institutional:**

1. **Free Web Demo** (itch.io, WASM): 10 starter words, 1 NPC quest chain, 1 battle. No saving. Top-of-funnel for learners and educators.
2. **Paid Desktop/Mobile** ($9.99): Full 9,582-word database, all 12 NPCs, all 93 quests, local save progression.
3. **Expansion Packs** ($4.99): SAT Prep, Science Vocab (NGSS-aligned), Spanish-English Bridge.
4. **Learner Dashboard** ($7.99/mo): For self-directed learners, tutors, and parents — reads local save.json.
5. **Institutional Dashboard** (B2B site license / per-seat): For districts, tutoring centers, and adult-education programs. Reads aggregated save files, outputs CCSS-aligned reports, supports IEP data generation.

### Why This Model Works for High School and Adult Learners

- Learners control the budget — one-time $9.99, no microtransactions
- Demo is free and accessible — no risk to try before buying
- Save file is readable — learners and tutors can verify progress without the dashboard
- Expansion packs align with real skill needs — SAT prep, academic vocabulary, and professional communication are natural upsells
- Dashboard is optional — the game works fully without it

### Why This Model Works for Institutions

- **Local-first data** bypasses FERPA/COPPA cloud-review hurdles.
- **CCSS-aligned telemetry** satisfies evidence-based intervention requirements under ESSA.
- **IEP-ready outputs** provide continuous, quantifiable progress data for special-education teams.
- **Hardware inclusivity** — WASM and lightweight 2D builds run on older Chromebooks and low-bandwidth environments.
- **Grant alignment** — outputs match Title IV-A (SSAE), MLTI TeachWithTech, 21st CCLC, and adult-education funding priorities.

> *In simple terms: Free demo on the web. $10 for the full game. $5 expansion packs for specific topics. Optional $8/month learner dashboard to track your own progress. Institutional site licenses for districts and tutoring centers. No microtransactions. You stay in control of your data.*

---

## 21. Demo Scope

For the itch.io WASM release:

- 10 total words in the dictionary (curated for variety: mix of elements, classes, grades)
- 1 NPC quest chain (3 quests from one NPC)
- 1 battle encounter
- No local saving (progress resets on refresh)
- "Get Full Version" prompt after significant play
- Full visual polish — FACES expressions, particle effects, animations

### Demo Word Selection Criteria

The 10 demo words should showcase the system's range:
- 2 Fire element words (different roles)
- 2 Water element words
- 2 Earth element words
- 1 Air, 1 Shadow, 1 Light, 1 Normal
- Mix of word roles (Tank, Bruiser, Striker, Healer, Support) within the SemanticSlime class
- Mix of grade levels (mostly 6-8 and 9-12 for accessibility to the target audience)
- At least one word that produces a dramatic FACES expression

> *In simple terms: The free demo has 10 words, 1 character's quests, and 1 battle. No saving. It's a taste. If the player wants more, they can buy the full version for $10.*

---

## 22. Shipping Checklist

### Must Have Before Demo Ship

- [ ] Zero compiler warnings (`cargo check` clean — currently 12 warnings)
- [ ] All 8 integration tests passing (`cargo test`)
- [x] **"Arousal" renamed to "intensity" in all code + UI** (safety)
- [ ] **Profanity blocklist implemented in `submit_spelling_word()`** (safety)
- [ ] Main menu loads on launch
- [ ] Tutorial plays for first-time users
- [ ] Player can spell words and see visual feedback
- [ ] Pet card reveal animation (the Pokéball moment)
- [ ] Pet collection screen (browse collected pets)
- [ ] HUD displays required information
- [ ] Battle encounter completes (start → fight → win/lose)
- [ ] Quest completes (start → fill slots → reward)
- [ ] WASM build runs in browser without crashing
- [ ] Demo limitations apply correctly (10 words, no save)
- [ ] itch.io page copy and assets ready

### Nice to Have Before Demo Ship

- [ ] Roster selection (pick pets for battle)
- [ ] Rarity tiers displayed on pet cards
- [ ] RPS class modifier in combat
- [ ] Color-coded quest slots
- [ ] Companion pet follows player
- [ ] Critical hit screen shake + particles (already implemented, needs wiring)

### Post-Demo Roadmap

- [ ] Full 9,582-word database unlocked
- [ ] All 12 NPCs with quest chains
- [ ] Visual evolution system
- [ ] Nuisance letters
- [ ] Pet Dream Layer
- [ ] Parent Dashboard web app
- [ ] Android XR build
- [ ] Expansion pack system

---

## Appendix A: English Skills Covered

| Skill | Mechanic | Source Code |
|-------|----------|------------|
| Spelling | Collect letters, arrange into words | `letter.rs` |
| Vocabulary | Each word becomes a collectible pet | `components.rs` |
| Etymology | Root analysis determines pet element/stats | `database.rs` |
| Parts of speech | FACES Container/Focus/Action = noun/adverb/verb | `faces-protocol` |
| Synonyms/antonyms | Battle mechanic — match word relationships | `battle.rs` |
| Sentence structure | Mad-Lib quests (fill noun/verb/adj slots) | `quest.rs` |
| Psycholinguistics | Concreteness/Valence/Intensity/Dominance → stats | `database.rs` |
| Grammar | Suffix analysis determines pet role | `letter.rs:351` |
| Creative writing | Mad-Lib quest completion produces sentences | `quest.rs:94` |

## Appendix B: What We Cut (And Why)

| System | Why Cut | Replaced With |
|--------|---------|--------------|
| Three separate games (Trivium) | Can't ship 3 games | One game, RPS class balance |
| Symbol / ARCANA | Redundant with FACES | FACES Container/Focus/Action |
| SynergyLinks / Wu Xing | Over-engineered | Simple synonym/antonym matching |
| Sled Vector DB | Too heavy | JSON save file |
| 12-phase ADDIECRAPEYE | Over-engineered | GameState (10 states) |
| Autopoietic code mutation | Self-modifying code, unsafe | Cut entirely |
| DAG Curriculum graph | Over-engineered | Simple word list with grade levels |
| Janus-Pro-1B / Trellis (runtime) | Too heavy for WASM/mobile, breaks determinism | Three-tier hybrid (see below) |

## Appendix B2: Three-Tier Pet Generation Strategy

Instead of embedding AI generation in the core game loop, we use a hybrid approach:

**Tier 1: Procedural FACES (current system) — covers all 9,582 words instantly**

The FACES protocol produces 6,451,200 unique configurations (256 aura × 5 container × 6 focus × 5 action × 7 elements × 8 roles × 3 classes). Every word gets a pet. No waiting, no AI, no downloads. The pet's appearance is deterministic from the word's meaning — this is the core IP.

**Tier 2: Pregenerated glTF for top ~500 words — offline AI, shipped as assets**

Use Janus Pro + Trellis offline as **artist tools**, not runtime engines. Generate high-quality 3D models for the most common words (K-2 and 3-5 grade levels). Ship as embedded `.glb` files. The code at `render.rs:267` already loads these automatically. First 500 pets look hand-crafted; remaining 9,082 words use the procedural system.

**Tier 3: Pet Studio (paid desktop feature) — embedded AI for custom accessories**

Janus Pro + Trellis at runtime, desktop only. Not for generating whole pets — for generating **accessories** (hats, armor, aura effects, companion creatures). The base pet is always FACES-procedural. Accessories are small, fast to generate, and don't break the game loop if they fail.

**Why not full AI for every pet:**
- WASM binary size: Janus Pro 1B quantized = ~500MB-1GB, can't embed in WASM
- Runtime latency: 5-30 seconds per pet breaks instant gratification
- Quality inconsistency: some generations look bad
- Loss of determinism: FACES guarantees "inferno" always looks angry/red
- Loss of pedagogical link: the pet's appearance IS the word's meaning — AI breaks that
- Compute drain: mobile/XR targets can't run a 1B model

| Great Railway of Lexis lore | Over-scoped | 12 NPC districts in lore_db.json |

## Appendix C: The FACES Protocol

FACES (Focus, Action, Container, Element, System) is a 4-byte emotive state protocol. It is both a visual face system and a **semantic quantification register** that can be compared, combined, and graded with pure arithmetic.

### C.1 The 4-Byte Register

```
[Aura: 1 byte] [Container: 1 byte] [Focus: 1 byte] [Action: 1 byte]
    256 values        5 values          6 values         5 values
```

Packed into a 32-bit integer:

```
Register_32 = (Aura << 24) | (Container << 16) | (Focus << 8) | Action
```

Total unique states: 256 × 5 × 6 × 5 = **38,400**.

| Byte | Grammar Role | Semantic Role | Visual Output |
|---|---|---|---|
| **Aura** (256) | Adjective | Mood, atmosphere, qualitative tone | Pet color (ANSI-256 spectrum), emissive glow |
| **Container** (5) | Noun | Entity boundary, cognitive temperament | Head mesh: Neutral/Rigid/Fluid/Defensive/Sharp |
| **Focus** (6) | Adverb | Intensity, attention, processing load | Eye expression: Intense/Open/Closed/ etc. |
| **Action** (5) | Verb | Kinetic readiness, communicative intent | Mouth shape: Flat/Open/Curved/ etc. |

### C.2 The Grammar-to-FACES Isomorphism

The protocol is isomorphic to English sentence structure:

```
Adjective  → Aura     → Heart (emotion)
Noun       → Container → Mind (logic/identity)
Adverb     → Focus    → Body (sensation/attention)
Verb       → Action   → Will (output/action)
```

This is why a sentence in the game literally builds a FACES state. "The fierce dragon roared loudly" is not just grammar — it is a 4-byte emotional register that can be rendered, compared, and scored.

### C.3 Semantic Resonance Distance

Given two FACES states A and B, the resonance distance is a weighted per-byte difference:

```
d(A, B) = w_aura * |A.aura - B.aura| / 255
        + w_container * δ(A.container, B.container)
        + w_focus * δ(A.focus, B.focus)
        + w_action * δ(A.action, B.action)
```

where `δ(x, y)` is the discrete distance between enum variants (0 = same, 1 = adjacent, ... up to max). For the target game balance, the recommended weights are:

| Dimension | Weight | Reason |
|---|---|---|
| Aura | 0.4 | Mood is the strongest signal of connotation |
| Container | 0.2 | Boundary/temperament is structural |
| Focus | 0.2 | Intensity modulates without reversing |
| Action | 0.2 | Output intent is the behavioral tip |

A distance below 0.25 is **Resonant** (strong alignment). A distance above 0.75 is **Dissonant** (strong clash). Between is **Neutral**.

### C.4 Congruence Levels

When comparing three FACES layers — word (intrinsic), Slime (contextual), and situation (environmental) — the engine produces a congruence grade:

| Aligned Layers | Count | Grade | Game Effect |
|---|---|---|---|
| 0 | None | Dissonant | Reduced effect, possible recoil, tutorial hint |
| 1 | One | Mismatched | Baseline effect, combat log notes the tension |
| 2 | Two | Aligned | Bonus effect or XP |
| 3 | All three | Resonant | Critical/Resonant Cast, full VFX, mastery bonus |

### C.5 Optional Embedding Extension (Tier 2)

The FACES keyword detector is the universal baseline. On desktop or devices with an NPU, the game can optionally project a word's definition into a 384-dimensional embedding (e.g., a quantized `all-MiniLM-L6-v2` or Nomic-Embed) and use cosine similarity as a second semantic signal.

The final semantic score is a blend:

```
Semantic_Score = 0.7 * FACES_resonance + 0.3 * embedding_cosine_similarity
```

On watches, Chromebooks, and the WASM demo, the embedding term is omitted and the FACES term alone drives the system. This keeps the core loop identical across all platforms while allowing richer semantics where compute permits.

Detection is zero-compute at baseline: keyword matching on the word's dictionary definition. No neural networks, no API calls, no latency. The protocol is documented in `crates/faces-protocol/docs/` and implemented in the `faces-protocol` crate.

---

*This document is the single source of truth for LitTCG. All design decisions should reference this document. When this document and code disagree, the code is the current reality and this document is the target.*

*Last updated: July 2026*
