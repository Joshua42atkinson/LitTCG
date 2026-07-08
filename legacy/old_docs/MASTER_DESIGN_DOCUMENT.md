# **Master Systems Design Document & Technical Specification: Semantic Slime**

> *"Realize deeply that the present moment is all you ever have. Make the NOW the primary focus of your life."* — Eckhart Tolle  
> *"Be Here Now."* — Ram Dass

---

## **Part I: Analysis & The Pedagogical Engine**

### **1. Executive Summary & Core Vision**
**Semantic Slime: Communication Class** is a highly scalable, pet-based gamified educational engine tailored for K-12 English Language Arts (ELA). Built from the ground up on Bevy 0.18.1, the engine seamlessly bridges the abstract domains of linguistics (Grammar, Semantics, Rhetoric) and tangible, procedural 3D/2D gameplay. The player does not simply memorize what a word means—they physically construct the word, summon a procedural pet from its psycholinguistic metadata, and watch it come alive. In Semantic Slime, **the learning is the playing**.

> *In simple terms: Semantic Slime is a game that teaches English to kids. Instead of memorizing words, players use words to build and control virtual pets. It makes learning feel like playing a game.*

#### **The "OLD, NEW, NOW" Philosophy**
While our research encompasses bleeding-edge paradigms—Spatial Computing, Generative AI, and 40-hour curriculum matrices—the **NOW** dictates we remain grounded in the immediate, playable reality. 
- **The OLD:** Our foundational game mechanics and Bevy Entity Component System (ECS) architecture.
- **The NEW:** Advanced psycholinguistic pedagogy and spatial UI.
- **The NOW:** The actionable workflow required to build a polished, performant 2D/3D application today, ensuring we cross the gap from visionary concept to shipped product.

> *In simple terms: We have big dreams for the future (the NEW) and solid game rules from the past (the OLD), but right now (the NOW), we are focused on making a fun, working game that people can actually play today.*

**TODO:** Finalize the marketing copy for the itch.io page based on this core vision.

---

### **2. Pedagogical Foundations (The Science of Learning)**
The design of Semantic Slime is grounded in a rigorous, academic analysis of foundational pillars of educational and ludic theory. It rejects automated, black-box educational applications that fail due to a lack of constructivist scaffolding.

#### **2.1 Psychometric Cognitive Load Theory (CLT)**
To ensure K-12 accessibility, the curriculum adheres strictly to **Sweller’s Cognitive Load Theory (CLT)**, mapped directly through psychometric word data. Rather than arbitrary difficulty curves, the engine utilizes the `WordStats` struct loaded via `database.rs` to manipulate the gameplay physics dynamically based on:

- **Concreteness (C) & Age of Acquisition (AoA):** These values dictate the Intrinsic Load (the inherent difficulty). Highly abstract words (low Concreteness) learned later in life (high AoA) physically manifest as heavier, slower entities in the game, forcing the player to dedicate more cognitive resources to maneuvering them.
- **Valence (V) & Arousal (A):** These emotional metrics dictate the Germane Load. Words with high Valence (positive emotion) and high Arousal (high energy) trigger the `TimeScale` resource, granting `CombustionMultipliers` that accelerate the player's momentum and reward deep semantic integration.

> *In simple terms: We use real science to make the game's difficulty match how hard the words are to learn. Abstract, difficult words feel heavier and move slower. Happy, exciting words give the player a speed boost. This helps the brain physically feel the meaning of the words.*

**TODO:** Hook up the `WordStats` psychometrics to the Bevy physics engine (adjusting mass and velocity modifiers).

#### **2.2 The Mentor-in-the-Middle (Vygotsky's Zone of Proximal Development)**
By grounding the system in Lev Vygotsky’s *Zone of Proximal Development (ZPD)*, the game acknowledges that technology must bridge, not replace, human connection. The `StudentTrail` resource tracks every `SwipeChoice` (Yes, No, Deeper) the player makes. Without a mentor to provide context-sensitive assistance when the `SwipeChoice` indicates struggle, automated learning environments lead to cognitive overload. The Dashboard facilitates a symbiotic relationship where the AI handles data parsing, while the human mentor provides critical oversight.

