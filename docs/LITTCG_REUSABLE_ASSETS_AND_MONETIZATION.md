# LitTCG: Reusable Assets and Monetization Opportunities
## From Voix Vive, Legacy Docs, and Spatial Computing Blueprint

**Date:** July 7, 2026  
**Purpose:** Identify what existing code, design patterns, and intellectual property from nearby projects can accelerate LitTCG and create new revenue streams.

---

## 1. Reusable Code from Voix Vive (`/home/joshua/Workflow/Bertrand-Masterclass`)

Voix Vive is a spatial music education app built on Bevy 0.18 + OpenXR + Android XR. It shares the same stack as LitTCG and contains several modules that can be ported directly or adapted.

### 1.1 Real XR Hand Tracking
**What exists:** Voix Vive has working OpenXR hand tracking that maps `INDEX_TIP` to fret positions, calculates pinch thresholds, and distinguishes left/right hands.

**How it helps LitTCG:**
- Replaces the simulated/stub hand tracking in `LitTTC/src/hand_tracking.rs`.
- Enables real pinch-to-spell and pinch-to-select cards in XR.
- Makes the ASL fingerspelling feature actually possible.
- Unlocks the XR premium demo channel.

**Monetization impact:** Medium. XR is not the mass market, but it is the press and differentiation channel.

### 1.2 Spatial UI / Holographic Panels
**What exists:** `holographic_ui.rs`, `spatial_ui.rs`, `widgets.rs`, `system_menu.rs` are built but gated behind the `extras` feature due to Bevy 0.18 ParamSet conflicts.

**How it helps LitTCG:**
- Provides a foundation for XR menus, pet stat cards, and quest panels.
- Better than the current `spatial_ui.rs` scaffolding in LitTCG.
- Solves the B0001 query conflict pattern once, then applies everywhere.

**Monetization impact:** Low directly, but enables the XR premium experience.

### 1.3 Bevy 0.18 OpenXR Architecture
**What exists:** A clean `lib.rs` plugin structure, desktop emulator, native OpenXR binary, XR shell, and environment manager.

**How it helps LitTCG:**
- Provides a working reference for the `xr` feature build.
- Shows how to disable Bloom/SSAO for XR performance.
- Demonstrates the `bevy_mod_openxr` setup pattern.

**Monetization impact:** Medium. Faster XR build means faster press demos.

### 1.4 Android XR Native App (Kotlin + Jetpack XR)
**What exists:** A complete Kotlin Android XR app with `MainActivity.kt`, `HandTrackingManager.kt`, `PitchDetectionEngine.kt`, and `VoixViveXrApp.kt`.

**How it helps LitTCG:**
- Provides an alternative to the Bevy + `cargo ndk` path.
- Could be used for a high-quality Android XR LitTCG port.
- Demonstrates Jetpack XR SDK integration with ARCore hand tracking.

**Monetization impact:** Medium. Enables Android XR/Quest distribution without waiting for Bevy mobile maturity.

### 1.5 Voice Control / Hands-Free System
**What exists:** A two-tier voice system in the React companion app: 17 fast keyword commands + AI intent interpretation via Truebadour AI. Tool tags (`[TOOL:XXX]`) drive UI actions.

**How it helps LitTCG:**
- Enables voice-controlled spelling or pet commands ("spell courage").
- Provides accessibility for motor-impaired players.
- Could become a premium feature: "Voice mode for hands-free play."

**Monetization impact:** Medium. Strong accessibility and differentiation feature.

### 1.6 Google OAuth → Gemini Pattern
**What exists:** Students log in with Google; their OAuth token calls Gemini directly. The platform pays zero API cost because the user's own Google quota funds the AI.

**How it helps LitTCG:**
- Enables an optional AI tutor/Socratic guide without bankrupting you on API costs.
- Maintains "no cloud accounts" by making Google auth optional, not mandatory.
- Could power the Dynamic Lesson Engine (below) or conversation prompts.

**Monetization impact:** High. This is the only viable way to add AI to a low-cost children's product without subscriptions or ads.

### 1.7 PWA / Cloudflare Deployment Pattern
**What exists:** A React PWA deployed to Cloudflare Pages with manifest, service worker, `_redirects`, and build config.

**How it helps LitTCG:**
- Provides a template for the Parent Dashboard web app.
- Shows how to host a game companion site at a custom domain.
- Could be used for itch.io-adjacent landing pages.

**Monetization impact:** Low directly, but reduces hosting/devops overhead.

### 1.8 Internationalization (i18n) Framework
**What exists:** 700/700 English-French key parity with a `useLocale` hook and locale files.

