# Monolithic Master Systems Blueprint: Sovereign Systems Architecture Portfolio

This document is the absolute, unified repository compiling all files, components, enums, data structures, schemas, and logic snippets audited from the `/home/joshua/Workflow` workspace. It forms the technical and pedagogical basis for the **Communication Curriculum** project.

---

## 1. ECS Components

### Channel
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/components.rs`
- **What It Does:** Enforces the "Four Channels" framework (Mind/Heart/Body/Action) on word/card entities, defining color codes, query prompts, and attunement properties.
- **Reusable For:** Shared across all games to classify vocabulary words and pets.
- **Code Snippet:**
  ```rust
  #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub enum Channel {
      Mind,   // Green
      Heart,  // Orange
      Body,   // Blue
      Action, // Gold
  }

  impl Channel {
      pub fn color(&self) -> Color {
          match self {
              Channel::Mind   => Color::srgba(0.29, 0.62, 0.43, 1.0),
              Channel::Heart  => Color::srgba(0.83, 0.47, 0.24, 1.0),
              Channel::Body   => Color::srgba(0.29, 0.49, 0.71, 1.0),
              Channel::Action => Color::srgba(0.77, 0.64, 0.24, 1.0),
          }
      }
  }
  ```
- **Dependencies:** `bevy::prelude::Color`
- **Porting Notes:** Directly reusable.

### Stage
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/components.rs`
- **What It Does:** Defines the four cognitive progression stages (Hero, Outlaw, EdgeLord, BestSelf) mapping to word mastery tiers and UI indicator symbols.
- **Reusable For:** Shared across all three games for progression tracking.
- **Code Snippet:**
  ```rust
  #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
  pub enum Stage {
      Hero,
      Outlaw,
      EdgeLord,
      BestSelf,
  }

  impl Stage {
      pub fn stars(&self) -> &'static str {
          match self {
              Stage::Hero     => "★",
              Stage::Outlaw   => "★★",
              Stage::EdgeLord => "★★★",
              Stage::BestSelf => "★★★★",
          }
      }
  }
  ```
- **Dependencies:** Bevy ECS macros.
- **Porting Notes:** Directly compatible with Bevy 0.18.1.

### Symbol
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/components.rs`
- **What It Does:** Defines the "ARCANA Symbols" mapping parts of speech (Nouns, Verbs, Adjectives, Abstracts, Key Terms) to game board actions and interactions.
- **Reusable For:** *Gramer Golum* (direct grammar symbols mapping) and shared systems.
- **Code Snippet:**
  ```rust
  #[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub enum Symbol {
      Stone, // Noun (persists on field)
      Spark, // Verb (one-time cast effect)
      Prism, // Adjective/Adverb (attaches to boost another card)
      Void,  // Abstract concept (resonates with any card)
      Star,  // Key term / proper noun (anchors a synergy chain)
  }
  ```
- **Dependencies:** Bevy ECS macros.
- **Porting Notes:** Directly reusable. Maps to the grammar stage spelling mechanics.

### GlitchSlime
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/combat.rs`
- **What It Does:** Represents a slime entity with health, speed, and basic wander AI timer.
- **Reusable For:** *Semantic Slime* (as base logic for pet Slime movement or wild slimes).
- **Code Snippet:**
  ```rust
  #[derive(Component)]
  pub struct GlitchSlime {
      pub health: i32,
      pub speed: f32,
      pub wander_timer: Timer,
      pub direction: Vec3,
  }
  ```
- **Dependencies:** `bevy::prelude::{Component, Timer, Vec3}`
- **Porting Notes:** Update `Timer` declarations to comply with Bevy 0.18.1.

### HandJointMarker
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy/src/hand_tracking.rs`
- **What It Does:** Marks visual joint meshes representing bones in the XR hand tracking system.
- **Reusable For:** Android XR spatial hand tracking on Quest/Aura platforms.
- **Code Snippet:**
  ```rust
  #[derive(Component)]
  pub struct HandJointMarker {
      pub hand: u8,  // 0 = Left, 1 = Right
      pub joint: u8, // Joint index (0-25)
  }
  ```
- **Dependencies:** Bevy ECS Component macro.
- **Porting Notes:** Map to OpenXR bone layouts.

---

## 2. ECS Resources

### CharacterSheet
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-protocol/src/character_sheet.rs`
- **What It Does:** Tracks the player's cognitive attunement scores across the four channels, tracking overall levels and lifetime XP.
- **Reusable For:** Shared save system and stealth assessment dashboard across all games.
- **Code Snippet:**
  ```rust
  #[derive(Resource, Debug, Clone, Serialize, Deserialize)]
  pub struct CharacterSheet {
      pub user_id: Uuid,
      pub alias: String,
      pub user_class: UserClass,
      pub resonance_level: u32,
      pub total_xp: u64,
      pub current_coal: f32,
      pub mana_pool_vram: u32,
      pub stamina_ram: u32,
      pub agility_compute: u32,
      pub concurrency_mode: ConcurrencyMode,
      pub genre: Genre,
      pub vocabulary_pack_id: Option<Uuid>,
      pub creative_config: CreativeConfig,
      pub audio_preferences: AudioPreferences,
      pub skills: HashMap<SkillType, f32>,
      pub completed_contracts: Vec<Uuid>,
      pub vaam_profile: VaamProfile,
      pub appearance: Option<String>,
      pub backstory: Option<String>,
      pub alignment: Option<String>,
      pub current_quest_flavor: Option<String>,
      pub intent_posture: IntentPosture,
      pub session_intent: Option<String>,
      pub vulnerability: f32,
      pub grounding_complete: bool,
      pub shadow_status: ShadowStatus,
      pub current_steam: f32,
      pub track_friction: f32,
      pub cargo_slots: u8,
      pub locomotive_profile: LocomotiveProfile,
      pub thrash_count: u32,
      pub last_interaction_timestamp: u64,
      pub ldt_portfolio: LdtPortfolio,
      pub experience: Option<String>,
      pub audience: Option<String>,
      pub success_vision: Option<String>,
      pub consecutive_negatives: u8,
      pub scope_hope_backlog: Vec<String>,
  }
  ```
