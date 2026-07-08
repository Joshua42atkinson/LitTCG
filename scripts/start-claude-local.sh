#!/usr/bin/env bash
# ============================================================
# Claude Code + LM Studio — Local Free Coding Agent
# ============================================================
# Usage:
#   ./start-claude-local.sh              # Default: Hermes-4-70B
#   MODEL=qwen3.6-27b ./start-claude-local.sh  # Fast model
#   MODEL=hermes-4-70b ./start-claude-local.sh  # Best quality
#
# Requirements:
#   - LM Studio running with model loaded (GUI or `lms server start`)
#   - Context window set to at least 32K in LM Studio
#   - Claude Code installed: npm install -g @anthropic-ai/claude-code
# ============================================================

set -euo pipefail

# --- Config ---
MODEL="${MODEL:-qwen3.6-27b}"
PORT="${PORT:-1234}"
BASE_URL="http://localhost:${PORT}"

# --- Attribution header fix (critical for KV cache performance) ---
export CLAUDE_CODE_ATTRIBUTION_HEADER=0

# --- Point Claude Code at LM Studio's Anthropic-compatible endpoint ---
export ANTHROPIC_BASE_URL="${BASE_URL}"
export ANTHROPIC_AUTH_TOKEN="lmstudio"

# --- Set all model tiers to the same local model ---
export ANTHROPIC_DEFAULT_OPUS_MODEL="${MODEL}"
export ANTHROPIC_DEFAULT_SONNET_MODEL="${MODEL}"
export ANTHROPIC_DEFAULT_HAIKU_MODEL="${MODEL}"

# --- Verify LM Studio is running ---
if ! curl -sf "${BASE_URL}/api/v0/models" > /dev/null 2>&1; then
    echo "ERROR: LM Studio is not running on port ${PORT}"
    echo "Start it with: lms server start --port ${PORT}"
    echo "Or open the LM Studio GUI and start the server"
    exit 1
fi

# --- Check that a model is loaded ---
LOADED=$(curl -s "${BASE_URL}/api/v0/models" | python3 -c "
import sys, json
data = json.load(sys.stdin)
loaded = [m['id'] for m in data['data'] if m['state'] == 'loaded']
print(loaded[0] if loaded else 'NONE')
" 2>/dev/null || echo "NONE")

if [ "$LOADED" = "NONE" ]; then
    echo "WARNING: No model is loaded in LM Studio"
    echo "Load a model in the LM Studio GUI first"
    echo "Recommended: Qwen3.6-27B (Q4_K_M, ~20GB RAM, coding-tuned)"
    echo "Alternative: Hermes-4-70B (Q4_K_M, ~47GB RAM, more capable)"
    exit 1
fi

echo "============================================================"
echo "  Claude Code → Local LLM"
echo "  Backend:  LM Studio (${BASE_URL})"
echo "  Model:    ${LOADED}"
echo "  Cost:     \$0.00 (fully local)"
echo "============================================================"
echo ""
echo "Tips:"
echo "  - Set context to 32K+ in LM Studio for best results"
echo "  - Use --dangerously-skip-permissions for unattended work"
echo "  - Use --print for headless/non-interactive mode"
echo "  - Use -p < prompt.txt for scripted tasks"
echo ""

# --- Launch Claude Code ---
exec claude --model "${MODEL}" "$@"
