# LitTCG Implementation Plan

**Goal:** Close the gap between the current code (2D gray-box combat) and the v2 GDD (Vessel/Payload + FACES resonance + NPC scenario training + three-axis grading).

**Target audience:** High school and adult learners (ages 13+).

**Validation rule:** Run `cargo test` after every code change. All 8 integration tests must pass.

---

## Phase 1: Word Intrinsic FACES ✅

**Files:** `src/core/components.rs`, `src/core/letter.rs`, `src/core/deck.rs`, `src/core/save.rs`, `tests/integration_tests.rs`

**Changes:**
1. Added `pub faces: Option<PetFacesState>` to `SpellBookEntry`.
2. Updated `SpellBook::record_encounter()` to accept and store the word's intrinsic FACES.
3. Updated all `record_encounter()` call sites to pass the new parameter.
4. Added unit tests covering the new field.

**Validation:** `cargo test` → lib 53 passed, integration 34 passed.

**Success:** Every word in the SpellBook has a cached 32-bit FACES state.

---

## Phase 2: Slime Contextual FACES ✅

**Files:** `src/core/components.rs`, `src/core/battle.rs`, `src/core/commands.rs`

**Changes:**
1. Added `SlimeFace::to_faces_state()` mapping each 2D preset to a full `FacesState`.
2. Expanded `ActiveFace` to carry both the UI preset (`face: SlimeFace`) and the full register (`faces: FacesState`).
3. Updated face selection handlers to set both fields.
4. Changed `calculate_spell_damage()` to accept `FacesState` and derive the modifier via `nearest_slime_face_preset()`.
5. Updated `FaceChanged` and `SpellCast` events to carry `FacesState`.

**Validation:** `cargo test --lib` → 53 passed; integration tests → 34 passed.

**Success:** The Slime carries a full 32-bit FACES register that can be compared with word FACES.

---

## Phase 3: FACES Resonance Math ✅

**Files:** `src/core/battle.rs`

**Changes:**
1. Added `compute_resonance(intrinsic: FacesState, contextual: FacesState) -> f32` using byte-level comparison.
2. Added `resonance_multiplier(resonance: f32) -> f32` mapping alignment to damage modifiers.
3. Integrated resonance into `play_battle_card()` and `cast_sentence()`:
   - Looks up the played word's intrinsic FACES from `SpellBookEntry`.
   - Compares it to `ActiveFace.faces`.
   - Applies a resonant (+50%), partial (+10%), neutral (1.0x), or dissonant (-30%) multiplier.
4. Added fallback to nearest preset when a word has no cached FACES.
5. Added tests for perfect resonance, orthogonal dissonance, and multiplier tiers.

**Validation:** `cargo test --lib` → 56 passed; integration tests → 34 passed.

**Success:** Same word does different damage depending on the Slime's face and the situation.

---

## Phase 4: Three-Axis Grading ✅

**Files:** `src/core/components.rs`, `src/core/battle.rs`, `src/core/quest.rs`, `src/core/commands.rs`, `src/core/save.rs`

**Changes:**
1. Added `GradeScores { syntax, semantics, pragmatics }` struct with serde support.
2. Added `last_grades: GradeScores` to `CharacterSheet` for persistence.
3. In `play_battle_card()`:
   - **Syntax:** POS relationship between played word and typo (same/complementary POS score higher).
   - **Semantics:** synonym (< 2.0) or antonym (> 4.0) distance.
   - **Pragmatics:** raw FACES resonance before multiplier mapping.
4. Added `grades: GradeScores` to `BattleResult`.
5. Updated `complete_quest()` to accept `GameDatabase`, compute per-slot grades, aggregate them, and store on `CharacterSheet`.
6. Updated all command handlers and tests to pass the new parameter.

**Validation:** `cargo test --lib` → 58 passed; integration tests → 34 passed.

**Success:** Every cast/quest completion produces three axis scores.

---

## Phase 5: Environmental FACES on Quests ✅

**Files:** `src/core/database.rs`, `src/core/quest.rs`, `src/core/commands.rs`, `assets/quest_data.json`

**Changes:**
1. Extended `NpcQuest` (QuestData) with `expected_faces: Option<FacesState>` and `socratic_failure: Option<String>`.
2. Extended `QuestSession` with the same fields and copied them in `start_quest()`.
3. Updated `fill_slot()` to accept an optional `SpellBook` and return `Option<String>`:
   - Validates the played word's intrinsic FACES against the quest's expected environmental FACES.
   - On low resonance (< 0.55), returns the Socratic failure string and refuses the slot fill.