- **Dependencies:** `serde`, `uuid`.
- **Porting Notes:** Directly reusable.

### SpellBook
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/components.rs`
- **What It Does:** Stores the player's vocabulary inventory, recording mastery levels and times encountered for each word.
- **Reusable For:** Shared spelling collection inventory system.
- **Code Snippet:**
  ```rust
  #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
  pub enum MasteryLevel {
      Encountered,
      Experienced,
      Owned,
      Mastered,
  }

  #[derive(Clone, Debug)]
  pub struct SpellBookEntry {
      pub word: String,
      pub channel: Channel,
      pub mastery: MasteryLevel,
      pub times_encountered: u32,
  }

  #[derive(Resource, Debug, Default)]
  pub struct SpellBook {
      pub entries: Vec<SpellBookEntry>,
  }
  ```
- **Dependencies:** `bevy::prelude::Resource`
- **Porting Notes:** Directly reusable.

### HandTrackingState
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy/src/hand_tracking.rs`
- **What It Does:** Stores the tracked world-space positions of fingertips and wrists for both hands, along with calculated string/fret index positions.
- **Reusable For:** XR Shell / Android XR hand-gesture spatial interaction logic.
- **Code Snippet:**
  ```rust
  #[derive(Resource, Default)]
  pub struct HandTrackingState {
      pub left_index_tip: Option<Vec3>,
      pub right_index_tip: Option<Vec3>,
      pub left_wrist: Option<Vec3>,
      pub right_wrist: Option<Vec3>,
      pub detected_fret: Option<usize>,
      pub detected_string: Option<usize>,
  }
  ```
- **Dependencies:** `bevy::prelude::{Resource, Vec3}`
- **Porting Notes:** Directly compatible with Bevy 0.18.1.

### Inventory
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/inventory.rs`
- **What It Does:** Manages locked/unlocked state for tools and "Thinking Caps" (AI configurations).
- **Reusable For:** In-game inventory and equipment screens.
- **Code Snippet:**
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
  pub enum ToolId {
      OllamaCompass,
      LogicLens,
      FeedbackMirror,
      ThinkingCapPhi,
      ThinkingCapLlama,
      ThinkingCapMistral,
  }

  #[derive(Resource, Clone, Serialize, Deserialize)]
  pub struct Inventory {
      pub tools: HashMap<ToolId, bool>,
      pub active_tool: Option<ToolId>,
      pub active_hat: Option<ToolId>,
      pub newly_acquired: Option<ToolId>,
  }
  ```
- **Dependencies:** `bevy::prelude::Resource`, `serde`
- **Porting Notes:** Directly reusable.

### QuizManager
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/quiz.rs`
- **What It Does:** Tracks quiz questions, correct selections, and current scoring state.
- **Reusable For:** In-game curriculum evaluations.
- **Code Snippet:**
  ```rust
  #[derive(Resource)]
  pub struct QuizManager {
      pub current_question_index: usize,
      pub score: u32,
      pub questions: Vec<QuizQuestion>,
  }

  pub struct QuizQuestion {
      pub correct_index: usize,
      pub feedback_correct: String,
      pub feedback_incorrect: String,
  }
  ```
- **Dependencies:** `bevy::prelude::Resource`
- **Porting Notes:** Directly portable.

### MemoryStoreResource
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/ai/memory.rs`
- **What It Does:** Wraps the local `MemoryStore` into a Bevy ECS resource.
- **Reusable For:** Providing local-first AI memory access in systems.
- **Code Snippet:**
  ```rust
  #[derive(Resource, Clone)]
  pub struct MemoryStoreResource(pub Arc<MemoryStore>);
  ```
- **Dependencies:** `bevy::prelude::Resource`, `std::sync::Arc`
- **Porting Notes:** Directly reusable.

