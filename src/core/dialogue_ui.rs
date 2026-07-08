// dialogue_ui.rs — 2D Visual Novel Overlay for Worldbuilding & NPC Interaction
use bevy::prelude::*;
use crate::components::*;
use crate::database::*;
use crate::time_cycle::DayNightCycle;

pub struct DialogueUiPlugin;

impl Plugin for DialogueUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Questing), setup_dialogue_ui)
           .add_systems(Update, update_dialogue_ui.run_if(in_state(GameState::Questing)))
           .add_systems(OnExit(GameState::Questing), cleanup_dialogue_ui);
    }
}

#[derive(Component)]
pub struct DialogueRoot;

#[derive(Component)]
pub struct DialoguePortrait;

#[derive(Component)]
pub struct DialogueWidget;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueUiTag {
    Name,
    Text,
}

fn setup_dialogue_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _db: Res<GameDatabase>,
) {
    info!("Setting up 2D Dialogue UI");
    
    // We will spawn the root node
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..default()
        },
        DialogueRoot,
    )).with_children(|parent| {
        // Container for Portrait + TextBox
        parent.spawn(Node {
            width: Val::Percent(80.0),
            height: Val::Percent(45.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::FlexEnd,
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        }).with_children(|row| {
            // Portrait
            row.spawn((
                Node {
                    width: Val::Px(400.0),
                    height: Val::Px(400.0),
                    margin: UiRect::right(Val::Px(20.0)),
                    ..default()
                },
                ImageNode::new(asset_server.load(crate::asset_catalog::BARNABY_AVATAR)),
                DialoguePortrait,
            ));
            
            // Text Box
            row.spawn(Node {
                width: Val::Percent(70.0),
                height: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            }).insert(BackgroundColor(Color::srgba(0.1, 0.15, 0.3, 0.85)))
              .with_children(|text_box| {
                  // Name
                  text_box.spawn((
                      Text::new("Storykeeper"),
                      TextFont {
                          font_size: 32.0,
                          ..default()
                      },
                      TextColor(Color::srgb(0.9, 0.8, 0.3)),
                      DialogueUiTag::Name,
                      DialogueWidget,
                  ));
                  // Spacer
                  text_box.spawn(Node { height: Val::Px(15.0), ..default() });
                  // Dialogue Text
                  text_box.spawn((
                      Text::new("A tale unfolds..."),
                      TextFont {
                          font_size: 24.0,
                          ..default()
                      },
                      TextColor(Color::WHITE),
                      DialogueUiTag::Text,
                      DialogueWidget,
                  ));
              });
        });
    });
}

fn update_dialogue_ui(
    db: Res<GameDatabase>,
    mut q_portrait: Query<&mut ImageNode, With<DialoguePortrait>>,
    mut q_text: Query<(&mut Text, &DialogueUiTag), With<DialogueWidget>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    cycle: Res<DayNightCycle>,
    mut active_npc: Local<Option<String>>,
) {
    // For testing, let's toggle between Barnaby and Kael every 5 seconds
    let seconds = (time.elapsed_secs() / 5.0) as u32;
    let new_npc = if seconds.is_multiple_of(2) { "Barnaby" } else { "Kael" };

    if active_npc.as_deref() != Some(new_npc) {
        *active_npc = Some(new_npc.to_string());

        if let Some(npc_data) = db.npcs.get(new_npc) {
            let time_label = match cycle.current_phase {
                crate::time_cycle::TimeOfDay::Dawn => "Dawn",
                crate::time_cycle::TimeOfDay::Day => "Day",
                crate::time_cycle::TimeOfDay::Dusk => "Dusk",
                crate::time_cycle::TimeOfDay::Night => "Night",
            };
            let dialogue = crate::quest::get_npc_dialogue(new_npc, &db, time_label);

            for (mut text, tag) in &mut q_text {
                match tag {
                    DialogueUiTag::Name => text.0 = new_npc.to_string(),
                    DialogueUiTag::Text => text.0 = dialogue.clone(),
                }
            }

            if let Ok(mut portrait) = q_portrait.single_mut() {
                if let Some(path) = &npc_data.avatar_path {
                    portrait.image = asset_server.load(path);
                }
            }
        }
    }
}

fn cleanup_dialogue_ui(mut commands: Commands, query: Query<Entity, With<DialogueRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
