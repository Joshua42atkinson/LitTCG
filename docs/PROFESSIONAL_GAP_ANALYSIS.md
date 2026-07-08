# LitTCG Professional Gap Analysis

**Document Version:** 1.0  
**Date:** July 7, 2026  
**Prepared For:** Joshua Atkinson and Stakeholder Review  
**Product:** LitTCG — Literary Trading Card Game  
**Document Type:** Strategic Gap Assessment

---

## Executive Summary

This document compares the current state of LitTCG against the desired market-ready state, identifying gaps in product, technology, content, operations, and go-to-market readiness. It prioritizes each gap by impact and effort, and provides a remediation roadmap.

**Primary Finding:** The core engine and educational systems are largely complete. The largest gaps are in **product surface polish** (the demo experience), **platform readiness** (web/WASM and Chromebook), and **go-to-market infrastructure** (storefront, marketing assets, and parent communication). These are all solvable with focused engineering and marketing work.

---

## 1. Current State Assessment

### 1.1 What Is Complete
- 22 source files compile and run.
- 8/8 integration tests passing.
- 5 embedded JSON databases (9,582 words, quests, lore, etymology, synonyms).
- Full game state machine: Loading → MainMenu → Collecting → Constructing → Playing → Questing → Battling → Reviewing → Paywall.
- Word validation, etymology analysis, FACES detection, and procedural pet rendering.
- Battle system with semantic distance and class-specific combat.
- Mad-Lib quest engine with 93 templates.
- Save/load system (local JSON).
- HUD, main menu, tutorial, paywall scaffolding.
- XR and desktop feature flags in Cargo.toml.
- Command-driven architecture partially implemented.

### 1.2 What Is Incomplete or Stubbed
- Pet card reveal animation not implemented.
- Pet collection screen not implemented.
- Roster selection not implemented.
- Rarity system not implemented.
- Visual evolution not implemented.
- RPS class modifier not applied.
- ASL fingerspelling only detects A and L.
- Hand tracking is simulated, not real OpenXR.
- WASM build not verified in browser.
- Demo limit may not be fully enforced in code.
- 13 compiler warnings remain (per older ROADMAP; current task.md suggests some cleanup is done).
- Parent Dashboard is a prototype HTML file, not a polished product.
- Profanity blocklist not implemented.
- "Arousal" terminology still appears in some code/docs (mostly renamed per task.md).

---

## 2. Gap Analysis Framework

Each gap is scored on:
- **Impact:** How much it blocks sales, trust, or usability (1–5).
- **Effort:** Estimated engineering/marketing effort to close (1–5).
- **Priority:** High/Medium/Low based on impact/effort ratio.
- **Owner:** Engineering, Marketing, or Operations.

---

## 3. Product Experience Gaps

| Gap | Current State | Desired State | Impact | Effort | Priority | Owner |
|-----|--------------|---------------|--------|--------|----------|-------|
| **Pet Card Reveal** | Pet spawns immediately after word submission | Card flips, pet bursts out with VFX/sound | 5 | 3 | **High** | Engineering |
| **Pet Collection Screen** | No collection UI; only SpellBook data | Grid of pet cards, sortable by element/role/rarity | 5 | 3 | **High** | Engineering |
| **Roster Selection** | No pre-battle pet selection | Player picks 3–6 pets before battle | 4 | 3 | **High** | Engineering |
| **Rarity System** | Rarity enum not implemented | Rarity calculated and displayed on cards | 4 | 2 | **High** | Engineering |
| **Visual Evolution** | Mastery tracked but no visual change | Pets evolve visually at mastery thresholds | 4 | 4 | Medium | Engineering |
| **RPS Class Modifier** | Class-specific combat exists but no RPS bonus | +50%/-25% damage based on class matchup | 3 | 2 | Medium | Engineering |
| **Color-Coded Quest Slots** | Text-only slot labels | Visual color coding by part of speech | 3 | 2 | Medium | Engineering |
| **Companion Follow System** | Not implemented | One pet follows player through world | 3 | 3 | Medium | Engineering |
| **Nuisance Letters** | Not implemented | Roaming letters add challenge/variety | 2 | 3 | Low | Engineering |
| **Pet Dream Layer** | Not implemented | Mastered pets whisper etymology poetry | 2 | 2 | Low | Engineering |

### Gap Analysis Notes
The **Pet Card Reveal** is the highest-impact gap. It is the emotional hook that makes the game feel like a real product rather than a tech demo. Every marketing asset (trailer, GIFs, screenshots) depends on it. This should be the next major engineering priority after safety and stability.

