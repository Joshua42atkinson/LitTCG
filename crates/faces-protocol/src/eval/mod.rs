//! Evaluation harness for FACES detection — measures semantic correctness.
//!
//! This module provides:
//! - JSONL loader for labeled examples
//! - Per-dimension precision/recall/F1 metrics
//! - Latency benchmarks (p95, p99)
//! - End-to-end evaluation runner
//!
//! Only compiled with `--features eval` to keep default crate zero-dep.

pub mod loader;
pub mod metrics;
pub mod runner;
pub mod benchmark;

pub use loader::LabeledExample;
pub use metrics::{ConfusionMatrix, OverallMetrics};
pub use runner::EvalReport;
pub use benchmark::{LatencyStats, BenchmarkReport};
