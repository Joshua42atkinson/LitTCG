# LitTCG Cross-Project Integration Plan
## What to Pull from Voix Vive, Trinity, and Other Workspace Projects

**Date:** July 7, 2026  
**Purpose:** Identify concrete code, patterns, and IP from nearby projects that can accelerate LitTCG and unlock revenue.

---

## 1. Voix Vive (`/home/joshua/Workflow/Bertrand-Masterclass`)

Voix Vive is a spatial music education app. Same stack: Bevy 0.18 + OpenXR + Android XR + Rust. High direct reuse.

| Asset | File(s) | Use in LitTCG | Priority |
|-------|---------|---------------|----------|
| **Real OpenXR hand tracking** | `apps/spatial-engine-bevy/src/hand_tracking.rs` | Replace LitTCG hand-tracking stubs with working fingertip/pinch logic for XR spelling and card selection | **P0** |
| **Spatial UI panels** | `apps/spatial-engine-bevy/src/holographic_ui.rs`, `spatial_ui.rs`, `widgets.rs` | Fix Bevy ParamSet conflicts and adapt panels for XR menus/pet cards | **P1** |
| **Android XR manifest/build** | `apps/xr-prototype/android-xr/*` | Reference for Jetpack XR SDK path if Bevy + NDK path stalls | **P2** |
| **Voice control system** | `apps/companion-app/src/hooks/useHandsFreeCoach.js`, `useTruebadourAI.js` | Optional voice spelling/commands for accessibility and cool factor | **P2** |
| **Google OAuth → Gemini** | `apps/companion-app/src/lib/geminiOAuth.js` | Optional AI tutor where student's Google quota pays API costs | **P1** |
| **i18n framework** | `apps/companion-app/src/locales/en.json`, `fr.json`, `useLocale` hook | Localization pattern for dashboard and companion app | **P2** |
| **PWA / Cloudflare deploy** | `apps/companion-app/*` | Template for LitTCG dashboard or marketing site | **P2** |
| **BSL 1.1 license** | `LICENSE` / `NOTICE` | IP protection model for LitTCG | **P1** |

**Licensing note:** Voix Vive is under BSL 1.1. Since you own both projects, reuse is allowed, but verify third-party attributions in `NOTICE` if porting code directly.

---

## 2. Voix Vive iOS (`/home/joshua/Workflow/VoixVive-iOS`)

React/Vite/Tauri iOS app. Less direct overlap, but useful for mobile companion strategy.

| Asset | File(s) | Use in LitTCG | Priority |
|-------|---------|---------------|----------|
| **Tauri iOS build pattern** | `src-tauri/`, `build-ios.sh` | If LitTCG ever needs an iOS native wrapper | **P3** |
| **React dashboard scaffold** | `src/components/`, `src/pages/` | Parent Dashboard UI components if we build a React dashboard | **P2** |
| **Tailwind + Vite config** | `vite.config.js`, `tailwind.config.js` | Fast dashboard styling setup | **P2** |

---

## 3. Trinity ID AI OS (`/home/joshua/Workflow/TRINITYIDAIOS`)

A Rust/Axum local AI OS. Not a game engine, but has patterns for AI orchestration, memory, and instructional design.

| Asset | File(s) | Use in LitTCG | Priority |
|-------|---------|---------------|----------|
| **ADDIECRAPEYE state machine** | `crates/trinity-quest/` | Project/task management methodology; already used informally in `task.md` | **P2** |
| **Socratic prompting pattern** | `README.md`, agent prompts | Optional AI tutor behavior: questions before answers | **P2** |
| **Persistent memory / RAG** | `crates/trinity/`, SQLite | Optional dashboard feature: long-term student memory across sessions | **P3** |
| **EYE Package Export** | `README.md` "EYE Package Export" | Export learning reports as HTML5 mini-games or DOCX; premium parent/teacher feature | **P3** |
| **Hotel Manager** | `README.md` "Hotel Manager" | Local model orchestration if we add an on-device LLM for AI tutor | **P3** |

**Important:** Trinity is a platform, not a product component. Do not make LitTCG depend on Trinity running. Only adopt patterns that can be embedded or copied.

---

## 4. LDTAtkinson Business (`/home/joshua/Workflow/LDTAtkinson`)

Business infrastructure. Needed for sales and grants.

| Asset | File | Use in LitTCG | Priority |
|-------|------|---------------|----------|
| **Polar.sh payment setup** | `MONETIZATION_AND_BUSINESS_PLAN.md` | Direct sales for LitTCG | **P0** |
| **Grant pipeline** | Same | MTI, MEA, SDVOSB, PenFed for non-dilutive funding | **P1** |
| **DBA/EIN/banking** | Same | Required before first sale | **P0** |

---

## 5. What NOT to Pull

| Project | What to Avoid | Why |
|---------|---------------|-----|
| **SpawnForge / project-forge** | Do not use as the game engine | LitTCG is a game; SpawnForge is an engine. Switching now resets the 90-day clock and throws away working code. |
| **Trinity** | Do not make LitTCG require a Trinity backend | Adds deployment complexity and kills local-first/COPPA positioning. |
| **Voix Vive iOS** | Do not port the full React app | Use only specific components (dashboard) if needed. |

---

## 6. Pull Sequence (Next 90 Days)

**Week 1–2:**
1. Port real hand tracking from Voix Vive to LitTCG XR mode.
2. Add BSL 1.1 license to LitTCG.

**Week 3–4:**
3. Prototype Google OAuth → Gemini as optional AI tutor.
4. Port spatial UI panels for XR menus.

**Month 2:**
5. Build Parent Dashboard using Voix Vive PWA patterns.
6. Add i18n scaffolding for EN/ES expansion.

**Month 3+:**
7. Evaluate Trinity's EYE export or persistent memory for premium dashboard features.
8. Consider iOS/Tauri build using VoixVive-iOS as reference.

---

## 7. Immediate Next Step

The highest-value, lowest-risk pull is **real OpenXR hand tracking** from Voix Vive. It directly improves the XR demo without adding dependencies or cloud costs.

**Action:** Compare `LitTTC/src/hand_tracking.rs` with `Bertrand-Masterclass/apps/spatial-engine-bevy/src/hand_tracking.rs` and port the working joint/pinch logic.
