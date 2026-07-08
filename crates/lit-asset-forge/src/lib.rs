// lit-asset-forge — LitTCG AI asset pipeline
//
// Drives LongCat-Image through ComfyUI to generate pet portraits and 3D assets.
// Designed to run headless overnight, and to be portable to Trinity later.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod comfy;
pub mod lore;
pub mod manifest;
pub mod pipeline;
pub mod prompts;

/// Configuration for the forge pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    /// ComfyUI base URL, e.g. http://127.0.0.1:8188
    pub comfy_url: String,
    /// LM Studio base URL, e.g. http://127.0.0.1:1234
    pub lm_studio_url: String,
    /// LM Studio model to use for prompt crafting.
    pub lm_model: String,
    /// Output directory for generated assets.
    pub output_dir: PathBuf,
    /// LongCat model directory name inside ComfyUI models/diffusion_models.
    pub longcat_model: String,
    /// Default inference steps for LongCat.
    pub longcat_steps: u32,
    /// Default guidance scale.
    pub guidance_scale: f32,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            comfy_url: "http://127.0.0.1:8188".to_string(),
            lm_studio_url: "http://127.0.0.1:1234".to_string(),
            lm_model: "g3-12b-storyteller".to_string(),
            output_dir: PathBuf::from("assets/generated"),
            longcat_model: "LongCat-Image".to_string(),
            longcat_steps: 50,
            guidance_scale: 4.5,
        }
    }
}

/// A generated portrait asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortraitAsset {
    pub word: String,
    pub element: String,
    pub role: String,
    pub summon_class: String,
    pub prompt: String,
    pub lore: lore::PetLore,
    pub portrait_path: PathBuf,
    pub relative_path: PathBuf,
    pub seed: u64,
    pub generation_time_ms: u64,
}

/// A 3D body asset (placeholder for Phase 2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body3DAsset {
    pub word: String,
    pub archetype: String,
    pub mesh_path: PathBuf,
    pub relative_path: PathBuf,
}
