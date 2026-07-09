# LitTCG — Maine Technology Institute Business Innovation Funding Brief

**Project:** LitTCG (Literary Trading Card Game)  
**Applicant:** [TBD — Maine-based entity / founder-resident]  
**Program:** MTI Business Innovation Funding (BIF), pre-revenue tier  
**Target Ask:** $30,000  
**Match:** 70% cash ($21,000) / 30% in-kind for the $15,001–$30,000 tier  

---

## 1. Problem Statement

Adolescent and adult literacy tools fall into two weak categories:

1. **Decontextualized drills** (flashcards, worksheets) measure vocabulary breadth without capturing how a learner uses words in context.
2. **Gamified drills** add engagement but lack psychometric validity, so they cannot satisfy federal evidence-based intervention requirements under ESSA or produce IEP-ready progress data.

High schoolers, adult learners, and educators need a literacy intervention that is **engaging enough to drive voluntary practice** and **rigorous enough to generate standards-aligned evidence**.

## 2. Innovation

LitTCG is a local-first, game-based psychometric assessment engine. Every player action — spelling a word, casting it in battle, filling a Mad-Lib quest slot — silently produces evidence mapped to the Common Core State Standards (CCSS) ELA Grades 9–12.

Key technical differentiators:

- **Evidence-Centered Design (ECD)** framework: Competency, Task, and Evidence models are embedded directly into gameplay.
- **HD-D and MTLD lexical diversity metrics** computed over a rolling window of player casts, mitigating the text-length bias that makes raw TTR unreliable.
- **Syntactic complexity ratio** derived from literary-device combo usage (oxymoron, hyperbole, palindrome, alliteration).
- **FACES pragmatics axis** measures contextual flexibility by comparing a word's intrinsic emotional face against the Slime's active face.
- **Local-first architecture**: all telemetry stays in `save.json` on the learner's device. No cloud, no accounts, no third-party trackers — satisfying FERPA/COPPA review by default.

## 3. Technical Proof

The engine computes HD-D as the mean probability, across all word types in the rolling cast window, of drawing at least one token of each type in a random sample. MTLD uses a factor-size procedure to measure how many tokens are required to accumulate a fixed number of unique types. Both metrics are standard in psycholinguistics and have been unit-tested in the Rust engine with property-based assertions.

CCSS mapping is data-driven: etymology roots and suffixes carry standard tags (e.g., `L.9-10.5` for figurative language, `L.11-12.3` for language functions), and gameplay mechanics add mechanical mappings (FACES resonance → `L.9-10.5`, combos → `L.11-12.3`, quest slots → `L.9-10.4`).

## 4. Commercialization

- **B2C:** $9.99 one-time desktop/web purchase; $4.99 expansion packs.
- **B2B:** site-license Institutional Dashboard that reads aggregated `save.json` files and outputs CCSS-aligned, IEP-ready reports for districts and adult-education programs.

## 5. Use of Funds

| Activity | Allocation |
|---|---|
| Finalize HD-D/MTLD integration and dashboard rendering | $10k |
| CCSS metadata expansion across the 9,582-word database | $8k |
| Maine district / tutoring-center pilot design | $7k |
| Grant-writing support (SBIR Phase I narrative, MTI TAP) | $5k |

## 6. Expected Outcomes

- A playable 2D demo with live psychometric output.
- A Maine pilot agreement in principle with at least one high school or adult-education partner.
- SBIR Phase I proposal ready for submission within 6 months of MTI award.
