// letter.rs — Letter collection, spelling, and etymology-based pet spawning
use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::database::*;

const PET_LOGOS_MULTIPLIER: f32 = 20.0;
const PET_PATHOS_MULTIPLIER: f32 = 10.0;
const PET_ETHOS_MULTIPLIER: f32 = 10.0;
const PET_SPEED_MULTIPLIER: f32 = 10.0;
const LETTER_CRYSTAL_SIZE: f32 = 0.5;
const LETTER_CRYSTAL_PICKUP_DISTANCE: f32 = 1.8;

#[derive(Component)]
pub struct LetterCrystal {
    pub letter: char,
    pub bob_speed: f32,
    pub hover_timer: f32,
}

#[derive(Resource, Default, Debug)]
pub struct LetterStash {
    pub letters: Vec<char>,
}

#[derive(Resource, Default, Debug)]
pub struct CurrentSpelling {
    pub word: String,
}

pub fn spawn_letter_crystals(
    mut commands: Commands,
    crystals: Query<&LetterCrystal>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    db: Res<GameDatabase>,
    grade_manager: Res<crate::quest::GradeManager>,
) {
    if crystals.iter().count() >= 5 {
        return;
    }

    let mut rng = rand::thread_rng();
    
    // Curriculum-biased spawning: get grade-appropriate words
    let valid_grades = grade_manager.get_valid_grade_levels();
    let grade_words: Vec<&String> = db.words.iter()
        .filter(|(_, stats)| valid_grades.contains(&stats.grade_level.as_str()))
        .map(|(word, _)| word)
        .collect();
    
    // Build letter frequency map from grade-appropriate words
    let mut letter_counts: std::collections::HashMap<char, u32> = std::collections::HashMap::new();
    for word in &grade_words {
        for c in word.to_uppercase().chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }
    }
    
    // Convert to weighted letter list
    let mut weighted_letters: Vec<char> = Vec::new();
    for (letter, count) in &letter_counts {
        for _ in 0..*count {
            weighted_letters.push(*letter);
        }
    }
    
    // Fallback to alphabet if no grade words available
    let letter_pool = if weighted_letters.is_empty() {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect()
    } else {
        weighted_letters
    };
    
    // Spawn a letter crystal with curriculum bias
    let letter = letter_pool[rng.gen_range(0..letter_pool.len())];
    let x = rng.gen_range(-5.0..5.0);
    let z = rng.gen_range(-5.0..5.0);
    let y = 1.0 + rng.gen_range(-0.2..0.2);

    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.8, 1.0),
        emissive: Color::srgb(0.0, 0.3, 0.4).into(),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(LETTER_CRYSTAL_SIZE, LETTER_CRYSTAL_SIZE, LETTER_CRYSTAL_SIZE))),
        MeshMaterial3d(mat),
        Transform::from_xyz(x, y, z),
        LetterCrystal {
            letter,
            bob_speed: rng.gen_range(1.0..2.0),
            hover_timer: 0.0,
        },
    ));
}

