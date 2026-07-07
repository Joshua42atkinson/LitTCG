# Communication Class — Autonomous Task Tracker

> Single source of truth for Hermes Kanban + Windsurf Diapers Mode.
> Synced with MASTER_TASK_LIST.md (285 tasks across 10 phases).
> Kanban board: `communication-class` (37 grouped tasks)

## Integration Roadmap (4-Step Sprint)

### Step 1: Defuse the Landmines (Phase 0)
- [x] Rename arousal→intensity (7 code locations, use serde alias)
- [ ] Create profanity blocklist module + integration in letter.rs
- [ ] Glitch entity UI masking (prevent slur screenshots)
- [ ] Clean 12 compiler warnings
- [ ] Remove unused Summon component
- [ ] cargo test (8/8 pass) + cargo check (0 warnings) + clippy clean

### Step 2: Command-Driven Architecture (Phase 1)
- [ ] Create GameCommand enum as Bevy Events (EventWriter/EventReader)
- [ ] Reroute all input systems to command events
- [ ] Integration tests for command flow
- [ ] Architecture validator script (check_arch.py)
- [ ] Windsurf hooks (clippy, test, session start/stop)

### Step 3: Bridge Isolation & WASM (Phase 2)
- [ ] Quick fix: feature-gate TTS module for WASM
- [ ] Directory split: src/core/ + src/bridge/
- [ ] Async JSON loading via AssetServer
- [ ] Dual WASM build (WebGPU + WebGL2 + wasm-opt)
- [ ] PWA manifest + service worker + offline

### Step 4: Visual Polish + Pokémon Moment (Phase 3-4)
- [ ] Element-specific materials (fire/water/earth/air/shadow/light/normal)
- [ ] Fade transitions between game states
- [ ] Quality presets (Low/Medium/High/Ultra auto-detect)
- [ ] Data-driven pet prefabs (.ron)
- [ ] Pet card reveal animation (THE emotional hook)
- [ ] Pet collection system + roster selection
- [ ] Rarity tiers (Common→Mythic)

## Post-Sprint Phases (Decompose Later)
- [ ] Phase 5: P1 Features (50 tasks — evolution, RPS, combat, ASL, letter spawning)
- [ ] Phase 6: P2 Features (19 tasks — companion, nuisance letters, dream layer)
- [ ] Phase 7: Demo Ship (21 tasks — 10 words, itch.io, testing)
- [ ] Phase 8: Full Game (29 tasks — unlock, Aura XR, expansions, dashboard)
- [ ] Phase 9: Collaboration (20 tasks — shared crates, Tier 2 pet gen, Pet Studio)
- [ ] Phase 10: Future (17 tasks — shaders, GPU particles, multiplayer, voice)

## Hermes Kanban Commands
```bash
# Check board status
hermes kanban list

# Watch task progress live
hermes kanban watch

# Start daemon (auto-dispatches tasks)
hermes kanban daemon --interval 60 --verbose

# Start swarm (parallel workers)
hermes kanban swarm \
  --worker 'default:Safety Worker' \
  --worker 'default:Architecture Worker' \
  --worker 'default:Build Worker' \
  --verifier 'default:Verifier' \
  --synthesizer 'default:Synthesizer' \
  'Ship Communication Class demo with all safety, architecture, build, and visual polish tasks complete'

# Check daemon stats
hermes kanban stats

# View specific task
hermes kanban show <task_id>
```
