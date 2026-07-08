# LitTCG Lore & World Brainstorm

## How this ties to the combat brainstorm doc

The combat doc asks: *"What is the emotional fantasy of combat?"* The lore doc answers that question. The world gives combat its **meaning**. Without a compelling world, the literary devices and economy systems are just mechanics. With a compelling world, they become a story the child is living inside.

This doc is designed to be read alongside `docs/COMBAT_BRAINSTORM.md`. Combat is the verbs; lore is the setting, stakes, and characters that make those verbs matter.

---

## What we already have

### 1. Twelve districts (`assets/lore_db.json` + GDD)

| # | District | Theme | Grade | Why it matters |
|---|----------|-------|-------|----------------|
| 1 | Garden District | Growth, nature, beginnings | 1 | Safe starting area. Teaches simple nouns/verbs. |
| 2 | Outlaw Outpost | Rebellion, rule-breaking | 2 | Introduces prefixes that undo meaning (`un-`, `de-`). |
| 3 | Shadow Library | Mystery, hidden knowledge | 3 | Introduces roots and etymology. |
| 4 | Great Railway | Journey, connection | 4 | Connects words across districts. |
| 5 | Maintenance Bay | Repair, practical work | 5 | Introduces suffixes that turn words into jobs/actions. |
| 6 | Irony Junction | Contradiction, humor | 6 | Introduces oxymoron, irony, and rhetorical play. |
| 7 | Adjective Valley | Description, color | 7 | Deepens imagery and sensory language. |
| 8 | Central Station | Hub, crossroads | 8 | Recap and synthesis of earlier skills. |
| 9 | Metaphor Mountains | Figurative language | 9 | Metaphor, simile, symbolism. |
| 10 | Logic Labyrinth | Reasoning, structure | 10 | Advanced grammar, argument, and rhetoric. |
| 11 | Semantic Sea | Meaning, depth | 11 | Synonym/antonym nuance, connotation. |
| 12 | Mastery Monolith | Final mastery | 12 | Endgame synthesis. |

### 2. Twelve NPCs (`assets/lore_db.json`)

Each NPC has:
- **Archetype** (The Innocent, The Hero, The Rebel, etc.)
- **District** (which zone they belong to)
- **PreferredElement** and **PreferredClass** (what kinds of pets they like)
- **Teaches** (a list of roots/suffixes/prefixes)
- **EvolutionRole** (what kind of word evolution they help with)
- **Dialogue** pools for Dawn, Day, Dusk, Night
- **AvatarPath** (portrait texture)

Current NPCs: Barnaby, Yorick, Kael, Martha, Gribble, Nyx, Vlad, Pygmalion, Chesty, Ozymandias, Zafir, Ignis.

### 3. Pet lore generation (`crates/lit-asset-forge/src/lore.rs`)

Two paths:
- **LLM path:** calls LM Studio to generate `PetLore` (title, description, habitat, behavior, fun_fact, etymology_hook, npc_guardian).
- **Deterministic path:** uses `generate_lore_deterministic()` to build lore from word + element + class + role + synonyms + root.

The deterministic generator already maps elements to districts and NPCs:
- Fire → Action Alley, Zafir
- Water → Heartwood Grove, Vlad
- Earth → Brainy Borough, Yorick
- Air → Whisper Winds, Nyx
- Light → Action Alley, Kael
- Shadow → The Dim Archives, Ozymandias
- Ice → Whisper Winds, Pygmalion

### 4. Time-of-day system (`src/core/time_cycle.rs`)

Four phases: Dawn, Day, Dusk, Night. Each NPC has dialogue for each phase. Currently each phase is 30 seconds for demo purposes.

### 5. Quest system (`src/core/quest.rs`)

NPCs have quest chains. Quests are Mad-Lib templates with slots like `{ADJECTIVE}`, `{NOUN}`, `{VERB}`. Completing quests gives XP and evolves mastery.

---

## What is missing from the lore

