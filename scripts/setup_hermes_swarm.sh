#!/bin/bash
# Bulk-create kanban tasks for LitTCG swarm
# Run from anywhere — uses the LitTCG board

set -e

BOARD="LitTCG"
WORKDIR="/home/joshua/LitTCG/LitTCG"

echo "=== Creating Phase 0: Safety Landmines ==="

hermes kanban create "P0.1: Rename arousal→intensity in database.rs (use serde alias)" \
  --body "Rename arousal field to intensity in database.rs:66. Use #[serde(alias = \"arousal\", alias = \"A\")] so existing JSON datasets still work. File: src/database.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 100 \
  --idempotency-key "p0.1" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.2: Rename arousal→intensity in letter.rs" \
  --body "Rename in letter.rs:373 (speed calc: word_stats.arousal * 10.0 → word_stats.intensity * 10.0). File: src/letter.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 99 \
  --idempotency-key "p0.2" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.3: Rename arousal→intensity in battle.rs (semantic distance)" \
  --body "Rename in battle.rs:21 (a.arousal - b.arousal → a.intensity - b.intensity). File: src/battle.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 98 \
  --idempotency-key "p0.3" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.4: Rename arousal→intensity in battle.rs (social combat)" \
  --body "Rename in battle.rs:81 (typo_stats.arousal + typo_stats.valence → typo_stats.intensity + typo_stats.valence). File: src/battle.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 97 \
  --idempotency-key "p0.4" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.5: Rename arousal→intensity in components.rs (PetAvatar2D)" \
  --body "Rename in components.rs:414 (PetAvatar2D struct field). File: src/components.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 96 \
  --idempotency-key "p0.5" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.6: Rename arousal→intensity in render.rs (animation pulse)" \
  --body "Rename in render.rs:720 (avatar.arousal * 5.0 → avatar.intensity * 5.0). File: src/render.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 95 \
  --idempotency-key "p0.6" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.7: Verify all GDD references say Intensity not Arousal" \
  --body "Scan GDD.md for remaining 'arousal' references and replace with 'intensity'. Most done, scan remaining. File: GDD.md" \
  --workspace "dir:$WORKDIR" \
  --priority 94 \
  --idempotency-key "p0.7" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.8-0.10: Create profanity blocklist module" \
  --body "Source LDNOOBW word list. Create blocklist.rs module with pub fn is_banned(word: &str) -> bool using HashSet. Embed as const array or include JSON at compile time. File: src/blocklist.rs (new)" \
  --workspace "dir:$WORKDIR" \
  --priority 93 \
  --idempotency-key "p0.8" \
  --max-runtime 600 2>&1 || true

hermes kanban create "P0.11-0.11a: Add blocklist check + glitch entity UI masking" \
  --body "Add blocklist check in submit_spelling_word() at letter.rs:307. If banned: clear spelling silently, stay in Constructing state, no glitch entity. Also mask raw text string in UI (replace with [ANOMALY] or !#?@*) BEFORE handing to render system. Prevents screenshots of slurs floating above pets. File: src/letter.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 92 \
  --idempotency-key "p0.11" \
  --max-runtime 600 2>&1 || true

hermes kanban create "P0.12-0.13: Add blocklist integration tests" \
  --body "Add test: banned word returns no pet, no state change, no entity spawned. Add test: normal word still works after blocklist integration. File: tests/integration_tests.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 91 \
  --idempotency-key "p0.12" \
  --max-runtime 600 2>&1 || true

hermes kanban create "P0.14: Clean up 12 compiler warnings" \
  --body "Fix: count_hits unused (faces-protocol), AsyncReadExt unused import (2 locations), mutable variables (2 locations), bonus_evolution never read (quest.rs), LetterStash unused import, db unused variable, XR stubs unused (spawn_vr_hand/cleanup_vr_hand/vr_quest_interaction/vr_battle_interaction). Use #[allow(dead_code)] for XR stubs, fix or remove the rest." \
  --workspace "dir:$WORKDIR" \
  --priority 90 \
  --idempotency-key "p0.14" \
  --max-runtime 900 2>&1 || true

hermes kanban create "P0.15: Remove or allow(dead_code) Summon component" \
  --body "The Summon component in components.rs:260 is now unused after grammar_fusion fix (which now queries PetAvatar). Remove it or mark #[allow(dead_code)]. File: src/components.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 89 \
  --idempotency-key "p0.15" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P0.16-0.17: Run cargo test + cargo check (zero warnings)" \
  --body "Run cargo test — all 8 tests must pass. Run cargo check — zero warnings. Run cargo clippy --workspace -- -D warnings. All must be green before Phase 0 is complete." \
  --workspace "dir:$WORKDIR" \
  --priority 88 \
  --idempotency-key "p0.16" \
  --max-runtime 600 2>&1 || true