### AudioState
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/companion-app/src-tauri/src/audio.rs`
- **What It Does:** Tracks recording status state variables.
- **Reusable For:** Managing audio recording locks inside Tauri/Tauri NDK backends.
- **Code Snippet:**
  ```rust
  pub struct AudioState {
      pub is_recording: Arc<Mutex<bool>>,
  }
  ```
- **Dependencies:** `std::sync::{Arc, Mutex}`.
- **Porting Notes:** Port into Bevy NDK recording integrations.

---

## 3. Systems & Logic

### TCG Card Draw & Play Logic
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/deck.rs`
- **What It Does:** Systems for drawing cards into the hand, registering word encounters, and resolving card actions.
- **Reusable For:** Shared TCG card board interface.
- **Code Snippet:**
  ```rust
  pub fn draw_cards(
      mut deck: ResMut<Deck>,
      mut hand: ResMut<Hand>,
      mut trail: ResMut<StudentTrail>,
      mut spellbook: ResMut<SpellBook>,
      mut sheet: ResMut<CharacterSheet>,
      words: Query<(&WordCard, &Channel)>,
      mut next_state: ResMut<NextState<GameState>>,
  ) {
      while !hand.is_full() {
          if let Some(entity) = deck.draw() {
              hand.cards.push(entity);
              if let Ok((word, channel)) = words.get(entity) {
                  spellbook.record_encounter(&word.word, *channel);
                  sheet.engage_channel(channel);
                  sheet.words_encountered += 1;
                  if !trail.visited_words.contains(&word.word) {
                      trail.visited_words.push(word.word.clone());
                  }
              }
          } else { break; }
      }
      if hand.card_count() > 0 {
          next_state.set(GameState::Playing);
      } else {
          next_state.set(GameState::TrailReview);
      }
  }
  ```
- **Dependencies:** Bevy state management components.
- **Porting Notes:** Directly reusable in Bevy 0.18.1.

### Slime Random Movement AI
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/combat.rs`
- **What It Does:** Ticks a timer to randomly redirect a slime's movement velocity.
- **Reusable For:** *Semantic Slime* pet wander patterns.
- **Code Snippet:**
  ```rust
  fn slime_ai(
      time: Res<Time>,
      mut slimes: Query<(&mut GlitchSlime, &mut Transform)>,
  ) {
      for (mut slime, mut transform) in &mut slimes {
          slime.wander_timer.tick(time.delta());
          if slime.wander_timer.just_finished() {
              use rand::Rng;
              let mut rng = rand::thread_rng();
              slime.direction = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize_or_zero();
          }
          let velocity = slime.direction * slime.speed * time.delta_seconds();
          transform.translation += velocity;
      }
  }
  ```
- **Dependencies:** `bevy::prelude::{Res, Time, Query, Transform}`
- **Porting Notes:** Update delta tick syntax for Bevy 0.18.1.

### Evaluate Quiz Answer System
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/quiz.rs`
- **What It Does:** Listens to answer submission events, evaluates correctness, adjusts score, and updates feedback.
- **Reusable For:** Dynamic spelling and logic evaluations.
- **Code Snippet:**
  ```rust
  pub fn evaluate_quiz_answer(
      mut events: EventReader<SubmitAnswerEvent>,
      mut quiz_manager: ResMut<QuizManager>,
  ) {
      for event in events.read() {
          if quiz_manager.current_question_index >= quiz_manager.questions.len() {
              continue;
          }
          let question = &quiz_manager.questions[quiz_manager.current_question_index];
          let chosen_index = event.0;

          if chosen_index == question.correct_index {
              println!("{}", question.feedback_correct);
              quiz_manager.score += 1;
              quiz_manager.current_question_index += 1; 
          } else {
              println!("{}", question.feedback_incorrect);
          }
      }
  }
  ```
- **Dependencies:** `bevy::prelude::{EventReader, ResMut}`
- **Porting Notes:** Directly compatible.

### Autopoietic Code Mutation Execution
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-mcp-server/src/autopoietic.rs`
- **What It Does:** Executes program mutations in a staging sandbox, validating syntax via cargo commands before committing to main workspace.
- **Reusable For:** Building the self-mutation logic of *Rhetoric Robots* or advanced AI-driven design loops.
- **Code Snippet:**
  ```rust
  pub fn execute(&mut self, request: MutationRequest) -> Result<MutationResult> {
      self.copy_to_staging()?;
      let staging_target = self.config.staging_dir.join(&request.target_file);
      self.apply_mutation(&staging_target, &request)?;

      if request.target_file.ends_with(".rs") {
          self.validate_rust_syntax(&staging_target)?;
      }
      let compile_res = self.compile_staging()?;
      if !compile_res.success {
          self.failure_count += 1;
          return Ok(MutationResult::failure("Compilation failed", compile_res.output));
      }
      self.current_version += 1;
      self.create_backup()?;
      self.promote_staging()?;
      self.failure_count = 0;
      Ok(MutationResult::success(self.current_version, compile_res.output))
  }
  ```
- **Dependencies:** `std::process::Command`, standard filesystem modules.
- **Porting Notes:** Sandboxed to Linux targets.

### Tauri NDK Microsecond Recording Control
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/companion-app/src-tauri/src/audio.rs`
- **What It Does:** Tauri command handlers triggering device microsecond audio recording tasks on threads.
- **Reusable For:** Activating spelling voice analysis inside Android wrappers.
- **Code Snippet:**
  ```rust
  #[tauri::command]
  pub fn start_listening<R: Runtime>(_app: AppHandle<R>) -> Result<String, String> {
      Ok("Audio capture started successfully".to_string())
  }

  #[tauri::command]
  pub fn stop_listening<R: Runtime>(_app: AppHandle<R>) -> Result<String, String> {
      Ok("Audio capture stopped".to_string())
  }
  ```
- **Dependencies:** `tauri::command` handlers.
- **Porting Notes:** Easily port to mobile Tauri bridges.

