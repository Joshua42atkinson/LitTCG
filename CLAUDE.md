# LitTCG — Claude Code Instructions

## Project
Bevy 0.18.1 XR EdTech game where kids spell words to summon pets and battle typos.
Rust code lives in `LitTCG/`.

## Build Commands
```bash
cd LitTCG && cargo test          # Run tests (must pass)
cd LitTCG && cargo check          # Compile check
cd LitTCG && cargo clippy -- -D warnings  # Lint
```

## Architecture Rules
- NEVER add game logic to `main.rs` — it only wires systems
- Keep rendering, state transitions, database queries, and input parsing in their modules
- Use Bevy Events (EventWriter/EventReader) for command flow, NOT &mut World
- No bare unwrap() in production code
- No web_sys outside bridge module (when created)

## Current State
- 8 integration tests passing
- 12 compiler warnings to clean
- Phase 0 safety tasks in progress (see task.md)
- Full task list in MASTER_TASK_LIST.md (285 tasks)

## File Map
- `src/main.rs` — Desktop entry, system registration
- `src/lib.rs` — Library + Android entry
- `src/components.rs` — ECS components
- `src/database.rs` — JSON curriculum loader
- `src/letter.rs` — Letter crystals, word construction, pet spawning
- `src/battle.rs` — Semantic distance combat
- `src/quest.rs` — Mad-Lib quests, NPC dialogue
- `src/render.rs` — Procedural 3D pet meshes
- `src/input.rs` — Input handling
- `src/chat.rs` — Pet bonding, TTS (feature-gated)
- `src/save.rs` — JSON save/load
- `src/hand_tracking.rs` — XR hand tracking, ASL stub

## Safety Priorities
- Rename "arousal" → "intensity" everywhere (use serde alias)
- Profanity blocklist in spelling validation
- No inappropriate text visible in UI under any circumstances