echo ""
echo "=== Creating Phase 1: Architecture Scaffolding ==="

hermes kanban create "P1.1-1.6: Create GameCommand enum as Bevy Events" \
  --body "Create src/commands.rs module. Define GameCommand enum (SpawnPet, StartBattle, PlayBattleCard, StartQuest, FillQuestSlot, CompleteQuest, SelectCard, DrawCard, StartCollecting, StartConstructing, PetInteraction, SaveGame, LoadGame, SkipToQuest, RetreatFromBattle, CancelQuest). Define PetAction enum. Define GameEvent enum for return values. Implement as Bevy Events (EventWriter/EventReader) NOT monolithic handle_command(&mut World). Add mod commands to main.rs and lib.rs." \
  --workspace "dir:$WORKDIR" \
  --priority 80 \
  --idempotency-key "p1.1" \
  --max-runtime 1200 2>&1 || true

hermes kanban create "P1.7-1.16: Reroute all input systems to command events" \
  --body "Reroute: submit_spelling_word→SpawnPet event, start_battle→StartBattle event, play_battle_card→PlayBattleCard event, start_quest→StartQuest event, fill_slot→FillQuestSlot event, complete_quest→CompleteQuest event, save/load→command events, input.rs handle_ui_button_interactions→send events, XR pinch handlers→send events, keyboard spelling→send events. All via EventWriter<GameCommand>." \
  --workspace "dir:$WORKDIR" \
  --priority 79 \
  --idempotency-key "p1.7" \
  --max-runtime 1800 2>&1 || true

hermes kanban create "P1.17-1.20: Add integration tests for command system" \
  --body "Test: SpawnPet(fire)→verify PetSpawned event. Test: StartBattle(None)→verify BattleStarted event. Test: StartQuest(Barnaby)→FillQuestSlot(0,brave)→CompleteQuest→verify XP gained. Test: banned word via command→verify Error event, no entity spawned." \
  --workspace "dir:$WORKDIR" \
  --priority 78 \
  --idempotency-key "p1.17" \
  --max-runtime 900 2>&1 || true

hermes kanban create "P1.21: Create scripts/check_arch.py architecture validator" \
  --body "Enforce: (1) main.rs has no game logic, (2) render.rs no database imports, (3) database.rs no render imports, (4) no web_sys outside bridge, (5) all state transitions via NextState, (6) no bare unwrap() in production code, (7) public functions documented or #[allow(dead_code)]. File: scripts/check_arch.py" \
  --workspace "dir:$WORKDIR" \
  --priority 77 \
  --idempotency-key "p1.21" \
  --max-runtime 600 2>&1 || true

hermes kanban create "P1.22-1.25: Set up Windsurf hooks + task.md + clippy" \
  --body "Create .windsurf/hooks/on-session-start.sh (run cargo test). Create .windsurf/hooks/post-edit-lint.sh (run cargo clippy --workspace -- -D warnings). Create .windsurf/hooks/on-stop.sh (run cargo test). Initialize task.md with 4-step Integration Roadmap from TECHNICAL_MANUAL.md." \
  --workspace "dir:$WORKDIR" \
  --priority 76 \
  --idempotency-key "p1.22" \
  --max-runtime 600 2>&1 || true

echo ""
echo "=== Creating Phase 2: Bridge Isolation & Build ==="

hermes kanban create "P2.9d: Quick fix — feature-gate TTS module for WASM" \
  --body "Feature-gate the entire TTS module with #[cfg(not(target_arch = \"wasm32\"))] since reqwest::blocking panics in WASM. Kokoro TTS is already disabled in WASM per GDD. Zero-cost fix that unblocks WASM builds immediately. File: src/chat.rs" \
  --workspace "dir:$WORKDIR" \
  --priority 70 \
  --idempotency-key "p2.9d" \
  --max-runtime 300 2>&1 || true

hermes kanban create "P2.1-2.9: Directory restructure (core/bridge split)" \
  --body "Create src/core/ and src/bridge/ directories. Move all game logic into src/core/. Move TTS code into src/bridge/tts.rs. Create mod.rs re-exports. Update main.rs and lib.rs module declarations. Update Cargo.toml — reqwest only in bridge feature. File: src/core/, src/bridge/" \
  --workspace "dir:$WORKDIR" \
  --priority 69 \
  --idempotency-key "p2.1" \
  --max-runtime 1800 2>&1 || true