---

## 4. Platform and Technical Gaps

| Gap | Current State | Desired State | Impact | Effort | Priority | Owner |
|-----|--------------|---------------|--------|--------|----------|-------|
| **WASM Build Verification** | `trunk serve` exists but not verified in browser | Stable, performant web demo on Chrome, Firefox, Safari | 5 | 3 | **High** | Engineering |
| **Demo Limit Enforcement** | UI says 10 words; code may not fully enforce | Hard limit at 10 words with paywall transition | 5 | 2 | **High** | Engineering |
| **Async JSON Loading** | 3.3MB databases loaded synchronously | Async loading with progress spinner for web | 4 | 3 | **High** | Engineering |
| **PWA/Offline Support** | No service worker or manifest | Chromebook/offline-capable PWA | 4 | 3 | Medium | Engineering |
| **Compiler Warnings** | Some warnings remain | Zero warnings on all feature flags | 3 | 2 | **High** | Engineering |
| **Android XR Build** | `cargo ndk` configured but not tested | Verified APK for XR headsets | 3 | 4 | Medium | Engineering |
| **Hand Tracking / ASL** | Simulated/stub input | Real pinch and full A-Z fingerspelling | 3 | 4 | Medium | Engineering |
| **Quality Presets** | No performance scaling | Low/Medium/High presets for particles, bloom, MSAA | 3 | 2 | Medium | Engineering |
| **TTS Offline Fallback** | Kokoro TTS requires sidecar; fails on WASM | Graceful fallback or web TTS integration | 2 | 2 | Low | Engineering |

### Gap Analysis Notes
Web performance is a make-or-break issue for the Chromebook market. Synchronous loading of 3.3MB of embedded JSON will freeze the browser on low-end Chromebooks and fail the first impression. This must be addressed before any public web demo.

---

## 5. Safety and Compliance Gaps

| Gap | Current State | Desired State | Impact | Effort | Priority | Owner |
|-----|--------------|---------------|--------|--------|----------|-------|
| **Profanity Blocklist** | No blocklist | Banned words fail silently, no pet spawned | 5 | 2 | **High** | Engineering |
| **Arousal Terminology** | Mostly renamed; verify all references | All user-facing code/docs use "intensity" | 4 | 2 | **High** | Engineering |
| **Glitch Entity Text Masking** | Invalid word text may appear in UI | Raw text replaced with [ANOMALY] or !#?@* | 4 | 2 | **High** | Engineering |
| **COPPA Documentation** | Designed in, not documented externally | Public privacy policy and COPPA statement | 3 | 2 | Medium | Operations |
| **FERPA Readiness** | Local-first, but no school-specific data policy | Clear data handling for institutional deployments | 3 | 2 | Medium | Operations |

### Gap Analysis Notes
Safety issues are launch-blocking. A single screenshot of a slur-generated pet or a parent noticing the word "arousal" in a children's product can destroy trust permanently. These must be closed before any public demo.

---

## 6. Content and Curriculum Gaps

| Gap | Current State | Desired State | Impact | Effort | Priority | Owner |
|-----|--------------|---------------|--------|--------|----------|-------|
| **Demo Word Curation** | No curated 10-word demo list | 10 words showcasing elements, classes, grades, dramatic FACES | 5 | 2 | **High** | Content |
| **Standards Alignment Matrix** | No mapping to CCSS/state standards | Document mapping game skills to literacy standards | 3 | 3 | Medium | Content |
| **Teacher Guide** | No educator materials | One-page guide explaining how to use LitTCG in class | 3 | 2 | Medium | Content |
| **Expansion Pack Content** | Ideas only (SAT, science, Spanish) | Defined word lists and quest templates | 2 | 4 | Low | Content |
| **Parent Onboarding Copy** | No parent-facing explanation | Short guide explaining learning mechanics and dashboard | 3 | 2 | Medium | Content |

---

## 7. Go-to-Market Gaps

