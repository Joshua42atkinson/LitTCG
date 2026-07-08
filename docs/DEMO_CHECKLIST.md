# LitTCG 30-Day Demo Sprint Checklist
## Goal: A playable web demo your wife (and customers) can understand in 60 seconds

**Start:** July 2026  
**End:** August 2026  
**Success:** A public web demo where a player can collect letters, spell a word, see a pet appear, and view a collection — in a browser on a Chromebook.

---

## Week 1: Core Demo Loop

### Day 1–2: Pet Card Reveal (The Pokémon Moment)
- [ ] Animate the face-down card flipping over when a word is submitted.
- [ ] Burst the pet out with particles and a satisfying sound.
- [ ] Display rarity, element icon, and pet name.
- [ ] Ensure this works in both 3D desktop and `flat2d` modes.

### Day 3–4: Async JSON Loading for Web
- [ ] Convert database loading to async so the WASM build does not freeze on startup.
- [ ] Add a loading screen with progress bar.
- [ ] Verify `trunk serve` builds and runs in Chrome.

### Day 5–7: Demo Limit & Paywall
- [ ] Enforce `DemoSettings.max_words` (default 10 words).
- [ ] Show paywall screen after demo limit with "Unlock Full Game" link.
- [ ] Ensure paywall does not block the initial free experience.

---

## Week 2: Browser & Mobile Polish

### Day 8–9: Touch-First UI
- [ ] Larger tap targets for letters and buttons.
- [ ] Swipe spelling works on touchscreen.
- [ ] UI scales correctly on small screens.

### Day 10–11: Pet Collection Screen
- [ ] Grid view of all collected pets.
- [ ] Sort by element, rarity, or word length.
- [ ] Tap a pet to see stats and FACES expression.

### Day 12–14: Settings & Difficulty
- [ ] Settings screen: sound, music, TTS toggle.
- [ ] Difficulty menu: word grade range, hints on/off.
- [ ] Save settings to local storage.

---

## Week 3: Parent-Facing Features

### Day 15–17: Parent Progress Report
- [ ] Simple HTML dashboard reading `save.json`.
- [ ] Show words learned this week, total words, favorite element.
- [ ] One conversation prompt generated from the child's play.

### Day 18–19: Weekly Email Report (Optional)
- [ ] Generate a plain-text summary.
- [ ] Provide copy-to-clipboard for parents to paste in a message.

### Day 20–21: Landing Page
- [ ] One-page site with trailer GIF or screenshot.
- [ ] "Try Demo" button linking to the web build.
- [ ] "Buy Full Game" button linking to Polar.sh or itch.io.

---

## Week 4: Ship & Test

### Day 22–24: Internal Testing
- [ ] Run `cargo test` and `cargo clippy`.
- [ ] Test web demo on Chromebook.
- [ ] Test Android build.
- [ ] Test desktop build.

### Day 25–26: Beta Users
- [ ] Share demo with 3–5 homeschool families or friends.
- [ ] Collect feedback on the pet reveal and the first 5 minutes.
- [ ] Fix critical bugs.

### Day 27–30: Public Demo
- [ ] Deploy web demo to itch.io or your domain.
- [ ] Post in 2–3 homeschool forums or groups.
- [ ] Record a 60-second demo video for social media.
- [ ] Set up Polar.sh product for full game unlock.

---

## Definition of Done

- [ ] A player can open the demo in a browser and play without reading instructions.
- [ ] Spelling a word produces a visible, unique pet within 3 seconds.
- [ ] The demo ends naturally after 10 words with a clear purchase path.
- [ ] A parent can view a simple progress report.
- [ ] The demo runs on a mid-range Chromebook at 30+ FPS.
- [ ] No console errors or broken UI in the happy path.

---

## Anti-Goals (Do Not Do)

- [ ] Do not add new game modes.
- [ ] Do not integrate AI tutors yet.
- [ ] Do not build school admin tools.
- [ ] Do not switch engines or port to Kotlin.
- [ ] Do not add multiplayer or cloud sync.

---

## Success Metrics

| Metric | Target |
|--------|--------|
| Demo completion rate (spells first word) | >70% |
| Demo-to-paid conversion | >2% |
| Average session length | 5–10 minutes |
| Wife understands the game after 60 seconds | Yes |
| Zero critical bugs on Chromebook | Yes |
