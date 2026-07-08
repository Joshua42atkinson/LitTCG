# LitTCG Marketing & Sales Research
## For Wife Review — Go/No-Go on Educator Sales

**Date:** July 7, 2026  
**Product:** LitTCG — Literary Trading Card Game  
**Status:** Alpha (engine complete, product surface in progress)  
**Author:** Claude (AI pair programmer)  
**Purpose:** Research the market, pick our platform, and decide what must ship before we try to sell to parents, homeschoolers, or schools.

---

## 1. Executive Summary

LitTCG is a pet-collection game where kids spell real English words to summon unique 3D creatures. The word's meaning determines the pet's color, stats, face, and combat role. It is the anti-flashcard: vocabulary *is* the game mechanic.

**Bottom line:** The idea is strong, the homeschool market is large, and the Chromebook channel is the best first sales target. But the product is **not ready to sell today**. We need a working web demo first, then we can sell to homeschool parents, then expand to school Chromebook fleets.

**Three decisions for your review:**
1. Do we target **homeschool parents first** or **school districts first**?
2. Do we make the **web demo the primary product** for Chromebooks, or do we prioritize **desktop + XR**?
3. Do we stick with **$9.99 one-time purchase** or add a free-to-play tier with optional subscription?

---

## 2. What LitTCG Actually Is Today

### The Core Hook
- Kid spells a word (e.g., "inferno" or "serenity").
- Game checks against 9,582 real English words with psycholinguistic data.
- A unique 3D pet appears with element, stats, role, and facial expression derived from the word's meaning.
- Pets battle "Typos" and complete grammar quests.

### What Makes It Different
- **No two words produce the same pet.** This is our defensible IP.
- **Mechanic = skill:** spelling is summoning, synonyms are combat, grammar is questing.
- **COPPA-safe by design:** no cloud, no accounts, no tracking. Save file is local JSON.
- **Cross-platform:** same Rust/Bevy engine runs on desktop, web (WASM), and Android XR.

### Current Technical Status (from code & docs)
- **Engine:** 22 source files, 8/8 integration tests passing.
- **Content:** 9,582 words, 93 quest templates, 12 NPCs, 5 embedded databases.
- **Input:** keyboard/mouse, touch, and XR hand tracking (pinch + ASL fingerspelling).
- **State machine:** menu → collect letters → spell → spawn pet → quest → battle → review.
- **Warning:** product surface is still rough. The card-flip "Pokéball moment," collection screen, demo limit, and web demo polish are **not yet complete**.

---

## 3. Market Opportunity

### Homeschool Market (Primary Target)
- **3.4 million homeschool students** in the US (2024–2025), ~6.3% of all K–12 students.
- Homeschooling roughly doubled since the pandemic and is still growing.
- **Homeschool market valued at $38.6B in 2026**, projected to reach $102B by 2035.
- Homeschool parents actively buy curriculum and supplemental tools; median income above $75k; they prioritize quality over lowest cost.
- **Why this fits us:** these parents want screen time to count as learning, hate subscriptions, and value data privacy. LitTCG is built for exactly this.

### K–12 School Market (Secondary Target)
- **Chromebooks hold 60.1% of the global K–12 device market.**
- **93% of US school districts** planned Chromebook purchases in 2025–2026.
- **38 million Chromebooks** active in schools worldwide; 52% of US K–12 device shipments.
- **Google Workspace for Education** reaches 170 million students and educators across 230 countries.
- **Why this matters:** if the game runs in a Chromebook browser, it can reach the largest education hardware platform without an app store submission. This is a huge advantage.

### XR / Headset Market (Future Target)
- Google Aura / Xreal / Android XR is a small, premium education niche today.
- Headsets are expensive, and schools are slow to adopt them.
- **Strategic value:** this is the wow-factor for press, demos, and premium positioning, but **not** the volume channel.

---

## 4. Audience Clarity: Who Plays vs. Who Pays

### The Player: Curious Kai (ages 6–12)
- Loves Pokémon, Yu-Gi-Oh!, Slay the Spire, Prodigy Math.
- Wants to collect creatures and feel smart when a hard word becomes a rare pet.
- Would rather play a game than do a worksheet.

### The Buyer: Overwhelmed Olivia (homeschool parent)
- Wants her 3rd grader to enjoy reading and spelling.
- Hates subscription traps, ads, and manipulative microtransactions.
- Needs visible progress she can understand without a PhD.
- Values offline play, no accounts, and COPPA safety.
- **Key insight:** she buys when she can see her child learning *and* having fun.

