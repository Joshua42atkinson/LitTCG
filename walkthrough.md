# LitTCG Demo Sprint Walkthrough

## What Was Completed

This document summarizes the Phase 3 Demo Sprint work done in the latest session.

### Core Gameplay

- **P3.2 Pet Card Reveal Animation**
  - Face-down card spawns after spelling a valid word.
  - Card flips with particles and sound in both 3D and `flat2d` modes.
  - Pet bursts from the card at the end of the animation.
  - Floating label shows rarity, element, and name above the pet.
  - State transitions: `Constructing` → `RevealingPet` → `Playing`.

- **P3.4 Async JSON Loading**
  - Databases load asynchronously via Bevy's asset system.
  - Loading screen displays "Summoning vocabulary..." with a progress bar.
  - Registered in both desktop (`main.rs`) and Android (`lib.rs`) entry points.

- **P3.5 Demo Limit + Paywall**
  - `DemoSettings` resource tracks `words_used` against `max_words` (default 10).
  - Paywall UI shows words used and a "Unlock Full Game — $9.99" CTA.
  - Purchase button opens `https://polar.sh/your-product` via `web_sys` on WASM.

### Visual + Mobile Polish

- **P3.1 Element-Specific Materials**
  - Added `Element::material()` returning PBR presets with emissive/metallic/roughness per element.
  - Reveal card front and spawned pet use the element-specific material.

- **P3.3 Pet Collection Screen**
  - `SpellBookEntry` extended with `element`, `role`, `stats`, and `companion`.
  - Collection screen has a sortable grid (Word / Element / Mastery).
  - Detail panel shows stats and a Set Companion button.

- **P3.6 Touch-First UI**
  - Main menu buttons enlarged to 280x70 with more spacing.
  - Letter crystals enlarged to 0.5 with 1.8 pickup distance.
  - XR holographic letters and submit button scaled up.
  - HUD fonts, action buttons, hand cards, and progress bar enlarged.

- **P3.7 Settings / Difficulty**
  - `GameSettings` resource with sound/music volume, TTS toggle, hints toggle.
  - Settings save/load to `settings.json`.
  - Reset Save button removes `save.json`.
  - Difficulty screen sets `GradeManager.active_grade`.

### Architecture

- **P2.2 Directory Split**
  - Moved all game logic modules into `src/core/`.
  - Extracted platform-specific code into `src/bridge/`:
    - `url_opener.rs` for `web_sys` purchase URL handling.
    - `tts_client.rs` for the `reqwest` Kokoro TTS sidecar client.
  - `main.rs` and `lib.rs` declare `core` and `bridge` modules and re-export `core::*`.

### Parent + Storefront

- **P3.8 Parent Progress Report**
  - `parent_report.html` lets a parent upload `save.json` and view rank, XP, words learned, favorite element, recent words, and a conversation prompt.

- **P3.9 Landing Page**
  - `landing_page.html` one-page site with hero, features, demo CTA, and purchase link.

- **P3.10 Storefront Setup**
  - `itch_page.md` has page copy, pricing ($9.99), platforms, and demo/purchase CTAs.
  - Paywall button links to the Polar.sh purchase URL.

## Verification

Run these commands to confirm the current state:

```bash
cd /home/joshua/LitTCG/LitTTC
cargo clippy --features desktop   # 0 warnings
cargo test                         # 81 tests passing
cargo check --features flat2d      # clean
cargo check --features xr          # clean
trunk build                        # WASM build succeeds
```

## What Remains

The following tasks require manual testing, external accounts, or user outreach:

- **P3.11 Internal testing** — Test `trunk serve` in Chrome, Android phone, and Chromebook.
- **P3.12 Beta families** — Share the demo with 3–5 families and collect feedback.
- **P3.13 Public demo deploy** — Deploy to itch.io/domain, post in forums, record a 60-second video.
- **Definition of Done**
  - Browser playtest without instructions.
  - Chromebook 30+ FPS check.
  - "Wife understands the game after 60 seconds" test.

## Next Recommended Steps

1. Run `trunk serve` locally and play through the 10-word demo happy path.
2. Create an itch.io project and upload the `dist/` folder produced by `trunk build`.
3. Create the Polar.sh product and update `PURCHASE_URL` in `src/bridge/url_opener.rs`.
4. Share the demo link with beta families.