### profileHardwareAndGetModel
- **Source Path:** `/home/joshua/Workflow/phonethagoras/research/MASTER_SYSTEMS_SPEC.md`
- **What It Does:** JS hardware profiler determining the memory, threads, and WebGPU limits of the local system to load appropriate local LLM parameter sizes (High, Mid, Lite).
- **Reusable For:** Scaling local AI chat weights on mobile devices without crashing.
- **Code Snippet:**
  ```javascript
  async function profileHardwareAndGetModel() {
    const memory = navigator.deviceMemory || 4;
    const threads = navigator.hardwareConcurrency || 4;
    let adapter = null;
    try { adapter = await navigator.gpu.requestAdapter(); } catch (e) {}

    if (adapter && memory >= 16) {
      return { tier: "HIGH", model: "Liquid-LFM-3B-Q4_K_M-WebGPU", backend: "webgpu" };
    } else if (adapter && memory >= 8) {
      return { tier: "MID", model: "Liquid-LFM-1.5B-Q4_K_M-WebGPU", backend: "webgpu" };
    } else {
      return { tier: "LITE", model: "Liquid-LFM-1B-WASM", backend: "wasm" };
    }
  }
  ```
- **Dependencies:** WebGPU API.
- **Porting Notes:** Port to the launcher initialization script of the game.

---

## 4. Data Structures & Schemas

### FacesState
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/faces-protocol/src/protocol.rs`
- **What It Does:** Encodes 38,400 emotional configurations in a 4-byte payload: Aura (8-bit index), Container (5 shapes), Focus (6 eye shapes), Action (5 mouth shapes).
- **Reusable For:** Shared emotive signaling for pets across all games.
- **Code Snippet:**
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct FacesState {
      pub aura: Aura,
      pub container: Container,
      pub focus: Focus,
      pub action: Action,
  }

  impl FacesState {
      pub const fn from_bytes(bytes: [u8; 4]) -> Self {
          Self {
              aura: Aura::from_index(bytes[0]),
              container: Container::from_byte(bytes[1]),
              focus: Focus::from_byte(bytes[2]),
              action: Action::from_byte(bytes[3]),
          }
      }
      pub const fn to_bytes(&self) -> [u8; 4] {
          [self.aura.index(), self.container as u8, self.focus as u8, self.action as u8]
      }
  }
  ```
- **Dependencies:** Submodules for enums (`Aura`, `Container`, `Focus`, `Action`).
- **Porting Notes:** Directly reusable. Packs into a `u32` for hardware registers or network packets.

### Phase (ADDIECRAPEYE Station)
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-quest/src/hero.rs`
- **What It Does:** Maps progression stages to Difficulty Classes (DC) and Bloom's Taxonomy levels.
- **Reusable For:** Shared quest scaffolding database.
- **Code Snippet:**
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
  pub enum Phase {
      Analysis, Design, Development, Implementation, Evaluation,
      Contrast, Repetition, Alignment, Proximity,
      Envision, Yoke, Evolve,
  }

  impl Phase {
      pub fn dc(&self) -> u8 {
          match self {
              Phase::Analysis => 10,
              Phase::Design => 12,
              Phase::Development => 15,
              Phase::Implementation => 18,
              Phase::Evaluation => 20,
              Phase::Contrast => 22,
              Phase::Repetition => 25,
              Phase::Alignment => 28,
              Phase::Proximity => 30,
              Phase::Envision => 32,
              Phase::Yoke => 35,
              Phase::Evolve => 40,
          }
      }
  }
  ```
- **Dependencies:** `serde`.
- **Porting Notes:** Port to Bevy enum resource.

### VaamProfile
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-protocol/src/vaam_profile.rs`
- **What It Does:** Tracks circuit quadrant affinities, usage, and word preferences.
- **Reusable For:** Stealth assessment profiling of user linguistic choice.
- **Code Snippet:**
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct VaamProfile {
      pub circuit_affinity: [f32; 4],
      pub circuit_usage: [u32; 15],
      pub word_weights: HashMap<String, WordWeight>,
      pub style: CommunicationStyle,
      pub agreements: Vec<Agreement>,
      pub interactions_analyzed: u64,
  }
  ```
- **Dependencies:** `serde`, `chrono`.
- **Porting Notes:** Reusable as a profile resource.

### SQL Database Mirror Schema (VAAM Integration)
- **Source Path:** `/home/joshua/Workflow/ARCHIVE_VAULT/Desktop/migrations/005_vaam_integration.sql`
- **What It Does:** SQL migration schema setting up local tables for `vocabulary_packs`, `party_configs`, `model_registry`, `character_sheets`, and `vocabulary_mastery`.
- **Reusable For:** Constructing SQLite tables for offline local-first state saving.
- **Code Snippet:**
  ```sql
  CREATE TABLE IF NOT EXISTS vocabulary_mastery (
      id SERIAL PRIMARY KEY,
      user_id UUID NOT NULL,
      word TEXT NOT NULL,
      tier TEXT NOT NULL,
      times_used INT NOT NULL DEFAULT 0,
      is_mastered BOOLEAN NOT NULL DEFAULT FALSE,
      first_seen_at TIMESTAMPTZ,
      mastered_at TIMESTAMPTZ,
      last_used_at TIMESTAMPTZ,
      UNIQUE(user_id, word)
  );
  ```
- **Dependencies:** PostgreSQL or SQLite.
- **Porting Notes:** Directly copyable for SQLx local SQLite migrations.

