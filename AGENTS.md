# LitTCG Workspace

## Project Overview
This workspace contains **LitTCG** (*Literary Trading Card Game*), a Bevy 0.18.1 XR EdTech game engine where kids spell words to summon pets and battle typos. The core game code lives in `LitTTC/`.

## Workspace Layout
```
/home/joshua/LitTTC/
  AGENTS.md                    # This file — workspace rules
  README.md                    # Project overview and build instructions
  GDD.md                       # ★ Single source of truth — Game Design Document
  TECHNICAL_MANUAL.md          # SpawnForge collaboration analysis & integration patterns
  ARCHITECTURE.md              # ECS architecture, module list, data flow
  ROADMAP.md                   # Development phases, shipping checklist, status
  CONTRIBUTING.md              # Contribution and pedagogical guidelines
  itch_page.md                 # Marketing copy for storefront
  legacy/                      # Archived design and audit documents
    Design_Doc.md              # Previous design doc (superseded by GDD.md)
    codebase_audit.md          # Old Workflow codebase audit (porting complete)
    old_docs/                  # Superseded docs (MASTER_DESIGN_DOCUMENT, FACES research specs, etc.)
  LitTTC/                      # ★ Main game engine (Rust/Bevy)
    src/
      main.rs                  # Desktop entry point, app scaffold, system registration
      lib.rs                   # Library crate + Android NDK entry point
      components.rs            # ECS components: Cards, Pets, Attunement, GameState
      database.rs              # Embedded JSON curriculum loader (5 databases)
      deck.rs                  # Card deck shuffling, draw, hand management
      input.rs                 # Swipe gesture & keyboard input decoding
      letter.rs                # 3D letter crystal spawning & word constructor
      quest.rs                 # Mad-Lib quest engine + CurriculumManager
      battle.rs                # Synonym/antonym card combat system
      chat.rs                  # FACES pet dialogue, taming, Kokoro TTS
      render.rs                # Procedural 3D pet meshes, FACES morphs, glTF bones
      time_cycle.rs            # Day/Night cycle and timing states
      hand_tracking.rs         # XR hand joint tracking + ASL fingerspelling + pinch
      save.rs                  # JSON save/load persistence
      spatial_ui.rs            # Floating holographic UI panels in 3D space
      spatial_deck.rs          # 3D spatial card deck for XR mode
      altar.rs                 # Pet altar/summoning system
      dialogue_ui.rs           # NPC dialogue UI panels
      hud.rs                   # 2D screen space UI
      menu.rs                  # Main menu state
      paywall.rs               # Demo limitations
      tutorial.rs              # Player onboarding
    tests/
      integration_tests.rs     # 8 integration tests (all passing)
    assets/                    # Embedded JSON curriculum databases
    Cargo.toml                 # Build config with desktop/xr/wasm feature flags
    index.html                 # WASM trunk entry point
    AndroidManifest.xml        # Android XR manifest
  crates/
    faces-protocol/            # FACES emotional protocol crate
      docs/                    # FACES research, specs, and workflows
```

## Build Commands
```bash
# Run tests (always do this after changes)
cd "/home/joshua/LitTTC/LitTTC"
cargo test

# Desktop build check
cargo check --features desktop

# WASM build (serves at localhost:8080)
trunk serve

# Android cross-compile check
ANDROID_HOME="/home/joshua/Android/Sdk" \
NDK_HOME="/home/joshua/Android/Sdk/ndk/30.0.14904198" \
cargo ndk -t aarch64-linux-android check
```

## Environment
- **ANDROID_HOME**: `/home/joshua/Android/Sdk`
- **OS**: Linux
- **Shell**: bash
- **Rust**: stable toolchain
- **Bevy**: 0.18.1

## Key Dependencies
- **faces-protocol**: FACES emotional protocol (path dep at `../crates/faces-protocol`)
- **bevy_mod_openxr**: Optional XR runtime (feature-gated behind `xr`)
- **bevy_panorbit_camera**: Optional desktop orbit camera (feature-gated behind `desktop`)

## Development Rules

### Continuous Validation
- Run `cargo test` after any structural or functional changes to ensure compilation and logical correctness.
- All 8 integration tests must pass before considering a phase complete.

### Scaffold Progression
- Maintain clear updates in `task.md` using standard checklists (`[ ]`, `[/]`, `[x]`).
- Never leave a checklist item empty if the corresponding code has been checked in.

### Architectural Isolation
- Keep rendering, state transition, database queries, and input parsing isolated in their respective modules.
- Do not add game logic to `main.rs` — it should only wire systems together.

### Warning Prevention
- Avoid unused imports or dangling dead code.
- Mark development test helpers or unused API fields with `#[allow(dead_code)]` if necessary.

### Autonomous Chaining (Diapers Mode)
- When executing long-horizon tasks, invoke the `schedule` tool at the end of each turn with `DurationSeconds=1` to continuously trigger the next turn until all tasks in `task.md` are completed and verified.
- See `.agents/skills/diapers/SKILL.md` for the full protocol.