> *In simple terms: Kids learn best when a teacher helps them just enough to succeed. The game tracks every choice the student makes (the Student Trail) so teachers and parents can see exactly where they are struggling and step in to help.*

**TODO:** Build the Teacher Dashboard web interface to display the `StudentTrail` telemetry data.

#### **2.3 Procedural Rhetoric & Constructionism**
Following Ian Bogost’s concept of *Procedural Rhetoric*, Semantic Slime argues that games make arguments through processes. The game teaches grammatical boundaries not through text blocks, but through functional blueprints in the Bevy ECS. Drawing parallels to Seymour Papert’s *Constructionism*, spelling is treated as an act of physical, geometric block assembly to form `Morpheme` sequences (Prefix, Root, Suffix). The consequences of grammatical error are procedural and immediate.

> *In simple terms: We learn best by doing. Players build words like Lego blocks using roots and suffixes. If the word doesn't make sense, the pieces literally won't fit together in the game.*

**TODO:** Finalize the `Morpheme` block-snapping UI and the visual failure states for incorrect grammar.

---

## **Part II: System Design & Curriculum Architecture**

### **3. The Data Ingestion Engine (`GameDatabase`)**
The core computational bridge between static educational content and reactive gameplay is managed by the `GameDatabase` resource in `database.rs`. It bypasses the need for hardcoded logic by injecting five distinct JSON datasets into the Bevy runtime:

1. **`word_database.json`**: Hydrates the `WordStats` mappings (psychometrics and Common Core Standards).
2. **`synonym_database.json`**: Maps relational linguistics and combat `Element` typing (Fire, Water, Earth, etc.).
3. **`etymology_db.json`**: Defines the morphological `RootData` and `SuffixData` for physical word construction.
4. **`quest_data.json`**: Populates the narrative `QuestTemplate` structures based on required semantic roles.
5. **`lore_db.json`**: Injects `NpcData` for the various world districts and determines their preferred evolutionary classes.

> *In simple terms: Instead of writing new code for every level, the game loads all the words, monsters, and quests from five simple text files. This makes it incredibly easy for teachers to add new vocabulary to the game.*

**TODO:** Finish populating `etymology_db.json` and `quest_data.json` for all 40 weeks of the curriculum.

### **4. Emergent Player Classes & The Attunement Channels**
The gameplay strictly adheres to a psychographic profiling system via the `CharacterSheet`. As players draw cards into their `Hand` from their `Deck`, they interact with four primary linguistic `Channel` types:
- **Mind** (Logic/Syntax)
- **Heart** (Emotion/Pathos)
- **Body** (Physical/Concrete)
- **Action** (Verbs/Kinematics)

As the player engages these channels, their `CharacterSheet` tracks attunement percentages. This dynamically calculates their `emergent_class`—if a player heavily favors the Heart channel, they become **The Bard**; if they favor Action, they become **The Templar**. This ensures the gameplay loop naturally adapts to the student's unique psychological approach to language.

> *In simple terms: The game watches what kind of words you like to use. If you use a lot of action words, the game turns you into a "Templar" warrior. If you use emotional words, you become a "Bard." Your playstyle shapes your character.*

**TODO:** Balance the attunement progression curves inside `engage_channel()` so emergent classes trigger at the correct milestones.

### **5. The Pet Evolution & Mastery System**
Semantic Slime entities undergo a rigorous evolutionary progression tracked by the `SpellBook` resource. As players encounter words in their `StudentTrail`, the engine upgrades their `MasteryLevel`:
1. **Encountered (🔮):** The word is drawn into the hand.
2. **Experienced (⚡):** The word is successfully deployed in combat.
3. **Owned (🌟):** The word is combined into a complex sentence.
4. **Mastered (👑):** The word is utilized dynamically across multiple contexts.

Simultaneously, the physical avatars evolve based on the `SummonClass` enum. A basic noun forms a primitive `SemanticSlime`. Integrating verbs and adjectives evolves it into a `GrammarGolem`. Finally, deploying complex persuasive arguments summons the ultimate `RhetoricRobot`.