### The Educator Buyer (schools, later)
- Wants curriculum alignment, deployment simplicity, and classroom management.
- Needs to run on Chromebooks without IT friction.
- Wants reporting (Parent Dashboard) and price predictability.
- **Key insight:** schools prefer web apps over installs; they prefer per-student or site licenses over subscriptions pushed to kids.

---

## 5. Platform Strategy: Where Do We Build First?

You said we can build for Chromebooks, Xreal Aura/Google, or whatever. Here is the data-driven recommendation.

### Tier 1: Web / Chromebook (ship first)
- **Why:** 60% of education devices are Chromebooks; 93% of US districts are buying them; web apps run instantly without app store approval.
- **How:** our `trunk serve` WASM build already targets web browsers. We need to verify it works, add a demo limit, and publish to itch.io.
- **Sales path:** free web demo → paid web full version (or Chromebook install via PWA) → school licenses.
- **Effort:** medium. The engine is already cross-platform; the work is web polish and demo limits.

### Tier 2: Desktop (Windows/Mac/Linux)
- **Why:** easiest to develop; best graphics; good for homeschool families with a home computer.
- **How:** `cargo run --features desktop` works now. Sell on itch.io or direct.
- **Sales path:** premium download for families who want the full experience.
- **Effort:** low. Mostly storefront and installer work.

### Tier 3: Android XR / Google Aura (ship later)
- **Why:** immersive differentiation, press appeal, premium education positioning.
- **How:** `cargo ndk` cross-compile with `xr` feature; Android XR manifest already exists.
- **Sales path:** premium school/district pilot after desktop/web is proven.
- **Effort:** high. Needs device testing, hand tracking polish, performance tuning.

### Recommendation
**Lead with web/Chromebook.** It is the largest reachable audience and the fastest path to revenue. Desktop is the backup premium product. XR is the future flagship.

---

## 6. Sales Model & Pricing

### Current Plan (from `MARKETING_PLAN.md`)
| Tier | Price | Contents |
|------|-------|----------|
| Free Demo | $0 | 10 words, 1 quest, 1 battle, no save |
| Full Game | $9.99 | 9,582 words, 12 NPCs, 93 quests, save |
| Expansion Packs | $4.99 | SAT Prep, Science Vocab, Spanish-English |
| Parent Dashboard | $7.99/mo | Reads save.json, tracks mastery |

### Competitive Context
- **Prodigy Math:** free base game + memberships at $6.25–$14.95/mo per child. Many families pay $75–$180/year.
- **LitTCG:** $9.99 one-time is radically cheaper and parent-friendly. This is a real differentiator.
- **Risk:** one-time purchase has low lifetime value per customer unless we sell expansions or dashboard subscriptions well.

### Recommendation
Keep the **$9.99 one-time purchase** as the main offer. It matches our anti-subscription brand promise and homeschool parent values. Treat the Parent Dashboard as an **optional annual add-on** ($29.99–$49.99/year) rather than $7.99/mo to reduce subscription fatigue. Use expansion packs to re-engage existing customers.

---

## 7. Competitive Positioning

| Game | What They Do | What LitTCG Does Differently |
|------|--------------|------------------------------|
| **Pokémon** | Creature collection + type advantage | Words become creatures; stats come from real linguistics |
| **Prodigy Math** | Math problems inside RPG combat | Literacy skills ARE the combat, not an overlay |
| **Duolingo** | Drills with gamification | Open-ended collection + tactical battles; no forced streaks |
| **Hearthstone / Slay the Spire** | Deckbuilding tactics | Your deck is your vocabulary; cards are forged from words |
| **Roblox word games** | Social mini-games | Polished single-player campaign + XR future |

**Core differentiator:** LitTCG is the only game where a word's *meaning* becomes the creature's appearance, personality, and combat role.

---

## 8. What Must Ship Before We Try to Sell

The product is not demo-ready. From `task.md` and `ROADMAP.md`, these block sales:

### Must-Have for Demo
- [ ] Working web demo on itch.io (`trunk serve` verified in browser).
- [ ] Demo limit enforced: 10 words, 1 quest, 1 battle, no save.
- [ ] "Buy Full Version" prompt at demo limit.
- [ ] Pet card reveal animation (the "Pokéball moment" — this is the emotional hook).
- [ ] Zero compiler warnings and all tests passing.
- [ ] Stable desktop build.

### Must-Have for Full Release
- [ ] Full game loop works: menu → collect → spell → reveal → quest → battle → save.
- [ ] Pet collection screen (browse all summoned pets).
- [ ] Difficulty/settings menus.
- [ ] Quality presets for low-end Chromebooks.
- [ ] Parent Dashboard polished and branded as LitTCG.
- [ ] Itch.io page with screenshots, GIFs, and trailer.

