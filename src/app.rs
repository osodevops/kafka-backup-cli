use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::config::DemoScenario;
use crate::dashboard;
use crate::data::demo_engine::DemoEngine;
use crate::data::events::DashboardEvent;
use crate::data::models::*;
use crate::effects::{self, AppEffectManager};

/// Target frame duration for ~60fps.
const TICK_RATE: Duration = Duration::from_millis(16);

/// Application state machine.
pub struct App {
    pub running: bool,
    pub paused: bool,
    pub is_demo: bool,

    // Dashboard data
    pub backup_info: BackupInfo,
    pub pipeline_stages: Vec<PipelineStage>,
    pub pipeline_mode: PipelineMode,
    pub partition_states: Vec<PartitionState>,
    pub summary: SummaryStats,
    pub log_entries: Vec<LogEntry>,
    pub three_phase: ThreePhaseState,
    pub cta: CtaState,

    // Demo engine
    pub demo_engine: Option<DemoEngine>,

    // Effects
    pub effect_manager: AppEffectManager,

    // Timing
    pub elapsed: Duration,
    pub last_tick: Instant,

    // View toggles
    pub show_detail: bool,
    pub show_throughput_chart: bool,
    pub show_log_expanded: bool,
}

impl App {
    pub fn new(is_demo: bool) -> Self {
        Self {
            running: true,
            paused: false,
            is_demo,
            backup_info: BackupInfo::default(),
            pipeline_stages: crate::dashboard::pipeline::default_backup_stages(),
            pipeline_mode: PipelineMode::Backup,
            partition_states: Vec::new(),
            summary: SummaryStats::default(),
            log_entries: Vec::new(),
            three_phase: ThreePhaseState::default(),
            cta: CtaState::default(),
            demo_engine: None,
            effect_manager: effects::new_effect_manager(),
            elapsed: Duration::ZERO,
            last_tick: Instant::now(),
            show_detail: false,
            show_throughput_chart: false,
            show_log_expanded: false,
        }
    }

    /// Create app from a demo scenario.
    pub fn from_scenario(scenario: DemoScenario) -> Self {
        let mut app = Self::new(true);

        let engine = DemoEngine::new(scenario.clone());

        // Initialize partitions from scenario topics
        for topic in &scenario.topics {
            for (i, &total) in topic.total_records.iter().enumerate() {
                app.partition_states
                    .push(PartitionState::new(&topic.name, i as u32, total));
            }
        }

        // Set backup info from scenario
        app.backup_info = BackupInfo {
            id: scenario.backup.id.clone(),
            mode: "BACKUP".to_string(),
            cluster_name: scenario.cluster.name.clone(),
            brokers: scenario.cluster.brokers,
            storage: scenario.backup.storage.clone(),
            phase: "STARTING".to_string(),
        };

        // Set total records in summary
        app.summary.total_records = scenario
            .topics
            .iter()
            .flat_map(|t| &t.total_records)
            .sum();

        app.demo_engine = Some(engine);
        app
    }

