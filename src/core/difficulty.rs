// difficulty.rs — Difficulty selection screen stub
use bevy::prelude::*;
use crate::components::GameState;
use crate::quest::GradeManager;

pub struct DifficultyPlugin;

#[derive(Component)]
pub struct DifficultyUiRoot;

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Difficulty), spawn_difficulty_ui)
           .add_systems(Update, difficulty_interaction.run_if(in_state(GameState::Difficulty)))
           .add_systems(OnExit(GameState::Difficulty), cleanup_difficulty_ui);
    }
}

fn spawn_difficulty_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.95)),
        DifficultyUiRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("CHOOSE YOUR RANK"),
            TextFont { font_size: 50.0, ..default() },
            TextColor(Color::WHITE),
            Node { margin: UiRect::bottom(Val::Px(30.0)), ..default() },
        ));
        for (label, rank) in [("Novice", 1), ("Apprentice", 2), ("Adept", 3), ("Sage", 4)] {
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                DifficultyRank(rank),
            )).with_children(|p| {
                p.spawn((
                    Text::new(label),
                    TextFont { font_size: 24.0, ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        }
        parent.spawn((
            Button,
            Node {
                width: Val::Px(250.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        )).with_children(|p| {
            p.spawn((
                Text::new("Main Menu"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

#[derive(Component)]
struct DifficultyRank(u32);

fn difficulty_interaction(
    mut next_state: ResMut<NextState<GameState>>,
    mut grade_manager: ResMut<GradeManager>,
    rank_buttons: Query<(&Interaction, &DifficultyRank), Changed<Interaction>>,
    menu_buttons: Query<&Interaction, (Changed<Interaction>, Without<DifficultyRank>)>,
) {
    for (interaction, rank) in &rank_buttons {
        if *interaction == Interaction::Pressed {
            grade_manager.active_grade = rank.0;
            info!("Difficulty set to rank {}", rank.0);
            crate::commands::log_state_transition(&GameState::Difficulty, GameState::MainMenu);
            next_state.set(GameState::MainMenu);
        }
    }
    for interaction in &menu_buttons {
        if *interaction == Interaction::Pressed {
            crate::commands::log_state_transition(&GameState::Difficulty, GameState::MainMenu);
            next_state.set(GameState::MainMenu);
        }
    }
}

fn cleanup_difficulty_ui(mut commands: Commands, query: Query<Entity, With<DifficultyUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