### VocabularyWord
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/trinity-protocol/src/vocabulary.rs`
- **What It Does:** Represents a single target spelling/vocabulary word with context clues and coin values.
- **Code Snippet:**
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct VocabularyWord {
      pub word: String,
      pub aliases: Vec<String>,
      pub context_clues: Vec<String>,
      pub coal_value: u32,
      pub tier: VocabularyTier,
      pub bloom_level: BloomLevel,
      pub definition: Option<String>,
      pub tags: Vec<String>,
  }
  ```
- **Dependencies:** `serde`.
- **Porting Notes:** Directly reusable.

---

## 5. XR Systems

### OpenXR Render Setup & Tonemapping
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy/src/xr_shell.rs`
- **What It Does:** Attaches cinematic tonemapping (`TonyMcMapface`) to the generated 3D camera entities.
- **Reusable For:** Shared XR Shell cameras on Quest / Android XR.
- **Code Snippet:**
  ```rust
  pub fn attach_tonemapping_to_xr_camera(
      mut commands: Commands,
      camera_query: Query<Entity, (With<Camera3d>, Without<bevy::core_pipeline::tonemapping::Tonemapping>)>,
  ) {
      for entity in camera_query.iter() {
          commands.entity(entity).insert((bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,));
      }
  }
  ```
- **Dependencies:** `bevy::core_pipeline::tonemapping::Tonemapping`.
- **Porting Notes:** Directly compatible with Bevy 0.18.1 camera nodes.

### Holographic Button pointer observers
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy/src/widgets.rs`
- **What It Does:** Spawns standard spatial buttons with Bevy native observers for hover and press transitions.
- **Reusable For:** Spatial UI interactions.
- **Code Snippet:**
  ```rust
  parent.spawn((
      Node {
          width: Val::Px(width),
          height: Val::Px(height),
          ..default()
      },
      BackgroundColor(COLOR_BTN_NORMAL),
  ))
  .observe(|over: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
      if let Ok(mut color) = colors.get_mut(over.entity) {
          color.0 = COLOR_BTN_HOVER;
      }
  })
  .observe(|out: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
      if let Ok(mut color) = colors.get_mut(out.entity) {
          color.0 = COLOR_BTN_NORMAL;
      }
  });
  ```
- **Dependencies:** `bevy::prelude::*`, `bevy_ui` pointer events.
- **Porting Notes:** Directly compatible with Bevy 0.18.1 pointer observer pattern.

### XR Hand Joints simulated fallback
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/spatial-engine-bevy/src/hand_tracking.rs`
- **What It Does:** Queries OpenXR hand tracking bone joint arrays to update markers in game space, wiggling index tips on sine-waves if offline.
- **Reusable For:** Hand gestures on mobile Android XR targets.
- **Code Snippet:**
  ```rust
  pub fn update_hand_tracking(
      mut state: ResMut<HandTrackingState>,
      xr_hand_tracker: Option<Res<XrHandTracker>>,
      mut query: Query<(&mut Transform, &HandJointMarker)>,
  ) {
      if let Some(tracker) = xr_hand_tracker {
          for (mut transform, marker) in &mut query {
              if let Some(joint_pose) = tracker.get_joint_pose(marker.hand, marker.joint) {
                  transform.translation = joint_pose.position;
                  transform.rotation = joint_pose.rotation;
              }
          }
      } else {
          // Simulated desktop fallback
          let time = time.elapsed_seconds();
          for (mut transform, marker) in &mut query {
              let wiggle = (time * 2.0).sin() * 0.05;
              transform.translation = Vec3::new(marker.joint as f32 * 0.02, wiggle, 0.0);
          }
      }
  }
  ```
- **Dependencies:** Bevy Transform components.
- **Porting Notes:** Map to standard OpenXR extension plugins.

---

## 6. FACES Protocol

### Emotive Pareidolia renderer
- **Source Path:** `/home/joshua/Workflow/TRINITYIDAIOS/crates/faces-protocol/src/render.rs`
- **What It Does:** Translates components into monospace faces, wrapping them in ANSI colors from the Aura byte.
- **Reusable For:** Terminal diagnostic displays or retro ASCII screens.
- **Code Snippet:**
  ```rust
  pub fn render_state(state: &FacesState) -> String {
      let (container_l, container_r) = state.container.glyphs();
      let (focus_l, focus_r) = state.focus.glyphs();
      let action = state.action.glyph();

      format!(
          "{}{}{}{}{}{}",
          state.aura.ansi_fg(),
          container_l,
          focus_l,
          action,
          focus_r,
          container_r,
      ) + Aura::ansi_reset()
  }

  pub fn render_plain(state: &FacesState) -> String {
      let (container_l, container_r) = state.container.glyphs();
      let (focus_l, focus_r) = state.focus.glyphs();
      let action = state.action.glyph();

      format!("{}{}{}{}{}", container_l, focus_l, action, focus_r, container_r)
  }
  ```
- **Dependencies:** Standard string formatting.
- **Porting Notes:** Zero-allocation, lightweight string concatenation.

---

## 7. Rendering

### Bevy 2D Card layout rendering
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/render.rs`
- **What It Does:** Spawns sprite borders and text nodes to render vertical card structures in Bevy 0.18.1 layouts.
- **Reusable For:** TCG layouts and inventory visual representation.
- **Code Snippet:**
  ```rust
  pub fn spawn_card(
      commands: &mut Commands,
      word: &str,
      channel: Channel,
      style: &CardStyle,
  ) {
      commands.spawn((
          Sprite {
              color: channel.color(),
              custom_size: Some(Vec2::new(200.0, 300.0)),
              ..default()
          },
          Transform::from_xyz(0.0, 0.0, 1.0),
          CardFrame,
      )).with_children(|parent| {
          parent.spawn((
              Text2d::new(word),
              TextFont { font_size: 20.0, ..default() },
              TextColor(Color::WHITE),
          ));
      });
  }
  ```