> *In simple terms: Every time you see a word, it goes into your Spell Book. As you use the word correctly in battles, you master it. Your pets also evolve from a simple Slime into a giant Robot as your sentences get more complex.*

**TODO:** Implement the 3D model swapping for the `SummonClass` transitions (Slime -> Golem -> Robot).

---

## **Part III: Gameplay Execution & Mechanics**

### **6. The Psychometric Combat Engine (Asymmetrical Typo Battles)**
Combat within Semantic Slime is fully asymmetrical, utilizing a Euclidean distance calculation across a four-dimensional psychometric space to map linguistic concepts directly to tactical strikes. The engine discards arbitrary health-point bloat in favor of emergent semantic vulnerability. 

When a "Wild Typo" manifests (a corrupted entity possessing 50 baseline HP), it inherently possesses the psychometric coordinates (Concreteness, Valence, Arousal, Dominance) of its root word. The player must deduce this entity's position in the psychometric vector space and deploy a card from their `Hand` that represents a profound semantic opposition. In this system, victory is achieved not by matching, but by presenting conceptual friction.

> *In simple terms: When you fight a monster (a "Wild Typo"), you don't just hit it with a sword. The monster represents a word. To beat it, you have to play a word card that means the exact opposite of what the monster represents. If the monster is "Fire", you play "Water".*

**TODO:** Visually represent the enemy's psychometric stats on the `EnemyHealthBar` so players can strategically deduce the opposite concept.

### **7. Semantic Distance Mathematics & The Damage Formula**
The tactical efficacy of any student-deployed card is determined by the `semantic_distance` function located in `src/battle.rs`. The engine calculates the Euclidean distance between the Typo's `WordStats` and the Played Card's `WordStats`:

`Distance = √((ΔC)² + (ΔV)² + (ΔD)² + (ΔA)²)`

- **Critical Efficacy (Distance > 4.0):** If the player deploys an oppositional concept, the system recognizes a high semantic distance. The `damage_multiplier` scales logarithmically: `1.5 + (Distance - 4.0) * 0.2`. This is a Critical Hit, instantly upgrading the played word to `MasteryLevel::Owned` within the `SpellBook`.
- **Ineffective Recursion (Distance < 2.0):** If the player deploys a conceptually adjacent word (a synonym), the semantic distance drops below the 2.0 threshold. The `damage_multiplier` collapses to `0.5`, and the Typo executes a devastating counter-attack, stripping 20 HP from the player's 100 HP reserve.

> *In simple terms: The game uses a math formula to figure out how different your word is from the monster's word. If the words are completely different (like total opposites), you do massive damage. If the words mean the same thing, your attack fails and the monster hits you back!*

**TODO:** Implement screen shake and particle effects when a Critical Hit (Distance > 4.0) is achieved to maximize positive feedback.

### **8. The Card Deck & Hand Engine (`deck.rs`)**
To manage the tactical pacing, the game uses a turn-based array system managed via ECS events and a `GameState` state machine within `src/deck.rs`. The player's active vocabulary serves as their deck.

The `draw_cards` system strictly enforces a maximum `Hand` size (defaulting to 3). During the `GameState::Playing` phase, cards are popped from the `Deck` vector into the `Hand`. Crucially, this draw action triggers an immediate telemetry event:
- The `SpellBook` records a new encounter.
- The `StudentTrail` pushes the word to the `visited_words` array, ensuring the curriculum manager tracks exposure.
- The `CharacterSheet` engages the word's inherent `Channel` (Mind, Heart, Body, Action), subtly shifting the player's emergent class attunement simply by drawing the card into their cognitive workspace.

Once the hand is depleted, the engine seamlessly transitions to `GameState::Reviewing`, forcing a reflective pedagogical pause before the next cycle begins.

> *In simple terms: You can hold up to 3 word cards in your hand at a time. Every time you draw a card, the game remembers that you've seen that word and starts adjusting your character class based on the words you pull. When you run out of cards, you take a break to review what you've learned.*