| Gap | Current State | Desired State | Impact | Effort | Priority | Owner |
|-----|--------------|---------------|--------|--------|----------|-------|
| **itch.io Storefront** | `itch_page.md` draft exists | Live page with screenshots, GIFs, trailer, buy button | 5 | 3 | **High** | Marketing |
| **Trailer / GIFs** | No marketing assets | 60-second trailer + 5 key GIFs | 5 | 3 | **High** | Marketing |
| **Email Capture** | No landing page or list | ConvertKit/ConvertKit landing page connected to demo | 4 | 2 | **High** | Marketing |
| **Press Kit** | Draft exists (`PRESS_KIT.md`) | Polished press kit with screenshots, bio, facts | 4 | 2 | Medium | Marketing |
| **Social Media Presence** | No handles claimed | `@LitTCGGame` claimed on major platforms | 3 | 2 | Medium | Marketing |
| **Community Outreach** | No posts or campaigns | Launch posts in r/homeschool, Facebook groups | 4 | 2 | **High** | Marketing |
| **Parent Dashboard Product** | Prototype HTML file | Polished hosted dashboard with branding | 4 | 3 | Medium | Engineering/Marketing |
| **Pricing Page** | Pricing in marketing plan only | Clear pricing page on storefront and website | 3 | 1 | Medium | Marketing |
| **Analytics / Metrics** | No analytics in demo | Basic funnel tracking (views → plays → purchases) | 3 | 2 | Medium | Engineering |

---

## 8. Competitive Gap Analysis

| Competitor | Their Strength | LitTCG's Gap | LitTCG's Advantage |
|------------|---------------|--------------|-------------------|
| **Prodigy Math/English** | Large user base, school penetration, adaptive content | Brand awareness, institutional relationships | Ethical pricing, word-meaning creature generation, no subscriptions for kids |
| **WonderLang** | One-time purchase, language RPG | Not native English literacy, no parent dashboard | English vocabulary as the creature, homeschool-friendly COPPA design |
| **Echoes** | Reading comprehension focus, AI companions | No equivalent narrative/AI companion depth | Vocabulary + grammar + etymology in one game loop |
| **Epic!** | Huge content library, school partnerships | Passive reading, no game mechanics | Active learning through play |
| **Lexia Core5** | School trust, standards alignment | Consumer appeal, fun factor | Game-first engagement, transparent pricing |
| **Roblox Word Games** | Social, free | Polish, safety, educational depth | Single-player campaign, COPPA-safe, no UGC risk |

**Strategic Gap:** LitTCG's biggest competitive gap is **brand awareness and distribution**. The product concept is stronger than most competitors, but none of them know we exist yet. The first marketing priority is closing the awareness gap through demo distribution and homeschool community engagement.

---

## 9. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Web demo crashes on low-end Chromebooks | Medium | High | Async JSON loading, quality presets, PWA testing |
| Parent sees inappropriate word generated | Low | Critical | Profanity blocklist + glitch text masking |
| Competitor copies word-to-creature concept | Medium | High | Accelerate FACES patent/trade secret protection; build brand first |
| Demo fails to convert to purchases | Medium | High | A/B test pricing, iterate on demo length, add email nurture |
| XR build delays flagship positioning | Medium | Medium | Lead with web/desktop; treat XR as press channel |
| Homeschool market slower to adopt than expected | Medium | Medium | Expand to school Chromebook pilots and speech therapy channels |
| Subscription fatigue makes dashboard hard to sell | Medium | Medium | Price dashboard annually, not monthly; emphasize value |

---

## 10. Prioritized Remediation Roadmap

### Phase 1: Safety and Stability (Weeks 1–2)
1. Implement profanity blocklist and glitch text masking.
2. Verify "intensity" terminology everywhere.
3. Achieve zero compiler warnings on all feature flags.
4. Verify all tests pass.

### Phase 2: Demo Experience (Weeks 3–6)
1. Implement pet card reveal animation.
2. Implement demo word limit enforcement.
3. Curate 10 demo words.
4. Verify WASM build in browser and on Chromebook.
5. Add async JSON loading with loading screen.

### Phase 3: Product Surface (Weeks 7–10)
1. Build pet collection screen.
2. Implement roster selection.
3. Implement rarity system and display.
4. Add color-coded quest slots.
5. Polish HUD, main menu, and transitions.

### Phase 4: Go-to-Market (Weeks 11–14)
1. Create trailer and GIFs.
2. Launch itch.io page with email capture.
3. Publish press kit.
4. Post in homeschool communities.
5. Soft launch to friends/family for feedback.

### Phase 5: Expansion (Post-Launch)
1. Parent Dashboard annual plan.
2. Classroom/teacher features.
3. Android XR build and press demos.
4. Expansion packs.

---

## 11. Conclusion

LitTCG is conceptually complete and technically advanced, but it is not yet a market-ready product. The critical gaps are the **demo experience**, **web platform polish**, and **safety/compliance**. Closing these gaps will convert the engine from a promising prototype into a sellable product. The go-to-market gaps are significant but secondary; they can be addressed in parallel once the demo is stable.

**The single most important action:** implement the pet card reveal and ship a stable, safe web demo. Everything else in marketing and sales depends on that first impression.
