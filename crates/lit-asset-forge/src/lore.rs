// lore.rs — Lore generation for LitTCG pets and worldbuilding
//
// Each pet gets a short lore entry that connects the word to the world of
// Syllable Springs, its NPC guardians, its etymology, and its elemental nature.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Lore entry for a single pet / word.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl PetLore {
    /// Combine the lore into a single paragraph suitable for image prompts.
    pub fn to_prompt_blob(&self) -> String {
        format!(
            "{} ({}). {}. Habitat: {}. Behavior: {}. Fun fact: {}. {}. Guardian: {}.",
            self.title,
            self.word,
            self.description,
            self.habitat,
            self.behavior,
            self.fun_fact,
            self.etymology_hook,
            self.npc_guardian
        )
    }
}

/// Context used to generate or derive lore for a word.
#[derive(Debug, Clone, Default)]
pub struct LoreContext {
    pub word: String,
    pub element: String,
    pub role: String,
    pub summon_class: String,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
    pub etymology_root: Option<String>,
    pub grade_level: Option<String>,
    pub district: Option<String>,
    pub npc_guardian: Option<String>,
}

/// Generate lore using an LLM in LM Studio.
pub async fn generate_lore(
    client: &Client,
    base_url: &str,
    model: &str,
    ctx: &LoreContext,
) -> Result<PetLore> {
    let system = "You are the Lorekeeper of Syllable Springs, a whimsical educational RPG where words become pets. Write a short, kid-friendly lore entry for a word-creature. Be vivid and fun, but keep each field to one sentence.";

    let user = format!(
        "Word: {word}\nElement: {element}\nRole: {role}\nClass: {class}\nSynonyms: {syns}\nAntonyms: {ants}\nEtymology root: {ety}\nGrade level: {grade}\nDistrict: {district}\nGuardian NPC: {npc}\n\nReturn JSON only with these fields: title, description, habitat, behavior, fun_fact, etymology_hook, npc_guardian.",
        word = ctx.word,
        element = ctx.element,
        role = ctx.role,
        class = ctx.summon_class,
        syns = ctx.synonyms.join(", "),
        ants = ctx.antonyms.join(", "),
        ety = ctx.etymology_root.as_deref().unwrap_or("unknown"),
        grade = ctx.grade_level.as_deref().unwrap_or("all ages"),
        district = ctx.district.as_deref().unwrap_or("Syllable Springs"),
        npc = ctx.npc_guardian.as_deref().unwrap_or("a friendly NPC"),
    );

    let body = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user},
        ],
        "temperature": 0.8,
        "max_tokens": 400,
    });

    let resp = client
        .post(format!("{}/v1/chat/completions", base_url.trim_end_matches('/')))
        .json(&body)
        .send()
        .await
        .context("failed to contact LM Studio for lore generation")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        anyhow::bail!("LM Studio lore generation failed: {} - {}", status, text);
    }

    let json: serde_json::Value = resp.json().await.context("failed to parse LM Studio lore response")?;
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .context("LM Studio lore response had no content")?
        .to_string();

    let mut lore: PetLore = serde_json::from_str(&content)
        .context("LM Studio returned invalid lore JSON")?;
    lore.word = ctx.word.clone();

    info!("Generated lore for '{}' (guardian: {})", lore.word, lore.npc_guardian);
    Ok(lore)
}

