// commands.rs — GameCommand message bridge: input decoupled from game logic
use bevy::prelude::*;
use crate::components::{GameState, SwipeChoice};

/// A player-intent message fired by input systems and consumed by command handlers.
///
/// This enum decouples raw input (mouse clicks, keyboard, VR pinches, swipes)
/// from the game systems that mutate state. Every variant captures the data
/// needed to perform a high-level action without knowing which input device
/// triggered it.
///
/// Variants are intentionally allowed as dead code during Phase 1.1; they will
/// be wired to input systems in P1.2 and P1.3.
#[allow(dead_code)]
#[derive(Message, Clone, Debug, PartialEq)]
pub enum GameCommand {
    // ─── SPELLING / WORD CONSTRUCTION ─────────────────────────────────
    /// Submit the word currently being typed in the spelling pad.
    SubmitSpelling,
    /// Add a single letter to the current spelling.
    AddLetter(char),
    /// Remove the last letter from the current spelling.
    Backspace,
    /// Clear the entire current spelling.
    ClearSpelling,

    // ─── HAND / CARD INTERACTIONS ───────────────────────────────────────
    /// Select the card at the given index in the player's hand.
    SelectCard(usize),
    /// Play the currently selected card (state-dependent interpretation).
    PlayCard,
    /// Skip / cancel the current action (state-dependent interpretation).
    SkipCard,

    // ─── BATTLE ─────────────────────────────────────────────────────────
    /// Start a battle against a wild Typo.
    StartBattle,
    /// Play the card at the given index as a battle attack.
    PlayBattleCard(usize),
    /// Retreat from the current battle.
    FleeBattle,

    // ─── QUEST ──────────────────────────────────────────────────────────
    /// Start a quest from the named NPC.
    StartQuest(String),
    /// Fill the next empty quest slot with the card at the given index.
    FillQuestSlot(usize),
    /// Mark the current quest as complete.
    CompleteQuest,
    /// Cancel the active quest.
    CancelQuest,

    // ─── SWIPE / DIALOGUE ──────────────────────────────────────────────
    /// Commit to a swipe choice in a dialogue encounter.
    Swipe(SwipeChoice),

    // ─── REVIEW ─────────────────────────────────────────────────────────
    /// Dismiss the post-battle review screen and return to exploration.
    DismissReview,

    // ─── MENU ───────────────────────────────────────────────────────────
    /// Start a new game (clear save, begin tutorial, go to Collecting).
    NewGame,
    /// Continue from an existing save file.
    ContinueGame,
    /// Open the settings screen.
    OpenSettings,

    // ─── DIRECT STATE (used sparingly for transitions without extra data) ─
    /// Transition to a specific game state. Prefer semantic variants above.
    TransitionTo(GameState),
}

/// Resource that tracks the most recently fired command for debugging and replay.
#[allow(dead_code)]
#[derive(Resource, Default, Debug, Clone)]
pub struct LastCommand(pub Option<GameCommand>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_command_variants_round_trip() {
        let commands = vec![
            GameCommand::SubmitSpelling,
            GameCommand::SelectCard(3),
            GameCommand::PlayCard,
            GameCommand::StartBattle,
            GameCommand::PlayBattleCard(1),
            GameCommand::FleeBattle,
            GameCommand::StartQuest("Barnaby".to_string()),
            GameCommand::FillQuestSlot(0),
            GameCommand::CompleteQuest,
            GameCommand::CancelQuest,
            GameCommand::Swipe(SwipeChoice::Yes),
            GameCommand::DismissReview,
            GameCommand::NewGame,
            GameCommand::ContinueGame,
            GameCommand::TransitionTo(GameState::Playing),
        ];
        // Every variant must be PartialEq so this compiles and asserts equality.
        assert_eq!(commands, commands.clone());
    }
}