pub fn handle_pinch_crystals(
    mut commands: Commands,
    pinch_events: Res<crate::hand_tracking::PinchEvents>,
    mut stash: ResMut<LetterStash>,
    crystals: Query<(Entity, &Transform, &LetterCrystal)>,
) {
    for pinch in &pinch_events.events {
        for (entity, transform, crystal) in crystals.iter() {
            if transform.translation.distance(pinch.position) < LETTER_CRYSTAL_PICKUP_DISTANCE {
                // Collect crystal
                stash.letters.push(crystal.letter);
                info!("Collected letter: {}", crystal.letter);
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn animate_crystals(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut LetterCrystal)>,
) {
    for (mut tf, mut crystal) in &mut query {
        crystal.hover_timer += time.delta_secs() * crystal.bob_speed;
        tf.translation.y += crystal.hover_timer.sin() * 0.005;
        tf.rotate_y(time.delta_secs() * 0.5);
    }
}

pub fn collect_letters(
    mut commands: Commands,
    mut stash: ResMut<LetterStash>,
    crystals: Query<(Entity, &Transform, &LetterCrystal)>,
    camera: Query<&Transform, With<Camera3d>>,
) {
    let cam_tf = match camera.iter().next() {
        Some(t) => t,
        None => return,
    };

    for (entity, tf, crystal) in &crystals {
        let dist = cam_tf.translation.distance(tf.translation);
        if dist < LETTER_CRYSTAL_PICKUP_DISTANCE {
            stash.letters.push(crystal.letter);
            info!("Collected letter: {}", crystal.letter);
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_keyboard_spelling(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<crate::commands::GameCommand>,
) {
    for key in keys.get_just_pressed() {
        if let Some(c) = key_to_char(*key) {
            writer.write(crate::commands::GameCommand::AddLetter(c));
        }
    }

    if keys.just_pressed(KeyCode::Backspace) {
        writer.write(crate::commands::GameCommand::Backspace);
    }

    if keys.just_pressed(KeyCode::Enter) {
        info!("Ink word submitted from keyboard!");
        writer.write(crate::commands::GameCommand::SubmitSpelling);
    }
}

fn key_to_char(key: KeyCode) -> Option<char> {
    match key {
        KeyCode::KeyA => Some('a'), KeyCode::KeyB => Some('b'), KeyCode::KeyC => Some('c'),
        KeyCode::KeyD => Some('d'), KeyCode::KeyE => Some('e'), KeyCode::KeyF => Some('f'),
        KeyCode::KeyG => Some('g'), KeyCode::KeyH => Some('h'), KeyCode::KeyI => Some('i'),
        KeyCode::KeyJ => Some('j'), KeyCode::KeyK => Some('k'), KeyCode::KeyL => Some('l'),
        KeyCode::KeyM => Some('m'), KeyCode::KeyN => Some('n'), KeyCode::KeyO => Some('o'),
        KeyCode::KeyP => Some('p'), KeyCode::KeyQ => Some('q'), KeyCode::KeyR => Some('r'),
        KeyCode::KeyS => Some('s'), KeyCode::KeyT => Some('t'), KeyCode::KeyU => Some('u'),
        KeyCode::KeyV => Some('v'), KeyCode::KeyW => Some('w'), KeyCode::KeyX => Some('x'),
        KeyCode::KeyY => Some('y'), KeyCode::KeyZ => Some('z'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_to_char_maps_all_letters() {
        assert_eq!(key_to_char(KeyCode::KeyA), Some('a'));
        assert_eq!(key_to_char(KeyCode::KeyZ), Some('z'));
        assert_eq!(key_to_char(KeyCode::KeyM), Some('m'));
    }

    #[test]
    fn key_to_char_returns_none_for_non_letters() {
        assert_eq!(key_to_char(KeyCode::Digit1), None);
        assert_eq!(key_to_char(KeyCode::Space), None);
        assert_eq!(key_to_char(KeyCode::Escape), None);
    }

    #[test]
    fn letter_stash_defaults_empty() {
        let stash = LetterStash::default();
        assert!(stash.letters.is_empty());
    }

    #[test]
    fn current_spelling_defaults_empty() {
        let spelling = CurrentSpelling::default();
        assert!(spelling.word.is_empty());
    }
}

#[derive(Component)]
#[cfg(feature = "xr")]
pub struct HolographicLetter(pub char);

#[derive(Component)]
#[cfg(feature = "xr")]
pub struct SubmitSpellingButton;

#[cfg(feature = "xr")]
pub fn spawn_holographic_stash(
    mut commands: Commands,
    stash: Res<LetterStash>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn letters in an arc in front of the player
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.8, 0.3),
        emissive: Color::srgb(0.0, 0.3, 0.1).into(),
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let radius = 1.0;
    let count = stash.letters.len();
    for (i, &letter) in stash.letters.iter().enumerate() {
        let angle = (i as f32 / (count.max(1) as f32)) * std::f32::consts::PI - (std::f32::consts::PI / 2.0);
        let x = angle.sin() * radius;
        let z = -1.5 + angle.cos() * radius;
        let pos = Vec3::new(x, 1.2, z);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.35, 0.35, 0.08))),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(pos),
            HolographicLetter(letter),
        )).with_children(|inner| {
            inner.spawn((
                Text2d::new(letter.to_string()),
                TextFont { font_size: 56.0, ..default() },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 0.0, 0.03),
            ));
        });
    }

    // Spawn submit button
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.7, 0.28, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.8, -1.2),
        SubmitSpellingButton,
    )).with_children(|inner| {
        inner.spawn((
            Text2d::new("SUBMIT"),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, 0.0, 0.04),
        ));
    });
}

#[cfg(feature = "xr")]
pub fn handle_vr_spelling(
    pinch_events: Res<crate::hand_tracking::PinchEvents>,
    mut writer: MessageWriter<crate::commands::GameCommand>,
    mut commands: Commands,
    letter_query: Query<(Entity, &GlobalTransform, &HolographicLetter)>,
    submit_query: Query<&GlobalTransform, With<SubmitSpellingButton>>,
) {
    for event in &pinch_events.events {
        // Check submit button
        for submit_tf in &submit_query {
            if event.position.distance(submit_tf.translation()) < 0.3 {
                info!("Submit spelling pinched!");
                writer.write(crate::commands::GameCommand::SubmitSpelling);
                return;
            }
        }

        // Check letter blocks
        for (entity, tf, letter) in &letter_query {
            if event.position.distance(tf.translation()) < 0.2 {
                writer.write(crate::commands::GameCommand::AddLetter(letter.0));
                commands.entity(entity).despawn();
                info!("Pinched letter: {}", letter.0);
                return; // One pinch per frame
            }
        }
    }
}