/// Deterministic lore generator that uses the game databases directly.
/// This is the fallback when the LLM is unavailable or for batch testing.
pub fn generate_lore_deterministic(ctx: &LoreContext) -> PetLore {
    let district = ctx.district.clone().unwrap_or_else(|| district_for_element(&ctx.element));
    let npc = ctx.npc_guardian.clone().unwrap_or_else(|| npc_for_element(&ctx.element));
    let habitat = habitat_for_element(&ctx.element, &district);
    let behavior = behavior_for_class(&ctx.summon_class, &ctx.word);

    let etymology_hook = if let Some(root) = &ctx.etymology_root {
        format!("Scholars say its ancient root, {}, still hums inside its syllables.", root)
    } else {
        format!("The letters of '{}' wiggle when no one is looking.", ctx.word)
    };

    let title = if ctx.word.len() <= 4 {
        format!("The Little {}", capitalize(&ctx.word))
    } else {
        format!("{} of {}", capitalize(&ctx.summon_class), capitalize(&ctx.word))
    };

    PetLore {
        word: ctx.word.clone(),
        title,
        description: format!(
            "A {} {} born from the word '{}', carrying the {} element of {}",
            ctx.role.to_lowercase(),
            ctx.summon_class.to_lowercase(),
            ctx.word,
            ctx.element.to_lowercase(),
            district
        ),
        habitat,
        behavior,
        fun_fact: format!(
            "It gets excited whenever someone says '{}' near it.",
            ctx.synonyms.first().cloned().unwrap_or_else(|| ctx.word.clone())
        ),
        etymology_hook,
        npc_guardian: npc,
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn district_for_element(element: &str) -> String {
    match element.to_lowercase().as_str() {
        "fire" | "light" => "Action Alley",
        "water" | "ice" => "Heartwood Grove",
        "earth" => "Brainy Borough",
        "air" => "Whisper Winds",
        "shadow" => "The Dim Archives",
        _ => "Syllable Springs",
    }
    .to_string()
}

fn npc_for_element(element: &str) -> String {
    match element.to_lowercase().as_str() {
        "fire" => "Zafir the Magician",
        "water" => "Vlad the Lover",
        "earth" => "Yorick the Everyman",
        "air" => "Nyx the Rebel",
        "light" => "Kael the Hero",
        "shadow" => "Ozymandias the Sage",
        "ice" => "Pygmalion the Creator",
        _ => "Barnaby the Innocent",
    }
    .to_string()
}

fn habitat_for_element(element: &str, district: &str) -> String {
    match element.to_lowercase().as_str() {
        "fire" => "Warm stone ledges near bubbling ink-wells.",
        "water" => "Shallow pools fed by the Synthesizer's runoff.",
        "earth" => "Tunnels beneath the Grammar Golem foundry.",
        "air" => "High shelves where loose vowels drift like dust.",
        "light" => "Sunlit balconies above the Slime Synthesizer.",
        "shadow" => "Quiet corners where forgotten definitions sleep.",
        "ice" => "Frozen syllable stacks in the Cryo-Library.",
        _ => "A cozy corner of the district.",
    }
    .to_string()
    + &format!(" ({})", district)
}

fn behavior_for_class(class: &str, word: &str) -> String {
    match class.to_lowercase().as_str() {
        "semanticslime" => format!("Oozes around syllables and tries to absorb the meaning of '{}'.", word),
        "grammargolem" => format!("Stomps around correcting the grammar of anyone who mispronounces '{}'.", word),
        "rhetoricrobot" => format!("Recites speeches about '{}' until its battery runs low.", word),
        _ => format!("Quietly watches over the word '{}'.", word),
    }
}

/// Pick a guardian NPC and district for a word based on its element and the lore database.
pub fn pick_guardian(
    element: &str,
    _npc_db: &HashMap<String, Vec<String>>,
) -> (String, String) {
    let npc = npc_for_element(element);
    let district = district_for_element(element);
    (npc, district)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_lore_includes_word_and_element() {
        let ctx = LoreContext {
            word: "thunder".to_string(),
            element: "Air".to_string(),
            role: "Bruiser".to_string(),
            summon_class: "SemanticSlime".to_string(),
            synonyms: vec!["boom".to_string(), "rumble".to_string()],
            etymology_root: Some("Thor".to_string()),
            ..Default::default()
        };
        let lore = generate_lore_deterministic(&ctx);
        assert_eq!(lore.word, "thunder");
        assert!(lore.description.contains("air"));
        assert!(lore.description.contains("semanticslime"));
        assert!(lore.fun_fact.contains("boom"));
        assert!(lore.etymology_hook.contains("Thor"));
        assert!(lore.npc_guardian.contains("Nyx"));
    }

    #[test]
    fn lore_to_prompt_blob_is_non_empty() {
        let ctx = LoreContext {
            word: "joy".to_string(),
            element: "Light".to_string(),
            role: "Support".to_string(),
            summon_class: "SemanticSlime".to_string(),
            synonyms: vec!["delight".to_string()],
            ..Default::default()
        };
        let lore = generate_lore_deterministic(&ctx);
        let blob = lore.to_prompt_blob();
        assert!(!blob.is_empty());
        assert!(blob.contains("joy"));
        assert!(blob.contains("light"));
    }

    #[test]
    fn element_mapping_selects_known_npc() {
        assert!(npc_for_element("Fire").contains("Zafir"));
        assert!(npc_for_element("Water").contains("Vlad"));
        assert!(npc_for_element("Earth").contains("Yorick"));
        assert!(npc_for_element("Air").contains("Nyx"));
        assert!(npc_for_element("Light").contains("Kael"));
        assert!(npc_for_element("Shadow").contains("Ozymandias"));
    }
}
