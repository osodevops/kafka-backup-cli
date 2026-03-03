use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::data::models::{PipelineMode, PipelineStage, StageStatus};
use crate::theme::colors;

pub fn render(frame: &mut Frame, area: Rect, stages: &[PipelineStage], mode: &PipelineMode) {
    let title = match mode {
        PipelineMode::Backup => " Pipeline ",
        PipelineMode::Restore => " Pipeline (Restore) ",
    };

    let block = Block::default()
        .title(title)
        .title_style(Style::default().fg(colors::TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BG_ELEVATED))
        .style(Style::default().bg(colors::BG_PRIMARY));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 2 || inner.width < 10 {
        return;
    }

    // Build the pipeline flow string
    let mut flow_spans: Vec<Span> = Vec::new();
    flow_spans.push(Span::raw("  "));

    for (i, stage) in stages.iter().enumerate() {
        let (style, prefix) = match stage.status {
            StageStatus::Complete => (
                Style::default().fg(colors::STAGE_COMPLETE).add_modifier(Modifier::BOLD),
                "✓ ",
            ),
            StageStatus::Active => (
                Style::default().fg(colors::STAGE_ACTIVE).add_modifier(Modifier::BOLD),
                "▸ ",
            ),
            StageStatus::Error => (
                Style::default().fg(colors::ERROR).add_modifier(Modifier::BOLD),
                "✗ ",
            ),
            StageStatus::Pending => (
                Style::default().fg(colors::STAGE_PENDING),
                "○ ",
            ),
        };

        flow_spans.push(Span::styled("[", style));
        flow_spans.push(Span::styled(prefix, style));
        flow_spans.push(Span::styled(stage.name.to_uppercase(), style));
        flow_spans.push(Span::styled("]", style));

        if i < stages.len() - 1 {
            let connector_style = if stage.status == StageStatus::Complete || stage.status == StageStatus::Active {
                Style::default().fg(colors::STAGE_ACTIVE)
            } else {
                Style::default().fg(colors::STAGE_PENDING)
            };
            flow_spans.push(Span::styled(" ══▸ ", connector_style));
        }
    }

    let line = Line::from(flow_spans);

    // Build status indicators line
    let mut status_spans: Vec<Span> = Vec::new();
    status_spans.push(Span::raw("  "));

    for (i, stage) in stages.iter().enumerate() {
        let (indicator, style) = match stage.status {
            StageStatus::Complete => ("●●●", Style::default().fg(colors::STAGE_COMPLETE)),
            StageStatus::Active => ("▓▓▓", Style::default().fg(colors::STAGE_ACTIVE)),
            StageStatus::Error => ("✗✗✗", Style::default().fg(colors::ERROR)),
            StageStatus::Pending => ("○○○", Style::default().fg(colors::STAGE_PENDING)),
        };

        // Pad to match the stage box width
        let name_width = stage.name.len() + 4; // brackets + prefix
        let padded = format!("{:^width$}", indicator, width = name_width);
        status_spans.push(Span::styled(padded, style));

        if i < stages.len() - 1 {
            status_spans.push(Span::raw("     ")); // connector spacing
        }
    }

    let status_line = Line::from(status_spans);
    let paragraph = Paragraph::new(vec![line, status_line]);
    frame.render_widget(paragraph, inner);
}

/// Default backup pipeline stages.
pub fn default_backup_stages() -> Vec<PipelineStage> {
    vec![
        PipelineStage { name: "Kafka".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Consume".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Compress".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Store".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Done".to_string(), status: StageStatus::Pending, throughput: 0.0 },
    ]
}

/// Default restore pipeline stages.
pub fn default_restore_stages() -> Vec<PipelineStage> {
    vec![
        PipelineStage { name: "S3".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Read".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Decompress".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "PITR Filter".to_string(), status: StageStatus::Pending, throughput: 0.0 },
        PipelineStage { name: "Produce".to_string(), status: StageStatus::Pending, throughput: 0.0 },
    ]
}
