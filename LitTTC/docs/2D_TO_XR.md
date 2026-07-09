# 2D ↔ XR Feature Mapping

This document tracks how the `flat2d` proving ground maps to the XR target. It is intended to keep the two implementations aligned while the 2D build is used to iterate on game feel and validate mechanics quickly.

## Feature Flags

- `flat2d` — Enables the 2D top-down prototype and disables PBR/3D-only plugins.
- `xr` — Enables OpenXR hand tracking, holographic UI, and spatial interactions.
- `graybox` — Currently a no-op feature gate; the 2D gray-box battle systems have been merged into the standard `flat2d` battle flow.

## Core Loop Mapping

| 2D (`flat2d`) | XR (`xr`) | Notes |
|---|---|---|
| `Exploring` state with top-down overworld (`overworld.rs`) | `Playing` state with 3D world (`render.rs`) | 2D overworld is the fast iteration ground; XR loop uses the same command messages (`ScanObject`, `StartQuest`, `StartBattle`). |
| Walk to scannable objects and press `E` | Point / pinch at a world object | Both produce `GameCommand::ScanObject`. |
| Walk to NPCs and press `E` | Talk to an NPC in 3D space | Both produce `GameCommand::StartQuest`. |
| Touch wild typos to enter battle | Trigger battle proximity / gesture | Both transition to `GameState::Battling`. |
| `Constructing` — type letters from stash | VR letter crystal pinch + assembly | Both use `GameCommand::AddLetter`, `Backspace`, `SubmitSpelling`. |
| `Questing` — click cards into slots | Holographic quest board + hand pinch | Both use `GameCommand::FillQuestSlot` and `CompleteQuest`. |
| `Battling` — click cards into Plot, then `Cast Spell` | Spatial sentence constructor + cast gesture | Both use `GameCommand::AddToPlot`, `RemoveFromPlot`, `CastSentence`. |
| `Reviewing` — press Enter | Dismiss spatial review panel | Both use `GameCommand::DismissReview`. |

## Command System (Single Source of Truth)

`commands.rs` defines `GameCommand` as the message bridge. Input systems are thin translators; `handle_game_commands` is the single interpreter. Any new interaction in either dimension must be expressible as a `GameCommand` variant.

Key battle commands:
- `AddToPlot(index)` — adds a hand card to the sentence Plot.
- `RemoveFromPlot(index)` — removes a card from the Plot.
- `CastSentence` — casts the assembled sentence at the typo.
- `SetFace(name)` — changes the active Slime face (FACES).

## UI Mapping

| 2D | XR | Source |
|---|---|---|
| HUD root with stats, stash, hand cards, XP bar | Spatial UI panels (`spatial_ui.rs`) | `hud.rs` / `spatial_ui.rs` |
| Bottom hand cards with POS + synonym hints | Spatial card deck (`spatial_deck.rs`) | `hud.rs` / `spatial_deck.rs` |
| 2D battle panel with enemy word, HP bars, Plot preview, FACES buttons | Holographic battle UI (`battle.rs`) | `battle.rs` |
| 2D quest board (`QUEST_BOARD`) | Holographic quest board (`quest.rs`) | `quest.rs` |
| Companion square color = active face | 3D companion label + PetFacesState | `overworld.rs` / `companion.rs` |

## FACES / Emotion System

- `SlimeFace` (Fierce, Joyful, Calm, Angry) is the gameplay-facing enum used for damage modifiers in battle.
- 2D: buttons in the battle UI set the active face via `GameCommand::SetFace`.
- 2D: the overworld companion square changes color to match the active face.
- 3D: the companion label updates with the active face; `PetFacesState` drives the procedural pet expression.
- Damage modifiers are applied in `cast_sentence` and `play_battle_card` regardless of dimension.

## Juiciness

| Dimension | Screen Shake | Particles | SFX |
|---|---|---|---|
| 2D | `apply_screen_shake_2d` shakes `Camera2d` | `animate_burst_particles_2d` moves 2D sprites | `play_battle_sfx` plays `SOUND_ATTUNE` on spell cast |
| 3D | `apply_screen_shake` shakes `Camera` | `animate_burst_particles` moves 3D spheres | `play_battle_sfx` plays `SOUND_ATTUNE` on spell cast |

Both systems are triggered by spawning a `CriticalHitTrigger` component when `CastSentence` resolves.

## Defeat / Tutor Loop

When player health reaches 0 in either dimension, `battle::start_tutor_loop` is invoked. It removes the battle session and transitions to `GameState::Questing` with a quest targeting the failed word. The player must complete the quest to learn the word before returning to battle.

## Adding a New Feature

1. Implement the mechanic in the command handler (`commands.rs`) as a `GameCommand` variant.
2. Add the 2D input/UI layer first (`hud.rs`, `input.rs`, `overworld.rs`, `battle.rs`, `quest.rs`).
3. Add the XR input/UI layer as a parallel path under `#[cfg(feature = "xr")]` (usually `hand_tracking.rs`, `spatial_ui.rs`, `spatial_deck.rs`).
4. Keep shared state mutations in `commands.rs` or the module-specific pure functions (e.g., `fill_slot`, `cast_sentence`).
5. Run `cargo test` and `cargo check --features flat2d` before considering the phase complete.

## Verification Commands

```bash
cd "/home/joshua/LitTCG/LitTTC"
cargo test
cargo check --features flat2d
cargo check --features xr
```
