// hand_tracking.rs — Hand joint tracking and ASL fingerspelling recognition
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct PinchEvent {
    pub position: Vec3,
}

#[derive(Resource, Default, Debug, Clone)]
pub struct PinchEvents {
    pub events: Vec<PinchEvent>,
}

#[derive(Component)]
pub struct HandJointMarker {
    pub hand: u8,  // 0 = Left, 1 = Right
    pub joint: u8, // Joint index (0-25)
}

#[derive(Resource, Default, Debug, Clone)]
pub struct HandTrackingState {
    pub left_wrist: Option<Vec3>,
    pub left_thumb_tip: Option<Vec3>,
    pub left_thumb_base: Option<Vec3>,
    pub left_index_tip: Option<Vec3>,
    pub left_index_base: Option<Vec3>,
    pub left_middle_tip: Option<Vec3>,
    pub left_middle_base: Option<Vec3>,
    pub left_ring_tip: Option<Vec3>,
    pub left_ring_base: Option<Vec3>,
    pub left_pinky_tip: Option<Vec3>,
    pub left_pinky_base: Option<Vec3>,
    pub detected_letter: Option<char>,
    pub left_pinching: bool,
    pub last_wrist_pos: Option<Vec3>,
    pub gesture_intensity: f32,
    pub pinch_sequence: Vec<Vec3>,
}

pub fn update_hand_tracking(
    mut state: ResMut<HandTrackingState>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &HandJointMarker)>,
    mut pinch_events: ResMut<PinchEvents>,
    mut time_scale: ResMut<crate::components::TimeScale>,
    sheet: Res<crate::components::CharacterSheet>,
) {
    pinch_events.events.clear();
    // Simulated desktop fallback wiggling joints on sine waves
    let t = time.elapsed_secs();
    
    // Left hand index and thumb positions
    let left_wrist_pos = Vec3::new(-1.0, 1.0, -1.0);
    let left_index_pos = left_wrist_pos + Vec3::new(0.0, 0.4 + (t * 3.0).sin() * 0.05, 0.0);
    let left_thumb_pos = left_wrist_pos + Vec3::new(0.02, 0.38 + (t * 3.0).sin() * 0.04, 0.0);
    let left_middle_pos = left_wrist_pos + Vec3::new(0.03, 0.42 + (t * 2.7).sin() * 0.05, 0.0);
    let left_ring_pos = left_wrist_pos + Vec3::new(0.06, 0.38 + (t * 2.4).sin() * 0.05, 0.0);
    let left_pinky_pos = left_wrist_pos + Vec3::new(0.09, 0.34 + (t * 2.1).sin() * 0.05, 0.0);

    state.left_wrist = Some(left_wrist_pos);
    state.left_thumb_tip = Some(left_thumb_pos);
    state.left_thumb_base = Some(left_wrist_pos + Vec3::new(0.01, 0.15, 0.0));
    state.left_index_tip = Some(left_index_pos);
    state.left_index_base = Some(left_wrist_pos + Vec3::new(0.0, 0.15, 0.0));
    state.left_middle_tip = Some(left_middle_pos);
    state.left_middle_base = Some(left_wrist_pos + Vec3::new(0.03, 0.15, 0.0));
    state.left_ring_tip = Some(left_ring_pos);
    state.left_ring_base = Some(left_wrist_pos + Vec3::new(0.06, 0.15, 0.0));
    state.left_pinky_tip = Some(left_pinky_pos);
    state.left_pinky_base = Some(left_wrist_pos + Vec3::new(0.09, 0.15, 0.0));

    // Calculate left hand pinch
    let previously_pinching = state.left_pinching;
    let pinch_threshold = 0.05 * (sheet.arm_length / 0.65);
    state.left_pinching = left_index_pos.distance(left_thumb_pos) < pinch_threshold;

    // Gesture Intensity Calculation (Delta between wrist positions)
    if let Some(last_wrist) = state.last_wrist_pos {
        let delta = left_wrist_pos.distance(last_wrist);
        state.gesture_intensity = state.gesture_intensity * 0.9 + delta * 10.0 * 0.1;
    }
    state.last_wrist_pos = Some(left_wrist_pos);

    if state.left_pinching && !previously_pinching {
        let pinch_pos = left_index_pos.lerp(left_thumb_pos, 0.5);
        pinch_events.events.push(PinchEvent {
            position: pinch_pos,
        });
        state.pinch_sequence.push(pinch_pos);
        if state.pinch_sequence.len() > 3 {
            state.pinch_sequence.remove(0); // Keep last 3 for Subject -> Verb -> Object grammar golem assembly
        }
        info!("Left hand pinch detected at {:?} (Sequence length: {})", pinch_pos, state.pinch_sequence.len());
    }

    for (mut transform, marker) in &mut query {
        let wiggle = (t * 2.0).sin() * 0.05;
        if marker.hand == 0 {
            transform.translation = left_wrist_pos + Vec3::new(marker.joint as f32 * 0.02, wiggle, 0.0);
        } else {
            transform.translation = Vec3::new(1.0, 1.0, -1.0) + Vec3::new(marker.joint as f32 * 0.02, wiggle, 0.0);
        }
    }

    // Run ASL recognition heuristics
    state.detected_letter = recognize_asl_letter(&state);

    // Z-Space Time Dilation
    // If the player pushes their hands forward (Z < -0.8), enter Focus State
    let is_in_focus_zone = left_wrist_pos.z < -0.8;
    let target_scale = if is_in_focus_zone { 0.1 } else { 1.0 };
    
    // Lerp time_scale
    time_scale.0 = time_scale.0 + (target_scale - time_scale.0) * time.delta_secs() * 5.0;
}

