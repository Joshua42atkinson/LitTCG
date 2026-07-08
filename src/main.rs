// main.rs — Core Bevy entry point for Desktop target
pub mod core {
    pub mod components;
    pub mod database;
    pub mod asset_catalog;
    pub mod generated_assets;
    pub mod deck;
    pub mod input;
    pub mod render;
    pub mod quest;
    pub mod battle;
    pub mod letter;
    pub mod hand_tracking;
    pub mod save;
    pub mod spatial_ui;
    pub mod chat;
    pub mod hud;
    pub mod menu;
    pub mod tutorial;
    pub mod paywall;
    pub mod settings;
    pub mod difficulty;
    pub mod pet_collection;
    pub mod pet_reveal;
    pub mod time_cycle;
    pub mod spatial_deck;
    pub mod altar;
    pub mod dialogue_ui;
    pub mod blocklist;
    pub mod commands;
    pub mod diagnostics;
    pub mod platform_paths;
    pub mod performance;
}

pub mod bridge {
    pub mod tts_client;
    pub mod url_opener;
}

pub use core::*;

use bevy::prelude::*;
use components::*;
use database::*;
use letter::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "LitTCG — Literary Trading Card Game".to_string(),
                    resolution: bevy::window::WindowResolution::new(1280, 720),
                    mode: bevy::window::WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }).set(bevy::asset::AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            }),
            render::RenderPlugin,
            chat::ChatPlugin,
            battle::BattlePlugin,
            quest::QuestPlugin,
            hud::HudPlugin,
            menu::MenuPlugin,
            tutorial::TutorialPlugin,
            paywall::PaywallPlugin,
            time_cycle::TimeCyclePlugin,
            spatial_ui::SpatialUiPlugin,
            database::DatabasePlugin,
            spatial_deck::SpatialDeckPlugin,
            altar::AltarPlugin,
            dialogue_ui::DialogueUiPlugin,
        ))
        .add_plugins((
            performance::PerformancePlugin,
            settings::SettingsPlugin,
            difficulty::DifficultyPlugin,
            pet_collection::PetCollectionPlugin,
            pet_reveal::PetRevealPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.3)))
        .init_state::<GameState>()
        .add_message::<commands::GameCommand>()
        .init_resource::<commands::LastCommand>()
        .init_resource::<GameDatabase>()
        .init_resource::<Deck>()
        .init_resource::<Hand>()
        .init_resource::<DiscardPile>()
        .init_resource::<input::DragState>()
        .init_resource::<LetterStash>()
        .init_resource::<CurrentSpelling>()
        .init_resource::<CharacterSheet>()
        .init_resource::<SpellBook>()
        .init_resource::<WordTrail>()
        .init_resource::<CurrentSlide>()
        .init_resource::<hand_tracking::HandTrackingState>()
        .init_resource::<diagnostics::FrameDiagnostics>()
        .init_resource::<quest::GradeManager>()
        .init_resource::<hand_tracking::PinchEvents>()
        .init_resource::<crate::components::TimeScale>()
        .init_resource::<ActiveGestures>()
        .insert_resource(crate::settings::GameSettings::load().unwrap_or_default());

    #[cfg(not(feature = "flat2d"))]
    app.add_systems(Startup, (render::setup_world, generated_assets::load_generated_assets));

    app.add_systems(OnEnter(GameState::Loading), (database::spawn_loading_ui, database::start_loading_database))
        .add_systems(Update, (
            database::update_loading_progress,
            database::check_database_loading,
        ).run_if(in_state(GameState::Loading)))
        .add_systems(OnExit(GameState::Loading), database::cleanup_loading_ui)
        .add_systems(Update, database::hot_reload_database)
        .add_systems(OnEnter(GameState::Collecting), deck::initialize_player_deck)
        .add_systems(Update, (
            spawn_letter_crystals,
            animate_crystals,
            collect_letters,
            handle_pinch_crystals,
        ).run_if(in_state(GameState::Collecting)))
        .add_systems(OnEnter(GameState::Playing), save::auto_save_system)
        .add_systems(Update, (
            deck::select_card_by_key,
        ).run_if(in_state(GameState::Playing)));

    #[cfg(feature = "xr")]
    {
        app.add_systems(OnEnter(GameState::Reviewing), (hud::spawn_review_ui_xr, save::auto_save_system))
           .add_systems(OnExit(GameState::Reviewing), hud::cleanup_review_ui_xr);
    }

    #[cfg(not(feature = "xr"))]
    {
        app.add_systems(OnEnter(GameState::Reviewing), (hud::spawn_review_ui_2d, save::auto_save_system))
           .add_systems(OnExit(GameState::Reviewing), hud::cleanup_review_ui_2d);
    }

    app.add_systems(Update, hud::review_input_system.run_if(in_state(GameState::Reviewing)))
       .add_systems(Update, commands::handle_game_commands)
       .add_systems(Update, (
            hand_tracking::update_hand_tracking,
            deck::draw_cards,
            diagnostics::update_frame_diagnostics,
            input::drag_start,
            input::drag_move,
            input::drag_end,
            input::keyboard_input,
            input::handle_touch_input,
            input::handle_hand_card_button_interactions,
            input::handle_play_card_button_interactions,
            input::handle_skip_button_interactions,
            input::handle_quest_action_button_interactions,
            input::handle_battle_action_button_interactions,
            hand_tracking::grammar_fusion_system,
            letter::handle_keyboard_spelling,
        ).before(commands::handle_game_commands));

    #[cfg(feature = "xr")]
    {
        app.add_systems(OnEnter(GameState::Constructing), letter::spawn_holographic_stash)
           .add_systems(Update, letter::handle_vr_spelling.run_if(in_state(GameState::Constructing)).before(commands::handle_game_commands))
           .add_systems(OnExit(GameState::Constructing), letter::cleanup_holographic_stash)
           .add_systems(OnEnter(GameState::Questing), hand_tracking::spawn_vr_hand)
           .add_systems(Update, hand_tracking::vr_quest_interaction.run_if(in_state(GameState::Questing)).before(commands::handle_game_commands))
           .add_systems(OnExit(GameState::Questing), hand_tracking::cleanup_vr_hand)
           .add_systems(OnEnter(GameState::Battling), hand_tracking::spawn_vr_hand)
           .add_systems(Update, hand_tracking::vr_battle_interaction.run_if(in_state(GameState::Battling)).before(commands::handle_game_commands))
           .add_systems(OnExit(GameState::Battling), hand_tracking::cleanup_vr_hand);
    }

    #[cfg(not(feature = "xr"))]
    {
        app.add_systems(Update, input::keyboard_quest_interaction.run_if(in_state(GameState::Questing)).before(commands::handle_game_commands))
           .add_systems(Update, input::keyboard_battle_interaction.run_if(in_state(GameState::Battling)).before(commands::handle_game_commands));
    }

    app.run();
}
