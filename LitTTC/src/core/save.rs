// save.rs — Simple local JSON saving and loading of word-walker progress
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use bevy::prelude::*;
use crate::components::*;
use crate::battle::VaamMetrics;
use crate::platform_paths::data_dir;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct SaveData {
    pub character_sheet: CharacterSheet,
    pub spellbook: SpellBook,
    pub word_trail: WordTrail,
    pub vaam_metrics: VaamMetrics,
}

impl SaveData {
    pub const SAVE_PATH: &str = "save.json";

    pub fn save_path() -> PathBuf {
        data_dir().join(Self::SAVE_PATH)
    }
}

/// Writes the current character sheet, spellbook, word trail, and VAAM metrics to save.json.
pub fn save_game(
    sheet: &CharacterSheet,
    spellbook: &SpellBook,
    trail: &WordTrail,
    metrics: &VaamMetrics,
) -> Result<(), std::io::Error> {
    let data = SaveData {
        character_sheet: sheet.clone(),
        spellbook: SpellBook { entries: spellbook.entries.clone() },
        word_trail: trail.clone(),
        vaam_metrics: metrics.clone(),
    };

    let serialized = serde_json::to_string_pretty(&data)?;

    let path = SaveData::save_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}

/// Loads SaveData from save.json, returning an error if the file is missing or corrupt.
pub fn load_game() -> Result<SaveData, std::io::Error> {
    let mut file = File::open(SaveData::save_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: SaveData = serde_json::from_str(&contents)?;
    Ok(data)
}

/// Bevy system that periodically saves progress. Disabled in demo mode.
pub fn auto_save_system(
    sheet: Res<CharacterSheet>,
    spellbook: Res<SpellBook>,
    trail: Res<WordTrail>,
    metrics: Res<VaamMetrics>,
    demo: Res<crate::paywall::DemoSettings>,
) {
    if demo.is_demo {
        return; // Disable saving in demo mode
    }

    if let Err(e) = save_game(&sheet, &spellbook, &trail, &metrics) {
        warn!("Failed to auto-save: {}", e);
    } else {
        info!("Auto-saved progress.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_data_roundtrips_through_json() {
        let sheet = CharacterSheet {
            mind_attunement: 0.5,
            heart_attunement: 0.3,
            body_attunement: 0.1,
            action_attunement: 0.05,
            emergent_class: "The Oracle".to_string(),
            words_encountered: 7,
            total_deeper_swipes: 2,
            total_xp: 120,
            active_summon_class: SummonClass::SemanticSlime,
            arm_length: 0.65,
            last_grades: GradeScores::default(),
            telemetry: TelemetrySeries::default(),
        };
        let mut spellbook = SpellBook::default();
        spellbook.record_encounter("clarity", Channel::Mind, None, None, None, None);
        spellbook.upgrade_mastery("clarity", MasteryLevel::Mastered);
        let trail = WordTrail {
            visited_words: vec!["clarity".to_string()],
            swipe_history: vec![SwipeChoice::Deeper],
            current_word: Some("clarity".to_string()),
        };

        let metrics = VaamMetrics::default();
        let data = SaveData {
            character_sheet: sheet.clone(),
            spellbook: SpellBook { entries: spellbook.entries.clone() },
            word_trail: trail.clone(),
            vaam_metrics: metrics.clone(),
        };
        let json = serde_json::to_string_pretty(&data).expect("should serialize");
        let loaded: SaveData = serde_json::from_str(&json).expect("should deserialize");
        assert_eq!(loaded.character_sheet.emergent_class, sheet.emergent_class);
        assert_eq!(loaded.character_sheet.total_xp, sheet.total_xp);
        assert_eq!(loaded.spellbook.entries.len(), 1);
        assert_eq!(loaded.spellbook.entries[0].mastery, MasteryLevel::Mastered);
        assert_eq!(loaded.word_trail.visited_words, trail.visited_words);
    }
}
