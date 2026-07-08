// battle.rs — Turn-based synonym/antonym card combat against wild typos
use bevy::prelude::*;
use crate::components::*;
use crate::database::*;
use crate::quest;

const WAND_DUEL_DISTANCE_BASE_MULTIPLIER: f32 = 1.5;
const WAND_DUEL_DISTANCE_SCALE: f32 = 0.2;
const SYNONYM_BOOST_MULTIPLIER: f32 = 2.0;

#[derive(Resource, Debug, Clone)]
pub struct BattleSession {
    pub typo_word: String,
    pub typo_health: i32,
    pub player_health: i32,
    pub failed_word: Option<String>, // Track word that caused defeat for Tutor Loop
}

#[derive(Component)]
pub struct CriticalHitTrigger;

pub fn semantic_distance(a: &WordStats, b: &WordStats) -> f32 {
    let dc = a.concreteness - b.concreteness;
    let dv = a.valence - b.valence;
    let dd = a.dominance - b.dominance;
    let da = a.intensity - b.intensity;
    (dc*dc + dv*dv + dd*dd + da*da).sqrt()
}

pub fn start_battle(
    commands: &mut Commands,
    db: &GameDatabase,
    grade_manager: &crate::quest::GradeManager,
    next_state: &mut NextState<GameState>,
    state: &State<GameState>,
) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let valid_grades = grade_manager.get_valid_grade_levels();

    let valid_words: Vec<&String> = db.words.iter()
        .filter(|(_, stats)| valid_grades.contains(&stats.grade_level.as_str()))
        .map(|(word, _)| word)
        .collect();

    let mut typo_word = "inferno".to_string(); // fallback
    if let Some(&word) = valid_words.choose(&mut thread_rng()) {
        typo_word = word.clone();
    }

    commands.insert_resource(BattleSession {
        typo_word: typo_word.clone(),
        typo_health: 50,
        player_health: 100,
        failed_word: None,
    });

    info!("A wild Typo ({}) emerges! Deduce its semantic weakness based on its stats!", typo_word.to_uppercase());
    crate::commands::log_state_transition(state.get(), GameState::Battling);
    next_state.set(GameState::Battling);
}

/// Start Tutor Loop quest after battle defeat
pub fn start_tutor_loop(
    commands: &mut Commands,
    db: &GameDatabase,
    grade_manager: &crate::quest::GradeManager,
    battle_session: &BattleSession,
    next_state: &mut NextState<GameState>,
    state: &State<GameState>,
) {
    let _sheet = &CharacterSheet::default(); // Unused in MVP
    if let Some(failed_word) = &battle_session.failed_word {
        let tutor_npc = quest::route_to_tutor_npc(failed_word, db);
        info!("Tutor Loop: Routing to {} for practice on '{}'", tutor_npc, failed_word);
        
        quest::start_quest(&tutor_npc, db, grade_manager, commands, next_state, state);
    } else {
        // Fallback if no failed word tracked
        quest::start_quest("Barnaby", db, grade_manager, commands, next_state, state);
    }
}

pub struct BattleResult {
    pub is_effective: bool,
    pub is_counter: bool, // High distance = antonym/counter
    pub is_synonym: bool, // Low distance = synonym/heavy attack
}

