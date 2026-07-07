// hud.rs - HUD Overlay for Character Sheet, XP, Mastery, and Stash
use bevy::prelude::*;
use crate::components::*;
use crate::letter::{LetterStash, CurrentSpelling};

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct BadgeNode;

#[derive(Component)]
pub struct BadgeText;

#[derive(Component)]
pub struct StashText;

#[derive(Component)]
pub struct SpellingText;

#[derive(Component)]
pub struct HandUiRoot;

#[derive(Component)]
pub struct HandCardUi(pub usize);

#[derive(Component)]
pub struct PlayCardButton;

#[derive(Component)]
pub struct SkipButton;

#[derive(Component)]
pub struct QuestActionButton;

#[derive(Component)]
pub struct BattleActionButton;

#[derive(Component)]
pub struct DeckCounterText;

#[derive(Component)]
pub struct XpProgressBarFill;

#[derive(Component)]
pub struct ActivePetText;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
           .add_systems(Update, (
               update_stats_ui,
               update_stash_ui,
               update_spelling_ui,
               update_hand_ui,
               update_deck_counter_ui,
               update_xp_progress_bar,
               update_active_pet_ui,
               update_badge_ui,
           ));
    }
}

fn setup_hud(mut commands: Commands) {
    // HUD Root Node (Absolute, full screen)
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        HudRoot,
    ))
    .with_children(|parent| {
        // Left Box: Stats
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(15.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
        )).with_children(|stats_parent| {
            stats_parent.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                },
            )).with_children(|row| {
                row.spawn((
                    Text::new("Class: Newcomer\nGrade: 1\nXP: 0\nWords: 0"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(Color::WHITE),
                    StatsText,
                ));
                row.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.8, 0.6, 0.2)),
                    BackgroundColor(Color::srgba(1.0, 0.8, 0.0, 0.8)),
                    BadgeNode,
                )).with_children(|badge| {
                    badge.spawn((
                        Text::new("N"), // Initial badge char
                        TextFont { font_size: 16.0, ..default() },
                        TextColor(Color::BLACK),
                        BadgeText,
                    ));
                });
            });

            // Deck counter
            stats_parent.spawn((
                Text::new("Deck: 0 cards"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                DeckCounterText,
            ));

            // XP Progress Bar label
            stats_parent.spawn((
                Text::new("XP Progress:"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            // XP Progress Bar container
            stats_parent.spawn((
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(15.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 1.0)),
                BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
            )).with_children(|bar| {
                bar.spawn((
                    Node {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                    XpProgressBarFill,
                ));
            });

            // Active Pet Text
            stats_parent.spawn((
                Text::new("Pet: None"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.4, 0.6, 0.9)),
                ActivePetText,
            ));
        });

        // Right Box: Stash & Controls
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(15.0)),
                align_items: AlignItems::FlexEnd,
                ..default()
            },
        )).with_children(|stash_parent| {
            stash_parent.spawn((
                Text::new("Stash: []"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.4, 0.8, 0.4)),
                StashText,
            ));
            stash_parent.spawn((
                Text::new("[P] Pet  [F] Feed  [T] Attune"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
    });

    // Left Side: Action Menu (Quest / Battle)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Percent(40.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(15.0),
            ..default()
        },
    )).with_children(|parent| {
        // Battle Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.9, 0.4, 0.4)),
            BackgroundColor(Color::srgba(0.3, 0.1, 0.1, 0.9)),
            BattleActionButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Explore (Battle)"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Quest Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::srgb(0.4, 0.4, 0.9)),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.3, 0.9)),
            QuestActionButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Talk (Quest)"),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });

    // Bottom Center: Controls (Play / Skip buttons)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(180.0),
            left: Val::Percent(50.0),
            width: Val::Px(300.0),
            margin: UiRect::left(Val::Px(-150.0)),
            justify_content: JustifyContent::SpaceBetween,
            column_gap: Val::Px(20.0),
            ..default()
        },
    )).with_children(|parent| {
        // Play Card Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(130.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::srgba(0.2, 0.6, 0.2, 0.9)),
            PlayCardButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Play Card"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // Skip Button
        parent.spawn((
            Button,
            Node {
                width: Val::Px(130.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::srgba(0.6, 0.2, 0.2, 0.9)),
            SkipButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Skip"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });

    // Bottom Center: Spelling text (absolute)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(230.0),
            left: Val::Percent(45.0),
            ..default()
        },
    )).with_children(|bottom_parent| {
        bottom_parent.spawn((
            Text::new(""),
            TextFont { font_size: 40.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.2)),
            SpellingText,
        ));
    });

    // Bottom: Cards container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            column_gap: Val::Px(15.0),
            ..default()
        },
        HandUiRoot,
    ));
}

fn update_stats_ui(
    sheet: Res<CharacterSheet>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    if sheet.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Class: {}\nGrade: {}\nXP: {}\nWords: {}", 
                sheet.emergent_class,
                (sheet.total_xp / 1000) + 1,
                sheet.total_xp,
                sheet.words_encountered
            );
        }
    }
}

