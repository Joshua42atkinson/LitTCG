# Contributing Guidelines

Thank you for contributing to **LitTCG** (*Literary Trading Card Game*)! We value robust code, thoughtful pedagogical design, and maintaining a high standard for our target demographic (homeschooling families).

## Quick Start for Fresh Setup

### Prerequisites
- **Rust**: Stable toolchain (install via rustup)
- **Bevy**: 0.18.1 (defined in Cargo.toml)
- **Android NDK**: 30.0.14904198 (for Android builds)
- **ANDROID_HOME**: `/home/joshua/Android/Sdk`
- **NDK_HOME**: `/home/joshua/Android/Sdk/ndk/30.0.14904198`

### Build Commands
```bash
# Run tests (always do this after changes)
cd LitTTC
cargo test

# Desktop build check
cargo check --features desktop

# Desktop run
cargo run

# WASM build (serves at localhost:8080)
trunk serve

# Android cross-compile check
ANDROID_HOME="/home/joshua/Android/Sdk" \
NDK_HOME="/home/joshua/Android/Sdk/ndk/30.0.14904198" \
cargo ndk -t aarch64-linux-android check
```

### Project Structure
```
LitTCG/
├── src/
│   ├── main.rs              # Desktop entry point
│   ├── lib.rs               # Android NDK entry point
│   ├── components.rs        # ECS components: Cards, Pets, GameState
│   ├── database.rs         # Embedded JSON curriculum loader
│   ├── deck.rs              # Card deck management
│   ├── input.rs             # Swipe gesture & keyboard input
│   ├── letter.rs            # 3D letter crystal spawning
│   ├── quest.rs             # Mad-Lib quest engine
│   ├── battle.rs            # Synonym/antonym card combat
│   ├── chat.rs              # FACES pet dialogue
│   ├── render.rs            # Procedural 3D pet meshes
│   ├── time_cycle.rs        # Day/Night cycle
│   ├── hand_tracking.rs     # XR hand joint tracking
│   ├── save.rs              # JSON save/load persistence
│   ├── spatial_ui.rs        # Floating holographic UI
│   ├── spatial_deck.rs      # 3D spatial card deck
│   ├── altar.rs             # Pet altar/summoning
│   ├── dialogue_ui.rs       # NPC dialogue UI
│   ├── hud.rs               # 2D screen space UI
│   ├── menu.rs              # Main menu state
│   ├── paywall.rs           # Demo limitations
│   ├── tutorial.rs          # Player onboarding
│   ├── settings.rs          # Game settings
│   ├── difficulty.rs        # Difficulty selection
│   ├── pet_collection.rs    # Pet gallery
│   ├── pet_reveal.rs        # Pet reveal animation
│   ├── diagnostics.rs       # Frame diagnostics
│   ├── platform_paths.rs    # Cross-platform paths
│   ├── performance.rs       # Performance monitoring
│   ├── blocklist.rs         # Content filtering
│   ├── commands.rs          # Game command bridge
│   └── asset_catalog.rs     # Asset constants
├── assets/                  # Embedded JSON curriculum databases
├── tests/
│   └── integration_tests.rs # 33 integration tests
├── Cargo.toml               # Build config with feature flags
├── index.html               # WASM trunk entry point
└── AndroidManifest.xml      # Android XR manifest
```

### Current Project Status
- **Codebase Size**: 8,564 lines of Rust code
- **Bevy Version**: 0.18.1
- **Integration Tests**: 32/33 passing (1 failing: `test_valid_spelling_transitions_through_reveal_pet`)
- **Desktop Build**: ❌ Window closes immediately after MainMenu (investigating)
- **WASM Build**: ⏸️ Paused (black screen issue - needs browser console debugging)
- **Demo Restrictions**: ✅ Implemented (10-word limit, save disabled)