pub fn play_battle_card(
    played_word: &str,
    session: &mut BattleSession,
    db: &GameDatabase,
    spellbook: &mut SpellBook,
    next_state: &mut NextState<GameState>,
    _sheet: &CharacterSheet,
    state: &State<GameState>,
) -> BattleResult {
    let lower_typo = session.typo_word.to_lowercase();
    let lower_played = played_word.to_lowercase();

    let mut damage_multiplier = 1.0;
    let mut is_effective = false;
    let mut is_counter = false;
    let mut is_synonym = false;

    if let (Some(typo_stats), Some(played_stats)) = (db.words.get(&lower_typo), db.words.get(&lower_played)) {
        let distance = semantic_distance(typo_stats, played_stats);
        
        // Wand Duel: High distance = antonym/counter, Low distance = synonym/heavy attack
        if distance > 4.0 {
            // Counter/Block: High semantic distance = opposing concepts
            damage_multiplier = WAND_DUEL_DISTANCE_BASE_MULTIPLIER + (distance - 4.0) * WAND_DUEL_DISTANCE_SCALE;
            is_effective = true;
            is_counter = true;
        } else if distance < 2.0 {
            // Synonym/Heavy Attack: Low semantic distance = similar concepts
            damage_multiplier = SYNONYM_BOOST_MULTIPLIER;
            is_effective = true;
            is_synonym = true;
        } else {
            // Mid-range: normal damage
            damage_multiplier = 1.0;
            is_effective = true;
        }
    }

    let base_damage = 25.0;
    let final_damage = (base_damage * damage_multiplier) as i32;

    if is_effective {
        session.typo_health -= final_damage;
        if is_counter {
            info!("COUNTER! Antonym blocks the Typo. Damage multiplier: {:.2}x. Typo health: {}", damage_multiplier, session.typo_health);
        } else if is_synonym {
            info!("HEAVY ATTACK! Synonym overwhelms the Typo. Damage multiplier: {:.2}x. Typo health: {}", damage_multiplier, session.typo_health);
        } else {
            info!("HIT! Normal damage. Typo health: {}", session.typo_health);
        }
        spellbook.upgrade_mastery(played_word, MasteryLevel::Owned);
    } else {
        session.typo_health -= final_damage;
        session.player_health -= 20;
        warn!("INEFFECTIVE! Damage multiplier: {:.2}x. Typo counters! Player health: {}", damage_multiplier, session.player_health);
    }

    if session.typo_health <= 0 {
        info!("Victory! The Typo has been corrected and the verse is clean.");
        spellbook.upgrade_mastery(played_word, MasteryLevel::Mastered);
        crate::commands::log_state_transition(state.get(), GameState::Reviewing);
        next_state.set(GameState::Reviewing);
    } else if session.player_health <= 0 {
        warn!("Defeat! The Typo overrode your verse. Entering Tutor Loop.");
        session.failed_word = Some(session.typo_word.clone()); // Track failed word for NPC routing
        // Note: The actual Tutor Loop quest start will be handled by the command handler
        // which has access to all needed resources. We just transition state here.
        crate::commands::log_state_transition(state.get(), GameState::Questing);
        next_state.set(GameState::Questing);
    }

    BattleResult {
        is_effective,
        is_counter,
        is_synonym,
    }
}

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct EnemyHealthBar;

#[derive(Component)]
pub struct BattleUiMarker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_distance_is_zero_for_identical_stats() {
        let stats = WordStats {
            concreteness: 2.0,
            valence: 3.0,
            intensity: 4.0,
            dominance: 5.0,
            grade_level: "6-8".to_string(),
        };
        assert!((semantic_distance(&stats, &stats) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn semantic_distance_increases_with_divergent_stats() {
        let a = WordStats {
            concreteness: 1.0,
            valence: 1.0,
            intensity: 1.0,
            dominance: 1.0,
            grade_level: "K-2".to_string(),
        };
        let b = WordStats {
            concreteness: 4.0,
            valence: 4.0,
            intensity: 4.0,
            dominance: 4.0,
            grade_level: "6-8".to_string(),
        };
        let dist = semantic_distance(&a, &b);
        assert!(dist > 5.0);
    }

    #[test]
    fn battle_result_defaults_are_false() {
        let result = BattleResult {
            is_effective: false,
            is_counter: false,
            is_synonym: false,
        };
        assert!(!result.is_effective);
        assert!(!result.is_counter);
        assert!(!result.is_synonym);
    }
}

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "xr")]
        app.add_systems(OnEnter(GameState::Battling), (spawn_battle_ui_xr, set_pet_battle_state))
           .add_systems(Update, update_battle_hp_bars_xr.run_if(in_state(GameState::Battling)))
           .add_systems(OnExit(GameState::Battling), (cleanup_battle_ui_xr, set_pet_idle_state));

        #[cfg(not(feature = "flat2d"))]
        app.add_systems(Update, handle_critical_hit_effects);

        #[cfg(not(feature = "xr"))]
        app.add_systems(OnEnter(GameState::Battling), (spawn_battle_ui_2d, set_pet_battle_state))
           .add_systems(Update, update_battle_hp_bars_2d.run_if(in_state(GameState::Battling)))
           .add_systems(OnExit(GameState::Battling), (cleanup_battle_ui_2d, set_pet_idle_state));
    }
}

#[cfg(not(feature = "flat2d"))]
pub fn handle_critical_hit_effects(
    trigger_query: Query<Entity, With<CriticalHitTrigger>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    for trigger_entity in trigger_query.iter() {
        commands.entity(trigger_entity).despawn();
        
        for entity in &camera_query {
            commands.entity(entity).insert(crate::render::ScreenShake { timer: 0.3, intensity: 0.2 });
        }
        
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            let vx = rng.gen_range(-4.0..4.0);
            let vy = rng.gen_range(2.0..6.0);
            let vz = rng.gen_range(-4.0..4.0);
            
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.06).mesh().ico(1).unwrap())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 0.9, 0.1),
                    emissive: Color::srgb(2.0, 1.8, 0.2).into(),
                    ..default()
                })),
                Transform::from_xyz(0.0, 1.5, -2.0),
                crate::render::BurstParticle {
                    velocity: Vec3::new(vx, vy, vz),
                    timer: 1.5,
                }
            ));
        }
    }
}