**TODO:** Ensure the `engage_channel()` function inside `draw_cards` pulls the actual Channel from the `SpellBookEntry` instead of defaulting to `Channel::Mind`.

---

## **Part IV: Technical Implementation**

### **9. Core Architecture & Cross-Platform Pipeline**
Semantic Slime is built entirely in Rust utilizing the Bevy 0.18.1 engine. The architecture ensures that the logic and rendering pipelines easily scale across desktop and mobile XR environments through a feature-flagged build system (`Cargo.toml`):

- **Desktop Orbit (`features = ["desktop"]`):** Designed for high-fidelity iteration, this target compiles to native executables utilizing Vulkan/Metal rendering backends with full post-processing (Bloom, HDR, SSAO).
- **Android XR (`features = ["xr"]`):** The primary deployment target for K-12 spatial environments. Utilizing the Android NDK, the application cross-compiles to `aarch64-linux-android`. The engine relies on a custom `android_main` JNI bridge (located in `src/lib.rs`) to hook the Bevy lifecycle directly into the Android OS, ensuring 90+ FPS stereoscopic rendering.

> *In simple terms: The game is built using a powerful tool called Bevy. It is designed to run beautifully on powerful computer screens (Desktop) and also run smoothly inside Virtual Reality headsets (Android XR).*

**TODO:** Finalize the GitHub Actions CI/CD pipeline to automatically run `cargo ndk` builds on every pull request.

### **10. The Holographic Stage & Input Parsing**
To reduce cognitive load and simplify interactions, the engine discards complex grid-based pathfinding in favor of a centralized 3D "Holographic Stage." 

- **Spatial Arena:** Entities are spawned at static `Transform` coordinates relative to a central neon cylinder, ensuring that the student's visual focus remains fixed on the semantic entity rather than environmental navigation.
- **Swipe Gestures (`src/input.rs`):** The 2D input parser converts continuous touchscreen drags or mouse movements into discrete `SwipeChoice` events (`Yes`, `No`, or `Deeper`). This binary/ternary decision tree removes the extraneous load of precise aiming, allowing the player to focus entirely on linguistic choices.

> *In simple terms: Instead of moving characters around a confusing map, the monster appears right in front of you on a glowing stage. You just swipe left, right, or down to make choices. It keeps the game simple and focused.*

**TODO:** Tune the `SWIPE_THRESHOLD` scalar to prevent accidental micro-drags from registering as a committed `SwipeChoice`.

### **11. Procedural Generation via FACES Protocol**
The engine utilizes the **FACES Emotional Protocol** to map invisible abstract concepts directly into 3D procedural meshes within `src/render.rs`. When a word is encountered, its metadata dictates the structural rendering of the `PetAvatar`:

- **Container (Morphology):** A `Neutral` concept spawns an Ico-Sphere. A `Rigid` concept spawns a Cuboid. A `Fluid` concept spawns a Torus.
- **Aura (Color):** The concept's base frequency maps to an ANSI-256 spectrum index, instantly generating the base albedo and emissive glowing shaders (e.g., deep purples vs. neon cyan).
- **Focus & Action (Expression):** The semantic intent dynamically scales the `PetEye` and `PetMouth` nodes. An `Intense` focus squints the eyes (`Vec3::new(1.0, 0.3, 1.0)`), while an `Open` focus widens them.

> *In simple terms: The game builds the monster's 3D body based on what the word means. A strict word becomes a block. A smooth word becomes a ring. Happy words glow bright, and intense words squint their eyes.*

**TODO:** Ensure `render.rs` falls back gracefully to standard primitive meshes if the `pets/{archetype}.glb` model is not found on the device.

### **12. Hand-Tracking & XR Gestures**
For spatial computing targets, the `ActiveGestures` touch system is completely bypassed in favor of raw XR joint tracking (`src/hand_tracking.rs`). Students literally reach out and "pinch" holographic UI cards suspended in 3D space (`VrHandCard`). 