1. **A central antagonist.** The "Typos" are a corruption type, but there is no character behind them. Why are words becoming corrupted? Who benefits?
2. **A creation myth.** Why do words become pets? Why is the world organized into 12 districts?
3. **Player identity.** Who is the child in this world? Are they a "Word Weaver," a "Librarian," a "Trainer"?
4. **District state.** Districts are static. There is no corruption level, no reputation, no visual change as the player helps.
5. **NPC relationships.** NPCs talk, but they don't remember, react, or grow based on player choices.
6. **Pet lore in gameplay.** Pet lore exists as data but is not surfaced during combat, questing, or bonding.
7. **A narrative arc.** There is no escalation from "help Barnaby" to "save the world from The Static."
8. **Enemy variety.** Typos are all the same. There are no boss types, no corrupted sentences, no enemy champions.

---

## World-building proposals

### Proposal 1: The creation myth of Syllable Springs

**The world:** Once, all words lived in a single Great Dictionary. Then the dictionary was torn apart by a catastrophe called **The Static**. Words scattered across 12 districts. Some words were corrupted into Typos. Others survived as wild letter crystals.

**The child:** A "Word Weaver" — someone who can hear words and reassemble them into living creatures. Only a Word Weaver can restore the Great Dictionary.

**The goal:** Travel through all 12 districts, rescue corrupted words, and rebind the dictionary. The final district is the Mastery Monolith, where the player confronts The Static.

**Why this works:**
- Gives every battle a purpose: you are rescuing a corrupted word, not just killing an enemy.
- Explains why words become pets: the dictionary's magic turns meaning into form.
- Explains the 12 districts: each is a torn page of the dictionary.
- Gives a long-term win condition: restore the Great Dictionary.

### Proposal 2: The Static as the antagonist

**What is The Static?** A force of forgetting, noise, and miscommunication. It thrives when words are misused, misspelled, or forgotten. It appears as visual static, glitch effects, and corrupted text.

**How it fights:**
- Sends Typos to corrupt districts.
- Spreads "fog of forgetting" that hides word definitions.
- Creates **Bosses** from corrupted sentences or famous misquotes.
- Can possess NPCs temporarily, changing their dialogue to nonsense.

**How the player fights back:**
- Correcting Typos reduces Static in a district.
- Completing quests restores clarity to the dictionary page.
- Mastering words creates "anchors" that The Static cannot erase.
- Evolving pets into their Mastered form creates a golden aura that repels Static nearby.

**Why this works:**
- Thematically perfect for a literacy game. The enemy is misunderstanding itself.
- Scales with difficulty: early Static is misspellings, late Static is logical fallacies and rhetoric traps.
- Gives visual identity: static, glitches, corrupted text.

### Proposal 3: Districts as living pages

Each district is a torn page from the Great Dictionary. As the player helps NPCs, the district becomes more vivid:

- **Corrupted state:** Grayed out, static flickers, NPCs are worried, Typos roam.
- **Restored state:** Colors brighten, music becomes clearer, NPCs celebrate, rare words spawn.
- **Mastered state:** A golden border appears on the map. The district's NPC gives the player a legendary word quest.

This is the **District Corruption / Reputation** system from the economy doc. It gives the world a visual and narrative arc.

### Proposal 4: The NPCs as guardians of meaning

Each NPC is not just a quest-giver but a **guardian** of a type of language:

| NPC | Archetype | Guards | How they help in combat |
|-----|-----------|--------|------------------------|
| Barnaby | Innocent | Simple words, kindness | Gives emergency vowels when the player is stuck. |
| Yorick | Everyman | Structure, roots | Identifies a hidden root for free once per battle. |
| Kael | Hero | Action, courage | Buffs the player's first attack in a battle. |
| Martha | Caregiver | Healing, comfort | Restores pet stamina between battles. |
| Gribble | Explorer | Discovery | Reveals one enemy weakness per district. |
| Nyx | Rebel | Subversion, negation | Can turn an enemy's prefix against it. |
| Vlad | Lover | Beauty, emotion | Adds Pathos damage to high-valence words. |
| Pygmalion | Creator | Form, craft | Builds a temporary shield from a sentence. |
| Chesty | Jester | Humor, play | Adds random (but usually helpful) effects. |
| Ozymandias | Sage | Knowledge, history | Reveals full etymology of a word once per day. |
| Zafir | Magician | Transformation | Changes a word's element for one battle. |
| Ignis | Ruler | Order, power | Adds a permanent +1 to a pet's mastery. |