4. Updated command handlers to pass the spellbook and surface Socratic failures.
5. Added example `ExpectedFaces`/`SocraticFailure` to the first Innocent quest in `quest_data.json`.
6. Added tests for FACES mismatch returning the Socratic prompt and preventing slot fill.

**Validation:** `cargo test --lib` → 59 passed; integration tests → 34 passed.

**Success:** Quests can require a specific emotional mood, and failure is a Socratic prompt.

---

## Phase 6: NPC Scenario Training ✅

**Files:** `src/core/database.rs`, `src/core/quest.rs`, `src/core/commands.rs`, `src/core/battle.rs`, `assets/lore_db.json`

**Changes:**
1. Added `subject: String` and `scenario_text: String` to both `NpcData` and `NpcQuest`.
2. Added the same fields to `QuestSession` and propagated them in `start_quest()` (quest values override NPC fallbacks).
3. Added scenario text and subject labels to the 2D quest UI spawn and update text.
4. Added `subject_mastery: HashMap<String, u32>` to `VaamMetrics` with `record_subject_mastery()`.
5. Updated `complete_quest()` to accept optional `VaamMetrics` and record mastery for the quest's subject.
6. Populated `Subject` and `ScenarioText` for all 12 NPCs in `assets/lore_db.json` (e.g., simple-past, negation, tone, active-voice, noun-formation).
7. Added test verifying subject mastery is recorded when a quest completes.

**Validation:** `cargo test --lib` → 59 passed; integration tests → 34 passed.

**Success:** Each NPC presents a narrative problem tied to a language subject, and subject mastery is tracked.

---

## Phase 7: Slime Levels and Card XP ✅

**Files:** `src/core/components.rs`, `src/core/battle.rs`, `src/core/quest.rs`, `src/core/commands.rs`, `src/lib.rs`

**Changes:**
1. Added `SlimeLevel { xp, level, evolution_stage }` resource with `xp_for_level()`, `recalc()`, and `add_xp()` helpers.
2. Added `card_xp: u32` to `SpellBookEntry` with `mastery_from_xp()`, `recalc_mastery()`, and `add_card_xp()` helpers.
3. Awarded card XP and global Slime XP in `play_battle_card()` for effective casts.
4. Awarded per-word card XP and global Slime XP in `cast_sentence()`.
5. Awarded per-word card XP and global Slime XP in `complete_quest()` on quest completion.
6. Mapped `card_xp` thresholds (10/50/100) to the existing `MasteryLevel` tiers (Encountered/Experienced/Owned/Mastered).
7. Wired `SlimeLevel` through `CommandContext`, registered it as a startup resource, and passed it to all relevant battle/quest call sites.
8. Added tests verifying card XP and Slime XP increase after battle casts and quest completion.

**Validation:** `cargo test --lib` → 59 passed; integration tests → 34 passed.

**Success:** Words level up individually, and the Slime levels up globally.

---

## Phase 8: FACES Connotation Metrics / Stealth Assessment Telemetry

**Files:** `src/core/components.rs`, `src/core/battle.rs`, `src/core/quest.rs`, `src/core/save.rs`

**Changes:**
1. Refactor `GradeScores` into a history-aware telemetry carrier by adding a per-session `cast_log: Vec<CastTelemetry>` field (no UI display).
2. Overhaul `VaamMetrics` to implement the **Evidence-Centered Design (ECD)** Evidence Model:
   - Lexical Diversity Score (LDS) using **HD-D** (Hypergeometric Distribution D) over the rolling cast/token window.
   - Secondary **MTLD** windowed index for cross-validation.
   - Syntactic Complexity ratio: frequency of multi-card literary-device combos (Echo/Armor Piercing/Overcharge) vs single-card casts over time.
   - Temporal arrays for `syntax`, `semantics`, `pragmatics`, `subject_mastery`, and `lexical_diversity`.
3. Update `play_battle_card()` and `complete_quest()` to append telemetry and recalculate HD-D/MTLD in real time.
4. Persist the new arrays in `save.json` through `CharacterSheet` and `VaamMetrics` serde.

**Success:** The engine silently outputs standardized, scientifically valid psychometric data without changing the visible game loop.

