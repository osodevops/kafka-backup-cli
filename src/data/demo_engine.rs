use std::time::Duration;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::config::{DemoScenario, Scene, SceneAction};
use crate::data::events::DashboardEvent;

/// Tick-based demo scenario player.
/// Advances through scenes, generates synthetic data, emits DashboardEvents.
pub struct DemoEngine {
    scenario: DemoScenario,
    current_scene_idx: usize,
    scene_elapsed_ms: f64,
    total_elapsed_ms: f64,
    rng: SmallRng,

    // Per-partition state for interpolation
    partition_start_progress: Vec<f64>,
    partition_target_progress: Vec<f64>,
    scene_start_ms: f64,

    // Track which log_stream messages have been emitted (scene index -> set of emitted indices)
    emitted_logs: Vec<Vec<bool>>,

    // Track which pipeline stage sequence transitions have fired
    stage_sequence_fired: Vec<bool>,

    pub finished: bool,
}

impl DemoEngine {
    pub fn new(scenario: DemoScenario) -> Self {
        let num_partitions: usize = scenario
            .topics
            .iter()
            .map(|t| t.total_records.len())
            .sum();

        // Pre-compute emitted logs for all scenes
        let emitted_logs: Vec<Vec<bool>> = scenario
            .scenes
            .iter()
            .map(|scene| {
                let max_msgs = scene
                    .actions
                    .iter()
                    .filter_map(|a| match a {
                        SceneAction::LogStream { messages } => Some(messages.len()),
                        _ => None,
                    })
                    .sum();
                vec![false; max_msgs]
            })
            .collect();

        Self {
            scenario,
            current_scene_idx: 0,
            scene_elapsed_ms: 0.0,
            total_elapsed_ms: 0.0,
            rng: SmallRng::seed_from_u64(42),
            partition_start_progress: vec![0.0; num_partitions],
            partition_target_progress: vec![0.0; num_partitions],
            scene_start_ms: 0.0,
            emitted_logs,
            stage_sequence_fired: Vec::new(),
            finished: false,
        }
    }

    /// Return the number of partitions in the scenario.
    pub fn partition_count(&self) -> usize {
        self.scenario
            .topics
            .iter()
            .map(|t| t.total_records.len())
            .sum()
    }

    /// Get topic/partition info at a flat index.
    pub fn partition_info(&self, idx: usize) -> (&str, u32, u64) {
        let mut offset = 0;
        for topic in &self.scenario.topics {
            if idx < offset + topic.total_records.len() {
                let part_idx = idx - offset;
                return (
                    &topic.name,
                    part_idx as u32,
                    topic.total_records[part_idx],
                );
            }
            offset += topic.total_records.len();
        }
        ("unknown", 0, 0)
    }

    #[allow(dead_code)]
    pub fn cluster_name(&self) -> &str {
        &self.scenario.cluster.name
    }

    #[allow(dead_code)]
    pub fn broker_count(&self) -> u32 {
        self.scenario.cluster.brokers
    }

    #[allow(dead_code)]
    pub fn backup_id(&self) -> &str {
        &self.scenario.backup.id
    }

    #[allow(dead_code)]
    pub fn storage(&self) -> &str {
        &self.scenario.backup.storage
    }

    #[allow(dead_code)]
    pub fn total_elapsed_secs(&self) -> f64 {
        self.total_elapsed_ms / 1000.0
    }

    /// Advance the demo by `delta` and return events.
    pub fn tick(&mut self, delta: Duration) -> Vec<DashboardEvent> {
        if self.finished {
            return vec![];
        }

        let delta_ms = delta.as_secs_f64() * 1000.0;
        self.scene_elapsed_ms += delta_ms;
        self.total_elapsed_ms += delta_ms;

        let mut events = Vec::new();

        let scene = match self.scenario.scenes.get(self.current_scene_idx) {
            Some(s) => s.clone(),
            None => {
                self.finished = true;
                return events;
            }
        };

        let scene_duration = scene.duration_ms as f64;
        let scene_progress = (self.scene_elapsed_ms / scene_duration).clamp(0.0, 1.0);

        // Process scene actions
        self.process_actions(&scene, scene_progress, &mut events);

        // Check if scene is complete
        if self.scene_elapsed_ms >= scene_duration {
            self.advance_scene(&mut events);
        }

        events
    }

    fn process_actions(
        &mut self,
        scene: &Scene,
        scene_progress: f64,
        events: &mut Vec<DashboardEvent>,
    ) {
        let mut log_msg_offset = 0;

        for action in &scene.actions {
            match action {
                SceneAction::SetPhase { phase } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::SetPhase(phase.clone()));
                    }
                }

