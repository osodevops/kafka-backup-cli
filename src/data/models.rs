use std::collections::VecDeque;

use chrono::{DateTime, Utc};

/// Status of a pipeline stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageStatus {
    Pending,
    Active,
    Complete,
    Error,
}

/// A single stage in the processing pipeline.
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub status: StageStatus,
    #[allow(dead_code)]
    pub throughput: f64,
}

/// The operating mode — backup or restore.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PipelineMode {
    #[default]
    Backup,
    Restore,
}

/// Per-partition progress state.
#[derive(Debug, Clone)]
pub struct PartitionState {
    pub topic: String,
    pub partition: u32,
    pub progress: f64,
    pub throughput_mbps: f64,
    pub throughput_history: VecDeque<u64>,
    pub records_processed: u64,
    #[allow(dead_code)]
    pub total_records: u64,
    pub status: PartitionStatus,
}

impl PartitionState {
    pub fn new(topic: &str, partition: u32, total_records: u64) -> Self {
        Self {
            topic: topic.to_string(),
            partition,
            progress: 0.0,
            throughput_mbps: 0.0,
            throughput_history: VecDeque::with_capacity(20),
            records_processed: 0,
            total_records,
            status: PartitionStatus::Active,
        }
    }

    pub fn label(&self) -> String {
        format!("{}:{}", self.topic, self.partition)
    }

    /// Push a throughput reading to the sparkline history.
    pub fn push_throughput(&mut self, mbps: f64) {
        if self.throughput_history.len() >= 20 {
            self.throughput_history.pop_front();
        }
        // Convert to u64 for Sparkline (scale by 10 for precision)
        self.throughput_history.push_back((mbps * 10.0) as u64);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PartitionStatus {
    Active,
    Complete,
    Stalled,
    Error,
}

/// Aggregate stats displayed in the summary panel.
#[derive(Debug, Clone, Default)]
pub struct SummaryStats {
    pub total_records: u64,
    pub records_processed: u64,
    pub uncompressed_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub throughput_mbps: f64,
    pub elapsed_secs: f64,
    pub eta_secs: Option<f64>,
}

impl SummaryStats {
    pub fn format_records(&self) -> String {
        format_number(self.records_processed)
    }

    pub fn format_total_records(&self) -> String {
        format_number(self.total_records)
    }

    pub fn format_size(&self) -> String {
        let uncompressed = format_bytes(self.uncompressed_size);
        let compressed = format_bytes(self.compressed_size);
        format!("{} → {}", uncompressed, compressed)
    }

    pub fn format_ratio(&self) -> String {
        if self.compression_ratio > 0.0 {
            format!("{:.1}:1", self.compression_ratio)
        } else {
            "—".to_string()
        }
    }

    pub fn format_throughput(&self) -> String {
        format!("{:.0} MB/s", self.throughput_mbps)
    }

    pub fn format_eta(&self) -> String {
        match self.eta_secs {
            Some(secs) if secs > 0.0 => {
                let m = (secs / 60.0) as u64;
                let s = (secs % 60.0) as u64;
                if m > 0 {
                    format!("{}m {}s", m, s)
                } else {
                    format!("{}s", s)
                }
            }
            _ => "—".to_string(),
        }
    }

    pub fn format_elapsed(&self) -> String {
        let secs = self.elapsed_secs as u64;
        let m = secs / 60;
        let s = secs % 60;
        if m > 0 {
            format!("{}m {}s", m, s)
        } else {
            format!("{}s", s)
        }
    }
}

/// A log entry displayed in the live log panel.
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
}

impl LogEntry {
    pub fn new(level: &str, message: &str) -> Self {
        Self {
            timestamp: Utc::now(),
            level: level.to_string(),
            message: message.to_string(),
        }
    }

    pub fn format_time(&self) -> String {
        self.timestamp.format("%H:%M:%S").to_string()
    }
}

/// Top-level backup/restore info displayed in the header.
#[derive(Debug, Clone)]
pub struct BackupInfo {
    pub id: String,
    pub mode: String,
    pub cluster_name: String,
    pub brokers: u32,
    pub storage: String,
    pub phase: String,
}

impl Default for BackupInfo {
    fn default() -> Self {
        Self {
            id: "production-backup-001".to_string(),
            mode: "BACKUP".to_string(),
            cluster_name: "prod-kafka".to_string(),
            brokers: 3,
            storage: "s3://backups/".to_string(),
            phase: "STARTING".to_string(),
        }
    }
}

/// Three-phase restore tracking.
#[derive(Debug, Clone, Default)]
pub struct ThreePhaseState {
    pub active_phase: u8, // 0 = none, 1-3
    #[allow(dead_code)]
    pub phase1_elapsed: f64,
    #[allow(dead_code)]
    pub phase2_elapsed: f64,
    #[allow(dead_code)]
    pub phase2_estimate: f64,
    #[allow(dead_code)]
    pub phase3_elapsed: f64,
    pub phase1_complete: bool,
    pub phase2_complete: bool,
    pub phase3_complete: bool,
}

/// CTA (call-to-action) overlay.
#[derive(Debug, Clone, Default)]
pub struct CtaState {
    pub visible: bool,
    pub text: String,
}

// --- Formatting helpers ---

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    const GB: u64 = 1024 * 1024 * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.0} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1847392), "1,847,392");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1 KB");
        assert_eq!(format_bytes(1024 * 1024 * 312), "312 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 + 200 * 1024 * 1024), "1.2 GB");
    }

    #[test]
    fn test_partition_label() {
        let p = PartitionState::new("orders", 0, 150234);
        assert_eq!(p.label(), "orders:0");
    }

    #[test]
    fn test_throughput_history() {
        let mut p = PartitionState::new("orders", 0, 100);
        for i in 0..25 {
            p.push_throughput(i as f64 * 10.0);
        }
        assert_eq!(p.throughput_history.len(), 20);
    }
}
