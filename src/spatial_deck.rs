// spatial_deck.rs — 3D Holographic Programmer Art for Cards and Decks
#![allow(dead_code)]
use bevy::prelude::*;
use crate::components::*;

#[derive(Component)]
pub struct SpatialCardUi {
    pub index: usize,
}

#[derive(Component)]
pub struct SpatialDeckUi;

pub struct SpatialDeckPlugin;

impl Plugin for SpatialDeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_spatial_hand);
    }
}

// Automatically sync the 3D cards in the world with the Hand resource
fn update_spatial_hand(
    mut commands: Commands,
    hand: Res<Hand>,
    mut query: Query<(Entity, &SpatialCardUi, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if !hand.is_changed() && query.iter().count() > 0 {
        // If hand didn't change and we already spawned cards, just animate them
        let t = time.elapsed_secs();
        for (_, card_ui, mut transform) in query.iter_mut() {
            let bob = (t * 2.0 + card_ui.index as f32).sin() * 0.05;
            let target_y = 1.0 + bob;
            
            // Highlight the selected card by pushing it forward and up
            if hand.selected == Some(card_ui.index) {
                transform.translation.y = 1.2 + bob;
                transform.translation.z = -1.2;
                transform.scale = Vec3::splat(1.2);
            } else {
                transform.translation.y = target_y;
                transform.translation.z = -1.5;
                transform.scale = Vec3::splat(1.0);
            }
        }
        return;
    }

    // Despawn old cards
    for (entity, _, _) in query.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new programmer art cards
    let card_width = 0.6;
    let card_height = 0.9;
    let spacing = 0.8;
    
    let total_width = (hand.cards.len() as f32 - 1.0) * spacing;
    let start_x = -total_width / 2.0;

    let card_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.1, 0.2, 0.4, 0.8),
        emissive: Color::srgb(0.0, 0.5, 1.0).into(),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });

    for (i, word) in hand.cards.iter().enumerate() {
        let x = start_x + (i as f32 * spacing);
        let pos = Vec3::new(x, 1.0, -1.5);
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(card_width, card_height, 0.02))),
            MeshMaterial3d(card_mat.clone()),
            Transform::from_translation(pos),
            SpatialCardUi { index: i },
        ))
        .with_children(|parent| {
            // Card Border / Frame (Programmer Art)
            let frame_mat = materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: Color::WHITE.into(),
                ..default()
            });
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(card_width + 0.04, card_height + 0.04, 0.01))),
                MeshMaterial3d(frame_mat),
                Transform::from_xyz(0.0, 0.0, -0.01),
            ));

            // Word Text
            parent.spawn((
                Text2d::new(word),
                TextFont { font_size: 40.0, ..default() },
                TextColor(Color::WHITE),
                Transform::from_xyz(0.0, 0.0, 0.02),
            ));
        });
    }
}