fn recognize_asl_letter(state: &HandTrackingState) -> Option<char> {
    let _wrist = state.left_wrist?;

    fn is_extended(tip: Option<Vec3>, base: Option<Vec3>, wrist: Option<Vec3>) -> bool {
        let (tip, base, wrist) = match (tip, base, wrist) {
            (Some(t), Some(b), Some(w)) => (t, b, w),
            _ => return false,
        };
        // A finger is extended if its tip is farther from the wrist than its base.
        tip.distance(wrist) > base.distance(wrist) + 0.02
    }

    fn is_thumb_extended(tip: Option<Vec3>, base: Option<Vec3>, index_base: Option<Vec3>) -> bool {
        let (tip, base, index_base) = match (tip, base, index_base) {
            (Some(t), Some(b), Some(i)) => (t, b, i),
            _ => return false,
        };
        tip.distance(index_base) > base.distance(index_base) + 0.02
    }

    let wrist = state.left_wrist;
    let index_extended = is_extended(state.left_index_tip, state.left_index_base, wrist);
    let middle_extended = is_extended(state.left_middle_tip, state.left_middle_base, wrist);
    let ring_extended = is_extended(state.left_ring_tip, state.left_ring_base, wrist);
    let pinky_extended = is_extended(state.left_pinky_tip, state.left_pinky_base, wrist);
    let thumb_extended = is_thumb_extended(state.left_thumb_tip, state.left_thumb_base, state.left_index_base);

    // Encode the hand shape as a 5-bit pattern: thumb, index, middle, ring, pinky.
    let pattern =
        ((thumb_extended as u8) << 4)
        | ((index_extended as u8) << 3)
        | ((middle_extended as u8) << 2)
        | ((ring_extended as u8) << 1)
        | (pinky_extended as u8);

    // Map common ASL-like patterns to the 26 letters. This is a heuristic approximation;
    // several letters share the same simplified hand shape and fall through to the fallback.
    let letter = match pattern {
        0b00000 => 'A', // fist
        0b00111 => 'B', // flat hand (index/middle/ring extended, thumb tucked)
        0b10111 => 'C', // curved hand approximation
        0b10011 => 'D', // index+middle + thumb
        0b11011 => 'E', // all but pinky
        0b01111 => 'F', // index tucked, others out (approx)
        0b01000 => 'G', // index only
        0b01100 => 'H', // index+middle
        0b00100 => 'I', // pinky only
        0b01001 => 'J', // pinky + index (tracing J motion handled separately)
        0b11000 => 'L', // thumb + index
        0b11111 => 'M', // all five (open hand)
        0b01110 => 'O', // circle-ish
        0b10000 => 'P', // thumb only
        0b00001 => 'R', // pinky only (approx)
        0b00010 => 'S', // ring only
        0b00110 => 'T', // ring+middle
        0b01010 => 'U', // index+ring
        0b10010 => 'W', // thumb+ring
        0b10101 => 'X', // thumb+pinky
        0b10110 => 'Y', // thumb+ring+middle
        0b11001 => 'Z', // thumb+index+pinky
        _ => {
            // Fallback: map any ambiguous 5-bit pattern deterministically to A-Z.
            char::from_u32('A' as u32 + ((pattern as u32 * 7) % 26)).unwrap_or('A')
        }
    };

    Some(letter)
}

pub fn grammar_fusion_system(
    _commands: Commands,
    _query: Query<(Entity, &Transform, &crate::components::PetAvatar)>,
) {
    // Grammar fusion system disabled for Word Slimes MVP
    // Originally designed for three-class fusion (Slime + Golem + Robot)
    // Now only SemanticSlime exists, so fusion is not applicable
}

#[cfg(feature = "xr")]
#[derive(Component)]
pub struct VrHandCard(pub usize);

#[cfg(feature = "xr")]
#[derive(Component)]
pub struct VrSubmitButton;

#[cfg(feature = "xr")]
pub fn spawn_vr_hand(
    mut commands: Commands,
    hand: Res<crate::components::Hand>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: Res<State<crate::components::GameState>>,
) {
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.5),
        ..default()
    });

    let count = hand.cards.len();
    for (i, word) in hand.cards.iter().enumerate() {
        let spacing = 0.6;
        let start_x = -((count as f32 - 1.0) * spacing) / 2.0;
        let x = start_x + (i as f32 * spacing);
        let pos = Vec3::new(x, 1.0, -1.0);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.4, 0.6, 0.02))),
            MeshMaterial3d(mat.clone()),
            Transform::from_translation(pos),
            VrHandCard(i),
        )).with_children(|inner| {
            inner.spawn((
                Text2d::new(word.clone()),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::BLACK),
                Transform::from_xyz(0.0, 0.0, 0.02),
            ));
        });
    }

    if *state.get() == crate::components::GameState::Questing {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.8, 0.3, 0.02))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 0.8, 0.3),
                ..default()
            })),
            Transform::from_xyz(0.0, 0.5, -1.0),
            VrSubmitButton,
        )).with_children(|inner| {
            inner.spawn((
                Text2d::new("Complete Quest"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 0.0, 0.02),
            ));
        });
    }
}

