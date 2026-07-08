// deck.rs — Deck, Hand, and Discard mechanics using String word identifiers
use bevy::prelude::*;
use crate::components::*;
use crate::database::GameDatabase;

/// Refills the player's hand from the deck and records encounters in the spellbook.
pub fn draw_cards(
    mut deck: ResMut<Deck>,
    mut hand: ResMut<Hand>,
    mut spellbook: ResMut<SpellBook>,
    mut sheet: ResMut<CharacterSheet>,
    mut trail: ResMut<WordTrail>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if *state.get() != GameState::Playing && *state.get() != GameState::Battling {
        return;
    }

    // Refill hand up to max size
    while hand.cards.len() < hand.max_size {
        if let Some(word) = deck.cards.pop() {
            hand.cards.push(word.clone());

            // Record encounter in SpellBook & CharacterSheet
            // For now, we attune to Mind by default, or look it up
            sheet.engage_channel(&Channel::Mind);
            sheet.words_encountered += 1;
            spellbook.record_encounter(&word, Channel::Mind, None, None, None);

            if !trail.visited_words.contains(&word) {
                trail.visited_words.push(word);
            }
        } else {
            break;
        }
    }

    if hand.cards.is_empty() && deck.cards.is_empty() {
        crate::commands::log_state_transition(state.get(), GameState::Collecting);
        next_state.set(GameState::Collecting);
    }
}

/// Selects or deselects a hand card using number keys (1-5) or Escape.
pub fn select_card_by_key(
    keys: Res<ButtonInput<KeyCode>>,
    mut hand: ResMut<Hand>,
) {
    let key_map = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
    ];

    for (key, index) in key_map {
        if keys.just_pressed(key) && index < hand.cards.len() {
            hand.selected = Some(index);
            return;
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        hand.selected = None;
    }
}

/// Initializes the player's deck from the save file or rank lexicon on first enter.
pub fn initialize_player_deck(
    db: Res<GameDatabase>,
    grade_manager: Res<crate::quest::GradeManager>,
    mut deck: ResMut<Deck>,
    mut spellbook: ResMut<SpellBook>,
    mut stash: ResMut<crate::letter::LetterStash>,
    mut sheet: ResMut<CharacterSheet>,
    mut trail: ResMut<WordTrail>,
) {
    info!("Shuffling deck from rank lexicon...");

    if let Ok(data) = crate::save::load_game() {
        *sheet = data.character_sheet;
        *spellbook = data.spellbook;
        *trail = data.word_trail;
        for entry in &spellbook.entries {
            deck.cards.push(entry.word.clone());
        }
        info!("Loaded chronicle!");
    } else {
        let valid_grades = grade_manager.get_valid_grade_levels();
        let mut pool: Vec<String> = db.words.iter()
            .filter(|(_, stats): &(&String, &crate::database::WordStats)| valid_grades.contains(&stats.grade_level.as_str()))
            .map(|(word, _): (&String, &crate::database::WordStats)| word.clone())
            .collect();
        pool.sort();

        if !pool.is_empty() {
            for word in pool.iter().take(15) {
                deck.cards.push(word.clone());
                spellbook.record_encounter(word, Channel::Mind, None, None, None);
            }
        } else {
            let default_words = ["abandoned", "abc", "ability", "patience", "clarity", "courage", "wisdom", "strength"];
            for &word in &default_words {
                deck.cards.push(word.to_string());
                spellbook.record_encounter(word, Channel::Mind, None, None, None);
            }
        }
    }

    stash.letters.extend("PATIENCECLARITYCOURAGEWISDOMSTRENGTH".chars());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_defaults_are_empty_and_unselected() {
        let hand = Hand::default();
        assert!(hand.cards.is_empty());
        assert_eq!(hand.max_size, 3);
        assert!(hand.selected.is_none());
    }

    #[test]
    fn deck_defaults_with_empty_cards() {
        let deck = Deck::default();
        assert!(deck.cards.is_empty());
        assert!(deck.active_summon_class.is_none());
    }
}