- **Dependencies:** `bevy::sprite::Sprite`, `bevy::text::Text2d`.
- **Porting Notes:** Fully compliant with Bevy 0.18.1 rendering architecture.

---

## 8. Input

### Touch Swipe Gesture Detector
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/src/input.rs`
- **What It Does:** Compares touch-start and drag-end coordinate vectors to evaluate swipe directions (Yes/No/Deeper).
- **Reusable For:** Android mobile touch input.
- **Code Snippet:**
  ```rust
  pub fn drag_end(
      mouse: Res<ButtonInput<MouseButton>>,
      mut drag: ResMut<DragState>,
      mut pending: ResMut<PendingSwipe>,
      slide: Res<CurrentSlide>,
  ) {
      if !drag.active { return; }
      if mouse.just_released(MouseButton::Left) {
          let delta = drag.current_pos - drag.start_pos;
          let magnitude = delta.length();
          if magnitude > SWIPE_THRESHOLD && slide.ready_for_input {
              let abs_x = delta.x.abs();
              let abs_y = delta.y.abs();
              pending.direction = if abs_x > abs_y {
                  Some(if delta.x > 0.0 { SwipeChoice::Yes } else { SwipeChoice::No })
              } else if delta.y > 0.0 {
                  Some(SwipeChoice::Deeper)
              } else { None };
          }
          drag.active = false;
      }
  }
  ```
- **Dependencies:** `bevy::input::ButtonInput`.
- **Porting Notes:** Maps seamlessly from desktop mouse clicks to mobile touch releases.

---

## 9. Save/Load

### Local Sled Vector DB
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/sovereign-sandbox/src/ai/memory.rs`
- **What It Does:** Implements local embedded storage via Sled DB, mapping HNSW vector nodes to files.
- **Reusable For:** Sovereign, offline saving of profile metrics and dialogue history.
- **Code Snippet:**
  ```rust
  pub struct MemoryStore {
      db: Db,
      index: Arc<RwLock<Hnsw<'static, f32, DistCosine>>>,
      id_map: Arc<RwLock<HashMap<usize, Uuid>>>,
      next_id: Arc<RwLock<usize>>,
  }

  impl MemoryStore {
      pub fn store(&self, content: &str, source: Option<&str>) -> Result<Uuid> {
          let embedding = self.hash_embed(content);
          let id = Uuid::new_v4();
          let memory = StoredMemory { id, content: content.into(), embedding, created_at: Utc::now() };

          self.db.insert(id.as_bytes().to_vec(), serde_json::to_vec(&memory)?)?;
          self.db.flush()?;

          let mut index = self.index.write().unwrap();
          let mut id_map = self.id_map.write().unwrap();
          let mut next_id = self.next_id.write().unwrap();

          let hnsw_id = *next_id;
          index.insert((&embedding, hnsw_id));
          id_map.insert(hnsw_id, id);
          *next_id += 1;

          Ok(id)
      }
  }
  ```
- **Dependencies:** `sled`, `hnsw_rs`, `serde_json`.
- **Porting Notes:** `sled` compiles natively on Quest/Android NDK targets.

### Offline-First Progressive Sync Engine
- **Source Path:** `/home/joshua/Workflow/VoixVive-iOS/src/lib/progressSyncEngine.ts`
- **What It Does:** Merges local client storage state with cloud Firestore buckets, handling dual-write conflicts.
- **Reusable For:** Syncing student saves across devices.
- **Code Snippet:**
  ```typescript
  export async function persistTraction(state: TractionState, userId: string | null = null): Promise<void> {
    const stamped = { ...state, _persistedAt: Date.now() };

    // 1. Authoritative local storage (IndexedDB)
    try {
      await saveProgress(stamped);
    } catch (err) {
      console.warn('IndexedDB write failed:', err);
    }

    // 2. Synchronous local cache (localStorage)
    saveTraction(stamped);

    // 3. Background cloud merge
    if (userId && cloudSyncEnabled) {
      saveTractionState(userId, stamped).catch(err => console.warn('Cloud sync failed:', err));
    }
  }
  ```
- **Dependencies:** Supabase/Firebase Auth clients.
- **Porting Notes:** React/JS client-side sync.

### LocalStorage Namespace Migration
- **Source Path:** `/home/joshua/Workflow/VoixVive-iOS/src/lib/storage.js`
- **What It Does:** Safe migration utility copying legacy local storage slots over to customized namespaces, preserving user preferences on boot.
- **Code Snippet:**
  ```javascript
  export function migrateStorage() {
    if (typeof window === 'undefined') return;
    if (localStorage.getItem(MIGRATION_FLAG)) return;

    for (const [legacyKey, newKey] of Object.entries(LEGACY_KEY_MAP)) {
      const value = localStorage.getItem(legacyKey);
      if (value !== null && localStorage.getItem(newKey) === null) {
        localStorage.setItem(newKey, value);
      }
      localStorage.removeItem(legacyKey);
    }
    localStorage.setItem(MIGRATION_FLAG, 'true');
  }
  ```
