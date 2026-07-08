# LitTCG Professional Needs Analysis

**Document Version:** 1.0  
**Date:** July 7, 2026  
**Prepared For:** Joshua Atkinson and Stakeholder Review  
**Product:** LitTCG — Literary Trading Card Game  
**Document Type:** Strategic Needs Assessment

---

## Executive Summary

LitTCG is a cross-platform EdTech game where children spell real English words to summon unique 3D creatures, then use those creatures in tactical battles and grammar-based quests. This document identifies the functional, emotional, technical, and commercial needs of the three primary stakeholder groups — children, parents, and educators — and prioritizes them to inform product development and go-to-market strategy.

**Primary Finding:** The strongest unmet need in the market is a literacy tool that is genuinely fun for children, visibly educational for parents, and trustworthy for educators. Current products force a trade-off between engagement and learning. LitTCG removes that trade-off by making literacy the game mechanic itself.

---

## 1. Market Context

### 1.1 Addressable Markets
- **US Homeschool Market:** 3.4 million K–12 students (2024–2025), $38.6B market in 2026.
- **Global K–12 Chromebook Market:** 60.1% of education devices, 38 million active units, 93% of US districts purchasing in 2025–2026.
- **AI Gamified Children's Literacy Apps:** $0.42B in 2025, projected $2.6B by 2034.

### 1.2 Market Trends
- Parents increasingly demand **guilt-free screen time** with measurable educational outcomes.
- Subscription fatigue is rising in children's apps; one-time and transparent pricing is gaining preference.
- COPPA, FERPA, and GDPR compliance are becoming baseline requirements, not differentiators.
- Chromebooks dominate school procurement due to low cost, centralized management, and web-based deployment.
- XR remains a niche education channel but serves as a powerful press and differentiation tool.

---

## 2. Stakeholder Needs Analysis

### 2.1 Children (Ages 6–12) — The Players

| Need Category | Specific Need | How LitTCG Addresses It | Priority |
|---------------|---------------|------------------------|----------|
| **Engagement** | Immediate fun in first 60 seconds | Pet reveal "Pokéball moment" | Must-have |
| **Agency** | Ability to choose words, pets, and strategies | Open-ended word collection and roster building | Must-have |
| **Progression** | Visible growth and collection | Pet mastery, rarity, evolution, districts | Must-have |
| **Fair Challenge** | Difficulty that scales with ability | Grade-level word filtering and spiral curriculum | Must-have |
| **Social Proof** | Sharing rare finds and achievements | Pet collection screen, exportable summaries | Should-have |
| **No Shame** | Safe environment for mistakes | Invalid words become glitch entities, not failures | Must-have |
| **Narrative** | A world that feels alive | 12 districts, NPCs, day/night cycle, companion pet | Should-have |

### 2.2 Parents — The Primary Buyers

| Need Category | Specific Need | How LitTCG Addresses It | Priority |
|---------------|---------------|------------------------|----------|
| **Trust & Safety** | COPPA-safe, no tracking, no ads | Local-first save, no cloud accounts, no ads | Must-have |
| **Visible Learning** | Clear evidence of vocabulary growth | Parent Dashboard, save.json, end-of-day reports | Must-have |
| **Predictable Pricing** | No subscription traps or surprise charges | $9.99 one-time purchase model | Must-have |
| **Device Compatibility** | Works on hardware they already own | Web/Chromebook, desktop, Android/XR from one codebase | Must-have |
| **Low Friction Trial** | Try before buying | Free web demo with 10 curated words | Must-have |
| **Family Connection** | Something to discuss with child | Generated conversation prompts from daily play | Should-have |
| **Time Control** | Ability to set or monitor play time | Local session data, no always-online requirement | Should-have |

### 2.3 Educators and Schools — The Institutional Buyers

| Need Category | Specific Need | How LitTCG Addresses It | Priority |
|---------------|---------------|------------------------|----------|
| **Hardware Fit** | Runs on school Chromebooks | Web/WASM deployment, no installation | Must-have |
| **Classroom Management** | Rosters, assignments, progress tracking | Classroom dashboard, CSV export, teacher controls | Should-have |
| **Standards Alignment** | Maps to grade-level literacy standards | Grade-level word database, skills matrix | Should-have |
| **Evidence of Growth** | Exportable reports for parents/administrators | Progress tracking, mastery levels, attunement profiles | Should-have |
| **Differentiation** | Serves gifted and struggling learners | Grade scaling, adaptive difficulty, spiral curriculum | Should-have |
| **Low Supervision** | Self-directed student play | Tutorial, clear UI, embedded feedback | Must-have |
| **Privacy Compliance** | COPPA and FERPA compliance | No accounts, local data, no tracking | Must-have |

---

## 3. Functional Needs Matrix

### 3.1 Core Game Loop (Must-Have)
1. Collect letter crystals in 3D world.
2. Arrange letters into a valid English word.
3. Submit word and trigger pet card reveal.
4. View pet stats, element, role, and FACES expression.
5. Add pet to persistent collection.
6. Use pets in battle or quest.
7. Earn XP and mastery upgrades.

### 3.2 Educational Mechanics (Must-Have)
- Spelling validation against 9,582-word curriculum.
- Vocabulary reinforcement through pet collection.
- Synonym/antonym reasoning through semantic distance combat.
- Parts-of-speech recognition through Mad-Lib quest slots.
- Etymology recognition through critical hits.
- Spiral curriculum returning words at higher difficulty.

