#!/usr/bin/env bash
# build_wasm.sh — Build the LitTCG WASM release with trunk + wasm-opt
# Usage: ./scripts/build_wasm.sh
set -euo pipefail

REPO_DIR="/home/joshua/LitTCG/LitTCG"

echo "=== Verifying LitTCG crate ==="
cd "$REPO_DIR"
cargo test

echo ""
echo "=== Building WASM release with Trunk ==="
trunk build --release

echo ""
echo "=== WASM build complete ==="
echo "Output: $REPO_DIR/dist/"

if command -v wasm-opt &> /dev/null; then
    echo ""
    echo "=== Running system wasm-opt on output ==="
    for wasm in "$REPO_DIR/dist"/*_bg.wasm; do
        if [ -f "$wasm" ]; then
            wasm-opt -O2 -o "$wasm.tmp" "$wasm"
            mv "$wasm.tmp" "$wasm"
            echo "Optimized: $(basename "$wasm")"
        fi
    done
else
    echo ""
    echo "Note: system wasm-opt not found; Trunk's bundled wasm-opt already optimized the output."
fi

echo ""
ls -lh "$REPO_DIR/dist"
echo ""
echo "=== Done ==="
echo "Serve locally with: cd $REPO_DIR && trunk serve"