    /// Create app with sample data for development/testing.
    pub fn with_sample_data(is_demo: bool) -> Self {
        let mut app = Self::new(is_demo);

        let topics = vec![
            ("orders", vec![150234u64, 148891, 152456]),
            ("payments", vec![78234, 81023]),
            ("events", vec![234567, 228901, 241234]),
        ];

        for (topic, totals) in &topics {
            for (i, &total) in totals.iter().enumerate() {
                let mut p = PartitionState::new(topic, i as u32, total);
                let progress = match (*topic, i) {
                    ("orders", 1) | ("events", 0) => 1.0,
                    ("payments", 0) => 0.92,
                    ("payments", 1) => 0.85,
                    ("events", 1) => 0.96,
                    ("events", 2) => 0.77,
                    ("orders", 0) => 0.78,
                    ("orders", 2) => 0.58,
                    _ => 0.5,
                };
                p.progress = progress;
                p.throughput_mbps = 100.0 + (i as f64 * 15.0);
                p.records_processed = (total as f64 * progress) as u64;
                p.status = if progress >= 1.0 {
                    PartitionStatus::Complete
                } else {
                    PartitionStatus::Active
                };
                for j in 0..15 {
                    let t = 90.0 + (j as f64 * 3.0).sin() * 20.0;
                    p.push_throughput(t);
                }
                app.partition_states.push(p);
            }
        }

        app.pipeline_stages = vec![
            PipelineStage { name: "Kafka".to_string(), status: StageStatus::Complete, throughput: 0.0 },
            PipelineStage { name: "Consume".to_string(), status: StageStatus::Complete, throughput: 127.0 },
            PipelineStage { name: "Compress".to_string(), status: StageStatus::Complete, throughput: 127.0 },
            PipelineStage { name: "Store".to_string(), status: StageStatus::Active, throughput: 127.0 },
            PipelineStage { name: "Done".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        ];

        app.backup_info.phase = "STORE".to_string();

        app.summary = SummaryStats {
            total_records: 1_315_540,
            records_processed: 1_047_392,
            uncompressed_size: 1_288_490_188,
            compressed_size: 327_155_712,
            compression_ratio: 3.8,
            throughput_mbps: 127.0,
            elapsed_secs: 120.0,
            eta_secs: Some(83.0),
        };

        let logs = vec![
            ("INFO", "Connected to Kafka cluster (3 brokers)"),
            ("INFO", "Discovering partitions for 3 topics..."),
            ("INFO", "Backing up: orders (3), payments (2), events (3)"),
            ("INFO", "Segment 47/156 written"),
            ("INFO", "orders:2 checkpoint saved"),
            ("INFO", "payments:0 segment complete"),
            ("INFO", "Compressing batch 48..."),
            ("INFO", "events:1 offset 892341"),
        ];
        for (level, msg) in logs {
            app.log_entries.push(LogEntry::new(level, msg));
        }

        app
    }

    /// Main run loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            let now = Instant::now();
            let delta = now.duration_since(self.last_tick);
            self.last_tick = now;

            if !self.paused {
                self.elapsed += delta;
                self.summary.elapsed_secs = self.elapsed.as_secs_f64();

                // Tick the demo engine — collect events first to avoid borrow conflict
                let demo_events = if let Some(ref mut engine) = self.demo_engine {
                    engine.tick(delta)
                } else {
                    vec![]
                };
                for event in demo_events {
                    self.process_event(event);
                }
            }

            // Render
            terminal.draw(|frame| {
                dashboard::render(frame, &self);

                // Process tachyonfx effects on the buffer
                let area = frame.area();
                effects::process_effects(
                    &mut self.effect_manager,
                    delta,
                    frame.buffer_mut(),
                    area,
                );
            })?;

            // Poll for events
            let timeout = TICK_RATE.saturating_sub(now.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key.code);
                    }
                }
            }
        }

        Ok(())
    }

    fn process_event(&mut self, event: DashboardEvent) {
        match event {
            DashboardEvent::SetPhase(phase) => {
                self.backup_info.phase = phase.clone();
                self.backup_info.mode = match phase.as_str() {
                    "RESTORE" | "RECOVERED" => "RESTORE".to_string(),
                    _ => "BACKUP".to_string(),
                };
            }

            DashboardEvent::PartitionProgress {
                topic,
                partition,
                progress,
                throughput_mbps,
                records_processed,
            } => {
                if let Some(p) = self.partition_states.iter_mut().find(|p| {
                    p.topic == topic && p.partition == partition
                }) {
                    p.progress = progress;
                    p.throughput_mbps = throughput_mbps;
                    p.records_processed = records_processed;
                    p.push_throughput(throughput_mbps);
                }

                // Update summary
                self.update_summary_from_partitions();
            }

            DashboardEvent::PartitionComplete { topic, partition } => {
                if let Some(p) = self.partition_states.iter_mut().find(|p| {
                    p.topic == topic && p.partition == partition
                }) {
                    p.status = PartitionStatus::Complete;
                    p.progress = 1.0;
                }
            }

            DashboardEvent::Log { level, message } => {
                self.log_entries.push(LogEntry::new(&level, &message));
                // Keep max 50 log entries
                if self.log_entries.len() > 50 {
                    self.log_entries.remove(0);
                }
            }

            DashboardEvent::PipelineStage(stage_name) => {
                let name_lower = stage_name.to_lowercase();
                let mut found = false;
                for stage in &mut self.pipeline_stages {
                    if stage.name.to_lowercase() == name_lower {
                        stage.status = StageStatus::Active;
                        found = true;
                    } else if found {
                        // Stages after the active one remain pending
                    } else {
                        // Stages before the active one are complete
                        stage.status = StageStatus::Complete;
                    }
                }
            }

            DashboardEvent::PipelineError(stage_name) => {
                let name_lower = stage_name.to_lowercase();
                for stage in &mut self.pipeline_stages {
                    if stage.name.to_lowercase() == name_lower {
                        stage.status = StageStatus::Error;
                    }
                }
            }

            DashboardEvent::UpdateSummary {
                compression_ratio,
                throughput_mbps,
                records_processed,
                uncompressed_size,
                compressed_size,
            } => {
                if let Some(r) = compression_ratio {
                    self.summary.compression_ratio = r;
                }
                if let Some(t) = throughput_mbps {
                    self.summary.throughput_mbps = t;
                }
                if let Some(r) = records_processed {
                    self.summary.records_processed = r;
                }
                if let Some(u) = uncompressed_size {
                    self.summary.uncompressed_size = u;
                }
                if let Some(c) = compressed_size {
                    self.summary.compressed_size = c;
                }
            }

            DashboardEvent::SwitchPipeline(mode) => {
                match mode.as_str() {
                    "restore" => {
                        self.pipeline_mode = PipelineMode::Restore;
                        self.pipeline_stages =
                            crate::dashboard::pipeline::default_restore_stages();
                    }
                    _ => {
                        self.pipeline_mode = PipelineMode::Backup;
                        self.pipeline_stages =
                            crate::dashboard::pipeline::default_backup_stages();
                    }
                }
            }

            DashboardEvent::SetThreePhase(phase) => {
                // Mark previous phases as complete
                if phase >= 2 {
                    self.three_phase.phase1_complete = true;
                }
                if phase >= 3 {
                    self.three_phase.phase2_complete = true;
                }
                self.three_phase.active_phase = phase;
            }

            DashboardEvent::ResetPartitions => {
                for p in &mut self.partition_states {
                    p.progress = 0.0;
                    p.throughput_mbps = 0.0;
                    p.records_processed = 0;
                    p.status = PartitionStatus::Active;
                    p.throughput_history.clear();
                }
            }

            DashboardEvent::ShowCta(text) => {
                self.cta.visible = true;
                self.cta.text = text;
            }

            DashboardEvent::TriggerEffect(effect_name) => {
                effects::trigger_named_effect(
                    &mut self.effect_manager,
                    &effect_name,
                    ratatui::layout::Rect::default(),
                );
            }

            DashboardEvent::NextScene => {
                // Handled internally by the demo engine
            }
        }
    }

    fn update_summary_from_partitions(&mut self) {
        let total_processed: u64 = self.partition_states.iter().map(|p| p.records_processed).sum();
        let avg_throughput = if self.partition_states.is_empty() {
            0.0
        } else {
            self.partition_states.iter().map(|p| p.throughput_mbps).sum::<f64>()
                / self.partition_states.len() as f64
        };

        self.summary.records_processed = total_processed;
        self.summary.throughput_mbps = avg_throughput;

        // Estimate sizes based on records
        if self.summary.total_records > 0 {
            let ratio = total_processed as f64 / self.summary.total_records as f64;
            // Assume ~1KB per record uncompressed
            self.summary.uncompressed_size = (self.summary.total_records as f64 * 1024.0 * ratio) as u64;
            if self.summary.compression_ratio > 0.0 {
                self.summary.compressed_size =
                    (self.summary.uncompressed_size as f64 / self.summary.compression_ratio) as u64;
            }
        }

        // Estimate ETA
        if avg_throughput > 0.0 && self.summary.total_records > total_processed {
            let remaining_records = self.summary.total_records - total_processed;
            // Rough: remaining_records * avg_bytes_per_record / throughput
            let remaining_bytes = remaining_records as f64 * 1024.0;
            let throughput_bytes = avg_throughput * 1024.0 * 1024.0;
            self.summary.eta_secs = Some(remaining_bytes / throughput_bytes);
        } else {
            self.summary.eta_secs = None;
        }
    }

    fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Char(' ') if self.is_demo => self.paused = !self.paused,
            KeyCode::Char('p') => self.show_detail = !self.show_detail,
            KeyCode::Char('t') => self.show_throughput_chart = !self.show_throughput_chart,
            KeyCode::Char('l') => self.show_log_expanded = !self.show_log_expanded,
            _ => {}
        }
    }
}