hermes kanban create "P2.9a-2.9c: Async JSON loading via AssetServer" \
  --body "Transition from include_str! + synchronous serde_json::from_str() to async loading via Bevy AssetServer in GameState::Loading. Add LoadingScreen UI with spinner. Only transition to Menu after all 5 databases loaded. Prevents black screen freeze on Chromebook WASM." \
  --workspace "dir:$WORKDIR" \
  --priority 68 \
  --idempotency-key "p2.9a" \
  --max-runtime 1200 2>&1 || true

hermes kanban create "P2.10-2.13: Verify isolation (cargo check all targets)" \
  --body "cargo check (core without browser deps), cargo check --target wasm32-unknown-unknown, cargo ndk -t aarch64-linux-android check, run scripts/check_arch.py." \
  --workspace "dir:$WORKDIR" \
  --priority 67 \
  --idempotency-key "p2.10" \
  --max-runtime 900 2>&1 || true

hermes kanban create "P2.14-2.21: Dual WASM build (WebGPU + WebGL2)" \
  --body "Create build_wasm.sh: build WebGL2 binary (trunk build --release), build WebGPU binary, wasm-opt -Oz both, output to dist/. Update index.html with navigator.gpu detection. Test in browser." \
  --workspace "dir:$WORKDIR" \
  --priority 66 \
  --idempotency-key "p2.14" \
  --max-runtime 1200 2>&1 || true

hermes kanban create "P2.22-2.25: PWA manifest + service worker + offline" \
  --body "Create manifest.json (PWA manifest). Create service-worker.js (cache WASM + assets). Register in index.html. Test offline mode." \
  --workspace "dir:$WORKDIR" \
  --priority 65 \
  --idempotency-key "p2.22" \
  --max-runtime 600 2>&1 || true

echo ""
echo "=== Creating Phase 3: Visual Polish ==="

hermes kanban create "P3.1-3.8: Element-specific materials" \
  --body "Fire: emissive orange-red, roughness 0.15, flicker. Water: semi-transparent blue, alpha 0.7, roughness 0.05. Earth: brown, roughness 0.9. Air: near-transparent white, alpha 0.3. Shadow: dark purple, metallic 1.0, Fresnel rim. Light: bright yellow-white, high emissive. Normal: neutral gray. Apply in render.rs:spawn_avatar_visuals()." \
  --workspace "dir:$WORKDIR" \
  --priority 60 \
  --idempotency-key "p3.1" \
  --max-runtime 1200 2>&1 || true

hermes kanban create "P3.9-3.15: Fade transitions between game states" \
  --body "Create FadeOverlay resource. Implement transition_to_state() with fade-out/fade-in. Replace all direct next_state.set() calls. 0.3s out + 0.3s in = 0.6s total." \
  --workspace "dir:$WORKDIR" \
  --priority 59 \
  --idempotency-key "p3.9" \
  --max-runtime 900 2>&1 || true

hermes kanban create "P3.16-3.23: Quality presets (Low/Medium/High/Ultra)" \
  --body "Create QualityPreset enum. Implement apply_quality() for MSAA, shadows, particles, bloom. Auto-detect GPU on startup. Add override in settings menu." \
  --workspace "dir:$WORKDIR" \
  --priority 58 \
  --idempotency-key "p3.16" \
  --max-runtime 900 2>&1 || true

hermes kanban create "P3.24-3.27: Data-driven pet prefabs" \
  --body "Create assets/pet_prefabs.ron with material/mesh definitions per element+class combo. Load in render.rs instead of hardcoded materials. Fallback to procedural if prefab missing." \
  --workspace "dir:$WORKDIR" \
  --priority 57 \
  --idempotency-key "p3.24" \
  --max-runtime 900 2>&1 || true

echo ""
echo "=== Creating Phase 4: Core Game Features (Pokemon Moment) ==="

hermes kanban create "P4.1-4.11: Pet card reveal animation" \
  --body "Create PetCard component (word, flipped, flip_timer, rarity). Define Rarity enum (Common→Mythic). Implement calculate_rarity(). Modify submit_spelling_word() to spawn face-down card. Card flip animation (0.5s Y rotation). On flip: spawn 3D pet with burst particles, screen shake, sound. Card shows word, element border, role icon, stats, rarity glow. This is the emotional hook — spend extra time on easing curves and particles." \
  --workspace "dir:$WORKDIR" \
  --priority 50 \
  --idempotency-key "p4.1" \
  --max-runtime 2400 2>&1 || true

