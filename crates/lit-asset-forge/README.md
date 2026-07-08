# lit-asset-forge

LitTCG's AI asset pipeline. Drives LongCat-Image through ComfyUI to generate pet portraits and (future) 3D bodies, with LM Studio crafting the prompts.

## What it does

- Connects to your local **ComfyUI** on `:8188` and **LM Studio** on `:1234`.
- Uses an LLM in LM Studio to turn a LitTCG word + element + role into a detailed LongCat-Image prompt.
- Submits the prompt as a ComfyUI workflow and polls for completion.
- Copies the generated portrait into `LitTTC/assets/generated/` and updates `asset_manifest.json`.
- LitTCG loads the manifest at startup and displays the portrait in the HUD when that pet is active.

## Prerequisites

- ComfyUI running with the `comfyui_longcat_image` custom node and `LongCat-Image` model.
- LM Studio running. The forge will auto-load `qwen3.6-27b` (or whatever you pass with `--lm-model`) if it is not loaded.

## Quick demo

```bash
cd /home/joshua/LitTCG

# Check services
cargo run --manifest-path crates/lit-asset-forge/Cargo.toml -- status

# Generate one portrait (fast, 20 steps)
cargo run --manifest-path crates/lit-asset-forge/Cargo.toml -- portrait thunder \
  --element Air --role Bruiser --summon-class SemanticSlime --steps 20

# Generate the full demo set (higher quality, 50 steps)
cargo run --manifest-path crates/lit-asset-forge/Cargo.toml -- demo --steps 50
```

Generated assets land in `LitTTC/assets/generated/textures/cards/portraits/` and a manifest is written to `LitTTC/assets/generated/asset_manifest.json`.

## Overnight batch workflow

To generate a large curated set, create a JSON file with words, element, role, and summon class, then run the CLI in a loop or extend the crate with a `batch` command. Example:

```bash
# Generate all 8 demo words at production quality
cargo run --manifest-path crates/lit-asset-forge/Cargo.toml -- demo --steps 50 --width 1344 --height 768
```

## Demo playthrough for funding

1. Run the demo generator to create 8 vivid pet portraits.
2. Start LitTCG desktop:
   ```bash
   cd /home/joshua/LitTCG/LitTTC
   cargo run --features desktop
   ```
3. In the collecting phase, spell one of the demo words: `thunder`, `joy`, `fortress`, `shadow`, `ocean`, `flame`, `mountain`, or `whisper`.
4. The summoned pet appears in 3D, and the AI-generated portrait shows in the top-left HUD.

This is the ideal 60-second pitch: a child spells a word, a creature is born, and the card art is unique to that word.

## Future phases

- **Phase 2:** Image-to-3D via TripoSR (`:8007`) or Hunyuan3D-2.1 (`:7860`) to produce `glb` bodies for each archetype.
- **Phase 3:** Blender headless cleanup for decimation, rigging, and UV fixes.
- **Phase 4:** Move the crate into the Trinity workspace once the pipeline is proven.