**How it helps LitTCG:**
- Provides a ready pattern for localizing the dashboard or companion app.
- Could be used for UK/Australia/Canada English or Spanish expansion.

**Monetization impact:** Medium. Internationalization expands addressable market.

### 1.9 BSL 1.1 License and IP Protection
**What exists:** Voix Vive uses Business Source License 1.1, which is non-commercial-OK but requires a commercial license. This protects IP while allowing community use.

**How it helps LitTCG:**
- LitTCG could adopt the same license to protect the FACES protocol and word-to-creature pipeline.
- Provides a clear commercial licensing path for schools or white-label partners.

**Monetization impact:** High. Protects the core IP from free commercial exploitation.

---

## 2. Revivable Ideas from LitTCG Legacy Docs (`/home/joshua/LitTCG/legacy/old_docs`)

The old docs contain several powerful ideas that were cut or deferred. These are the highest-leverage monetization opportunities.

### 2.1 Mentor-in-the-Middle Dashboard
**What it is:** A dashboard where AI parses data but a human mentor (parent/teacher) provides oversight. Real-time alerts for "cognitive fuel," "track friction," and "stagnant engagement."

**How it makes money:**
- Premium dashboard feature ($39.99/year).
- Justifies recurring revenue by providing human-in-the-loop insights.
- Appeals to homeschool parents who want to be involved, not replaced.

**Why it works now:** The current dashboard is a prototype. This gives it a clear philosophy and differentiated value.

### 2.2 Dynamic Lesson Engine (DLE)
**What it is:** Teachers/parents upload lesson plans or curriculum text. The system parses it using grammar/semantics/rhetoric, converts it to ECS data, and hot-swaps it into the live game.

**How it makes money:**
- Teacher/parent content creation tool.
- Custom curriculum packs sold as expansion packs ($4.99–$9.99 each).
- School license upsell: "Upload your own spelling lists and vocab."
- Reduces dependency on the fixed 9,582-word database.

**Why it works now:** The parsing logic (FACES, etymology) already exists. This adds an input layer and a content marketplace.

### 2.3 Curriculum Sandbox Editor
**What it is:** A controlled environment where mentors review AI-generated content before it enters the live game engine. Includes parsing review, rewriting, blessing, and live update.

**How it makes money:**
- Premium dashboard feature for teachers and parents.
- Enables user-generated content safely.
- Creates a "teacher co-author" experience.

**Why it works now:** Safety is a core concern for children's apps. This makes AI-assisted content safe.

### 2.4 Socratic Intervention Alerts
**What it is:** Real-time alerts that tell the mentor when to step in and provide a Socratic prompt (not a direct answer).

**How it makes money:**
- Premium dashboard feature.
- High value for parents: "Know exactly when your child needs help."
- Differentiates from passive progress dashboards.

**Why it works now:** The `StudentTrail` and `CharacterSheet` already track the needed signals.

### 2.5 PracticeRecorder & Somatic Portfolio Viewer
**What it is:** Audio/video/somatic recording with annotation for mentors. Includes pitch cents deviation, posture, fingerspelling accuracy.

**How it makes money:**
- Special education / speech therapy channel.
- Premium tier for SLPs and tutors.
- Could support articulation packs and dyslexia intervention.

**Why it works now:** This opens a high-trust niche market with less competition.

### 2.6 Jungian Archetypes + CCSS Telemetry
**What it is:** Map gameplay patterns to Jungian archetypes (Innocent, Rebel, Sage, Jester) and overlay Common Core State Standards (CCSS) mastery.

**How it makes money:**
- School sales: "Standards-aligned progress reports with personality insights."
- Parent reports: "Your child is a Sage archetype — here is how to support them."
- Premium dashboard upsell.

**Why it works now:** The current `emergent_class` (Oracle/Bard/Cultivator/Templar) is the start of this. The old docs provide a richer framework.

### 2.7 40-Week Curriculum Matrix
**What it is:** A structured 40-week curriculum mapped to the game districts and quests.

**How it makes money:**
- Sell as a structured homeschool curriculum supplement.
- Teacher classroom license: "One word list per week, aligned to ELA standards."
- Bundle with the game for higher price ($19.99–$29.99).

**Why it works now:** The quest database exists but is not yet packaged as a curriculum.

### 2.8 "Communication Class" Branding
**What it is:** The original product name was "Communication Class" / "Semantic Slime."

**How it makes money:**
- Could be a classroom-focused SKU or school product name.
- Less consumer-gamey than "LitTCG" for institutional buyers.
- "LitTCG" stays the consumer brand; "Communication Class" becomes the B2B brand.

**Why it works now:** You already have two brands. Use them for different markets.

