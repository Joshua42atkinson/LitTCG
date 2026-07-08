#!/usr/bin/env bash
# ============================================================
# Long-Horizon Agent Loop — Runs Claude Code on tasks until done
# ============================================================
# This is the main overnight script. It:
#   1. Reads the next unchecked task from task.md
#   2. Launches Claude Code headless with that task
#   3. Waits for completion
#   4. Verifies cargo test passes
#   5. If tests pass, loops to next task
#   6. If tests fail, logs the failure and moves on
#   7. Stops when all tasks are done or MAX_TASKS reached
#
# Usage:
#   ./agent-loop.sh                    # Run up to 10 tasks
#   MAX_TASKS=20 ./agent-loop.sh        # Run up to 20 tasks
#   MAX_TASKS=1 ./agent-loop.sh         # Run just one task (test mode)
#
# The loop is resilient:
#   - If Claude Code crashes, it logs and continues
#   - If cargo test fails, it logs and continues
#   - If LM Studio goes down, it waits and retries
#   - All activity logged to overnight.log with timestamps
# ============================================================

set -uo pipefail  # Don't exit on error — we want to continue

# --- Config ---
MODEL="${MODEL:-qwen3.6-27b}"
PORT="${PORT:-1234}"
BASE_URL="http://localhost:${PORT}"
PROJECT_DIR="/home/joshua/LitTCG/LitTCG"
TASK_FILE="${TASK_FILE:-${PROJECT_DIR}/task.md}"
MAX_TASKS="${MAX_TASKS:-10}"
MAX_TURNS="${MAX_TURNS:-50}"
MAX_RETRIES="${MAX_RETRIES:-2}"
LOG_FILE="${PROJECT_DIR}/overnight.log"

# --- Claude Code env ---
export CLAUDE_CODE_ATTRIBUTION_HEADER=0
export ANTHROPIC_BASE_URL="${BASE_URL}"
export ANTHROPIC_AUTH_TOKEN="lmstudio"
export ANTHROPIC_DEFAULT_OPUS_MODEL="${MODEL}"
export ANTHROPIC_DEFAULT_SONNET_MODEL="${MODEL}"
export ANTHROPIC_DEFAULT_HAIKU_MODEL="${MODEL}"

log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $*" | tee -a "${LOG_FILE}"
}