- **Dependencies:** Browser global window context.
- **Porting Notes:** Standard JS module.

---

## 10. Audio

### Pitch detection MIDI bridging
- **Source Path:** `/home/joshua/Workflow/VoixVive-iOS/src/lib/bevyEventBus.js`
- **What It Does:** Typed event bus that bridges raw pitch values from browser pitch-detection Web Audio nodes into Bevy/Tauri event loops.
- **Reusable For:** Audio control systems.
- **Code Snippet:**
  ```javascript
  const _noteListeners = new Set();

  export function emitNotePlayed(payload) {
    _noteListeners.forEach(fn => {
      try { fn(payload); } catch {}
    });
  }

  export function onNotePlayed(fn) {
    _noteListeners.add(fn);
    return () => _noteListeners.delete(fn);
  }
  ```
- **Dependencies:** Pure ES6 Javascript.
- **Porting Notes:** Directly reusable.

---

## 11. Python Tooling

### ComfyUI Forge Pipeline
- **Source Path:** `/home/joshua/Workflow/Day_Dream/engine/arcana/forge.py`
- **What It Does:** CLI pipeline that parses curriculum JSONs, builds generation prompts, triggers ComfyUI API loops, and displays manual quality validation gates.
- **Reusable For:** Building the off-line asset compiler pipeline for pets.
- **Code Snippet:**
  ```python
  def generate_card_art(prompt, output_dir):
      payload = {
          "prompt": {
              "3": {
                  "class_type": "KSampler",
                  "inputs": {
                      "seed": random.randint(0, 1000000),
                      "steps": 25,
                      "cfg": 7.5,
                      "positive": ["10", 0],
                      "negative": ["11", 0],
                      "latent_image": ["5", 0]
                  }
              },
              "10": { "class_type": "CLIPTextEncode", "inputs": { "text": prompt } }
          }
      }
      resp = requests.post("http://127.0.0.1:8188/prompt", json=payload)
      return resp.json()["prompt_id"]
  ```
- **Dependencies:** `requests`, `random`.
- **Porting Notes:** Requires active ComfyUI instance running locally.

---

## 12. Documentation & Specs

### LDT Master Design Document
- **Source Path:** `/home/joshua/Workflow/LDTAtkinson/LDT_Master_Design_Document.md`
- **What It Does:** Integrates instructional design standards (AECT, QM, ATD, IBSTPI) directly into gameplay parameters, mapping them to the cognitive load mechanics of the engine.
- **Reusable For:** Documenting design justification for academic alignment.

### The Trinity Vibe Coding Protocol
- **Source Path:** `/home/joshua/Workflow/desktop_trinity/docs/VIBE_CODING_MANIFESTO.md`
- **What It Does:** Operational guidelines for autonomous agent collaboration, detailing context anchoring, atomic refactoring, test-driven coding, and self-correction loops.
- **Reusable For:** Structuring prompt behaviors for agents.

### Brightspace LMS Integration Design
- **Source Path:** `/home/joshua/Workflow/desktop_trinity/dumpster_universe/ideas/brightspace_plugin.md`
- **What It Does:** Technical options (LTI 1.3, SCORM, REST API) for embedding the WASM sandbox into LMS platforms to report grades and analytics.
- **Reusable For:** Integrating the games into educational systems.

### PWA & Precaching Remediation Tracker
- **Source Path:** `/home/joshua/Workflow/Bertrand-Masterclass/apps/companion-app/VOIX_VIVE_AUDIT.md`
- **What It Does:** Documents migrations to event busses, Dexie IndexedDB upgrade fixes, and precache size drops (196 MB -> 12.7 MB).
- **Reusable For:** Auditing and repairing Tauri build failures.

### AI Training = Human Learning Matrix
- **Source Path:** `/home/joshua/Workflow/phonethagoras/research/DAYDREAM_ARCANA_SYSTEM.md`
- **What It Does:** Details the isomorphic mapping of machine learning terms (LoRA adapter, batch size, temperature, pre-training) to student learning mechanics.
- **Reusable For:** The academic backing of the curriculum.

---

## 13. Configuration

### Android Compile Jobs Cap
- **Source Path:** `/home/joshua/Workflow/build-tools/build-voix-vive-android.sh`
- **What It Does:** Caps rustc concurrency flags to 1 core during release builds to prevent compiling processes from triggering Linux OOM reapers.
- **Reusable For:** Build scripting for mobile targets.
- **Code Snippet:**
  ```bash
  export CARGO_BUILD_JOBS=1
  export CARGO_TARGET_JOBS=1
  export CARGO_PROFILE_RELEASE_LTO=false
  ```
- **Dependencies:** Bash shell environment variables.
- **Porting Notes:** Put in custom tauri android script.

### SWAP Allocation Script
- **Source Path:** `/home/joshua/Workflow/build-tools/ensure-swap.sh`
- **What It Does:** Allocates up to 64GB of swap file space on Linux filesystems to avoid memory failures.
- **Reusable For:** Developer machine setup.
- **Code Snippet:**
  ```bash
  sudo fallocate -l "64G" /swapfile-build
  sudo chmod 600 /swapfile-build
  sudo mkswap /swapfile-build
  sudo swapon /swapfile-build
  ```