This makes NPCs mechanically relevant, not just flavor.

### Proposal 5: Player identity — The Word Weaver

The child is a **Word Weaver** — a rare person who can:
- Collect scattered letters and bind them into words.
- Hear the meaning inside a word and give it a face.
- Summon a word's pet form in combat.
- Restore corrupted words back to the dictionary.

**Visual identity:** The player has a "loom" or "spindle" tool that they use to weave letters. The companion pet follows them like a familiar.

**Mechanical identity:** Word Weavers can "attune" to one of four channels (Mind/Heart/Body/Action), which determines their emergent class and combat bonuses.

**Why this works:**
- Gives the child a role in the world, not just a cursor.
- Explains why only the player can do the core mechanics.
- Opens up progression: the player themselves can level up as a Weaver.

### Proposal 6: Pet lore as gameplay

Every pet's lore should be discoverable and useful:

- **Habitat** tells you where to find more words of the same element.
- **Behavior** hints at a pet's special move in combat.
- **Etymology hook** gives a root that can be used for critical hits.
- **Fun fact** adds a memorable detail that helps the child remember the word.
- **NPC guardian** tells you which NPC will be most excited to see this pet.

Example: A pet with habitat "Warm stone ledges near bubbling ink-wells" suggests Fire/Earth elements. Behavior "Stomps around correcting grammar" suggests a Golem. Etymology hook "root 'rupt' still hums" tells the player to use root attacks.

### Proposal 7: Enemy champions and corrupted sentences

Instead of every enemy being a single random word, introduce enemy types with lore:

| Enemy type | What it is | Combat twist |
|------------|-----------|--------------|
| **Typo** | A single misspelled word | Basic enemy. Counter with correct spelling. |
| **Malaprop** | A word used in the wrong place | Sound-based confusion. Counter with a homophone or correct word. |
| **Run-On** | A sentence with no punctuation | Attacks multiple times but weakly. Counter with a punctuation-themed word. |
| **Double Negative** | A sentence that says the opposite of what it means | Reverses damage. Counter with a clarification word. |
| **Fragment** | An incomplete sentence | Fragile but hides behind shields. Counter with completion words. |
| **Boss: The Lost Paragraph** | A corrupted paragraph | Multi-slot battle. The child must fill 3-5 grammar slots to defeat it. |
| **Boss: The Static Avatar** | A champion of The Static | Uses advanced rhetoric/irony. Requires a fully evolved pet to challenge. |

These enemies should be introduced by NPCs as "Static creatures are getting smarter. Watch out for the ones that..."

### Proposal 8: Seasons, weather, and world events

Beyond the day/night cycle, add:

- **Vowel Storms:** Rare weather where vowels spawn more often.
- **Root Quakes:** Earth-themed events where root shards appear.
- **The Long Night:** A world event where Static is stronger, but rare Shadow words appear.
- **Festival of Rhymes:** A holiday where rhyming words give double rewards.
- **Dictionary Rebinding Day:** When the player has restored enough districts, a celebration unlocks a final quest.

These events make the world feel alive and give players reasons to return.

---

## How lore connects to combat mechanics

| Lore element | Combat impact |
|--------------|---------------|
| **The Static** | Enemy type, visual theme, boss design, stakes. |
| **District corruption** | Enemy difficulty, spawn rates, available quests. |
| **NPC guardians** | Passive bonuses, pre-battle hints, special move unlocks. |
| **Pet lore** | Weakness hints, habitat-based matchups, root criticals. |
| **Time of day** | Different enemy behaviors, rare word spawns. |
| **Weather/events** | Temporary combat modifiers and bonus conditions. |
| **Player as Word Weaver** | Channel attunement, emergent class, ultimate ability. |

