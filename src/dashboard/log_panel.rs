use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::data::models::LogEntry;
use crate::theme::colors;

pub fn render(frame: &mut Frame, area: Rect, entries: &[LogEntry]) {
    let block = Block::default()
        .title(" Live Log ")
        .title_style(Style::default().fg(colors::TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BG_ELEVATED))
        .style(Style::default().bg(colors::BG_PRIMARY));

    let inner_height = block.inner(area).height as usize;

    // Show most recent entries that fit
    let visible: Vec<&LogEntry> = entries
        .iter()
        .rev()
        .take(inner_height)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let items: Vec<ListItem> = visible
        .iter()
        .map(|entry| {
            let line = Line::from(vec![
                Span::styled(
                    format!("[{}] ", entry.format_time()),
                    colors::log_timestamp_style(),
                ),
                Span::styled(
                    &entry.message,
                    colors::log_level_style(&entry.level),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
