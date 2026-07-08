// manifest.rs — Asset manifest for generated LitTCG art

use crate::PortraitAsset;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

const MANIFEST_FILENAME: &str = "asset_manifest.json";

/// Full asset manifest consumed by LitTCG at runtime.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetManifest {
    pub version: String,
    pub generated_at: String,
    pub portraits: HashMap<String, PortraitEntry>,
    pub bodies_3d: HashMap<String, Body3DEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortraitEntry {
    pub relative_path: PathBuf,
    pub prompt: String,
    pub element: String,
    pub role: String,
    pub summon_class: String,
    pub seed: u64,
    pub lore: crate::lore::PetLore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body3DEntry {
    pub relative_path: PathBuf,
    pub archetype: String,
}

impl AssetManifest {
    pub fn new() -> Self {
        Self {
            version: "0.1.0".to_string(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            portraits: HashMap::new(),
            bodies_3d: HashMap::new(),
        }
    }

    /// Load manifest from the output directory if it exists.
    pub fn load(output_dir: &std::path::Path) -> Result<Self> {
        let path = output_dir.join(MANIFEST_FILENAME);
        if !path.exists() {
            return Ok(Self::new());
        }
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Reading manifest at {:?}", path))?;
        let manifest: AssetManifest = serde_json::from_str(&content)
            .with_context(|| format!("Parsing manifest at {:?}", path))?;
        Ok(manifest)
    }

    /// Save manifest to the output directory.
    pub fn save(&self, output_dir: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(output_dir)
            .with_context(|| format!("Creating output dir {:?}", output_dir))?;
        let path = output_dir.join(MANIFEST_FILENAME);
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)
            .with_context(|| format!("Writing manifest to {:?}", path))?;
        info!("Saved asset manifest to {:?}", path);
        Ok(())
    }

    /// Add a portrait entry to the manifest.
    pub fn add_portrait(&mut self, asset: &PortraitAsset) {
        self.portraits.insert(
            asset.word.clone(),
            PortraitEntry {
                relative_path: asset.relative_path.clone(),
                prompt: asset.prompt.clone(),
                element: asset.element.clone(),
                role: asset.role.clone(),
                summon_class: asset.summon_class.clone(),
                seed: asset.seed,
                lore: asset.lore.clone(),
            },
        );
    }
}
