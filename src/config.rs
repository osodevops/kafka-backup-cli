use serde::Deserialize;

/// Top-level demo scenario loaded from YAML.
#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DemoScenario {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub terminal_size: Option<TerminalSize>,
    pub cluster: ClusterConfig,
    pub backup: BackupConfig,
    pub topics: Vec<TopicConfig>,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct TerminalSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClusterConfig {
    pub name: String,
    pub brokers: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct BackupConfig {
    pub id: String,
    pub storage: String,
    #[serde(default = "default_compression")]
    pub compression: String,
}

fn default_compression() -> String {
    "zstd".to_string()
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct TopicConfig {
    pub name: String,
    pub partitions: u32,
    pub total_records: Vec<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Scene {
    pub name: String,
    pub duration_ms: u64,
    #[serde(default)]
    pub effect: Option<String>,
    #[serde(default)]
    pub actions: Vec<SceneAction>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum SceneAction {
    ShowHeader {
        #[serde(default)]
        text: Option<String>,
    },
    SetPhase {
        phase: String,
    },
    Log {
        message: String,
        level: String,
    },
    LogStream {
        messages: Vec<DelayedLog>,
    },
    AnimatePartitions {
        target_progress: f64,
        throughput_range: [f64; 2],
        #[serde(default)]
        throughput_variance: Option<f64>,
        #[serde(default)]
        stagger_completion: Option<bool>,
    },
    PipelineStage {
        active: String,
    },
    PipelineStageSequence {
        stages: Vec<String>,
        #[serde(default)]
        transition_at: Option<f64>,
    },
    PipelineError {
        stage: String,
        #[serde(default)]
        effect: Option<String>,
    },
    UpdateSummary {
        #[serde(default)]
        compression_ratio: Option<f64>,
    },
    SwitchPipeline {
        mode: String,
    },
    SetThreePhase {
        active_phase: u8,
    },
    ResetPartitions {
        #[serde(default)]
        effect: Option<String>,
    },
    ShowCta {
        text: String,
        #[serde(default)]
        effect: Option<String>,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct DelayedLog {
    pub delay: u64,
    pub message: String,
    pub level: String,
}

/// Parse a YAML scenario file.
pub fn load_scenario(path: &str) -> color_eyre::Result<DemoScenario> {
    let contents = std::fs::read_to_string(path)?;
    let scenario: DemoScenario = serde_yaml::from_str(&contents)?;
    Ok(scenario)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal_scenario() {
        let yaml = r#"
name: "Test Demo"
description: "A test"
cluster:
  name: "test-kafka"
  brokers: 1
backup:
  id: "test-001"
  storage: "s3://test/"
topics:
  - name: "orders"
    partitions: 2
    total_records: [1000, 2000]
scenes:
  - name: "start"
    duration_ms: 1000
    actions:
      - type: set_phase
        phase: "BACKUP"
      - type: log
        message: "Starting backup"
        level: "INFO"
"#;
        let scenario: DemoScenario = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(scenario.name, "Test Demo");
        assert_eq!(scenario.topics.len(), 1);
        assert_eq!(scenario.scenes.len(), 1);
        assert_eq!(scenario.scenes[0].actions.len(), 2);
    }

    #[test]
    fn test_parse_log_stream() {
        let yaml = r#"
name: "Test"
cluster:
  name: "test"
  brokers: 1
backup:
  id: "test"
  storage: "s3://test/"
topics: []
scenes:
  - name: "logs"
    duration_ms: 5000
    actions:
      - type: log_stream
        messages:
          - { delay: 500, message: "First log", level: "INFO" }
          - { delay: 1000, message: "Second log", level: "WARN" }
"#;
        let scenario: DemoScenario = serde_yaml::from_str(yaml).unwrap();
        if let SceneAction::LogStream { messages } = &scenario.scenes[0].actions[0] {
            assert_eq!(messages.len(), 2);
            assert_eq!(messages[0].delay, 500);
        } else {
            panic!("Expected LogStream action");
        }
    }

    #[test]
    fn test_parse_animate_partitions() {
        let yaml = r#"
name: "Test"
cluster:
  name: "test"
  brokers: 1
backup:
  id: "test"
  storage: "s3://test/"
topics: []
scenes:
  - name: "animate"
    duration_ms: 8000
    actions:
      - type: animate_partitions
        target_progress: 0.5
        throughput_range: [95, 142]
        throughput_variance: 0.15
"#;
        let scenario: DemoScenario = serde_yaml::from_str(yaml).unwrap();
        if let SceneAction::AnimatePartitions {
            target_progress,
            throughput_range,
            ..
        } = &scenario.scenes[0].actions[0]
        {
            assert!((target_progress - 0.5).abs() < f64::EPSILON);
            assert!((throughput_range[0] - 95.0).abs() < f64::EPSILON);
        } else {
            panic!("Expected AnimatePartitions action");
        }
    }
}
