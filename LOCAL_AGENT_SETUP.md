# Local Agent System — Setup Guide

> How to use your AMD Strix Halo (122GB unified memory) for free local AI coding.
> Claude Code + LM Studio = $0.00 per token, fully offline capable.

---

## Architecture

```
You (planning, review, decisions)
    │
    ├── Windsurf (Cascade) — cloud model, high IQ
    │   Use for: architecture decisions, complex design, evaluation
    │   Cost: Windsurf subscription (already have)
    │
    ├── Claude Code + LM Studio — local model, free
    │   Use for: code execution, refactoring, tests, overnight work
    │   Cost: $0.00 (local compute)
    │
    └── Hermes Agent (optional) — local model, autonomous kanban
        Use for: long unattended runs with task board
        Cost: $0.00 (local compute)
```

## Hardware

| Component | Value |
|-----------|-------|
| APU | AMD Strix Halo (Radeon 8050S/8060S) |
| Memory | 122GB unified (CPU+GPU shared) |
| GPU | ROCm, no dedicated VRAM (uses system RAM) |
| NPU | Present (unused for LLM) |

## Models (Already Downloaded)

| Model | RAM | Quality | Speed | Use Case |
|-------|-----|---------|-------|----------|
| **Qwen3.6-27B (Q4_K_M)** | ~20GB | High | Fast | **Best for Claude Code** — coding-tuned, fast, low RAM |
| Hermes-4-70B (Q4_K_M) | ~47GB | Higher | Medium | Complex tasks, more RAM headroom needed |
| Leanstral-1.5-119B | ~75GB | Highest | Slow | ❌ Too close to 122GB ceiling — do NOT use with agents |

## Setup (Already Done)

1. ✅ `~/.claude/settings.json` — points Claude Code at LM Studio
2. ✅ `scripts/start-claude-local.sh` — interactive launcher
3. ✅ `scripts/overnight-claude.sh` — headless single-task worker
4. ✅ `scripts/agent-loop.sh` — long-horizon loop (chains multiple tasks)
5. ✅ `CLAUDE.md` — project rules for Claude Code
6. ✅ `LitTTC/task.md` — shared task tracker

## How to Use

### Interactive (sit at terminal, work with Claude Code)

```bash
# 1. Open LM Studio GUI, load Qwen3.6-27B
# 2. Set context to 32K (or 64K if RAM allows)
# 3. Start the server (Developer tab → Start Server)
# 4. In terminal:
cd "/home/joshua/LitTTC"
./scripts/start-claude-local.sh
```

### Overnight (headless, single task)

```bash
# 1. Open LM Studio, load Qwen3.6-27B, set context to 32K, start server
# 2. In terminal:
cd "/home/joshua/LitTTC"
./scripts/overnight-claude.sh
# Reads next task from task.md, works on it, commits, exits
```

### Long-Horizon Loop (headless, multiple tasks — THE overnight script)

```bash
# 1. Open LM Studio, load Qwen3.6-27B, set context to 32K, start server
# 2. In terminal:
cd "/home/joshua/LitTTC"
./scripts/agent-loop.sh                  # Runs up to 10 tasks
MAX_TASKS=20 ./scripts/agent-loop.sh     # Runs up to 20 tasks
MAX_TASKS=1 ./scripts/agent-loop.sh      # Test mode — just 1 task

# The loop:
#   - Reads next unchecked task from task.md
#   - Launches Claude Code headless with that task
#   - Waits for completion (30 min timeout per task)
#   - Runs cargo test to verify
#   - If tests pass, moves to next task
#   - If tests fail, git stashes broken changes and moves on
#   - Stops when all tasks done or MAX_TASKS reached
#   - All activity logged to LitTTC/overnight.log (workspace folder unchanged)
```

### With Windsurf (me, Cascade)

1. I plan the work, update `task.md` and `MASTER_TASK_LIST.md`
2. You run Claude Code locally to execute the tasks
3. I review the git log and test results
4. We iterate — I plan next phase, Claude Code executes

### Switching Back to Cloud Claude Code

```bash
# Remove the local settings to use Anthropic's API again:
rm ~/.claude/settings.json
# Or just unset for current session:
unset ANTHROPIC_BASE_URL ANTHROPIC_AUTH_TOKEN
```

## Critical Settings

### Context Window
Set to **32K minimum** in LM Studio. Claude Code's system prompt is large.
64K is better if RAM allows. 16K will fail — too small for tool schemas + code.

### Attribution Header
Already disabled in settings.json. Without this, KV cache is invalidated
on every request, making inference ~90% slower.

### Model Selection
- **Qwen3.6-27B**: Default — fast, coding-tuned, only ~20GB RAM, leaves 100GB headroom
- **Hermes-4-70B**: For complex tasks — more capable reasoning, ~47GB RAM
- **Do NOT use Leanstral-119B with Claude Code** — too close to 122GB ceiling

## Limitations of Local vs Cloud

| Feature | Local (Hermes-4-70B) | Cloud (Claude Sonnet/Opus) |
|---------|---------------------|---------------------------|
| Cost | $0.00 | API costs |
| Privacy | Fully local | Data sent to Anthropic |
| Speed | 5-15 tok/s | 50-100+ tok/s |
| Code quality | Good | Excellent |
| Complex reasoning | Sometimes struggles | Handles anything |
| Tool use | Works, occasional format issues | Rock solid |
| Context | 32K-64K practical | 200K+ |
| Offline | ✅ Yes | ❌ No |

**Strategy**: Use local for mechanical work (renames, tests, boilerplate).
Use cloud (Windsurf/me) for architecture, design, complex debugging.

## Troubleshooting

### "ConnectionRefused" error
LM Studio server isn't running. Open LM Studio GUI → Developer tab → Start Server.

### Tool calls malformed / `<unused24>` tokens
Model isn't formatting tool calls correctly. Hermes-4-70B handles this well.
If issues arise, try Qwen3.6-27B as alternative.

### Claude Code stops mid-edit
Context exhaustion. Reduce context target or use a faster quant.
Check that context is set to 32K+ in LM Studio.

### Model not found / 404
The `--model` flag must match the model ID in LM Studio exactly.
Check `curl http://localhost:1234/api/v0/models` for correct ID.

### OOM / system crash
Use Hermes-4-70B (47GB) not Leanstral-119B (75GB).
Never run multiple inference workers simultaneously on unified memory.
