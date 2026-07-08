#!/usr/bin/env bash
# generate-demo-assets.sh — Generate the LitTCG demo portrait set
# Run this overnight or before a demo to produce AI-generated card art.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FORGE_CRATE="$ROOT/crates/lit-asset-forge"
OUTPUT="$ROOT/LitTTC/assets/generated"

echo "=== LitTCG Demo Asset Generator ==="
echo "Checking ComfyUI + LM Studio health..."

cargo run --manifest-path "$FORGE_CRATE/Cargo.toml" -- status

echo ""
echo "Generating 8 demo pet portraits at 50 steps..."
echo "This will take roughly 20–40 minutes depending on GPU speed."

cargo run --manifest-path "$FORGE_CRATE/Cargo.toml" -- demo \
  --steps 50 \
  --width 1024 \
  --height 1024 \
  --output "$OUTPUT"

echo ""
echo "Demo assets generated in: $OUTPUT"
echo "Manifest: $OUTPUT/asset_manifest.json"
echo ""
echo "Next step: run the game with:"
echo "  cd $ROOT/LitTTC && cargo run --features desktop"
