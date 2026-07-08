---
description: Phase 11 — FACES W8-W10: Terminal UI emulator with state display, detection simulation, and fleet orchestration. Depends on Phase 10.
---

# Phase 11: FACES W8-W10 — Emulator & Fleet

## Objective

Build the FACES terminal UI emulator (W8-W9) and multi-agent fleet with LED matrix physical simulation (W10).

## Prerequisites

- Phase 10 complete (W7 eval harness)
- FACES W1-W6 complete (283 tests passing)

## Note

These workflows already exist:
- `/w8-emulator-core` — Terminal UI emulator with state display
- `/w9-emulator-interaction` — Detection simulation and interaction
- `/w10-emulator-fleet-physical` — Multi-agent fleet with LED matrix

Refer to those workflows for detailed steps.

## Completion Criteria

- W8: Terminal UI emulator renders FACES states with multiple display modes
- W9: Emulator simulates detection on text input in real-time
- W10: Fleet orchestrator runs multiple emulators with behavior mapping
- LED matrix physical simulation works (if hardware available)
- All tests pass
- `PROGRESS.md` updated with W8-W10 results