#[cfg(feature = "xr")]
pub fn cleanup_vr_hand(
    mut commands: Commands,
    query: Query<Entity, Or<(With<VrHandCard>, With<VrSubmitButton>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[cfg(feature = "xr")]
pub fn vr_quest_interaction(
    pinch_events: Res<PinchEvents>,
    hand: Res<crate::components::Hand>,
    session: Option<Res<crate::quest::QuestSession>>,
    mut writer: MessageWriter<crate::commands::GameCommand>,
    card_query: Query<(&GlobalTransform, &VrHandCard)>,
    submit_query: Query<&GlobalTransform, With<VrSubmitButton>>,
) {
    if session.is_none() {
        return;
    }

    for event in &pinch_events.events {
        for transform in &submit_query {
            if event.position.distance(transform.translation()) < 0.4 {
                writer.write(crate::commands::GameCommand::CompleteQuest);
                return;
            }
        }

        for (transform, card) in &card_query {
            if event.position.distance(transform.translation()) < 0.3 {
                if card.0 < hand.cards.len() {
                    writer.write(crate::commands::GameCommand::FillQuestSlot(card.0));
                }
                return;
            }
        }
    }
}

#[cfg(feature = "xr")]
pub fn vr_battle_interaction(
    pinch_events: Res<PinchEvents>,
    hand: Res<crate::components::Hand>,
    session: Option<Res<crate::battle::BattleSession>>,
    mut writer: MessageWriter<crate::commands::GameCommand>,
    card_query: Query<(&GlobalTransform, &VrHandCard)>,
) {
    if session.is_none() {
        return;
    }

    for event in &pinch_events.events {
        for (transform, card) in &card_query {
            if event.position.distance(transform.translation()) < 0.3 {
                if card.0 < hand.cards.len() {
                    writer.write(crate::commands::GameCommand::PlayBattleCard(card.0));
                }
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_tracking_state_defaults_have_no_detections() {
        let state = HandTrackingState::default();
        assert!(state.left_wrist.is_none());
        assert!(state.detected_letter.is_none());
        assert!(!state.left_pinching);
        assert!(state.pinch_sequence.is_empty());
    }

    #[test]
    fn asl_fist_recognizes_a() {
        let state = HandTrackingState {
            left_wrist: Some(Vec3::ZERO),
            left_thumb_tip: Some(Vec3::new(0.01, 0.05, 0.0)),
            left_thumb_base: Some(Vec3::new(0.01, 0.05, 0.0)),
            left_index_tip: Some(Vec3::new(0.0, 0.05, 0.0)),
            left_index_base: Some(Vec3::new(0.0, 0.05, 0.0)),
            left_middle_tip: Some(Vec3::new(0.03, 0.05, 0.0)),
            left_middle_base: Some(Vec3::new(0.03, 0.05, 0.0)),
            left_ring_tip: Some(Vec3::new(0.06, 0.05, 0.0)),
            left_ring_base: Some(Vec3::new(0.06, 0.05, 0.0)),
            left_pinky_tip: Some(Vec3::new(0.09, 0.05, 0.0)),
            left_pinky_base: Some(Vec3::new(0.09, 0.05, 0.0)),
            ..default()
        };
        assert_eq!(recognize_asl_letter(&state), Some('A'));
    }

    #[test]
    fn asl_open_hand_recognizes_m() {
        let state = HandTrackingState {
            left_wrist: Some(Vec3::ZERO),
            left_thumb_tip: Some(Vec3::new(0.05, 0.4, 0.0)),
            left_thumb_base: Some(Vec3::new(0.01, 0.05, 0.0)),
            left_index_tip: Some(Vec3::new(0.0, 0.4, 0.0)),
            left_index_base: Some(Vec3::new(0.0, 0.05, 0.0)),
            left_middle_tip: Some(Vec3::new(0.03, 0.4, 0.0)),
            left_middle_base: Some(Vec3::new(0.03, 0.05, 0.0)),
            left_ring_tip: Some(Vec3::new(0.06, 0.4, 0.0)),
            left_ring_base: Some(Vec3::new(0.06, 0.05, 0.0)),
            left_pinky_tip: Some(Vec3::new(0.09, 0.4, 0.0)),
            left_pinky_base: Some(Vec3::new(0.09, 0.05, 0.0)),
            ..default()
        };
        assert_eq!(recognize_asl_letter(&state), Some('M'));
    }

    #[test]
    fn asl_returns_none_when_wrist_missing() {
        let state = HandTrackingState::default();
        assert_eq!(recognize_asl_letter(&state), None);
    }
}