This physical engagement reinforces the constructivist philosophy: language is not a typed string on a screen, but a tangible object built and wielded by the human hand.

> *In simple terms: When playing in VR, you don't use a controller. You use your real hands to physically grab floating word cards and throw them at the enemy!*

**TODO:** Map the pinch-to-select distance threshold dynamically based on the student's arm length calibration.

---

## **Part V: Telemetry & Progression UI**

### **13. The Telemetry & HUD Overlay (`hud.rs`)**
To monitor student progression without immersion-breaking pauses, Semantic Slime utilizes a real-time Heads-Up Display (HUD) overlay rather than external web dashboards.

- **Character Sheet Stats:** The UI continually queries the `CharacterSheet` resource to display the student's Grade, XP (Experience Points), and total words encountered. 
- **The Stash & Deck:** The student's current inventory of collected letters (`LetterStash`) and their active draw deck (`DeckCounterText`) remain visible at the bottom of the screen to minimize working memory load.

> *In simple terms: You have a screen that shows your level, experience points, and how many word cards you have in your deck. You can also see your "stash" of letters you've collected to build new words.*

**TODO:** Animate the `XpProgressBarFill` with a smooth easing function when the student gains XP instead of snapping instantly.

### **14. The Four Attunement Channels**
Instead of simple pass/fail grading, the engine tracks the types of words a student relies on using Four Attunement Channels. Playing specific element cards increases these hidden attunement scores, ultimately shifting the student's `emergent_class` to reflect their playstyle:

- **Mind (The Oracle):** Favors logical, structural, or intellectual vocabulary.
- **Heart (The Bard):** Favors emotional, social, or empathetic vocabulary.
- **Body (The Cultivator):** Favors physical, concrete, or tangible vocabulary.
- **Action (The Templar):** Favors aggressive, fast, or dynamic verbs.

By categorizing behavior rather than punishing failure, the game celebrates the student's intrinsic linguistic style.

> *In simple terms: The game watches what kind of words you use the most. If you use smart words, you become an Oracle. If you use fast action words, you become a Templar. It's like a personality test based on your vocabulary!*

**TODO:** Add a visual badge on the HUD to display the student's current `emergent_class` icon.

### **15. Spatial Chat & Kokoro TTS Dialogues (`chat.rs`)**
To provide scaffolding (hints) and emotional connection, the game features a 3D Spatial Chat panel floating next to the pet.

- **Dynamic Taming Inputs:** By pressing `[P]` (Pet), `[F]` (Feed), or `[T]` (Attune), the student modifies the pet's internal `Focus` and `Action` states (e.g., making it Happy/Playful or Intense/Assertive).
- **Kokoro TTS Integration:** These state changes generate context-aware strings. The engine automatically sends a POST request to a local Kokoro TTS (Text-to-Speech) sidecar, creating high-quality synthesized voice dialogue that plays spatially in the 3D world.

> *In simple terms: You can pet or feed your monster! When you do, it will talk to you using a real AI voice, telling you how it feels or giving you hints for the next battle.*

**TODO:** Add error-handling in `chat.rs` so the game doesn't stutter if the Kokoro TTS server is offline.

---

## **Part VI: World-Building & Lore Integration**

### **16. The Lore Database (`lore_db.json`)**
To provide narrative structure, the world is populated by Non-Player Characters (NPCs) driven by the embedded `database.rs` JSON pipeline.

- **NpcData Structure:** Each NPC is assigned an `Archetype`, a `District`, and `PreferredClass` alignments. They act as mentors who teach specific vocabulary (`Teaches` list).
- **Time-Cycle Dialogue:** Based on the game's Day/Night cycle, NPCs draw from different dialogue pools (`Dawn`, `Day`, `Dusk`, `Night`), ensuring the world feels alive and reactive over long play sessions.

> *In simple terms: The game has other characters you can meet. They live in different districts and teach you new words. What they say changes depending on whether it is daytime or nighttime in the game!*

**TODO:** Implement a `spawn_npc` function in `render.rs` to physically render the characters based on their Archetype.