---

## 3. New Monetization Opportunities These Unlock

### 3.1 Premium Dashboard Tier ($39.99–$59.99/year)
Combine:
- Mentor-in-the-Middle alerts.
- Socratic intervention prompts.
- Dynamic Lesson Engine (upload custom content).
- Somatic portfolio (audio/video of practice).
- CCSS + archetype reports.

This is the strongest path to recurring revenue without subscriptions-for-kids.

### 3.2 Teacher Content Marketplace ($4.99–$9.99/pack)
- Sell pre-made curriculum packs (40-week ELA, science vocab, SAT prep).
- Let teachers sell their own packs (revenue share).
- Use the Curriculum Sandbox Editor for moderation.

### 3.3 Special Education / SLP Tier ($99–$199/year)
- PracticeRecorder for articulation and fingerspelling.
- Custom phoneme packs.
- Therapist dashboard with IEP-aligned reports.
- This is a high-trust, lower-competition market.

### 3.4 XR Premium Edition ($19.99 one-time)
- Port hand tracking from Voix Vive.
- Sell as a separate "XR Edition" on Quest/Android XR stores.
- Lower volume but higher perceived value and press value.

### 3.5 AI Tutor Add-On (Google OAuth → Gemini)
- Optional Socratic AI tutor using the student's own Google quota.
- Zero platform cost.
- Could be a premium dashboard feature or $9.99 one-time add-on.
- Provides the agentic experience you described without making it the foundation.

### 3.6 White-Label / Licensing
- With BSL 1.1, LitTCG can license the engine to other curriculum creators.
- Example: "Build your own vocabulary game on the LitTCG engine."
- High-margin B2B revenue.

---

## 4. What to Build First (Ranked by Revenue Impact)

| Priority | Feature | Source | Revenue Impact | Effort |
|----------|---------|--------|----------------|--------|
| **P0** | Real hand tracking for XR | Voix Vive | Medium | High |
| **P0** | Pet card reveal + collection | Current LitTCG gap | Very High | Medium |
| **P1** | Parent Dashboard v1 | Current prototype | High | Medium |
| **P1** | Google OAuth → Gemini AI tutor | Voix Vive | High | Medium |
| **P2** | Mentor-in-the-Middle alerts | Old docs | Medium | Medium |
| **P2** | Dynamic Lesson Engine | Old docs | High | High |
| **P2** | PracticeRecorder | Old docs | Medium | High |
| **P3** | Content marketplace | Old docs | Medium | High |
| **P3** | XR Premium Edition | Voix Vive + LitTCG | Medium | High |
| **P3** | White-label licensing | BSL 1.1 | High | Low |

---

## 5. What to Avoid

### 5.1 Do Not Rebuild the Voix Vive PWA for LitTCG
The Voix Vive companion app is a React/Vite/Cloudflare stack. LitTCG's dashboard can be simpler (static HTML + JS reading `save.json`). Do not over-engineer.

### 5.2 Do Not Make AI the Foundation
The Google OAuth → Gemini pattern is great as an **optional add-on**. Making it mandatory destroys the local-first/COPPA advantage.

### 5.3 Do Not Revive the 40-Week Curriculum Before the Core Game Ships
It is a strong expansion idea, but the base game must be fun first.

### 5.4 Do Not Port Everything to Kotlin Android XR Before the Bevy Path Proves Itself
The Bevy + Android NDK path is already started in LitTCG. The Kotlin path is an alternative, not a replacement.

---

## 6. Immediate Action Plan

If you want to leverage these assets now, the highest-impact sequence is:

1. **Week 1:** Port real hand tracking from Voix Vive to LitTCG XR mode.
2. **Week 2:** Implement pet card reveal and pet collection screen.
3. **Week 3:** Build Parent Dashboard v1 (weekly report, conversation prompts).
4. **Week 4:** Prototype Google OAuth → Gemini as optional AI tutor.
5. **Week 5–6:** Test and ship web demo + Android demo.
6. **Week 7–12:** Launch, gather feedback, then plan Dynamic Lesson Engine or special education tier based on what users ask for.

---

## 7. Conclusion

The strongest monetization levers are already in your workspace:
- **Voix Vive** gives you real XR hand tracking, spatial UI, and the Google OAuth → Gemini pattern.
- **Old docs** give you the Mentor-in-the-Middle Dashboard, Dynamic Lesson Engine, and PracticeRecorder — premium features that justify recurring revenue.
- **BSL 1.1** gives you an IP protection strategy.

The core rule remains: **do not build these until the base game is fun and shipped.** The biggest money comes from selling the core word-to-creature experience first, then layering premium tools on top.