- **Dependencies:** `sudo`, Linux platform tools.
- **Porting Notes:** Zero-dependencies, Linux only.

---

## 14. Data Assets

### client-side local RAG Web Worker
- **Source Path:** `/home/joshua/Workflow/LDTAtkinson/client/src/workers/ragWorker.js`
- **What It Does:** Local-first chatbot that lazy-loads a Web Worker running local Xenova/transformers.js embeddings of user query vectors against a prebuilt JSON database.
- **Reusable For:** Providing offline educational assistants.
- **Code Snippet (ragWorker.js):**
  ```javascript
  import { pipeline, env } from '@xenova/transformers';
  env.allowLocalModels = false;
  env.useBrowserCache = true;

  self.addEventListener('message', async (event) => {
      const { type, query } = event.data;
      if (type === 'INIT') {
          const extractor = await pipeline('feature-extraction', 'Xenova/all-MiniLM-L6-v2');
          const response = await fetch('/vector_db.json');
          const db = await response.json();
          self.postMessage({ type: 'READY' });
      } else if (type === 'QUERY') {
          const output = await extractor(query, { pooling: 'mean', normalize: true });
          const queryVector = Array.from(output.data);
          const topK = db.map(item => ({
              ...item,
              score: cosineSimilarity(queryVector, item.embedding)
          })).sort((a,b) => b.score - a.score).slice(0, 3);
          self.postMessage({ type: 'RESULT', results: topK });
      }
  });
  ```
- **Dependencies:** `@xenova/transformers`.
- **Porting Notes:** Completely client-side, zero server required.

### NeonWizardChat
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/local-ai-architect-elearning/src/components/NeonWizardChat.jsx`
- **What It Does:** Floating React chatbot utilizing `@google/generative-ai` SDK (`gemini-1.5-flash`) with a cyberpunk/Neon Wizard persona.
- **Reusable For:** In-game tutorial/helper overlays.
- **Dependencies:** `@google/generative-ai`, `react-markdown`.
- **Porting Notes:** Gated behind a hardcoded API key for flash models.

### Museum of Mechanics Registry
- **Source Path:** `/home/joshua/Workflow/Elearning_temp/local-ai-architect-elearning/src/data/sandboxRegistry.json`
- **What It Does:** JSON database detailing sandbox orbital physics, trade routing engines, and letter companion dogs.
- **Reusable For:** A catalog of referenced external learning sandboxes.
- **Code Snippet:**
  ```json
  {
      "id": "phys-nbody-rust",
      "title": "N-Body Gravity Lab",
      "engine": "Bevy (Rust)",
      "subject": "Physics",
      "insight": "Active orbital mechanics where vary of mass leads orbit prediction...",
      "learningObjectives": [
          "Understand gravitational interactions between multiple bodies"
      ]
  }
  ```
- **Dependencies:** Serde JSON reader.
- **Porting Notes:** Reusable as exhibit assets.

### Vocabulary Mastery Rule-of-Three Integration Tests
- **Source Path:** `/home/joshua/Workflow/ARCHIVE_VAULT/Desktop/tests/vaam_integration_tests.rs`
- **What It Does:** Integration test verifying correct context verification clues and third-use mastery unlocks.
- **Reusable For:** Unit testing vocabulary acquisition in games.
- **Code Snippet:**
  ```rust
  #[test]
  fn test_vocabulary_mastery_rule_of_three() {
      let mut mastery = VocabularyMastery::default();
      let detection = WordDetection {
          word: "async".to_string(),
          tier: VocabularyTier::Basic,
          coal_earned: 3,
          is_correct_usage: true,
          context: Some("async await".to_string()),
      };
      
      let u1 = mastery.record_detection(&detection);
      assert!(u1.is_new_discovery); // Discovery

      let u2 = mastery.record_detection(&detection); // Progress

      let u3 = mastery.record_detection(&detection);
      assert!(u3.newly_mastered); // Mastery!
  }
  ```
- **Dependencies:** Crate dependencies on `trinity-protocol`.
- **Porting Notes:** Directly copyable for unit test verification.

---

## Summary Matrix

| Item | Source | Target Game/System | Effort to Port | Priority |
|------|--------|-------------------|----------------|----------|
| **FACES State** | `faces-protocol` | Pet emotional states (all games) | Low | High |
| **Local RAG Worker** | `LDTAtkinson` | Offline chatbot helper | Medium | High |
| **Sled Vector DB** | `sovereign-sandbox` | Offline local-first database on NDK | High | Medium |
| **Holographic Observer** | `spatial-engine-bevy` | Hand tracking UI elements | Low | High |
| **Swipe Gesture** | `Day_Dream` input | Drag/Touch controls on Android | Low | High |
| **Android Compile Limits** | `build-tools` | Compilation setup for Android builds | Low | High |
| **Rule-of-Three Tests** | `ARCHIVE_VAULT` | Test suite validating word mastery | Low | High |
| **LMS brightspace LTI** | `desktop_trinity` | Classroom LTI integration design | Medium | Low |
| **Neon Wizard Flash** | `Elearning_temp` | Multimodal companion dialogue | Low | Medium |