### **17. The 12 Districts Encyclopedia (Curriculum Mapping)**
The game world is divided into 12 distinct biomes (Districts). Progression through these districts is managed by the `QuestData` JSON and directly mirrors the escalation of K-12 learning standards.

- **Archetype Quests:** Each district features a unique set of Mad-Lib style quests that require specific roles to complete (e.g., needing a strong "Noun" or "Adjective").
- **NPC Chains:** Completing quests grants XP, Insight, and Evolution Points, allowing the player to progress deeper into the Districts and unlock more complex vocabulary databases.

> *In simple terms: The game world is huge! There are 12 different areas to explore, and each one gets a little bit harder, teaching you bigger and better words as you complete quests.*

**TODO:** Connect the `CurriculumManager` to the Quest completion logic so unlocking a new district automatically updates the `active_grade` in the database.

---

## **Part VII: Spatial Computing & Hardware Optimization**

### **18. Cross-Platform Build Configurations (`Cargo.toml`)**
Semantic Slime utilizes Bevy's feature flags to manage dual-target compilation without requiring heavy kernel modifications or proprietary APU architectures.

- **Desktop Orbit (`features = ["desktop"]`):** Used for rapid development, utilizing native Vulkan/Metal graphics and standard keyboard inputs.
- **Android XR (`features = ["xr"]`):** Targets standalone headsets (like Quest 3) via the Android NDK (`aarch64-linux-android`). The engine embeds Android Manifest permissions directly into the build pipeline for seamless APK generation.

> *In simple terms: The game can be built for a regular computer screen or built for a VR headset using the exact same code, just by flipping a switch when we compile it.*

**TODO:** Ensure `Cargo.toml` automatically pulls the keystore passwords from environment variables rather than hardcoding them for Android builds.

### **19. XR Rendering Pipeline (`src/main.rs`)**
The transition to 3D XR utilizes the `bevy_mod_openxr` integration. To maintain a locked 90Hz frame rate while rendering entities, the pipeline enforces strict rendering rules:

- **Desktop Rendering:** Uses high-end post-processing (HDR, Bloom, Screen-Space Ambient Occlusion) to ensure the scene looks beautiful on standard monitors.
- **XR Rendering & Hand Tracking:** Bypasses heavy post-processing for VR. It uses raw XR joint tracking (`src/hand_tracking.rs`), where the engine calculates the distance between the student's `ThumbTip` and `IndexTip`. When the distance falls below a threshold, it fires a `PinchEvent`, simulating a mouse click without needing plastic controllers.

> *In simple terms: When playing on a computer, the game uses fancy lighting to look beautiful. When playing in VR, it turns some of that off so it runs fast and doesn't make you sick, and lets you use your real hands to pinch things!*

**TODO:** Dynamically disable the Bloom and SSAO pipelines in `main.rs` if the `xr` feature flag is active to save battery life on standalone headsets.

---

## **Part VIII: AI Tooling & Generative Systems**

### **20. Kokoro TTS Sidecar Integration (`chat.rs`)**
Rather than relying on heavy zero-MQ (ZMQ) architectures or bloated Jupyter kernels, the game relies on lightweight REST APIs to communicate with generative AI models. 

- **Local Synthesized Speech:** The game generates dynamic conversational text when pets are interacted with, then sends a lightweight JSON POST payload to a local Kokoro TTS instance (`http://localhost:8200/v1/audio/speech`). 
- **Audio Playback:** The sidecar returns an MP3 byte stream which is instantly saved and played back through Bevy's `AudioPlayer`, giving characters a dynamic, reactive voice.

> *In simple terms: The game doesn't need a supercomputer to talk to you. It just sends a quick text message to a small voice program running in the background, which reads it out loud!*

**TODO:** Convert the blocking `reqwest` calls in `chat.rs` into asynchronous tasks using `bevy_tasks` to prevent the game from freezing while waiting for audio generation.

### **21. Embedded JSON Databases (`database.rs`)**
To ensure the game loads instantly without relying on external databases or complex server architectures, the entire K-12 curriculum is deeply embedded directly into the binary.

