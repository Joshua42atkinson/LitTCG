# LitTCG Grant Capitalization Strategy

This document translates the LitTCG stealth-assessment architecture into a fundable roadmap for Maine-based and federal educational technology grants.

## 1. Core Technical Thesis

LitTCG is a local-first, game-based psychometric assessment engine. It embeds **Evidence-Centered Design (ECD)** into ordinary play:

- **Competency Model:** Common Core State Standards (CCSS) ELA Grades 9–12 — vocabulary acquisition, syntax, and figurative/pragmatic language comprehension.
- **Task Model:** The existing combat loop and NPC Mad-Lib quest slots.
- **Evidence Model:** Real-time computation of **HD-D lexical diversity**, **MTLD**, **syntactic complexity ratios**, and **FACES pragmatics resonance**, persisted to `save.json` for external analysis.

This framework satisfies the federal mandate for **evidence-based interventions** under ESSA and provides the scientific rigor required by SBIR/STTR reviewers.

## 2. Near-Term: Maine Technology Institute (MTI)

### Program: Business Innovation Funding (BIF)

| Field | Value |
|---|---|
| **Tier** | Pre-revenue / very early revenue |
| **Target ask** | Up to **$30,000** |
| **Match** | 1:1 minimum; 50% cash for $0–$15k; 70% cash for $15,001–$30k |
| **Eligibility** | Maine-based org with C-suite W2 or principal founder resident in Maine full-time |
| **Use of funds** | Finalize Phase 8–10: HD-D/MTLD telemetry, CCSS metadata tagging, Institutional Dashboard serialization |

### Innovation Argument

Existing literacy tools fall into two weak categories:

1. **Decontextualized drills** (flashcards, worksheets) — no stealth assessment, no gameplay.
2. **Gamified drills** (basic vocabulary apps) — engagement without psychometric validity.

LitTCG is **substantially different** because the game mechanic *is* the assessment:

- A synonym/antonym cast produces semantic distance evidence.
- A 3-card sentence plot with alliteration/oxymoron/hyperbole produces syntactic-complexity evidence.
- A word cast against an environmental FACES state produces pragmatic-context evidence.

The HD-D and MTLD implementations turn these short-burst interactions into standardized lexical-diversity scores, removing the text-length bias that makes raw TTR unreliable.

## 3. Mid-Term: Federal SBIR / STTR

### Phase I: Feasibility

| Field | Value |
|---|---|
| **Amount** | Up to **$256,000** |
| **Duration** | 6–12 months |
| **Success rate** | 10–15% |
| **Core question** | Can game-based psycholinguistics unobtrusively measure adolescent and adult literacy complexity? |

### Phase II: Development & Commercialization

| Field | Value |
|---|---|
| **Amount** | Up to **$1,250,000** |
| **Duration** | ~24 months |
| **Success rate** | 40–50% (Phase I awardees only) |
| **Deliverables** | Validated dashboard, district pilots, commercialization plan |

### SBIR/STTR Eligibility Checklist

- [ ] For-profit, U.S. owned and operated
- [ ] < 500 employees
- [ ] Research performed by the applicant (LitTCG engine + metric implementations)
- [ ] Principal investigator committed ≥ 10% effort

### MTI TAP Support

MTI offers free **Technical Assistance Program (TAP)** support for SBIR/STTR applications, including strategy meetings and proposal reviews.

## 4. Institutional / District Funding Streams

### 4.1 Title IV, Part A — Student Support and Academic Enrichment (SSAE)

- Maine FY2025 allocation: ~**$6,693,000**.
- Allowable uses include effective-use-of-technology programming, blended learning projects, and digital-learning access for rural/underserved areas.
- LitTCG alignment:
  - Local-first WASM build runs on low-end hardware and poor internet.
  - Combines literacy instruction with interactive digital environment.
  - Serves high-need and underrepresented student populations.

### 4.2 Maine Learning Technology Initiative (MLTI) TeachWithTech