### 3.3 Progress and Reporting (Must-Have for Parents, Should-Have for Schools)
- Local save file with readable JSON.
- Parent Dashboard displaying words, mastery, emergent class, and time.
- End-of-session summary suitable for messaging or email.
- Exportable progress reports for teachers.

### 3.4 Platform and Deployment (Must-Have)
- Stable web/WASM build for Chromebooks and browsers.
- Stable desktop build for Windows, macOS, Linux.
- Android/XR build path for future premium positioning.
- Offline-capable PWA option.

---

## 4. Emotional Needs Matrix

| Stakeholder | Emotional Need | Product Response |
|-------------|---------------|------------------|
| Parent | Relief from screen-time guilt | Game-first design with visible learning evidence |
| Parent | Confidence in safety | COPPA-safe, no ads, local data |
| Parent | Pride in child's progress | Daily reports, mastery badges, emergent class |
| Child | Joy of discovery | Pet reveal, rare words, evolving creatures |
| Child | Sense of competence | Hard words become rare pets, clear feedback loops |
| Child | Ownership and identity | Pet collection, companion pet, emergent class |
| Teacher | Trust in educational value | Standards-aligned skills, progress reports |
| Teacher | Reduced workload | Self-directed play, auto-generated reports |

---

## 5. Technical and Compliance Needs

| Requirement | Description | Status |
|-------------|-------------|--------|
| **COPPA Compliance** | No collection of PII, no cloud accounts, parental consent not required for local play | Designed in |
| **FERPA Alignment** | Student data stays local; school deployments require clear data handling | Future consideration |
| **Cross-Platform Engine** | Bevy 0.18.1 with feature-flagged desktop/web/XR builds | In progress |
| **WASM Performance** | 3.3MB JSON databases must load asynchronously without freezing browser | Not yet implemented |
| **Offline Capability** | PWA/service worker for Chromebook classroom use | Not yet implemented |
| **Accessibility** | Keyboard, mouse, touch, and XR input support | Partially implemented |
| **Content Safety** | Profanity/slur blocklist in word submission | Not yet implemented |
| **Stability** | Zero compiler warnings, all tests passing | In progress |

---

## 6. Commercial Needs

| Need | Description | Strategic Response |
|------|-------------|-------------------|
| **Low Customer Acquisition Cost** | Homeschool market is reachable through organic communities | Free demo + Reddit/Facebook/YouTube channels |
| **High Trust at First Touch** | Parents are skeptical of children's apps | One-time pricing, COPPA-safe design, free demo |
| **Recurring Revenue Potential** | One-time purchase has limited LTV | Expansion packs + optional annual dashboard |
| **Institutional Sales Path** | Schools need predictable pricing and compliance | Future site licenses, standards alignment docs |
| **Viral Mechanics** | Kids sharing discoveries drives awareness | Pet collection screen, exportable summaries |
| **Press Differentiation** | Need a memorable story | FACES protocol, word-to-creature IP, XR potential |

---

## 7. Prioritized Needs Summary

### Must-Have (Non-Negotiable Before Launch)
1. Stable, crash-free web demo on Chromebook and desktop browsers.
2. Pet card reveal animation (the "Pokéball moment").
3. 10-word demo limit with clear "Buy Full Version" prompt.
4. Local save system and basic Parent Dashboard.
5. Profanity blocklist and "intensity" terminology (not "arousal").
6. Zero compiler warnings and passing test suite.
7. One-time $9.99 pricing communicated clearly.

### Should-Have (Strongly Recommended for Launch)
1. Pet collection screen with sorting/filtering.
2. Roster selection for battles.
3. Rarity tiers and visual evolution.
4. Color-coded grammar quest slots.
5. PWA/offline support for Chromebooks.
6. End-of-session parent report with conversation prompts.
7. itch.io storefront with screenshots and trailer.

### Nice-to-Have (Post-Launch)
1. Full Android XR build.
2. Classroom roster and teacher dashboard.
3. Expansion packs (SAT, science vocab, Spanish-English).
4. Nuisance letters and companion follow system.
5. Pet Dream Layer and advanced social sharing.

---

## 8. Validation Plan

| Need | Validation Method | Success Metric |
|------|-------------------|----------------|
| Child engagement | Playtesting with 5–10 kids ages 6–12 | 80%+ complete one full game loop |
| Parent trust | Survey of 20 homeschool parents | 70%+ say they would buy after demo |
| Educational value | Pre/post vocabulary check with small group | Measurable improvement in target words |
| Platform fit | Test web demo on 3 Chromebook models | Loads in <10s, runs without crashes |
| Pricing acceptance | A/B test $9.99 vs. $14.99 on itch.io | Conversion rate >3% at chosen price |
| Teacher interest | Interviews with 5 teachers/literacy specialists | 3+ express interest in classroom pilot |

---

## 9. Conclusion

LitTCG addresses a genuine, multi-sided need: children want engaging play, parents want educational value, and educators want evidence. The product's core design — turning words into creatures — is the mechanism that satisfies all three. The highest-priority unmet needs are **trust, visible progress, and platform availability**. The product must ship a stable, polished demo that proves these needs are met before investing in broader marketing or institutional sales.

---

*This document should be reviewed and approved by stakeholders before proceeding to detailed product and marketing planning.*
