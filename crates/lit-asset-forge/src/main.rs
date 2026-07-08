// lit-asset-forge CLI — headless AI art pipeline for LitTCG

use anyhow::Result;
use clap::{Parser, Subcommand};
use lit_asset_forge::{
    lore::{generate_lore_deterministic, LoreContext},
    pipeline::{ForgePipeline, WordSpec},
    ForgeConfig,
};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "lit-asset-forge", version = "0.1.0", about = "LitTCG AI asset pipeline")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// ComfyUI base URL
    #[arg(long, default_value = "http://127.0.0.1:8188", global = true)]
    comfy_url: String,

    /// LM Studio base URL
    #[arg(long, default_value = "http://127.0.0.1:1234", global = true)]
    lm_url: String,

    /// LM Studio model
    #[arg(long, default_value = "g3-12b-storyteller", global = true)]
    lm_model: String,

    /// Output directory for generated assets
    #[arg(long, default_value = "../../LitTTC/assets/generated", global = true)]
    output: PathBuf,

    /// LongCat model name in ComfyUI
    #[arg(long, default_value = "LongCat-Image", global = true)]
    longcat_model: String,

    /// Inference steps
    #[arg(long, default_value_t = 50, global = true)]
    steps: u32,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a single portrait
    Portrait {
        word: String,
        #[arg(long, default_value = "Normal")]
        element: String,
        #[arg(long, default_value = "Bruiser")]
        role: String,
        #[arg(long, default_value = "SemanticSlime")]
        summon_class: String,
        /// Provide a hand-written LongCat prompt to skip the LLM entirely.
        #[arg(long)]
        prompt: Option<String>,
        #[arg(long, default_value_t = 1024)]
        width: u32,
        #[arg(long, default_value_t = 1024)]
        height: u32,
    },
    /// Generate the demo playthrough set
    Demo {
        #[arg(long, default_value_t = 1024)]
        width: u32,
        #[arg(long, default_value_t = 1024)]
        height: u32,
    },
    /// Check service health
    Status,
    /// Generate lore for a word (no image)
    Lore {
        word: String,
        #[arg(long, default_value = "Normal")]
        element: String,
        #[arg(long, default_value = "Bruiser")]
        role: String,
        #[arg(long, default_value = "SemanticSlime")]
        summon_class: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();
    let config = ForgeConfig {
        comfy_url: cli.comfy_url,
        lm_studio_url: cli.lm_url,
        lm_model: cli.lm_model,
        output_dir: cli.output,
        longcat_model: cli.longcat_model,
        longcat_steps: cli.steps,
        guidance_scale: 4.5,
    };

    let pipeline = ForgePipeline::new(config);

    match cli.command {
        Commands::Portrait {
            word,
            element,
            role,
            summon_class,
            prompt,
            width,
            height,
        } => {
            let manual_prompt = prompt.as_deref();
            if manual_prompt.is_some() {
                pipeline.wait_for_comfy().await?;
            } else {
                pipeline.wait_for_services().await?;
            }
            let lore_ctx = LoreContext {
                word: word.clone(),
                element: element.clone(),
                role: role.clone(),
                summon_class: summon_class.clone(),
                ..Default::default()
            };
            let asset = pipeline
                .generate_portrait(&word, &element, &role, &summon_class, width, height, Some(&lore_ctx), manual_prompt)
                .await?;
            info!("Lore: {}", asset.lore.to_prompt_blob());
            info!(
                "Portrait generated:\n  word: {}\n  path: {:?}\n  prompt: {}",
                asset.word, asset.portrait_path, asset.prompt
            );
        }
        Commands::Demo { width, height } => {
            pipeline.wait_for_services().await?;
            let words = demo_word_specs();
            info!("Generating demo set of {} portraits...", words.len());
            let manifest = pipeline.generate_portraits_batch(words, width, height).await?;
            info!("Demo manifest: {} portraits generated", manifest.portraits.len());
        }
        Commands::Lore {
            word,
            element,
            role,
            summon_class,
        } => {
            pipeline.wait_for_services().await?;
            let ctx = LoreContext {
                word: word.clone(),
                element: element.clone(),
                role: role.clone(),
                summon_class: summon_class.clone(),
                ..Default::default()
            };
            let lore = match pipeline.lm.generate_lore(&ctx).await {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("LLM lore failed, using deterministic fallback: {}", e);
                    generate_lore_deterministic(&ctx)
                }
            };
            println!("{}", serde_json::to_string_pretty(&lore)?);
        }
        Commands::Status => {
            let comfy_ok = pipeline.comfy.is_healthy().await;
            let lm_ok = pipeline.lm.is_healthy().await;
            println!("ComfyUI: {}", if comfy_ok { "ready" } else { "not reachable" });
            println!("LM Studio: {}", if lm_ok { "ready" } else { "not reachable" });
            if !comfy_ok || !lm_ok {
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn demo_word_specs() -> Vec<WordSpec> {
    let mut specs = vec![
        WordSpec::new("thunder", "Air", "Bruiser", "SemanticSlime"),
        WordSpec::new("joy", "Light", "Support", "SemanticSlime"),
        WordSpec::new("fortress", "Earth", "Tank", "GrammarGolem"),
        WordSpec::new("shadow", "Shadow", "Assassin", "SemanticSlime"),
        WordSpec::new("ocean", "Water", "Healer", "SemanticSlime"),
        WordSpec::new("flame", "Fire", "Striker", "SemanticSlime"),
        WordSpec::new("mountain", "Earth", "Tank", "GrammarGolem"),
        WordSpec::new("whisper", "Air", "Caster", "RhetoricRobot"),
    ];
    let mut synonyms = std::collections::HashMap::new();
    synonyms.insert("thunder".to_string(), vec!["boom".to_string(), "rumble".to_string()]);
    synonyms.insert("joy".to_string(), vec!["delight".to_string(), "glee".to_string()]);
    synonyms.insert("fortress".to_string(), vec!["stronghold".to_string(), "citadel".to_string()]);
    synonyms.insert("shadow".to_string(), vec!["shade".to_string(), "silhouette".to_string()]);
    synonyms.insert("ocean".to_string(), vec!["sea".to_string(), "wave".to_string()]);
    synonyms.insert("flame".to_string(), vec!["blaze".to_string(), "ember".to_string()]);
    synonyms.insert("mountain".to_string(), vec!["peak".to_string(), "summit".to_string()]);
    synonyms.insert("whisper".to_string(), vec!["murmur".to_string(), "rustle".to_string()]);

    for spec in &mut specs {
        if let Some(syns) = synonyms.get(&spec.word) {
            spec.synonyms = syns.clone();
        }
        spec.grade_level = Some("K-2".to_string());
    }
    specs
}