fn update_badge_ui(
    sheet: Res<CharacterSheet>,
    mut text_query: Query<&mut Text, With<BadgeText>>,
    mut node_query: Query<(&mut BackgroundColor, &mut BorderColor), With<BadgeNode>>,
) {
    if sheet.is_changed() {
        let (initial, bg_color, border_color) = match sheet.emergent_class.as_str() {
            "Newcomer" => ("N", Color::srgba(0.8, 0.8, 0.8, 0.8), Color::WHITE),
            "The Oracle" => ("O", Color::srgba(0.2, 0.8, 1.0, 0.9), Color::srgb(0.0, 0.5, 1.0)),
            "The Bard" => ("B", Color::srgba(1.0, 0.4, 0.8, 0.9), Color::srgb(1.0, 0.2, 0.5)),
            "The Scholar" => ("S", Color::srgba(0.9, 0.9, 0.2, 0.9), Color::srgb(1.0, 0.8, 0.0)),
            _ => (&sheet.emergent_class[0..1], Color::srgba(1.0, 0.8, 0.0, 0.8), Color::WHITE),
        };

        for mut text in &mut text_query {
            text.0 = initial.to_string();
        }
        for (mut bg, mut border) in &mut node_query {
            bg.0 = bg_color;
            *border = BorderColor::all(border_color);
        }
    }
}

fn update_stash_ui(
    stash: Res<LetterStash>,
    mut query: Query<&mut Text, With<StashText>>,
) {
    if stash.is_changed() {
        for mut text in &mut query {
            let stash_str: String = stash.letters.iter().collect();
            text.0 = format!("Stash: [{}]", stash_str);
        }
    }
}

fn update_spelling_ui(
    spelling: Res<CurrentSpelling>,
    mut query: Query<&mut Text, With<SpellingText>>,
) {
    if spelling.is_changed() {
        for mut text in &mut query {
            text.0 = spelling.word.clone();
        }
    }
}

fn update_hand_ui(
    mut commands: Commands,
    hand: Res<crate::components::Hand>,
    root_query: Query<Entity, With<HandUiRoot>>,
    card_query: Query<Entity, With<HandCardUi>>,
    asset_server: Res<AssetServer>,
) {
    if hand.is_changed() {
        for entity in &card_query {
            commands.entity(entity).despawn();
        }

        if let Some(root) = root_query.iter().next() {
            commands.entity(root).with_children(|parent| {
                for (i, word) in hand.cards.iter().enumerate() {
                    let is_selected = hand.selected == Some(i);
                    let border_color = if is_selected { Color::srgb(1.0, 0.84, 0.0) } else { Color::srgba(0.0, 0.0, 0.0, 0.0) };
                    parent.spawn((
                        Button,
                        HandCardUi(i),
                        DraggableCard { touch_id: None },
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(160.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.0)),
                            border: UiRect::all(Val::Px(4.0)),
                            ..default()
                        },
                        BorderColor::all(border_color),
                        ImageNode::new(asset_server.load("ui/card_background.png")),
                    )).with_children(|card| {
                        card.spawn((
                            Text::new(format!("[{}]\n\n{}", i + 1, word)),
                            TextFont { font_size: 18.0, ..default() },
                            TextColor(Color::WHITE),
                            TextLayout::new_with_justify(Justify::Center),
                        ));
                    });
                }
            });
        }
    }
}

fn update_deck_counter_ui(
    deck: Res<Deck>,
    mut query: Query<&mut Text, With<DeckCounterText>>,
) {
    if deck.is_changed() {
        let deck_name = match deck.active_summon_class {
            Some(SummonClass::SemanticSlime) => "Slime Deck",
            Some(SummonClass::GrammarGolem) => "Golem Deck",
            Some(SummonClass::RhetoricRobot) => "Robot Deck",
            None => "Deck",
        };
        for mut text in &mut query {
            text.0 = format!("{}: {} cards", deck_name, deck.cards.len());
        }
    }
}

fn update_xp_progress_bar(
    time: Res<Time>,
    sheet: Res<CharacterSheet>,
    mut query: Query<&mut Node, With<XpProgressBarFill>>,
) {
    let target_progress = (sheet.total_xp % 1000) as f32 / 1000.0;
    for mut node in &mut query {
        let current_width = match node.width {
            Val::Percent(p) => p / 100.0,
            _ => 0.0,
        };
        
        let dt = time.delta_secs();
        let mut new_progress = current_width + (target_progress - current_width) * 5.0 * dt;
        
        if target_progress < current_width - 0.5 {
            new_progress = target_progress;
        }

        node.width = Val::Percent(new_progress * 100.0);
    }
}

fn update_active_pet_ui(
    pet_query: Query<(&PetAvatar, &Element)>,
    mut hud_query: Query<&mut Text, With<ActivePetText>>,
) {
    for mut text in &mut hud_query {
        if let Some((pet, element)) = pet_query.iter().next() {
            text.0 = format!("Pet: {} ({:?})", pet.word.to_uppercase(), element);
        } else {
            text.0 = "Pet: None".to_string();
        }
    }
}

