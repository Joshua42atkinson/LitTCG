---
description: W7 — Evaluation harness with JSONL loader, per-byte F1 metrics, and latency benchmarks
---

# W7: Evaluation Harness + Latency Benchmarks

## Objective

Build an evaluation harness that loads labeled JSONL data (from W6 labeling
guide), runs the detection system against it, and computes per-dimension
metrics (precision, recall, F1) and latency benchmarks. This is how we
measure semantic correctness of the FACES detection system.

## Prerequisites

- W6 complete (labeling guide with JSONL format approved at Gate 2)
- W2 complete (scored detection with confidence values)
- W3 complete (multi-sentence detection)

## Steps

1. **Create `eval/` directory in `crates/faces-protocol/`**:
   - `eval/mod.rs` — module root
   - `eval/loader.rs` — JSONL data loader
   - `eval/metrics.rs` — metric calculations
   - `eval/benchmark.rs` — latency benchmarks
   - `eval/runner.rs` — end-to-end evaluation runner

2. **Implement JSONL loader** (`loader.rs`):
   - `LabeledExample` struct: text, expected_state (FacesState), expected_congruence, expected_confidence
   - `load_jsonl(path: &str) -> Vec<LabeledExample>` — parse JSONL file
   - Manual JSON parser (zero-dep) or simple line-by-line field extraction
   - Handle malformed lines gracefully (skip + log)
   - `load_from_str(jsonl: &str) -> Vec<LabeledExample>` — for embedded test data

3. **Implement metrics** (`metrics.rs`):
   - `ConfusionMatrix` struct per dimension (Aura, Container, Focus, Action)
   - `precision(&self) -> f32` — TP / (TP + FP)
   - `recall(&self) -> f32` — TP / (TP + FN)
   - `f1(&self) -> f32` — 2 * P * R / (P + R)
   - `accuracy(&self) -> f32` — correct / total
   - Per-dimension metrics: `aura_metrics()`, `container_metrics()`, etc.
   - `overall_metrics() -> OverallMetrics` — macro-averaged across dimensions
   - `congruence_accuracy() -> f32` — how often congruence detection matches labels

4. **Implement evaluation runner** (`runner.rs`):
   - `run_evaluation(examples: &[LabeledExample]) -> EvalReport`
   - For each example: run `detect_scored()`, compare to expected
   - Build confusion matrices per dimension
   - Compute overall metrics
   - `EvalReport` struct: per-dimension P/R/F1, overall P/R/F1, congruence accuracy, latency stats

5. **Implement latency benchmarks** (`benchmark.rs`):
   - `bench_single_detection(text: &str, iterations: u32) -> LatencyStats`
   - `bench_multi_sentence(text: &str, iterations: u32) -> LatencyStats`
   - `LatencyStats` struct: min, max, mean, median, p95, p99 (in microseconds)
   - Use `std::time::Instant` for timing (zero-dep)
   - `bench_suite() -> BenchmarkReport` — run standard benchmark suite

6. **Create sample labeled dataset**:
   - `eval/data/sample.jsonl` — 50+ labeled examples covering all dimensions
   - Include congruent, incongruent, and neutral examples
   - Include edge cases from labeling guide
   - This is the regression test dataset

7. **Implement report formatting**:
   - `EvalReport::to_summary() -> String` — human-readable text summary
   - `EvalReport::to_json() -> String` — JSON output for tooling
   - `BenchmarkReport::to_summary() -> String` — latency table
   - Include per-dimension breakdown

8. **Add eval as optional feature**:
   - `Cargo.toml`: `[features] eval = []`
   - Eval module compiled only with `--features eval`
   - Keeps default crate zero-dep and lightweight for NPU targets

## Testing

- Loader: parse valid JSONL, skip malformed lines, handle empty file
- Metrics: confusion matrix calculations correct for known inputs
- Precision/recall/F1: verify against hand-calculated values
- Runner: end-to-end on sample dataset, produces valid report
- Latency: benchmark produces non-zero timings, p95 > mean > min
- Report formatting: summary is readable, JSON is parseable
- Regression: sample dataset achieves target metrics (aura F1 > 0.6)

## Completion Criteria

- `eval/` module with loader, metrics, benchmark, runner
- JSONL loader with zero-dep JSON parsing
- Per-dimension P/R/F1 metrics
- Latency benchmarks with p95/p99
- 50+ example sample dataset
- Eval feature flag (opt-in, doesn't bloat default build)
- All tests pass (target: 230+ tests)
- Zero dependencies maintained in default build
- PROGRESS.md updated
