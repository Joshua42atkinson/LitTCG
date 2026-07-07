// input.rs — Input: Touch Drag / Mouse Swipe / Keyboard Gesture Detection
#![allow(dead_code)]
use bevy::prelude::*;
use crate::components::*;

#[derive(Resource, Default)]
pub struct DragState {
    pub active: bool,
    pub start_pos: Vec2,
    pub current_pos: Vec2,
}

#[derive(Resource, Default)]
pub struct PendingSwipe {
    pub direction: Option<SwipeChoice>,
}

const SWIPE_THRESHOLD: f32 = 150.0;

pub fn drag_start(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut drag: ResMut<DragState>,
    slide: Res<CurrentSlide>,
) {
    if !slide.ready_for_input || slide.depth_showing {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.iter().next() {
            if let Some(pos) = window.cursor_position() {
                drag.active = true;
                drag.start_pos = pos;
                drag.current_pos = pos;
            }
        }
    }
}

pub fn drag_move(
    windows: Query<&Window>,
    mut drag: ResMut<DragState>,
) {
    if !drag.active { return; }
    if let Some(window) = windows.iter().next() {
        if let Some(pos) = window.cursor_position() {
            drag.current_pos = pos;
        }
    }
}

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
            } else {
                None
            };
        }
        drag.active = false;
    }
}

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut pending: ResMut<PendingSwipe>,
    slide: Res<CurrentSlide>,
) {
    if !slide.ready_for_input || slide.depth_showing { return; }

    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        pending.direction = Some(SwipeChoice::Yes);
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        pending.direction = Some(SwipeChoice::No);
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS)
        || keys.just_pressed(KeyCode::Space) {
        pending.direction = Some(SwipeChoice::Deeper);
    }
}

pub fn handle_ui_button_interactions(
    mut commands: Commands,
    db_opt: Option<Res<crate::database::GameDatabase>>,
    mut curriculum: ResMut<crate::quest::CurriculumManager>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    mut hand: ResMut<Hand>,
    mut spellbook: ResMut<SpellBook>,
    mut sheet: ResMut<CharacterSheet>,
    mut session_battle: Option<ResMut<crate::battle::BattleSession>>,
    mut session_quest: Option<ResMut<crate::quest::QuestSession>>,
    mut chat_log: ResMut<crate::chat::ChatLog>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    
    // Interactions grouped to avoid 16 argument limit
    mut interactions: (
        Query<(&Interaction, &crate::hud::HandCardUi), (Changed<Interaction>, With<Button>)>,
        Query<&Interaction, (Changed<Interaction>, With<crate::hud::PlayCardButton>)>,
        Query<&Interaction, (Changed<Interaction>, With<crate::hud::SkipButton>)>,
        Query<&Interaction, (Changed<Interaction>, With<crate::hud::QuestActionButton>)>,
        Query<&Interaction, (Changed<Interaction>, With<crate::hud::BattleActionButton>)>,
    ),
) {
    let (card_buttons, play_button, skip_button, quest_button, battle_button) = &mut interactions;
    let db = match db_opt {
        Some(d) => d,
        None => return,
    };
    
    // 1. Hand card selection
    for (interaction, card_ui) in card_buttons {
        if *interaction == Interaction::Pressed {
            hand.selected = Some(card_ui.0);
            info!("Selected card index: {}", card_ui.0);
        }
    }

    // 2. Play Card Button clicked
    for interaction in play_button {
        if *interaction == Interaction::Pressed {
            info!("Play Card clicked!");
            match *state.get() {
                GameState::Playing => {
                    if hand.selected.is_some() {
                        crate::battle::start_battle(&mut commands, &db, &curriculum, &mut next_state);
                    } else {
                        warn!("Select a card first!");
                    }
                }
                GameState::Battling => {
                    if let Some(ref mut session) = session_battle {
                        if let Some(idx) = hand.selected {
                            if idx < hand.cards.len() {
                                let played_word = hand.cards.remove(idx);
                                let typo_word = session.typo_word.clone();
                                let result = crate::battle::play_battle_card(&played_word, session, &db, &mut spellbook, &mut next_state, &sheet);
                                if result.is_effective {
                                    commands.spawn(crate::battle::CriticalHitTrigger);
                                }
                                if result.social_combat_triggered {
                                    crate::chat::trigger_social_combat(
                                        &played_word,
                                        &typo_word,
                                        result.is_synonym_logic,
                                        time.elapsed_secs(),
                                        &mut chat_log,
                                        &mut commands,
                                        &asset_server
                                    );
                                }
                                hand.selected = None;
                            }
                        } else {
                            warn!("Select a card first to play!");
                        }
                    }
                }
                GameState::Questing => {
                    if let Some(ref mut session) = session_quest {
                        if session.filled_slots.len() >= session.slots.len() {
                            crate::quest::complete_quest(session, &mut sheet, &mut spellbook, &mut curriculum, &mut next_state, &mut commands);
                        } else if let Some(idx) = hand.selected {
                            if idx < hand.cards.len() {
                                let word = &hand.cards[idx];
                                let slots_count = session.slots.len();
                                for i in 0..slots_count {
                                    if !session.filled_slots.contains_key(&i) {
                                        crate::quest::fill_slot(i, word, None, session);
                                        break;
                                    }
                                }
                                hand.selected = None;
                            }
                        } else {
                            warn!("Select a card first or complete quest if full!");
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // 3. Quest Action Button (Flat screen)
    for interaction in quest_button {
        if *interaction == Interaction::Pressed {
            if *state.get() == GameState::Playing {
                crate::quest::start_quest("Barnaby", &db, &curriculum, &mut commands, &mut next_state);
            }
        }
    }

    // 4. Battle Action Button (Flat screen)
    for interaction in battle_button {
        if *interaction == Interaction::Pressed {
            if *state.get() == GameState::Playing {
                crate::battle::start_battle(&mut commands, &db, &curriculum, &mut next_state);
            }
        }
    }

    // 5. Skip Button
    for interaction in skip_button {
        if *interaction == Interaction::Pressed {
            info!("Skip clicked!");
            match *state.get() {
                GameState::Playing => {
                    crate::quest::start_quest("Barnaby", &db, &curriculum, &mut commands, &mut next_state);
                }
                GameState::Battling => {
                    info!("Retreating from battle!");
                    next_state.set(GameState::Playing);
                }
                GameState::Questing => {
                    info!("Canceling quest!");
                    next_state.set(GameState::Playing);
                }
                _ => {}
            }
        }
    }
}

pub fn handle_touch_input(
    mut touch_evr: MessageReader<bevy::input::touch::TouchInput>,
    mut gestures: ResMut<ActiveGestures>,
    mut cards: Query<(&mut Transform, &mut DraggableCard)>,
) {
    for ev in touch_evr.read() {
        match ev.phase {
            bevy::input::touch::TouchPhase::Started => {
                gestures.traces.insert(ev.id, vec![ev.position]);
            }
            bevy::input::touch::TouchPhase::Moved => {
                if let Some(trace) = gestures.traces.get_mut(&ev.id) {
                    trace.push(ev.position);
                }
                
                for (mut transform, card) in cards.iter_mut() {
                    if card.touch_id == Some(ev.id) {
                        transform.translation.x = ev.position.x;
                        transform.translation.y = ev.position.y;
                    }
                }
            }
            bevy::input::touch::TouchPhase::Ended | bevy::input::touch::TouchPhase::Canceled => {
                gestures.traces.remove(&ev.id);
            }
        }
    }
}