---

## Code inventory: what exists and how scalable it is

### What is already built

| System | File | Status | Scalability |
|--------|------|--------|-------------|
| JSON database loading | `database.rs` | Working | High. Adding new JSON data types is straightforward. |
| NPC data structure | `lore_db.json` | Working | High. Can add new fields per NPC easily. |
| Quest engine | `quest.rs` | Working | Medium. Slot-filling works, but branching dialogue is limited. |
| Time of day | `time_cycle.rs` | Working | Medium. Needs to be wired to more systems (spawn rates, enemy behavior). |
| Pet lore generation | `lit-asset-forge/lore.rs` | Working | High. LLM and deterministic paths exist. |
| Save/load | `save.rs` | Working | Medium. SaveData is small; adding new fields requires migration. |
| Command-driven input | `commands.rs` | Working | High. New game actions can be added as commands. |
| ECS plugin system | `main.rs` / `lib.rs` | Working | High. New systems are easy to add as plugins. |
| Render system | `render.rs` | Working | Medium. Procedural meshes scale with FACES states; adding new visual features requires mesh code. |
| Generated assets | `generated_assets.rs` | Working | High. Can load manifest of portraits/lore at runtime. |

### What is partially built or missing

| System | File | Status | Risk |
|--------|------|--------|------|
| PetCollection | `components.rs` | Data structure exists, UI in `pet_collection.rs` | Medium. Needs to be fully wired into gameplay loops. |
| Companion system | GDD only | Not implemented | Medium. Requires following AI and interaction. |
| District state (corruption/reputation) | Not implemented | Missing | High. This is a major world-economy feature. |
| Enemy variety beyond single-word typos | `battle.rs` | Hardcoded | Medium. Needs data-driven enemy types. |
| Boss battles / multi-slot combat | Not implemented | Missing | Medium. Requires extending `BattleSession`. |
| NPC relationship memory | `lore_db.json` only | Static | Low. Can add reputation fields to save data. |
| Antagonist progression | Not implemented | Missing | High. The Static has no mechanical presence yet. |
| World events / weather | Not implemented | Missing | Low. Can be added as time-based modifiers. |
| Player identity / Weaver progression | `components.rs` (channels) | Partial | Medium. Attunement exists but lacks impact. |

### Architectural strengths

1. **Data-driven JSON pipeline.** Adding new NPCs, quests, roots, suffixes, or words does not require recompiling.
2. **Hot reload.** Designers can edit `lore_db.json` or `quest_data.json` while the game is running and see changes.
3. **Command-driven input.** New input sources (AI, voice, etc.) can send the same commands as keyboard/touch.
4. **Modular plugins.** Each feature lives in its own module. We can turn features on/off by feature flags.
5. **Generated asset manifest.** The game can load AI-generated portraits and lore at runtime without code changes.

### Architectural risks

1. **Combat logic is not data-driven.** `battle.rs` hardcodes class-specific formulas. Adding new enemy types or attack patterns requires code changes.
2. **No district state persistence.** If we add corruption/reputation, we need to save it in `SaveData` and load it correctly.
3. **PetCollection is not the source of truth.** `SpellBook` is still used in some places. We need to unify on one collection resource.
4. **No formal world model.** There is no `World` resource that tracks district states, weather, or events. These are currently scattered.
5. **Save migration.** As we add new fields to `SaveData`, we need a migration strategy for old save files.

---

## Scaling and shifting recommendations

### If we want to make combat more lore-driven

1. Move enemy types into a JSON data file (`enemy_types.json`) with fields: `name`, `corruption_type`, `attack_pattern`, `weakness_device`, `shield_prefix`, `shield_suffix`, `boss_slots`.
2. Add an `EnemyType` component and update `battle.rs` to read from data instead of hardcoding.
3. Add a `DistrictState` resource that tracks corruption and reputation per district, and have it affect enemy spawn difficulty.
4. Add NPC guardian bonuses as passive components or resource buffs during combat.