- **`include_str!` Macro:** At compile-time, Rust embeds the `word_database.json`, `synonym_database.json`, `quest_data.json`, and `lore_db.json` directly into the executable memory.
- **Hot-Reloading:** During development, Bevy's `AssetServer` watches these files. If a teacher or designer edits a JSON file, the game instantly triggers a `hot_reload_database` event, parsing the new curriculum without restarting the application.

> *In simple terms: All the words, quests, and stories are packed directly into the game file. If a teacher changes a word in the text file, the game updates immediately without even needing to close it!*

**TODO:** Add schema validation tests in `database.rs` to ensure any edits to the JSON files by teachers don't accidentally crash the game.

### **22. Procedural Meshes via FACES (No Splatting)**
Semantic Slime avoids heavy generative pipelines like 3D Gaussian Splatting (3DGS) or multi-modal tensor generation, which would crush mobile XR headsets.

- **Parametric Rendering (`src/render.rs`):** Instead of generating millions of point clouds, the engine uses mathematically perfect parametric shapes (Ico-Spheres, Cuboids, Tori).
- **The FACES Protocol:** The game alters the size, scale, and color of these basic shapes dynamically using the FACES (Focus, Action, Container, Element, System) protocol, creating infinite visual variations of "Slimes" that cost almost zero processing power.

> *In simple terms: Instead of using massive AI programs to draw complex 3D models every time you spell a word, the game just takes basic shapes like spheres and blocks, and changes their color and size based on what the word means. It looks cool and runs super fast!*

**TODO:** Add more parametric shapes (like Pyramids or Cylinders) to the `Container` definitions in `render.rs` to increase the visual variety of the pets.

---

## **Part IX: Dual-Deck Architecture (Slimes, Golems, and Robots)**

### **23. Playing the Game: The Three Archetypes**
To transition students from basic vocabulary to advanced persuasive debate, the game utilizes a "Deck Archetype" system. You can think of these as three different "classes" of pets that you collect and use:

- **Semantic Slimes (The Baseline):** Focuses on basic vocabulary and lexical meaning (Semantics).
- **Syntax Golems (The Builder Deck):** Unlocked in early districts. Focuses on the structural rules of language (Grammar). You play Nouns, Verbs, and Adjectives to physically build a Golem that solves environmental puzzles and breaks barriers.
- **Rhetoric Robots (The Debater Deck):** Unlocked in late-game districts (like the Irony Junction). Focuses on logic, persuasion, and debate (Ethos/Pathos/Logos). You use these sleek robots for "Social Combat" to deconstruct enemy logic.

> *In simple terms: You start with a basic Slime. Later, you get a blocky Golem deck to build sentences, and eventually, a sleek Robot deck to win debates and persuade people!*

### **24. Building the Game: The ECS Implementation**
To implement this in the Bevy engine, we use the `SummonClass` enum (found in `src/components.rs`) which tracks `SemanticSlime`, `GrammarGolem`, and `RhetoricRobot`.

- **State Tracking:** The `Deck` and `CharacterSheet` components now store the `active_summon_class`.
- **Procedural Rendering (`src/render.rs`):** When `spawn_avatar_visuals` runs, it reads the `pet_type`. 
  - If it's a `GrammarGolem`, it overrides the material to be highly rigid (`roughness: 0.9`) and blocky (using `Cuboid` meshes).
  - If it's a `RhetoricRobot`, it overrides the material to be perfectly sleek and reflective (`metallic: 1.0`) and uses sharp, futuristic meshes (`Torus`).
- **HUD Integration (`src/hud.rs`):** The `DeckCounterText` dynamically reads the equipped archetype and updates the UI (e.g., displaying "Golem Deck: 15 cards" instead of a generic string).

> *In simple terms: The game checks which deck you have equipped. If it's a Golem, it draws a blocky rock monster. If it's a Robot, it draws a shiny metal machine.*

**TODO:** Update the `submit_spelling_word` function in `src/letter.rs` to dynamically pull the `pet_type` from the `CharacterSheet` resource instead of defaulting to `SemanticSlime`.
