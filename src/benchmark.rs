use crate::dns::{DnsBenchmark, DnsClient, DnsPacket};
use anyhow::Result;

use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::time::interval;

#[derive(Debug)]
pub struct BenchmarkStats {
    pub total_requests: Arc<AtomicU64>,
    pub successful_responses: Arc<AtomicU64>,
    pub failed_responses: Arc<AtomicU64>,
    pub start_time: Instant,
}

impl BenchmarkStats {
    pub fn new() -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            successful_responses: Arc::new(AtomicU64::new(0)),
            failed_responses: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }

    pub fn increment_total(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_success(&self) {
        self.successful_responses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_failure(&self) {
        self.failed_responses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_summary(&self) -> BenchmarkSummary {
        let elapsed = self.start_time.elapsed();
        let total = self.total_requests.load(Ordering::Relaxed);
        let success = self.successful_responses.load(Ordering::Relaxed);
        let failure = self.failed_responses.load(Ordering::Relaxed);

        let requests_per_sec = if elapsed.as_millis() > 0 {
            total as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        let success_rate = if total > 0 {
            (success as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        BenchmarkSummary {
            elapsed,
            total_requests: total,
            successful_responses: success,
            failed_responses: failure,
            requests_per_second: requests_per_sec,
            success_rate,
        }
    }
}

#[derive(Debug)]
pub struct BenchmarkSummary {
    pub elapsed: Duration,
    pub total_requests: u64,
    pub successful_responses: u64,
    pub failed_responses: u64,
    pub requests_per_second: f64,
    pub success_rate: f64,
}

impl BenchmarkSummary {
    pub fn print(&self) {
        println!("\n=== Benchmark Results ===");
        println!("Duration: {:.2}s", self.elapsed.as_secs_f64());
        println!("Total requests: {}", self.total_requests);
        println!("Successful responses: {}", self.successful_responses);
        println!("Failed responses: {}", self.failed_responses);
        println!("Requests per second: {:.2}", self.requests_per_second);
        println!("Success rate: {:.2}%", self.success_rate);
    }
}

impl DnsBenchmark {
    pub async fn run(&self, count: u64) -> Result<BenchmarkSummary> {
        let stats = BenchmarkStats::new();
        let stats_clone = stats.clone();

        let rate = self.rate;
        let domain = self.domain.clone();
        let client = self.client.clone();
        let mode = self.mode.clone();

        let pb = ProgressBar::new(count);
        pb.enable_steady_tick(Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({pos}/{len}) {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        let _start_time = Instant::now();

        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(1000 / rate as u64));
            let mut last_print = Instant::now();
            let mut sent_count = 0;

            while sent_count < count {
                interval.tick().await;

                let batch_size = rate.min((count - sent_count) as u32);

                let tasks: Vec<_> = (0..batch_size)
                    .map(|_| {
                        let client = client.clone();
                        let domain = domain.clone();
                        let stats = stats_clone.clone();
                        let mode = mode.clone();

                        tokio::spawn(async move {
                            let packet = DnsPacket::new(domain, &mode);
                            stats.increment_total();

                            match client.send_query(&packet).await {
                                Ok(_) => {
                                    stats.increment_success();
                                }
                                Err(_) => {
                                    stats.increment_failure();
                                }
                            }
                        })
                    })
                    .collect();

                for task in tasks {
                    let _ = task.await;
                }

                sent_count += batch_size as u64;

                let progress = stats.total_requests.load(Ordering::Relaxed);
                pb.set_position(progress);

                if last_print.elapsed() >= Duration::from_millis(500) {
                    let summary = stats.get_summary();
                    pb.set_message(format!(
                        "QPS: {:.1}, Success: {:.1}%",
                        summary.requests_per_second, summary.success_rate
                    ));
                    last_print = Instant::now();
                }
            }

            pb.finish_with_message("Benchmark completed");
            stats.get_summary()
        });

        match handle.await {
            Ok(summary) => {
                summary.print();
                Ok(summary)
            }
            Err(e) => Err(anyhow::anyhow!("Benchmark failed: {}", e)),
        }
    }
}

impl Clone for DnsBenchmark {
    fn clone(&self) -> Self {
        Self {
            client: DnsClient::new(self.client.target, self.client.timeout),
            domain: self.domain.clone(),
            rate: self.rate,
            mode: self.mode.clone(),
        }
    }
}

impl Clone for BenchmarkStats {
    fn clone(&self) -> Self {
        Self {
            total_requests: Arc::clone(&self.total_requests),
            successful_responses: Arc::clone(&self.successful_responses),
            failed_responses: Arc::clone(&self.failed_responses),
            start_time: self.start_time,
        }
    }
}