### Known Issues
1. **Desktop Window Closure**: Game loads successfully but window closes immediately after reaching MainMenu. Exit code: 0 (normal exit, not crash). Likely window/display configuration issue.
2. **WASM Black Screen**: Web demo shows black screen when served via trunk. Requires browser console debugging.
3. **Integration Test Failure**: `test_valid_spelling_transitions_through_reveal_pet` expects `GameState::Playing` but gets `GameState::RevealingPet`.

### Feature Flags
- `desktop`: Desktop build with 3D rendering + orbit camera
- `flat2d`: 2D-only rendering mode
- `xr`: XR/OpenXR build for VR headsets
- `wasm`: WASM-specific features (currently unused)

### Key Dependencies
- **faces-protocol**: FACES emotional protocol (path dep at `../crates/faces-protocol`)
- **bevy_mod_openxr**: Optional XR runtime (feature-gated behind `xr`)
- **bevy_panorbit_camera**: Optional desktop orbit camera (feature-gated behind `desktop`)

## 1. The Golden Rule
**"The learning is the playing. The playing is the learning."**
Do not build educational overlays. Build game mechanics that *require* English mastery to execute. Do not test memory; test application.

## 2. Rust & Bevy Standards
- **Zero Warnings**: The master branch must always compile with 0 warnings on `cargo check`. If you deprecate a feature intentionally or create scaffolding for future PRs, use `#![allow(dead_code)]` at the top of the file explicitly, but remove it before a 1.0 release.
- **Component Isolation**: Do not place business logic in `main.rs`. Keep logic inside dedicated modules (e.g., `battle.rs`, `quest.rs`).
- **Entity Despawning**: Bevy 0.14+ deprecated `despawn_recursive()` on EntityCommands in favor of `.despawn()`. Ensure you use the updated APIs.
- **Testing**: Any system modifying `SpellBook`, `CharacterSheet`, or `PetStats` must include an integration test in `tests/integration_tests.rs`.
- **ECS Query Conflicts**: When multiple queries access the same component (e.g., `BackgroundColor`), use `ParamSet` to avoid B0001 errors.

## 3. Modifying the Curriculum
Curriculum changes should be made carefully.
- The `assets/` folder contains embedded JSON databases.
- When modifying `words.json`, ensure the psychological vectors (`valence`, `arousal`, `dominance`, `concreteness`) are correctly normalized between 0.0 and 1.0, as the rendering engine relies on these limits to scale procedural meshes and stats.
- Keep the `etymology.json` aligned with standard Latin and Greek roots.

## 4. Agentic AI Workflows
If you are developing this via agentic AI:
- Use `task.md` checklists to track progress and prevent contextual drift.
- Write tests before completing implementation phases.
- Run `cargo test` after any structural or functional changes.
- Enable Bevy debug feature for better error messages: add `"debug"` to Bevy features in Cargo.toml.

## 5. Development Rules
- **Continuous Validation**: Run `cargo test` after any structural or functional changes.
- **Scaffold Progression**: Maintain clear updates in `task.md` using standard checklists (`[ ]`, `[/]`, `[x]`).
- **Architectural Isolation**: Keep rendering, state transition, database queries, and input parsing isolated in their respective modules.
- **Warning Prevention**: Avoid unused imports or dangling dead code. Mark development test helpers with `#[allow(dead_code)]` if necessary.

## 6. Workspace Rules (AGENTS.md)
- **Continuous Validation**: Run `cargo test` after changes to ensure compilation and logical correctness.
- **Scaffold Progression**: Maintain clear updates in `task.md` using standard checklists.
- **Architectural Isolation**: Keep rendering, state transition, database queries, and input parsing isolated.
- **No Game Logic in main.rs**: `main.rs` should only wire systems together.
- **Warning Prevention**: Avoid unused imports or dangling dead code.
- **Autonomous Chaining**: When executing long-horizon tasks, use the `schedule` tool with `DurationSeconds=1` to continuously trigger the next turn until all tasks in `task.md` are completed.