                SceneAction::Log { message, level } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::Log {
                            level: level.clone(),
                            message: message.clone(),
                        });
                    }
                }

                SceneAction::LogStream { messages } => {
                    for (i, msg) in messages.iter().enumerate() {
                        let global_idx = log_msg_offset + i;
                        if global_idx < self.emitted_logs[self.current_scene_idx].len()
                            && !self.emitted_logs[self.current_scene_idx][global_idx]
                            && self.scene_elapsed_ms >= msg.delay as f64
                        {
                            self.emitted_logs[self.current_scene_idx][global_idx] = true;
                            events.push(DashboardEvent::Log {
                                level: msg.level.clone(),
                                message: msg.message.clone(),
                            });
                        }
                    }
                    log_msg_offset += messages.len();
                }

                SceneAction::AnimatePartitions {
                    target_progress,
                    throughput_range,
                    throughput_variance,
                    stagger_completion,
                } => {
                    let variance = throughput_variance.unwrap_or(0.1);
                    let stagger = stagger_completion.unwrap_or(false);
                    let base_low = throughput_range[0];
                    let base_high = throughput_range[1];
                    let num_parts = self.partition_count();

                    for i in 0..num_parts {
                        let (topic, partition, total_records) = self.partition_info(i);
                        let topic = topic.to_string();

                        // Stagger: each partition finishes at slightly different times
                        let stagger_offset = if stagger {
                            (i as f64 / num_parts as f64) * 0.3
                        } else {
                            0.0
                        };
                        let adjusted_progress =
                            ((scene_progress - stagger_offset) / (1.0 - stagger_offset))
                                .clamp(0.0, 1.0);

                        // Interpolate progress
                        let start = self.partition_start_progress.get(i).copied().unwrap_or(0.0);
                        let current = start + (target_progress - start) * adjusted_progress;

                        // Synthetic throughput with sinusoidal variance
                        let base = base_low + (base_high - base_low) * self.rng.gen::<f64>();
                        let sin_phase = self.total_elapsed_ms / 1000.0 * 2.0 + i as f64 * 0.7;
                        let throughput = base + sin_phase.sin() * variance * base;
                        let throughput = throughput.max(0.0);

                        let records = (total_records as f64 * current) as u64;

                        events.push(DashboardEvent::PartitionProgress {
                            topic: topic.clone(),
                            partition,
                            progress: current.clamp(0.0, 1.0),
                            throughput_mbps: throughput,
                            records_processed: records,
                        });

                        // Fire completion event
                        if current >= 1.0 {
                            events.push(DashboardEvent::PartitionComplete {
                                topic,
                                partition,
                            });
                        }
                    }
                }

                SceneAction::PipelineStage { active } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::PipelineStage(active.clone()));
                    }
                }

                SceneAction::PipelineStageSequence {
                    stages,
                    transition_at,
                } => {
                    // Ensure stage_sequence_fired has enough entries
                    while self.stage_sequence_fired.len() < stages.len() {
                        self.stage_sequence_fired.push(false);
                    }

                    let transition_point = transition_at.unwrap_or(0.5);
                    for (idx, stage) in stages.iter().enumerate() {
                        let threshold = if stages.len() > 1 {
                            transition_point * idx as f64 / (stages.len() - 1) as f64
                        } else {
                            0.0
                        };
                        if scene_progress >= threshold && !self.stage_sequence_fired[idx] {
                            self.stage_sequence_fired[idx] = true;
                            events.push(DashboardEvent::PipelineStage(stage.clone()));
                        }
                    }
                }

                SceneAction::PipelineError { stage, effect } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::PipelineError(stage.clone()));
                        if let Some(fx) = effect {
                            events.push(DashboardEvent::TriggerEffect(fx.clone()));
                        }
                    }
                }

                SceneAction::UpdateSummary { compression_ratio } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::UpdateSummary {
                            compression_ratio: *compression_ratio,
                            throughput_mbps: None,
                            records_processed: None,
                            uncompressed_size: None,
                            compressed_size: None,
                        });
                    }
                }

                SceneAction::SwitchPipeline { mode } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::SwitchPipeline(mode.clone()));
                    }
                }

                SceneAction::SetThreePhase { active_phase } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::SetThreePhase(*active_phase));
                    }
                }

                SceneAction::ResetPartitions { effect } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::ResetPartitions);
                        if let Some(fx) = effect {
                            events.push(DashboardEvent::TriggerEffect(fx.clone()));
                        }
                    }
                }

                SceneAction::ShowCta { text, effect } => {
                    if self.scene_elapsed_ms < 20.0 {
                        events.push(DashboardEvent::ShowCta(text.clone()));
                        if let Some(fx) = effect {
                            events.push(DashboardEvent::TriggerEffect(fx.clone()));
                        }
                    }
                }

                SceneAction::ShowHeader { .. } => {
                    // Header is already shown; this is just for the fade_from_black effect
                    if self.scene_elapsed_ms < 20.0 {
                        if let Some(fx) = &self.scenario.scenes[self.current_scene_idx].effect {
                            events.push(DashboardEvent::TriggerEffect(fx.clone()));
                        }
                    }
                }
            }
        }
    }

    fn advance_scene(&mut self, events: &mut Vec<DashboardEvent>) {
        // Save current partition progress as start for next scene
        // (will be set when AnimatePartitions actions update them)
        self.partition_start_progress = self.partition_target_progress.clone();

        // Look ahead for target progress in next scene
        if let Some(next_scene) = self.scenario.scenes.get(self.current_scene_idx + 1) {
            for action in &next_scene.actions {
                if let SceneAction::AnimatePartitions { target_progress, .. } = action {
                    let num = self.partition_count();
                    self.partition_target_progress = vec![*target_progress; num];
                }
            }

            // Trigger scene transition effect
            if let Some(effect) = &next_scene.effect {
                events.push(DashboardEvent::TriggerEffect(effect.clone()));
            }
        }

        self.current_scene_idx += 1;
        self.scene_elapsed_ms = 0.0;
        self.scene_start_ms = self.total_elapsed_ms;
        self.stage_sequence_fired.clear();

        if self.current_scene_idx >= self.scenario.scenes.len() {
            self.finished = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    fn minimal_scenario() -> DemoScenario {
        DemoScenario {
            name: "Test".to_string(),
            description: String::new(),
            terminal_size: None,
            cluster: ClusterConfig {
                name: "test".to_string(),
                brokers: 1,
            },
            backup: BackupConfig {
                id: "test-001".to_string(),
                storage: "s3://test/".to_string(),
                compression: "zstd".to_string(),
            },
            topics: vec![TopicConfig {
                name: "orders".to_string(),
                partitions: 2,
                total_records: vec![1000, 2000],
            }],
            scenes: vec![
                Scene {
                    name: "start".to_string(),
                    duration_ms: 1000,
                    effect: None,
                    actions: vec![
                        SceneAction::SetPhase {
                            phase: "BACKUP".to_string(),
                        },
                        SceneAction::Log {
                            message: "Starting".to_string(),
                            level: "INFO".to_string(),
                        },
                    ],
                },
                Scene {
                    name: "progress".to_string(),
                    duration_ms: 5000,
                    effect: None,
                    actions: vec![SceneAction::AnimatePartitions {
                        target_progress: 1.0,
                        throughput_range: [100.0, 150.0],
                        throughput_variance: Some(0.1),
                        stagger_completion: Some(true),
                    }],
                },
            ],
        }
    }

    #[test]
    fn test_engine_creation() {
        let engine = DemoEngine::new(minimal_scenario());
        assert_eq!(engine.partition_count(), 2);
        assert!(!engine.finished);
    }

    #[test]
    fn test_engine_tick_emits_events() {
        let mut engine = DemoEngine::new(minimal_scenario());
        let events = engine.tick(Duration::from_millis(16));
        assert!(!events.is_empty());

        // Should have SetPhase and Log events
        let has_phase = events.iter().any(|e| matches!(e, DashboardEvent::SetPhase(_)));
        let has_log = events.iter().any(|e| matches!(e, DashboardEvent::Log { .. }));
        assert!(has_phase);
        assert!(has_log);
    }

    #[test]
    fn test_engine_scene_advance() {
        let mut engine = DemoEngine::new(minimal_scenario());

        // Tick past first scene (1000ms)
        for _ in 0..70 {
            engine.tick(Duration::from_millis(16));
        }

        // Should now be in second scene or beyond first
        assert!(engine.total_elapsed_ms > 1000.0);
    }

    #[test]
    fn test_engine_finishes() {
        let mut engine = DemoEngine::new(minimal_scenario());

        // Tick through entire scenario
        for _ in 0..500 {
            engine.tick(Duration::from_millis(16));
        }

        assert!(engine.finished);
    }
}
