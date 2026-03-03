use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::data::models::{BackupInfo, SummaryStats};
use crate::theme::colors;

pub fn render(frame: &mut Frame, area: Rect, info: &BackupInfo, stats: &SummaryStats) {
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(colors::BG_ELEVATED))
        .style(Style::default().bg(colors::BG_SECONDARY));

    let line1 = Line::from(vec![
        Span::styled(
            " ◉ kafka-backup monitor",
            Style::default()
                .fg(colors::PRIMARY)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("     "),
        Span::styled(
            &info.mode,
            colors::phase_style(&info.mode),
        ),
        Span::styled(" ▸ ", Style::default().fg(colors::TEXT_MUTED)),
        Span::styled(&info.id, Style::default().fg(colors::TEXT_PRIMARY)),
        Span::raw("     "),
        Span::styled("Phase: ", Style::default().fg(colors::TEXT_MUTED)),
        Span::styled(
            &info.phase,
            colors::phase_style(&info.phase),
        ),
    ]);

    let line2 = Line::from(vec![
        Span::styled(
            format!("   Cluster: {} ({} brokers)", info.cluster_name, info.brokers),
            Style::default().fg(colors::TEXT_SECONDARY),
        ),
        Span::raw("  "),
        Span::styled(
            format!("Storage: {}", info.storage),
            Style::default().fg(colors::TEXT_SECONDARY),
        ),
        Span::raw("     "),
        Span::styled(
            format!("Elapsed: {}", stats.format_elapsed()),
            Style::default().fg(colors::TEXT_SECONDARY),
        ),
    ]);

    let paragraph = Paragraph::new(vec![line1, line2]).block(block);
    frame.render_widget(paragraph, area);
}