### Nice-to-Have
- [ ] 2D mode (`flat2d` feature) for older Chromebooks.
- [ ] PWA + offline support for Chromebooks.
- [ ] Android XR build.

---

## 9. Short-Term Sales Plan (0–6 Months)

**Goal:** prove the demo converts to purchases and collect parent feedback.

1. **Ship itch.io demo** with 10 words, 1 quest, 1 battle.
2. **Add email capture** on the demo page for wishlist/launch notification.
3. **Price full game at $9.99** on itch.io desktop + web.
4. **Post in homeschool communities:** r/homeschool, Facebook groups, secular & faith-based homeschool forums.
5. **Create 60-second trailer** showing the card flip + pet reveal.
6. **Soft launch** to friends/family for feedback before public marketing.
7. **Track metrics:** demo views, plays, completion rate, email signups, conversion rate.

**Expected early audience:** homeschool parents with kids 6–12 who want literacy games that feel like real games.

---

## 10. Long-Term Sales Plan (6–24 Months)

**Goal:** expand from homeschool direct sales to school/district licensing and recurring revenue.

1. **Chromebook web version** as the primary school offering.
2. **School license tier:** site-wide or per-student pricing, COPPA compliance docs, curriculum alignment summary.
3. **Expansion packs:** SAT Prep, Science Vocab, Spanish-English Bridge at $4.99 each.
4. **Parent Dashboard annual plan:** $39.99/year for progress tracking, export, and reports.
5. **Android XR pilot:** premium classroom/therapy positioning with headset partners.
6. **Content marketing:** "Word of the Week," etymology posts, rare pet showcases, user-generated content campaigns.

---

## 11. Critical Decisions for Your Review

These are the questions we need answered before moving forward. Please mark approve, criticize, or revise.

### Decision 1: Primary First Audience
**Option A:** Homeschool parents (direct sales, low friction, brand-aligned).  
**Option B:** School districts (Chromebook fleets, larger contracts, longer sales cycle).  
**Recommendation:** A first, then B after 6–12 months of parent traction.

### Decision 2: Lead Platform
**Option A:** Web/Chromebook (largest reachable audience, fastest demo distribution).  
**Option B:** Desktop (best graphics, simpler storefront).  
**Option C:** Android XR (wow factor, smallest market, highest effort).  
**Recommendation:** A first, B second, C later.

### Decision 3: Pricing Model
**Option A:** $9.99 one-time purchase + optional expansions + optional dashboard annual plan.  
**Option B:** Freemium base + $7.99/mo subscription for full content (closer to Prodigy).  
**Recommendation:** A. It matches our brand and homeschool values, even if it caps short-term revenue.

### Decision 4: What to Ship Before First Public Demo
**Option A:** Wait until all Phase 2–4 features are done (card reveal, collection, collection screen, etc.).  
**Option B:** Ship a minimal but stable demo as soon as the web build works and the demo limit is enforced.  
**Recommendation:** B. A rough demo in front of real parents is better than a perfect demo that never ships. But it must not crash.

---

## 12. Immediate Action Items (This Week)

1. **Verify web demo builds and runs.** Run `trunk serve` and test in Chrome.
2. **Confirm demo limit logic.** The paywall currently hardcodes "10 words" in UI text but may not enforce it in code.
3. **Create itch.io page assets.** We need at least 5 screenshots and 1 GIF of the pet reveal.
4. **Set up email capture.** ConvertKit/Mailchimp landing page linked from demo.
5. **Draft a one-page "Why LitTCG?" handout** for homeschool parents.
6. **Review this document together.** Decide on the four questions above.

---

## 13. Appendices

### A. Key Files Referenced
- `GDD.md` — game design and pedagogy
- `MARKETING_PLAN.md` — existing marketing strategy
- `itch_page.md` — storefront copy
- `ROADMAP.md` — development status
- `task.md` — current engineering tasks
- `LitTTC/Cargo.toml` — platform features and build targets
- `LitTTC/dashboard/index.html` — Parent Dashboard prototype
- `LitTTC/docs/2D_MODE.md` — platform strategy

### B. Market Sources
- NHERI, "How Many Homeschool Students..." 2024–2025: 3.408 million homeschool students in the US.
- MarkWide Research, "Homeschooling Market 2026–2036": $38.6B market in 2026.
- About Chromebooks, "Education Sector Chromebook Adoption Statistics 2026": 60.1% K–12 device share, 38M active devices, 93% of US districts purchasing.
- ProdigyGame.com pricing pages, 2026: memberships $6.25–$14.95/mo per child.

---

*Prepared for review. Please add comments, criticisms, or decisions inline. Once we have your feedback, we will update the marketing plan and engineering priorities accordingly.*