- Competitive grants for technology supplies and professional learning.
- Priority for socioeconomically disadvantaged districts and underrepresented students.
- Requires grant recipients to share success (school-board presentation, conference talk, async professional-learning video, or model lesson).
- LitTCG alignment:
  - Exportable, anonymized student growth reports from the Institutional Dashboard.
  - Ready-made visualizations for presentations and model lessons.

### 4.3 21st Century Community Learning Centers (21st CCLC)

- Maine recently announced ~**$1.75M** for after-school and summer-learning programs.
- Targets high-need, economically disadvantaged communities.
- LitTCG alignment:
  - Self-directed, game-based Socratic tutoring for high schoolers and adult learners.
  - Complements school-day academic programs without requiring 1:1 staffing.

### 4.4 Maine Adult Education Grants

- Focus on ELL courses, vocational language training, and career/college success supports.
- LitTCG alignment:
  - Target demographic explicitly includes adult learners (ages 13+).
  - FACES protocol teaches pragmatic contextualization and advanced vocabulary.

## 5. Philanthropic & Cross-Disciplinary Pathways

### 5.1 Maine Space Grant Consortium (MSGC) STEM4ME

- Grants up to **$5,000/year for 2 years**.
- For publicly funded middle/high school educators.
- Requires fiscal sponsor / partner organization.
- LitTCG positioning:
  - Procedural generation, Rust engine, state-driven procedural music, and data telemetry bridge literacy and computational technology.
  - Can be framed as a STEAM project linking humanities and code.

### 5.2 Harold Alfond Center for the Advancement of Maine's Workforce

- Focus on adult learners and workforce transition.
- LitTCG positioning:
  - Pilot integration with Maine Community College System adult-education or workforce-transition programs.
  - Generate verifiable adult-literacy growth metrics via the Institutional Dashboard.

### 5.3 Maine Community Foundation (MaineCF)

- Community Building Grants up to **$100,000** for new projects.
- **Constraint:** For-profit entities cannot apply directly.
- Requires a **fiscal sponsor** (501(c)(3), public school, municipality, tribal government).
- LitTCG positioning:
  - Partner with a Maine public school, municipal library, or literacy nonprofit.
  - Serve as the core technological intervention in a joint grant proposal.
  - Required: formal letter of agreement from principal, superintendent, or partner organization.

## 6. Matching-Fund Financial Planning

| Cumulative MTI Funding | Minimum Cash Match | Permissible In-Kind |
|---|---|---|
| $0 – $15,000 | 50% | Sweat equity, in-kind contributions |
| $15,001 – $30,000 | 70% | In-kind up to 30% |
| $30,001 – $60,000 | 80% | In-kind up to 20% |
| > $60,000 | 100% | None |

**Eligible cash-match items:** actual dollars paid for project materials, IP costs, software-development expenses, and federal SBIR/STTR awards.

## 7. Recommended Application Sequence

1. **MTI BIF ($30k)** — finalize telemetry and dashboard; establish Maine presence.
2. **SBIR Phase I ($256k)** — validate game-based psychometric model with pilot cohort.
3. **MLTI / Title IV-A pilots** — generate district testimonials and efficacy data.
4. **SBIR Phase II ($1.25M)** — build enterprise dashboard and scale pilots.
5. **MaineCF / STEM4ME / Harold Alfond** — use fiscal sponsors for broader community pilots.

## 8. Key Documents to Produce

- [ ] MTI BIF application outline (technical + financial)
- [ ] SBIR Phase I proposal draft (specific aims, research design, commercialization plan)
- [ ] Institutional one-pager for district administrators (FERPA/COPPA, CCSS alignment, IEP outputs)
- [ ] Fiscal-sponsor solicitation letter template
- [ ] Pilot agreement template for Maine schools / tutoring centers

---

*This strategy is a living document. Update it as funding opportunities, eligibility rules, and pilot data evolve.*