### If we want to make the world more alive

1. Add a `WorldState` resource that tracks: current district, time of day, weather, active event, corruption levels, reputation levels.
2. Extend `time_cycle.rs` to emit phase-change events that other systems react to.
3. Add a `DistrictPlugin` that handles district-specific logic (spawn bias, NPC schedules, corruption effects).
4. Add a `WeatherPlugin` for temporary world modifiers.

### If we want to add the antagonist

1. Add a `StaticPresence` resource that tracks how much Static is in the current district.
2. Add `StaticEvent` messages that can spawn enemy waves, corrupt NPCs, or hide definitions.
3. Add a final boss encounter triggered by restoring all districts.
4. Add visual static effects in `render.rs` based on `StaticPresence`.

### If we want to shift to a more open-world structure

1. Replace the linear `Grade = XP / 1000` progression with **district-based unlocking**:
   - Complete 3 quests in a district → unlock the next district.
   - Master 5 words in a district → reduce corruption to zero.
2. Add a `WorldMap` resource that tracks which districts are unlocked/restored/mastered.
3. Keep the grade-level word filtering, but let the player choose which district to explore.

### If we want to shift to a more narrative-driven structure

1. Add a `StoryAct` resource with an enum of acts (Act 1: The First Typo, Act 2: The Static Spreads, etc.).
2. Add a `StoryBeat` system that checks conditions and triggers narrative events.
3. Use the command system to send `GameCommand::StoryEvent` messages that UI and NPCs react to.
4. Add cutscene/dialogue panels in `dialogue_ui.rs` that can display story beats.

---

## Homework for you and your wife

### Lore questions

1. **Which creation myth feels right?** A torn dictionary? A world built from spoken words? A library that came to life?
2. **What should the player be called?** Word Weaver, Lexicon Keeper, Syllable Sage, something else?
3. **What does The Static look and feel like?** TV static, ink smears, corrupted text, silence, noise?
4. **Which NPC is your favorite?** Why? Does their personality match their district and what they teach?
5. **Should the world feel safe or slightly dangerous?** Pokémon is mostly safe with patches of danger. Hollow Knight is dangerous. Where does LitTCG sit?

### World-economy questions

1. **Should districts be linear or open?** Do you unlock them one by one, or can you hop around?
2. **How much should NPCs remember?** Do they greet you differently after you've helped them? Do they comment on your favorite pets?
3. **Should there be a "home base"?** A sanctuary that grows as you restore the world?
4. **What is the final win state?** Restoring the Great Dictionary? Defeating a boss? Collecting every word? Something else?

### References to explore

- **Hollow Knight / Ori and the Blind Forest:** How do they make a world feel wounded and worth saving?
- **Pokémon:** How do towns and NPCs create regional identity?
- **Oxenfree / Night in the Woods:** How do they use dialogue and time of day to build character?
- **Cult of the Lamb:** How do they make a home base feel like it grows with your progress?
- **The Legend of Zelda: Breath of the Wild:** How do they make exploration feel rewarding without explicit quests?
- **Children's books:** *The Phantom Tollbooth* is the closest reference — a world where words are literally places and characters.

---

## Recommended next steps

1. **Pick a creation myth and player identity.** This decision affects every other system.
2. **Define The Static.** What does it want, what does it look like, and how does the player defeat it?
3. **Design the first three districts as a vertical slice.** Make Garden District, Outlaw Outpost, and Shadow Library feel distinct in gameplay, not just in theme.
4. **Wire district state into the save system.** Even a simple corruption/reputation number per district makes the world feel responsive.
5. **Add one data-driven enemy type.** Start with a `Malaprop` or `Run-On` enemy to prove the combat system can scale beyond single-word typos.

Once these are decided, we can implement the smallest slice: a `WorldState` resource, a `DistrictState` save field, and one new enemy type that uses a prefix/suffix shield. Then we will know the architecture can support the full vision.