# --- Check LM Studio ---
check_lmstudio() {
    if ! curl -sf "${BASE_URL}/api/v0/models" > /dev/null 2>&1; then
        log "ERROR: LM Studio not responding on ${BASE_URL}"
        return 1
    fi
    
    local loaded
    loaded=$(curl -s "${BASE_URL}/api/v0/models" | python3 -c "
import sys, json
data = json.load(sys.stdin)
loaded = [m['id'] for m in data['data'] if m['state'] == 'loaded']
print(loaded[0] if loaded else 'NONE')
" 2>/dev/null || echo "NONE")
    
    if [ "$loaded" = "NONE" ]; then
        log "ERROR: No model loaded in LM Studio"
        return 1
    fi
    
    echo "$loaded"
    return 0
}

# --- Get next unchecked task from task.md ---
get_next_task() {
    python3 -c "
import re
with open('${TASK_FILE}') as f:
    for line in f:
        if '- [ ]' in line:
            task = re.sub(r'^.*- \[ \]\s*\*?\*?(P\d+[^:]*):\s*', r'\1: ', line.strip())
            task = task.replace('**', '')
            print(task)
            return
" 2>/dev/null
}

# --- Count remaining tasks ---
count_remaining() {
    python3 -c "
count = 0
with open('${TASK_FILE}') as f:
    for line in f:
        if '- [ ]' in line:
            count += 1
print(count)
" 2>/dev/null || echo "0"
}

# --- Run cargo test and check result ---
verify_tests() {
    cd "${PROJECT_DIR}"
    local output
    output=$(cargo test 2>&1)
    local exit_code=$?
    
    if [ $exit_code -eq 0 ]; then
        local passed
        passed=$(echo "$output" | grep -oP 'test result: ok. \d+ passed' | grep -oP '\d+ passed' || echo "unknown")
        log "✓ cargo test passed (${passed})"
        return 0
    else
        log "✗ cargo test FAILED (exit $exit_code)"
        echo "$output" | tail -10 | while read -r line; do log "  $line"; done
        return 1
    fi
}

# --- Main loop ---
log "============================================================"
log "Long-Horizon Agent Loop starting"
log "  Model: ${MODEL}"
log "  Max tasks: ${MAX_TASKS}"
log "  Max turns per task: ${MAX_TURNS}"
log "============================================================"

# --- Initial checks ---
MODEL_LOADED=$(check_lmstudio)
if [ $? -ne 0 ]; then
    log "Cannot start — LM Studio not ready"
    exit 1
fi
log "LM Studio ready, model: ${MODEL_LOADED}"

TASKS_DONE=0
TASKS_FAILED=0

for i in $(seq 1 ${MAX_TASKS}); do
    REMAINING=$(count_remaining)
    if [ "$REMAINING" = "0" ]; then
        log "All tasks complete! Nothing left to do."
        break
    fi
    
    log ""
    log "============================================================"
    log "Task ${i}/${MAX_TASKS} — ${REMAINING} tasks remaining"
    log "============================================================"
    
    # Get next task
    TASK=$(get_next_task)
    if [ -z "$TASK" ] || [[ "$TASK" == ERROR* ]]; then
        log "Could not parse next task from task.md"
        break
    fi
    log "Task: ${TASK}"
    
    # Check LM Studio is still alive
    if ! check_lmstudio > /dev/null 2>&1; then
        log "LM Studio went down — waiting 30s and retrying..."
        sleep 30
        if ! check_lmstudio > /dev/null 2>&1; then
            log "LM Studio still down — stopping loop"
            break
        fi
        log "LM Studio recovered"
    fi
    
    # Build prompt
    FULL_PROMPT="You are working on the Communication Class Bevy game project.
Read CLAUDE.md for project rules and AGENTS.md for workspace layout.

Your task: ${TASK}

Instructions:
1. Read the relevant source files before making changes
2. Make the minimal changes needed — do not refactor unrelated code
3. Run 'cargo test' to verify (all 8 tests must pass)
4. Run 'cargo check' to verify no new warnings
5. If tests pass, commit with: git add -A && git commit -m \"${TASK}\"
6. Update task.md by checking off this task (change [ ] to [x])
7. If you cannot complete the task, explain why in a git commit and leave it unchecked

Do NOT skip cargo test. Do NOT mark a task complete unless tests pass.
Work in: ${PROJECT_DIR}"

    # Run Claude Code headless
    cd "${PROJECT_DIR}"
    RETRY=0
    SUCCESS=false
    
    while [ $RETRY -lt $MAX_RETRIES ]; do
        RETRY=$((RETRY + 1))
        log "Attempt ${RETRY}/${MAX_RETRIES}..."
        
        timeout 1800 claude --model "${MODEL}" \
            --print \
            --dangerously-skip-permissions \
            --max-turns "${MAX_TURNS}" \
            -p "${FULL_PROMPT}" 2>&1 | tee -a "${LOG_FILE}"
        
        EXIT_CODE=$?
        
        if [ $EXIT_CODE -eq 0 ]; then
            log "Claude Code exited successfully"
            SUCCESS=true
            break
        elif [ $EXIT_CODE -eq 124 ]; then
            log "Claude Code timed out (30 min limit)"
        else
            log "Claude Code exited with code ${EXIT_CODE}"
        fi
        
        if [ $RETRY -lt $MAX_RETRIES ]; then
            log "Retrying in 10s..."
            sleep 10
        fi
    done
    
    # Verify tests pass regardless of Claude's exit code
    if verify_tests; then
        TASKS_DONE=$((TASKS_DONE + 1))
        log "Task verified ✓"
    else
        TASKS_FAILED=$((TASKS_FAILED + 1))
        log "Task verification failed ✗ — tests not passing after Claude's work"
        log "Attempting git stash to revert broken changes..."
        cd "${PROJECT_DIR}"
        git stash 2>/dev/null && log "Reverted to last known good state" || log "No changes to stash"
    fi
done

# --- Summary ---
log ""
log "============================================================"
log "Agent Loop Complete"
log "  Tasks completed: ${TASKS_DONE}"
log "  Tasks failed:    ${TASKS_FAILED}"
log "  Remaining:       $(count_remaining)"
log "============================================================"

# Final state
log ""
log "=== Final git log ==="
cd "${PROJECT_DIR}"
git log --oneline -10 2>&1 | while read -r line; do log "  $line"; done
log ""
log "=== Final cargo test ==="
verify_tests

exit 0
