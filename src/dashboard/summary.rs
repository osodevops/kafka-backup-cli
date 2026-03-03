use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::data::models::SummaryStats;
use crate::theme::colors;

pub fn render(frame: &mut Frame, area: Rect, stats: &SummaryStats) {
    let block = Block::default()
        .title(" Summary ")
        .title_style(Style::default().fg(colors::TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BG_ELEVATED))
        .style(Style::default().bg(colors::BG_PRIMARY));

    let records = format!("{} / {}", stats.format_records(), stats.format_total_records());
    let size = stats.format_size();
    let ratio = stats.format_ratio();
    let throughput = stats.format_throughput();
    let eta = stats.format_eta();

    let lines = vec![
        summary_line("Records", &records),
        summary_line("Size", &size),
        summary_line("Ratio", &ratio),
        summary_line("Throughput", &throughput),
        summary_line("ETA", &eta),
    ];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}

fn summary_line<'a>(label: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(
            format!("  {:<12}", label),
            Style::default().fg(colors::TEXT_MUTED),
        ),
        Span::styled(
            value.to_string(),
            Style::default().fg(colors::TEXT_PRIMARY).add_modifier(Modifier::BOLD),
        ),
    ])
}