hermes kanban create "P4.12-4.21: Pet collection system" \
  --body "Create PetCollection resource (replaces SpellBook). Define PetEntry struct. Add to save.rs. Create Collection screen with grid of cards, sortable. Pet detail view with stats, FACES state, etymology. Set as Companion / Add to Roster buttons." \
  --workspace "dir:$WORKDIR" \
  --priority 49 \
  --idempotency-key "p4.12" \
  --max-runtime 1800 2>&1 || true

hermes kanban create "P4.22-4.30: Roster selection + battle integration" \
  --body "Create Roster resource (max 6 pets). Roster selection UI. Reroute start_battle() to use roster. Roster display in battle UI. Swap mechanic. Tests for collection, roster, rarity." \
  --workspace "dir:$WORKDIR" \
  --priority 48 \
  --idempotency-key "p4.22" \
  --max-runtime 1800 2>&1 || true

echo ""
echo "=== Phase 5-10 created as summary tasks (decompose later) ==="

hermes kanban create "P5: P1 Features (evolution, RPS, combat learning, quests, ASL, letter spawning)" \
  --body "50 tasks: Visual evolution stages, RPS class modifier, active combat learning with synonym/antonym challenges, combat feedback floaters, color-coded quest slots, curriculum-biased letter spawning (bumped from P2), full ASL fingerspelling A-Z. See MASTER_TASK_LIST.md Phase 5 for details." \
  --workspace "dir:$WORKDIR" \
  --priority 40 \
  --idempotency-key "p5" \
  --max-runtime 3600 2>&1 || true

hermes kanban create "P6: P2 Features (companion follow, nuisance letters, dream layer)" \
  --body "19 tasks: Companion follow system, nuisance letters with chase AI, pet dream layer with poetry. See MASTER_TASK_LIST.md Phase 6 for details." \
  --workspace "dir:$WORKDIR" \
  --priority 35 \
  --idempotency-key "p6" \
  --max-runtime 3600 2>&1 || true

hermes kanban create "P7: Demo preparation and ship to itch.io" \
  --body "21 tasks: Curate 10 demo words, demo filtering, paywall prompts, itch.io packaging (cover image, screenshots, GIF), browser testing, full playthrough test. See MASTER_TASK_LIST.md Phase 7." \
  --workspace "dir:$WORKDIR" \
  --priority 30 \
  --idempotency-key "p7" \
  --max-runtime 3600 2>&1 || true

hermes kanban create "P8: Full game unlock + Google Aura XR + expansions + parent dashboard" \
  --body "29 tasks: Remove 10-word limit, license check, all 9582 words, Google Aura XR build, expansion packs (SAT, Science, Spanish-English), parent dashboard. See MASTER_TASK_LIST.md Phase 8." \
  --workspace "dir:$WORKDIR" \
  --priority 25 \
  --idempotency-key "p8" \
  --max-runtime 3600 2>&1 || true

hermes kanban create "P9: SpawnForge collaboration (shared crates, Tier 2 pet generation)" \
  --body "20 tasks: Extract faces-protocol crate, psycholinguistic-data crate, Janus Pro + Trellis pregeneration pipeline, Pet Studio design. See MASTER_TASK_LIST.md Phase 9." \
  --workspace "dir:$WORKDIR" \
  --priority 20 \
  --idempotency-key "p9" \
  --max-runtime 3600 2>&1 || true

hermes kanban create "P10: Future polish (WGSL shaders, GPU particles, multiplayer, voice)" \
  --body "17 tasks: Custom WGSL shaders, GPU compute particles, SSAO, visual quest builder, multiplayer, voice recognition, adaptive difficulty, audio system. See MASTER_TASK_LIST.md Phase 10." \
  --workspace "dir:$WORKDIR" \
  --priority 15 \
  --idempotency-key "p10" \
  --max-runtime 3600 2>&1 || true

echo ""
echo "=== Setting up task dependencies ==="
# Phase 1 depends on Phase 0
# Phase 2 depends on Phase 1
# etc. — we'll let the daemon handle dispatch order by priority

echo ""
echo "=== Done! Task list: ==="
hermes kanban list 2>&1

echo ""
echo "=== Next: Start the swarm ==="
echo "Run: hermes kanban swarm --worker 'default:Phase 0 Safety Worker' --worker 'default:Phase 1 Architecture Worker' --worker 'default:Phase 2 Build Worker' --verifier 'default:Verifier' --synthesizer 'default:Synthesizer' 'Ship Communication Class demo with all safety, architecture, and build tasks complete'"
