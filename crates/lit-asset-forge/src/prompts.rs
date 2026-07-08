// prompts.rs — LM Studio driven prompt crafting for LitTCG pet portraits

use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use tracing::info;

use crate::lore::{LoreContext, PetLore};

/// Client for LM Studio's OpenAI-compatible chat completions endpoint.
#[derive(Debug, Clone)]
pub struct LmStudioClient {
    client: Client,
    base_url: String,
    model: String,
}

impl LmStudioClient {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client build"),
            base_url: base_url.into(),
            model: model.into(),
        }
    }

    /// Health check via the /v1/models endpoint.
    pub async fn is_healthy(&self) -> bool {
        self.client
            .get(format!("{}/v1/models", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Check if the configured model is currently loaded.
    pub async fn is_model_loaded(&self) -> bool {
        match self.client
            .get(format!("{}/v1/models", self.base_url))
            .send()
            .await
        {
            Ok(res) => match res.json::<ModelList>().await {
                Ok(list) => list.data.iter().any(|m| m.id == self.model),
                Err(_) => false,
            },
            Err(_) => false,
        }
    }

    /// Load the configured model into LM Studio.
    pub async fn load_model(&self) -> Result<()> {
        info!("Loading model '{}' in LM Studio", self.model);
        let res = self
            .client
            .post(format!("{}/api/v1/models/load", self.base_url))
            .json(&serde_json::json!({ "model": self.model }))
            .timeout(Duration::from_secs(300))
            .send()
            .await
            .context("Failed to send LM Studio load request")?;

        if !res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("LM Studio failed to load model: {}", body));
        }

        // Wait until the model appears in the loaded list.
        for attempt in 1..=60 {
            if self.is_model_loaded().await {
                info!("Model '{}' loaded after {}s", self.model, attempt * 2);
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        Err(anyhow::anyhow!("Model '{}' did not load within 2 minutes", self.model))
    }

    /// Generate lore for a word using the Lorekeeper prompt.
    pub async fn generate_lore(&self, ctx: &LoreContext) -> Result<PetLore> {
        crate::lore::generate_lore(&self.client, &self.base_url, &self.model, ctx).await
    }

    /// Craft a LongCat-Image prompt for a pet portrait.
    /// Optional `lore` enriches the prompt with worldbuilding context.
    pub async fn craft_pet_portrait_prompt(
        &self,
        word: &str,
        element: &str,
        role: &str,
        summon_class: &str,
        lore: Option<&str>,
    ) -> Result<String> {
        let system = build_pet_portrait_system_prompt();
        let mut user = format!(
            "Create a LongCat-Image prompt for a pet named '{}'.
Element: {}. Role: {}. Summon class: {}.",
            word, element, role, summon_class
        );
        if let Some(l) = lore {
            user.push_str(&format!("\n\nWorldbuilding / lore: {}\n", l));
        }
        user.push_str("\nReturn only the final prompt paragraph, no commentary.");

        let payload = json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user}
            ],
            "max_tokens": 400,
            "temperature": 0.7
        });

        info!("Crafting portrait prompt for '{}' via LM Studio", word);

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&payload)
            .send()
            .await
            .context("Failed to contact LM Studio")?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("LM Studio request failed: {}", body));
        }

        let result: ChatCompletionResponse = response
            .json()
            .await
            .context("Failed to parse LM Studio response")?;

        let content = result
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .map(|s| s.trim().to_string())
            .ok_or_else(|| anyhow::anyhow!("LM Studio returned empty completion"))?;

        Ok(content)
    }
}

fn build_pet_portrait_system_prompt() -> String {
    r#"You are ComfyBot, an expert prompt writer for the LongCat-Image diffusion model.
Your job is to create highly detailed, objective, visually concrete prompts for a children's educational trading-card game pet portrait.

Follow this EXACT structure in a single paragraph:
1. Image Type (e.g. "A vibrant trading-card portrait illustration").
2. Clear Central Subject — the creature, its body type, color, expression, and stance.
3. Detailed Imagery — textures, markings, glowing parts, eyes, mouth, claws, wings, etc.
4. Environment Description — a small magical backdrop that hints at the element without cluttering the subject.
5. Mood/Atmosphere — emotional tone matching the creature's role and element.
6. Artistic Style — polished game art, slightly stylized, clean outlines, rich colors, suitable for a collector card.
7. Style Execution — centered composition, soft rim lighting, transparent background or subtle gradient, high detail.

CRITICAL RULES:
- Use objective descriptions only. Avoid words like "beautiful", "amazing", "stunning".
- Describe the creature's physical appearance in concrete terms (scales, fur, metal plates, slime surface, crystal growths).
- Include the element as a color/lighting motif (fire = ember oranges, water = aqua blues, earth = moss browns, air = sky whites, shadow = violet blacks, light = gold whites).
- Keep the subject centered and clearly readable at small card size.
- Do not include text, runes, letters, or symbols the creature is spelling.
- Return ONLY the prompt paragraph. No extra commentary, no markdown, no quotes."#
        .to_string()
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModelList {
    data: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
struct ModelInfo {
    id: String,
}

/// Lower-level helper: escape a word for safe filename use.
pub fn safe_filename(word: &str) -> String {
    word.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "_")
        .replace("__", "_")
        .trim_matches('_')
        .to_string()
}
