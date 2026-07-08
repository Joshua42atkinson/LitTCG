//! Latency benchmarks for FACES detection.
//!
//! Measures single-detection and multi-sentence detection latency
//! using `std::time::Instant`. Zero-dependency.

use crate::detect::{detect_scored, detect_multi};
use std::time::Instant;

/// Latency statistics in microseconds.
#[derive(Debug, Clone, Default)]
pub struct LatencyStats {
    /// Minimum latency.
    pub min_us: u64,
    /// Maximum latency.
    pub max_us: u64,
    /// Mean (average) latency.
    pub mean_us: u64,
    /// Median latency.
    pub median_us: u64,
    /// 95th percentile latency.
    pub p95_us: u64,
    /// 99th percentile latency.
    pub p99_us: u64,
    /// Number of iterations run.
    pub iterations: u32,
}

/// Benchmark report for a suite of latency tests.
#[derive(Debug, Clone)]
pub struct BenchmarkReport {
    /// Single-detection latency stats.
    pub single_detection: LatencyStats,
    /// Multi-sentence detection latency stats.
    pub multi_sentence: LatencyStats,
}

impl BenchmarkReport {
    /// Human-readable summary.
    pub fn to_summary(&self) -> String {
        format!(
            "═══ FACES Latency Benchmarks ═══\n\
             \n\
             Single Detection (detect_scored):\n\
             │ Iterations: {}\n\
             │ Min:    {} μs\n\
             │ Mean:   {} μs\n\
             │ Median: {} μs\n\
             │ P95:    {} μs\n\
             │ P99:    {} μs\n\
             │ Max:    {} μs\n\
             \n\
             Multi-Sentence (detect_multi):\n\
             │ Iterations: {}\n\
             │ Min:    {} μs\n\
             │ Mean:   {} μs\n\
             │ Median: {} μs\n\
             │ P95:    {} μs\n\
             │ P99:    {} μs\n\
             │ Max:    {} μs",
            self.single_detection.iterations,
            self.single_detection.min_us,
            self.single_detection.mean_us,
            self.single_detection.median_us,
            self.single_detection.p95_us,
            self.single_detection.p99_us,
            self.single_detection.max_us,
            self.multi_sentence.iterations,
            self.multi_sentence.min_us,
            self.multi_sentence.mean_us,
            self.multi_sentence.median_us,
            self.multi_sentence.p95_us,
            self.multi_sentence.p99_us,
            self.multi_sentence.max_us,
        )
    }
}

/// Benchmark single-call detection latency.
pub fn bench_single_detection(text: &str, iterations: u32) -> LatencyStats {
    let mut timings = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = detect_scored(text);
        timings.push(start.elapsed().as_micros() as u64);
    }

    compute_stats(&timings, iterations)
}

/// Benchmark multi-sentence detection latency.
pub fn bench_multi_sentence(text: &str, iterations: u32) -> LatencyStats {
    let mut timings = Vec::with_capacity(iterations as usize);

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = detect_multi(text);
        timings.push(start.elapsed().as_micros() as u64);
    }

    compute_stats(&timings, iterations)
}

/// Run the standard benchmark suite.
pub fn bench_suite() -> BenchmarkReport {
    let single = bench_single_detection(
        "I'm feeling really excited and joyful about this project today!",
        1000,
    );
    let multi = bench_multi_sentence(
        "I'm happy. But also a bit anxious. Overall I think it will work out fine.",
        500,
    );
    BenchmarkReport {
        single_detection: single,
        multi_sentence: multi,
    }
}

fn compute_stats(timings: &[u64], iterations: u32) -> LatencyStats {
    if timings.is_empty() {
        return LatencyStats { iterations, ..Default::default() };
    }

    let mut sorted = timings.to_vec();
    sorted.sort_unstable();

    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let sum: u64 = sorted.iter().sum();
    let mean = sum / sorted.len() as u64;
    let median = sorted[sorted.len() / 2];
    let p95_idx = ((sorted.len() as f64) * 0.95) as usize;
    let p99_idx = ((sorted.len() as f64) * 0.99) as usize;
    let p95 = sorted[p95_idx.min(sorted.len() - 1)];
    let p99 = sorted[p99_idx.min(sorted.len() - 1)];

    LatencyStats {
        min_us: min,
        max_us: max,
        mean_us: mean,
        median_us: median,
        p95_us: p95,
        p99_us: p99,
        iterations,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bench_single_detection() {
        let stats = bench_single_detection("hello world", 100);
        assert_eq!(stats.iterations, 100);
        assert!(stats.min_us > 0 || stats.mean_us > 0 || stats.max_us > 0);
        assert!(stats.min_us <= stats.mean_us);
        assert!(stats.mean_us <= stats.max_us);
        assert!(stats.median_us <= stats.max_us);
    }

    #[test]
    fn test_bench_multi_sentence() {
        let stats = bench_multi_sentence("Hello. World. Test.", 50);
        assert_eq!(stats.iterations, 50);
        assert!(stats.p95_us >= stats.min_us);
        assert!(stats.p99_us >= stats.p95_us);
    }

    #[test]
    fn test_bench_suite() {
        let report = bench_suite();
        assert!(report.single_detection.iterations > 0);
        assert!(report.multi_sentence.iterations > 0);
        let summary = report.to_summary();
        assert!(summary.contains("FACES Latency Benchmarks"));
    }

    #[test]
    fn test_compute_stats_empty() {
        let stats = compute_stats(&[], 0);
        assert_eq!(stats.iterations, 0);
        assert_eq!(stats.min_us, 0);
    }

    #[test]
    fn test_compute_stats_single() {
        let stats = compute_stats(&[42], 1);
        assert_eq!(stats.min_us, 42);
        assert_eq!(stats.max_us, 42);
        assert_eq!(stats.mean_us, 42);
        assert_eq!(stats.median_us, 42);
    }
}
