---
description: Phase 10 — FACES W7: Evaluation harness with JSONL loader, per-byte F1 metrics, and latency benchmarks. Independent of Core phases.
---

# Phase 10: FACES W7 — Evaluation Harness

## Objective

Build an evaluation harness for the FACES protocol that loads ground-truth labeled data, runs detection, and computes per-byte F1 metrics plus latency benchmarks.

## Prerequisites

- FACES W1-W6 complete (283 tests passing)
- `faces-protocol` crate builds independently
- No dependency on Core phases — this can run in parallel

## Note

This workflow already exists as `/w7-eval-harness`. Refer to that workflow for detailed steps.
The key deliverables are:

1. JSONL loader for labeled training/eval data
2. Per-byte F1 metric computation (not per-sentence — per-byte to handle partial matches)
3. Latency benchmark suite (detection time per input)
4. Confusion matrix output
5. Baseline comparison (keyword detection vs scored detection)

## Steps

See `/w7-eval-harness` workflow for full details.

## Completion Criteria

- Eval harness runs against labeled data
- F1 metrics computed per byte (not per sentence)
- Latency benchmarks reported
- Results saved to `faces-protocol/eval/results/`
- `PROGRESS.md` updated with W7 results
