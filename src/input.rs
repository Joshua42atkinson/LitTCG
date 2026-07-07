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
    mut writer: MessageWriter<crate::commands::GameCommand>,
    slide: Res<CurrentSlide>,
) {
    if !drag.active { return; }

    if mouse.just_released(MouseButton::Left) {
        let delta = drag.current_pos - drag.start_pos;
        let magnitude = delta.length();

        if magnitude > SWIPE_THRESHOLD && slide.ready_for_input {
            let abs_x = delta.x.abs();
            let abs_y = delta.y.abs();

            let choice = if abs_x > abs_y {
                Some(if delta.x > 0.0 { SwipeChoice::Yes } else { SwipeChoice::No })
            } else if delta.y > 0.0 {
                Some(SwipeChoice::Deeper)
            } else {
                None
            };
            if let Some(choice) = choice {
                info!("Swipe detected: {:?} (magnitude: {:.1})", choice, magnitude);
                writer.write(crate::commands::GameCommand::Swipe(choice));
            }
        }
        drag.active = false;
    }
}

pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<crate::commands::GameCommand>,
    slide: Res<CurrentSlide>,
) {
    if !slide.ready_for_input || slide.depth_showing { return; }

    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        info!("Keyboard swipe: Yes");
        writer.write(crate::commands::GameCommand::Swipe(SwipeChoice::Yes));
    } else if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        info!("Keyboard swipe: No");
        writer.write(crate::commands::GameCommand::Swipe(SwipeChoice::No));
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS)
        || keys.just_pressed(KeyCode::Space) {
        info!("Keyboard swipe: Deeper");
        writer.write(crate::commands::GameCommand::Swipe(SwipeChoice::Deeper));
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_ui_button_interactions(
    mut writer: MessageWriter<crate::commands::GameCommand>,
    state: Res<State<GameState>>,
    hand: Res<Hand>,

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

    // 1. Hand card selection
    for (interaction, card_ui) in card_buttons {
        if *interaction == Interaction::Pressed {
            if card_ui.0 < hand.cards.len() {
                writer.write(crate::commands::GameCommand::SelectCard(card_ui.0));
                info!("Selected card index: {}", card_ui.0);
            } else {
                warn!("Hand card UI index {} out of bounds", card_ui.0);
            }
        }
    }

    // 2. Play Card Button clicked
    for interaction in play_button {
        if *interaction == Interaction::Pressed {
            info!("Play Card clicked!");
            writer.write(crate::commands::GameCommand::PlayCard);
        }
    }

    // 3. Quest Action Button (Flat screen)
    for interaction in quest_button {
        if *interaction == Interaction::Pressed {
            info!("Quest Action button clicked");
            if *state.get() == GameState::Playing {
                writer.write(crate::commands::GameCommand::StartQuest("Barnaby".to_string()));
            }
        }
    }

    // 4. Battle Action Button (Flat screen)
    for interaction in battle_button {
        if *interaction == Interaction::Pressed {
            info!("Battle Action button clicked");
            if *state.get() == GameState::Playing {
                writer.write(crate::commands::GameCommand::StartBattle);
            }
        }
    }

    // 5. Skip Button
    for interaction in skip_button {
        if *interaction == Interaction::Pressed {
            info!("Skip clicked!");
            match *state.get() {
                GameState::Playing => { writer.write(crate::commands::GameCommand::StartQuest("Barnaby".to_string())); }
                GameState::Battling => { writer.write(crate::commands::GameCommand::FleeBattle); }
                GameState::Questing => { writer.write(crate::commands::GameCommand::CancelQuest); }
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