#[cfg(feature = "xr")]
pub fn cleanup_holographic_stash(
    mut commands: Commands,
    query: Query<Entity, Or<(With<HolographicLetter>, With<SubmitSpellingButton>)>>,
) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

pub struct SpellingData<'a> {
    pub db: &'a GameDatabase,
    pub sheet: &'a CharacterSheet,
}

pub struct SpellingAssets<'a> {
    pub meshes: &'a mut Assets<Mesh>,
    pub materials: &'a mut Assets<StandardMaterial>,
}

#[allow(clippy::too_many_arguments)]
pub fn submit_spelling_word(
    spelling: &mut CurrentSpelling,
    next_state: &mut NextState<GameState>,
    commands: &mut Commands,
    data: &SpellingData,
    demo: &mut crate::paywall::DemoSettings,
    spellbook: &mut crate::components::SpellBook,
    assets: &mut SpellingAssets,
    state: &State<GameState>,
) {
    let SpellingData { db, sheet, .. } = data;
    let SpellingAssets { meshes, materials } = assets;

    if demo.is_demo && demo.words_used >= demo.max_words {
        crate::commands::log_state_transition(state.get(), GameState::Paywall);
        next_state.set(GameState::Paywall);
        return;
    }

    let word_lower = spelling.word.to_lowercase();

    // Profanity blocklist — banned words fail silently (no glitch entity, no reward)
    if !crate::blocklist::is_clean(&word_lower) {
        info!("Blocked word — not in our lore");
        spelling.word.clear();
        return;
    }

    // Validate against WordDatabase
    if let Some(word_stats) = db.words.get(&word_lower) {
        info!("Spelled valid word: {} (Rank: {})", spelling.word, word_stats.grade_level);

        // Determine Element & Role from EtymologyDB
        let mut element = Element::Normal;
        let mut role = Role::Bruiser;
        
        // Root analysis
        for (root_name, root_data) in &db.etymology.roots {
            if word_lower.contains(&root_name.to_lowercase()) {
                element = match root_data.element.as_str() {
                    "Fire" => Element::Fire,
                    "Water" => Element::Water,
                    "Earth" => Element::Earth,
                    "Air" => Element::Air,
                    "Shadow" => Element::Shadow,
                    "Light" => Element::Light,
                    _ => Element::Normal,
                };
                break;
            }
        }

        // Suffix analysis
        for (suffix_name, suffix_data) in &db.etymology.suffixes {
            if word_lower.ends_with(&suffix_name.to_lowercase()) {
                role = match suffix_data.role.as_str() {
                    "Tank" => Role::Tank,
                    "Bruiser" => Role::Bruiser,
                    "Striker" => Role::Striker,
                    "Assassin" => Role::Assassin,
                    "Caster" => Role::Caster,
                    "Support" => Role::Support,
                    "Buffer" => Role::Buffer,
                    "Healer" => Role::Healer,
                    _ => Role::Bruiser,
                };
                break;
            }
        }

        // Calculate stats
        let stats = PetStats {
            logos: word_stats.concreteness * PET_LOGOS_MULTIPLIER,
            pathos: word_stats.valence * PET_PATHOS_MULTIPLIER,
            ethos: word_stats.dominance * PET_ETHOS_MULTIPLIER,
            speed: word_stats.intensity * PET_SPEED_MULTIPLIER,
        };

        // Classify emotive state using FACES detector
        // We'll run the zero-compute keyword matcher on the word definition
        let detected = faces_protocol::detect::detect_scored(&word_lower);

        // Stage the reveal: store pet data and transition to the reveal state
        commands.insert_resource(crate::pet_reveal::PendingReveal {
            word: spelling.word.clone(),
            element,
            role,
            stats,
            faces_state: detected.state,
            pet_type: sheet.active_summon_class,
        });
        spellbook.record_encounter(&word_lower, Channel::Mind, Some(element), Some(role), Some(stats), Some(crate::components::PetFacesState(detected.state)));
        demo.words_used += 1;

        spelling.word.clear();
        crate::commands::log_state_transition(state.get(), GameState::RevealingPet);
        next_state.set(GameState::RevealingPet);
    } else {
        warn!("Word not in lexicon: {}. Summoning an Unstable Mutant!", spelling.word);
        
        let glitch_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.1, 0.9), // magenta glitch
            emissive: (Color::srgb(0.9, 0.1, 0.9).to_srgba() * 2.0).into(),
            ..default()
        });

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(2).unwrap())),
            MeshMaterial3d(glitch_mat),
            Transform::from_translation(PET_SPAWN_POSITION),
            PetAvatar { 
                word: spelling.word.clone(),
                pet_type: sheet.active_summon_class,
            },
            crate::components::UnstableWord { health: 100.0 },
            PetVisualState::Alert,
        ));
        
        spelling.word.clear();
        crate::commands::log_state_transition(state.get(), GameState::Playing);
        next_state.set(GameState::Playing);
    }
}
