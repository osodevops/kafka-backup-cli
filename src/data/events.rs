/// Events emitted by the demo engine or live data source,
/// consumed by the App to update dashboard state.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DashboardEvent {
    /// Set the current phase label (e.g. "BACKUP", "RESTORE", "ERROR")
    SetPhase(String),

    /// Update a partition's progress and throughput
    PartitionProgress {
        topic: String,
        partition: u32,
        progress: f64,
        throughput_mbps: f64,
        records_processed: u64,
    },

    /// Mark a partition as complete
    PartitionComplete {
        topic: String,
        partition: u32,
    },

    /// Append a log entry
    Log {
        level: String,
        message: String,
    },

    /// Set the active pipeline stage
    PipelineStage(String),

    /// Set a pipeline stage to error status
    PipelineError(String),

    /// Update summary stats
    UpdateSummary {
        compression_ratio: Option<f64>,
        throughput_mbps: Option<f64>,
        records_processed: Option<u64>,
        uncompressed_size: Option<u64>,
        compressed_size: Option<u64>,
    },

    /// Switch pipeline mode (backup / restore)
    SwitchPipeline(String),

    /// Set the three-phase restore active phase
    SetThreePhase(u8),

    /// Reset all partitions (e.g. after disaster)
    ResetPartitions,

    /// Show CTA overlay
    ShowCta(String),

    /// Trigger a named effect
    TriggerEffect(String),

    /// Advance to next scene (internal)
    NextScene,
}
