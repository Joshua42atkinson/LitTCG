#!/usr/bin/env bash
# ============================================================
# Claude Code Headless — Overnight Autonomous Worker
# ============================================================
# Runs Claude Code in --print mode (non-interactive) with a
# task prompt. Designed for unattended execution via cron or
# manual launch before you go to sleep.
#
# Usage:
#   ./overnight-claude.sh                    # Runs next task from task.md
#   ./overnight-claude.sh "specific task"     # Runs specific prompt
#   TASK_FILE=my_tasks.txt ./overnight-claude.sh  # Custom task file
#
# The script:
#   1. Verifies LM Studio is running with a model loaded
#   2. Reads the next unchecked task from task.md
#   3. Launches Claude Code in headless mode with that task
#   4. Claude Code edits files, runs cargo test, commits
#   5. Script exits when Claude Code finishes
# ============================================================

set -euo pipefail

# --- Config ---
MODEL="${MODEL:-qwen3.6-27b}"
PORT="${PORT:-1234}"
BASE_URL="http://localhost:${PORT}"
PROJECT_DIR="/home/joshua/LitTCG/LitTCG"
TASK_FILE="${TASK_FILE:-${PROJECT_DIR}/task.md}"
MAX_TURNS="${MAX_TURNS:-50}"

# --- Claude Code env ---
export CLAUDE_CODE_ATTRIBUTION_HEADER=0
export ANTHROPIC_BASE_URL="${BASE_URL}"
export ANTHROPIC_AUTH_TOKEN="lmstudio"
export ANTHROPIC_DEFAULT_OPUS_MODEL="${MODEL}"
export ANTHROPIC_DEFAULT_SONNET_MODEL="${MODEL}"
export ANTHROPIC_DEFAULT_HAIKU_MODEL="${MODEL}"

# --- Verify LM Studio ---
if ! curl -sf "${BASE_URL}/api/v0/models" > /dev/null 2>&1; then
    echo "[$(date)] ERROR: LM Studio not running on ${PORT}" | tee -a "${PROJECT_DIR}/overnight.log"
    exit 1
fi

LOADED=$(curl -s "${BASE_URL}/api/v0/models" | python3 -c "
import sys, json
data = json.load(sys.stdin)
loaded = [m['id'] for m in data['data'] if m['state'] == 'loaded']
print(loaded[0] if loaded else 'NONE')
" 2>/dev/null || echo "NONE")

if [ "$LOADED" = "NONE" ]; then
    echo "[$(date)] ERROR: No model loaded in LM Studio" | tee -a "${PROJECT_DIR}/overnight.log"
    exit 1
fi

# --- Get next task ---
if [ $# -gt 0 ]; then
    TASK_PROMPT="$*"
else
    # Extract first unchecked task from task.md
    TASK_PROMPT=$(python3 -c "
import re
with open('${TASK_FILE}') as f:
    for line in f:
        if '- [ ]' in line:
            # Extract task text after the checkbox
            task = re.sub(r'^.*- \[ \]\s*\*?\*?(P\d+[^:]*):\s*', r'\1: ', line.strip())
            # Remove markdown formatting
            task = task.replace('**', '')
            print(task)
            break
" 2>/dev/null || echo "ERROR: Could not parse task.md")
fi

if [[ "$TASK_PROMPT" == ERROR* ]]; then
    echo "[$(date)] $TASK_PROMPT" | tee -a "${PROJECT_DIR}/overnight.log"
    exit 1
fi

echo "============================================================" | tee -a "${PROJECT_DIR}/overnight.log"
echo "[$(date)] Starting overnight Claude Code session" | tee -a "${PROJECT_DIR}/overnight.log"
echo "  Model: $LOADED" | tee -a "${PROJECT_DIR}/overnight.log"
echo "  Task:  $TASK_PROMPT" | tee -a "${PROJECT_DIR}/overnight.log"
echo "  Max turns: $MAX_TURNS" | tee -a "${PROJECT_DIR}/overnight.log"
echo "============================================================" | tee -a "${PROJECT_DIR}/overnight.log"

# --- Build the full prompt ---
FULL_PROMPT="You are working on the Communication Class Bevy game project. 
Read CLAUDE.md for project rules and AGENTS.md for workspace layout.

Your task: ${TASK_PROMPT}

Instructions:
1. Read the relevant source files before making changes
2. Make the minimal changes needed
3. Run 'cargo test' to verify (all 8 tests must pass)
4. Run 'cargo check' to verify no warnings
5. If tests pass, commit with: git add -A && git commit -m \"[task ID]: [description]\"
6. Update task.md by checking off the completed task (change [ ] to [x])
7. If you cannot complete the task, explain why and leave it unchecked

Do NOT skip cargo test. Do NOT mark a task complete unless tests pass."

# --- Launch Claude Code in headless mode ---
cd "${PROJECT_DIR}"
claude --model "${MODEL}" \
    --print \
    --dangerously-skip-permissions \
    --max-turns "${MAX_TURNS}" \
    -p "${FULL_PROMPT}" 2>&1 | tee -a "${PROJECT_DIR}/overnight.log"

EXIT_CODE=$?
echo "[$(date)] Claude Code exited with code ${EXIT_CODE}" | tee -a "${PROJECT_DIR}/overnight.log"

# --- Post-run verification ---
echo "" | tee -a "${PROJECT_DIR}/overnight.log"
echo "=== Post-run verification ===" | tee -a "${PROJECT_DIR}/overnight.log"
cargo test 2>&1 | tail -5 | tee -a "${PROJECT_DIR}/overnight.log"
echo "=== Git status ===" | tee -a "${PROJECT_DIR}/overnight.log"
git log --oneline -3 | tee -a "${PROJECT_DIR}/overnight.log"

exit $EXIT_CODE
