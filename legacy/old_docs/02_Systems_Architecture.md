# 2. Systems Architecture

Communication Class runs on **Bevy 0.18.1**, utilizing a strict Entity Component System (ECS).

## Core Modules (`src/`)
- `main.rs`: Entry point, orchestrator, and plugin registration.
- `components.rs`: The centralized repository for ECS schemas (`GameState`, `CharacterSheet`, `SummonClass`).
- `database.rs`: Deserializes massive JSON curricula into Bevy Resources.
- `render.rs`: Procedural 3D mesh generation using the `faces-protocol` crate.
- `battle.rs`: The semantic combat engine.
- `quest.rs`: Mad-Lib style quests and the CurriculumManager.
- `chat.rs`: 3D spatial UI and Kokoro TTS integration.
- `hand_tracking.rs`: XR hand joint tracking and gesture heuristics.

## State Management
The application flows through `bevy::state::app::StatesPlugin` using `GameState`:
`Loading` ➔ `MainMenu` ➔ `Collecting` (Resources) ➔ `Constructing` (Spelling) ➔ `Playing` (Town) ➔ `Questing` or `Battling`.

## The Pipeline: Word to Pet
1. **Input**: `letter.rs` detects the spelled word.
2. **Validation**: Word is checked against `GameDatabase.words`.
3. **Etymology**: Roots define the `Element` (e.g., "pyr" = Fire). Suffixes define the `Role`.
4. **Psycholinguistics**: Valence, Arousal, Concreteness map to combat stats.
5. **FACES Protocol**: `faces-protocol` parses definitions to apply `Aura`, `Focus`, and `Action` visual states.
6. **Rendering**: `render.rs` translates all components into a procedural 3D model.
