# LitTCG Documentation Index

> Map of all project documents. Keep this current as the project evolves.
>
> **Last updated:** July 7, 2026 — refreshed after Phase 1.6 cleanup and demo sprint planning.

## Primary Documents

| Document | Purpose | Audience | Status |
|----------|---------|----------|--------|
| `GDD.md` | Single source of truth for game design | Team | Current |
| `ARCHITECTURE.md` | ECS architecture, module list, data flow | Developers | Current |
| `TECHNICAL_MANUAL.md` | Build pipeline, cross-project patterns, Voix Vive/Trinity integration | Developers | Current |
| `ROADMAP.md` | Development phases and shipping checklist | Team | Current |
| `BRAND_GUIDE.md` | Name usage, voice, tone, visual identity | Marketing | Current |

## Marketing & Sales

| Document | Purpose | Audience | Status |
|----------|---------|----------|--------|
| `docs/WIFE_REVIEW_ONE_PAGER.md` | One-page game explanation for non-technical review | Family, investors | Current |
| `docs/PROFESSIONAL_MARKETING_STRATEGY.md` | Full go-to-market strategy | Team | Current |
| `docs/LITTCG_SCHOOLS_GO_TO_MARKET_PLAN.md` | B2B school sales plan | Team | Current |
| `docs/LITTCG_ANDROID_PARENT_CHILD_SALES_PLAN.md` | Android consumer sales plan | Team | Superseded by schools + hybrid plan |
| `docs/LITTCG_PRODUCT_DIRECTION_DECISION.md` | Product direction options and recommendation | Team | Current |
| `docs/LITTCG_REUSABLE_ASSETS_AND_MONETIZATION.md` | Cross-project reusable assets and monetization | Team | Current |
| `docs/MARKETING_RESEARCH_WIFE_REVIEW.md` | Initial marketing research | Team | Archive after review |
| `docs/WHY_AND_AUDIENCE_RESEARCH.md` | Audience and ecosystem research | Team | Archive after review |
| `docs/PROFESSIONAL_NEEDS_ANALYSIS.md` | Stakeholder needs analysis | Team | Archive after review |
| `docs/PROFESSIONAL_GAP_ANALYSIS.md` | Product/technical gap analysis | Team | Current |
| `docs/ADDITIONAL_OPPORTUNITIES_AND_SALES_IMPROVEMENTS.md` | Opportunities and sales improvements | Team | Archive after review |

## Engineering

| Document | Purpose | Audience | Status |
|----------|---------|----------|--------|
| `LitTTC/task.md` | Autonomous task tracker | Developers | Current |
| `LitTTC/src/asset_catalog.rs` | Central asset path catalog | Developers | Current |
| `LitTTC/README.md` | Build instructions and tech overview | Developers | Current |
| `docs/2D_MODE.md` | 2D mode documentation | Developers | Current |
| `docs/DEMO_CHECKLIST.md` | 30-day sprint to ship a playable web demo | Developers | Current |
| `docs/CROSS_PROJECT_INTEGRATION_PLAN.md` | What to pull from Voix Vive, Trinity, etc. | Developers | Current |
| `docs/ANDROID_NDK_TODO.md` | Android total system integration plan (NDK/SDK/Play/CI) | Developers | Current |

## Product & Research

| Document | Purpose | Audience | Status |
|----------|---------|----------|--------|
| `legacy/old_docs/MASTER_DESIGN_DOCUMENT.md` | Previous full design document | Team | Historical |
| `legacy/old_docs/Semantic Slime_*` | Previous feature specs | Team | Historical |
| `legacy/old_docs/Spatial Computing Dev Workstation Blueprint.md` | Hardware workstation setup | Developers | Historical |

## External Project References

| Project | Location | Relevance |
|---------|----------|-----------|
| Voix Vive | `/home/joshua/Workflow/Bertrand-Masterclass` | Hand tracking, spatial UI, voice control, Google OAuth → Gemini, i18n |
| Voix Vive iOS | `/home/joshua/Workflow/VoixVive-iOS` | Tauri iOS build, React dashboard scaffold |
| Trinity ID AI OS | `/home/joshua/Workflow/TRINITYIDAIOS` | Socratic prompts, ADDIECRAPEYE, persistent memory, RAG, EYE export |
| LDTAtkinson Business | `/home/joshua/Workflow/LDTAtkinson` | Monetization, grants, business structure |

## Next Document Actions

- [x] Update `ARCHITECTURE.md` to reflect command-driven architecture and asset catalog.
- [x] Update `GDD.md` status block with current test count and completed cleanup.
- [x] Create `DEMO_CHECKLIST.md` for the next 30-day sprint.
- [x] Create `CROSS_PROJECT_INTEGRATION_PLAN.md` for Voix Vive / Trinity pull list.
- [ ] Archive or merge superseded marketing docs into one `STRATEGY_ARCHIVE.md`.
- [ ] Execute the 30-day demo sprint from `DEMO_CHECKLIST.md`.