---

## Phase 9: CCSS ELA Metadata & Standard Mapping

**Files:** `assets/etymology_db.json`, `assets/lore_db.json`, `src/core/database.rs`

**Changes:**
1. Add hidden `ccss_tags: Vec<String>` to etymology roots and literary-device records.
2. Map existing mechanics to Common Core State Standards ELA Grades 9–12:
   - FACES Resonance → L.9-10.5 (figurative language, word relationships, nuances).
   - Literary Device Combos → L.11-12.3 (language functions in different contexts).
   - Socratic Tutor Loop → L.9-10.3 (language functions in different contexts).
   - Mad-Lib Quest Slots → L.9-10.4 (unknown/multiple-meaning words and phrases).
3. Expose a `ccss_coverage: HashMap<String, u32>` in `VaamMetrics` that counts demonstrated standard exposures.

**Success:** Every gameplay action can be translated into an institutional standard code for dashboards and grant reporting.

---

## Phase 10: Institutional Telemetry Serialization

**Files:** `src/core/components.rs`, `src/core/save.rs`, `dashboard/index.html`

**Changes:**
1. Re-engineer `CharacterSheet` and `VaamMetrics` serde output to emit clean, standardized arrays:
   - `syntax_series`, `semantics_series`, `pragmatics_series`, `lexical_diversity_series`, `syntactic_complexity_series`, `subject_mastery`, `ccss_coverage`.
2. Keep all data local-first: no cloud upload, no accounts.
3. Update the existing HTML dashboard (`dashboard/index.html`) to parse the new arrays and render:
   - Per-standard coverage heatmap.
   - Lexical diversity trend line.
   - Syntactic complexity ratio over time.
   - IEP-friendly raw numbers table.

**Success:** An educator or administrator can drag a `save.json` into the local dashboard and generate a CCSS-aligned progress report.

---

## Phase 11: 2D Demo Polish

**Files:** `src/core/hud.rs`, `src/core/render.rs`, `src/core/battle.rs`

**Changes:**
1. Show the Slime's current face in the 2D battle UI.
2. Show the word card's intrinsic FACES icon.
3. Show the three-axis grade breakdown after each cast.
4. Show Socratic failure text in quest UI.

**Success:** The 2D demo communicates the new mechanics clearly.

---

## Phase 12: Grant Capitalization & Go-to-Market Documentation

**Files:** `docs/GRANT_STRATEGY.md` (new), `MARKETING_PLAN.md`, `GDD.md`

**Changes:**
1. Document the Maine Technology Institute (MTI) Business Innovation Funding path ($30k pre-revenue tier, 1:1 match sliding scale).
2. Document SBIR/STTR Phase I ($256k) / Phase II ($1.25M) pipeline and MTI TAP support.
3. Document Title IV-A (SSAE), MLTI TeachWithTech, 21st CCLC, and Maine adult-education grant alignment.
4. Document MSGC STEM4ME, Harold Alfond Center, and MaineCF fiscal-sponsorship pathways.
5. Update GDD Commercialization and Marketing Plan monetization tables with the B2B Institutional Dashboard tier.

**Success:** The project has a written capitalization roadmap that leverages the new telemetry as proof of technical innovation.

---

## Order of Execution

1. Phase 1 (Word FACES) — unblock everything else.
2. Phase 2 (Slime FACES) — unblock resonance.
3. Phase 3 (Resonance math) — first visible gameplay change.
4. Phase 4 (Three-axis grading) — assessment layer.
5. Phase 5 + 6 (Quest FACES + NPC scenarios) — LIT side.
6. Phase 7 (Slime/Card levels) — progression.
7. Phase 8 (Connotation metrics / stealth assessment) — analytics.
8. Phase 9 (CCSS metadata) — standard alignment.
9. Phase 10 (Institutional serialization + dashboard) — B2B reporting.
10. Phase 11 (2D polish) — ship the demo.
11. Phase 12 (Grant capitalization docs) — funding runway.

---

## Notes

- Keep all changes behind existing feature flags (`flat2d`, `desktop`, `xr`).
- Do not add new heavy dependencies. The FACES math stays byte arithmetic; HD-D/MTLD use only integer/floating arithmetic.
- Every phase must pass `cargo test` before the next phase starts.
- All telemetry additions must be silent — no new buttons, cards, or mandatory UI screens.
