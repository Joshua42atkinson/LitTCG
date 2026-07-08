// generated_assets.rs — Loads AI-generated portraits from the lit-asset-forge manifest.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Loaded asset manifest resource.
#[derive(Resource, Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeneratedAssets {
    pub portraits: HashMap<String, PortraitEntry>,
    pub loaded: bool,
}

/// Lore attached to a generated portrait.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PetLore {
    pub word: String,
    pub title: String,
    pub description: String,
    pub habitat: String,
    pub behavior: String,
    pub fun_fact: String,
    pub etymology_hook: String,
    pub npc_guardian: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortraitEntry {
    pub relative_path: PathBuf,
    pub prompt: String,
    pub element: String,
    pub role: String,
    pub summon_class: String,
    pub seed: u64,
    #[serde(default)]
    pub lore: PetLore,
}

impl GeneratedAssets {
    /// Attempt to load the manifest from `assets/generated/asset_manifest.json`.
    pub fn load_from_disk() -> Self {
        let path = PathBuf::from("assets/generated/asset_manifest.json");
        if !path.exists() {
            return Self::default();
        }
        match std::fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str::<GeneratedAssets>(&content) {
                Ok(mut assets) => {
                    assets.loaded = true;
                    assets
                }
                Err(e) => {
                    bevy::log::warn!("Failed to parse generated asset manifest: {}", e);
                    Self::default()
                }
            },
            Err(e) => {
                bevy::log::warn!("Failed to read generated asset manifest: {}", e);
                Self::default()
            }
        }
    }

    /// Return the portrait asset path for a word, if one exists.
    pub fn portrait_path(&self, word: &str) -> Option<String> {
        self.portraits
            .get(&word.to_lowercase())
            .map(|e| e.relative_path.to_string_lossy().to_string())
    }

    /// Return the portrait asset path for a word, falling back to a generic placeholder.
    pub fn portrait_path_or_fallback(&self, word: &str, element: &str) -> String {
        self.portrait_path(word)
            .unwrap_or_else(|| format!("textures/avatars/{}.png", element.to_lowercase()))
    }

    /// Return the lore entry for a word, if one exists.
    pub fn lore(&self, word: &str) -> Option<&PetLore> {
        self.portraits.get(&word.to_lowercase()).map(|e| &e.lore)
    }
}

/// Startup system that loads the manifest into a Bevy resource.
pub fn load_generated_assets(mut commands: Commands) {
    let assets = GeneratedAssets::load_from_disk();
    if assets.loaded {
        bevy::log::info!("Loaded {} generated portraits", assets.portraits.len());
    } else {
        bevy::log::info!("No generated asset manifest found; running with fallback art.");
    }
    commands.insert_resource(assets);
}