fn set_pet_battle_state(
    mut query: Query<&mut PetVisualState, With<PetAvatar>>,
) {
    for mut state in &mut query {
        *state = PetVisualState::Battle;
    }
}

fn set_pet_idle_state(
    mut query: Query<&mut PetVisualState, With<PetAvatar>>,
) {
    for mut state in &mut query {
        *state = PetVisualState::Idle;
    }
}

#[cfg(feature = "xr")]
fn spawn_battle_ui_xr(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    session: Res<BattleSession>,
) {
    let instruction_text = format!("WILD TYPO: {}", session.typo_word.to_uppercase());
    commands.spawn((
        BattleUiMarker,
        Text2d::new(instruction_text),
        TextFont { font_size: 36.0, ..default() },
        TextColor(Color::srgb(0.9, 0.9, 0.2)),
        Transform::from_xyz(0.0, 2.5, -2.0),
    ));
    // Player HP bar
    let player_bar = commands.spawn((
        PlayerHealthBar,
        BattleUiMarker,
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.1, 0.02))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.2),
            ..default()
        })),
        Transform::from_xyz(-1.5, 1.8, -2.0),
    )).id();

    let player_text = commands.spawn((
        BattleUiMarker,
        Text2d::new("Player HP: 100"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.15, 0.02),
    )).id();
    commands.entity(player_bar).add_child(player_text);

    // Enemy HP bar
    let enemy_bar = commands.spawn((
        EnemyHealthBar,
        BattleUiMarker,
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.1, 0.02))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(1.5, 1.8, -2.0),
    )).id();

    let enemy_text = commands.spawn((
        BattleUiMarker,
        Text2d::new("Typo HP: 50"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, 0.15, 0.02),
    )).id();
    commands.entity(enemy_bar).add_child(enemy_text);
}

#[cfg(feature = "xr")]
fn update_battle_hp_bars_xr(
    session: Option<Res<BattleSession>>,
    mut player_bar: Query<(&mut Transform, &Children), (With<PlayerHealthBar>, Without<EnemyHealthBar>)>,
    mut enemy_bar: Query<(&mut Transform, &Children), (With<EnemyHealthBar>, Without<PlayerHealthBar>)>,
    mut text_query: Query<&mut Text2d>,
) {
    let session = match session {
        Some(s) => s,
        None => return,
    };

    for (mut transform, children) in &mut player_bar {
        let ratio = (session.player_health as f32 / 100.0).clamp(0.0, 1.0);
        transform.scale.x = ratio;
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = format!("Player HP: {}", session.player_health);
            }
        }
    }

    for (mut transform, children) in &mut enemy_bar {
        let ratio = (session.typo_health as f32 / 50.0).clamp(0.0, 1.0);
        transform.scale.x = ratio;
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                text.0 = format!("Typo HP: {}", session.typo_health);
            }
        }
    }
}

#[cfg(feature = "xr")]
fn cleanup_battle_ui_xr(
    mut commands: Commands,
    query: Query<Entity, With<BattleUiMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(not(feature = "xr"))]
fn spawn_battle_ui_2d(
    mut commands: Commands,
    session: Res<BattleSession>,
) {
    let instruction_text = format!("WILD TYPO: {}", session.typo_word.to_uppercase());
    
    commands.spawn((
        BattleUiMarker,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new(instruction_text),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.2)),
        ));

        // Container for HP Bars
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Px(400.0),
                ..default()
            },
        )).with_children(|bars| {
            bars.spawn((
                PlayerHealthBar,
                Text::new("Player HP: 100"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.2, 0.8, 0.2)),
            ));

            bars.spawn((
                EnemyHealthBar,
                Text::new("Typo HP: 50"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.8, 0.2, 0.2)),
            ));
        });
    });
}

#[cfg(not(feature = "xr"))]
fn update_battle_hp_bars_2d(
    session: Option<Res<BattleSession>>,
    mut player_bar: Query<&mut Text, (With<PlayerHealthBar>, Without<EnemyHealthBar>)>,
    mut enemy_bar: Query<&mut Text, (With<EnemyHealthBar>, Without<PlayerHealthBar>)>,
) {
    let session = match session {
        Some(s) => s,
        None => return,
    };

    if let Some(mut text) = player_bar.iter_mut().next() {
        text.0 = format!("Player HP: {}", session.player_health);
    }
    
    if let Some(mut text) = enemy_bar.iter_mut().next() {
        text.0 = format!("Typo HP: {}", session.typo_health);
    }
}

#[cfg(not(feature = "xr"))]
fn cleanup_battle_ui_2d(
    mut commands: Commands,
    query: Query<Entity, With<BattleUiMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
