# LitTCG Development Roadmap

## Current Status: MVP Refactor — Word Slimes Pivot Complete

**Last verified:** January 2025 — 33/33 integration tests pass, 0 compiler warnings, Word Slimes MVP refactor complete.

---

## Phase 1-8: Engine Core — ✅ DONE

- [x] 22 source files compile cleanly
- [x] 5 embedded JSON databases (~3.3MB) loaded and parsed
- [x] Full GameState machine (Loading → MainMenu → Collecting → Constructing → Playing → Questing → Battling → Reviewing → Paywall)
- [x] Pet spawning pipeline (word → etymology → element/role → stats → FACES → 3D mesh)
- [x] Battle system with semantic distance combat
- [x] Quest system with Mad-Lib templates
- [x] Chat system with FACES dialogue and Kokoro TTS
- [x] Save/load system (local JSON, COPPA compliant)
- [x] HUD, Main Menu, Tutorial, Paywall UI
- [x] XR scaffolding (hand tracking, spatial UI, spatial deck)
- [x] Procedural rendering (FACES morphs, particles, screen shake)
- [x] 33 integration tests covering database, battle, quest, save, chat, curriculum

---

## Phase 9: Word Slimes MVP Refactor — ✅ DONE

**January 2025** — Refactored codebase for Web/WASM MVP targeting ages 7-12 homeschool market.

- [x] Deprecated GrammarGolem and RhetoricRobot classes (components.rs)
- [x] Updated SummonClass enum to only include SemanticSlime
- [x] Added Grimoire resource as physical inventory/deck representation
- [x] Removed RPS class modifiers from battle system
- [x] Implemented Wand Duel combat (1v1 Synonym/Antonym based on semantic distance)
- [x] Updated semantic_distance() logic for new combat rules
- [x] Removed Game Over screens from quest.rs and battle.rs
- [x] Implemented Tutor Loop state transition on player health = 0
- [x] Added NPC routing logic based on failed word (route_to_tutor_npc)
- [x] Integrated targeted Mad-Lib quest generation for tutoring (start_tutor_loop)
- [x] Removed pure RNG A-Z letter spawning from letter.rs
- [x] Implemented curriculum-biased letter spawning using GradeLevel
- [x] Added database query for grade-appropriate words
- [x] Updated integration tests for new combat mechanics
- [x] Updated integration tests for Tutor Loop failure routing
- [x] Fixed all compilation errors and warnings (cargo check passes with 0 warnings)
- [x] All 33 integration tests passing

---

## Phase 10: Product Surface (UI & Flow) — ⬜ PENDING

- [ ] Verify full game loop: menu → collect letters → spell word → pet spawns → quest → battle → review
- [ ] Polish visual letter collection and spelling feedback
- [ ] Add smooth XP bar animation (currently snaps)
- [ ] Add emergent class badge to HUD
- [ ] Add visual feedback for critical hits in battle (screen shake, particles)
- [ ] Show enemy psychometric stats on health bar for strategic deduction
- [ ] Polish pet renderer (ensure glTF fallback works)
- [ ] Tune swipe threshold to prevent accidental micro-drags
- [ ] Add error handling in chat.rs for offline Kokoro TTS

---

## Phase 11: Web Demo (WASM) — ⬜ PENDING

- [ ] Verify `trunk serve` runs without crashes in browser
- [ ] Implement demo word limit (10 words) in paywall.rs
- [ ] Disable save system in demo mode
- [ ] Add "Get Full Version" prompt after significant play
- [ ] Prepare itch.io page assets and copy
- [ ] Test on multiple browsers (Chrome, Firefox, Safari)

---

## Phase 12: Revenue Infrastructure — ⬜ FUTURE

- [ ] Scaffold Parent Dashboard (separate web app to view `save.json`)
- [ ] Design dashboard UI showing emergent class, mastery, grade level
- [ ] Add curriculum export for school district integration

---

## Phase 13: Android XR — ⬜ FUTURE

- [ ] Verify `cargo ndk` cross-compilation
- [ ] Test XR mode on HTC VIVE XR Elite
- [ ] Calibrate pinch-to-select distance threshold
- [ ] Disable Bloom/SSAO for XR battery life
- [ ] Test hand tracking ASL fingerspelling
- [ ] Package as APK

---

## Shipping Checklist

- [x] Zero compiler warnings
- [x] All integration tests passing
- [ ] Main menu loads on launch
- [ ] Tutorial plays for first-time users
- [ ] Player can spell words and see visual feedback
- [ ] HUD displays required information
- [ ] WASM build runs in browser without crashing
- [ ] Demo limitations apply correctly
- [ ] Itch.io page copy and assets ready
