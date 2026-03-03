pub mod footer;
pub mod header;
pub mod layout;
pub mod log_panel;
pub mod partitions;
pub mod pipeline;
pub mod summary;
pub mod three_phase;

use layout::LayoutAreas;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::theme::colors;

/// Main render orchestrator — draws all dashboard zones.
pub fn render(frame: &mut Frame, app: &App) {
    let areas = LayoutAreas::compute(frame.area());

    header::render(frame, areas.header, &app.backup_info, &app.summary);

    // Pipeline zone: split for three-phase indicator when active
    if app.three_phase.active_phase > 0 {
        let pipeline_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // pipeline stages
                Constraint::Min(2),   // three-phase indicator
            ])
            .split(areas.pipeline);
        pipeline::render(frame, pipeline_split[0], &app.pipeline_stages, &app.pipeline_mode);
        three_phase::render(frame, pipeline_split[1], &app.three_phase);
    } else {
        pipeline::render(frame, areas.pipeline, &app.pipeline_stages, &app.pipeline_mode);
    }

    partitions::render(
        frame,
        areas.partitions,
        &app.partition_states,
        app.show_detail,
        app.show_throughput_chart,
    );

    // Bottom zone: log expanded takes full width, otherwise 30/70 split
    if app.show_log_expanded {
        log_panel::render(frame, areas.bottom, &app.log_entries);
    } else {
        let bottom_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(areas.bottom);
        summary::render(frame, bottom_split[0], &app.summary);
        log_panel::render(frame, bottom_split[1], &app.log_entries);
    }

    footer::render(frame, areas.footer, app.is_demo);

    // CTA overlay (rendered last so it draws on top)
    if app.cta.visible {
        render_cta_overlay(frame, areas.partitions, &app.cta.text);
    }
}

fn render_cta_overlay(frame: &mut Frame, area: Rect, text: &str) {
    let text_width = (text.len() as u16).saturating_add(4).min(area.width);
    let text_height = 3u16.min(area.height);

    let x = area.x + area.width.saturating_sub(text_width) / 2;
    let y = area.y + area.height.saturating_sub(text_height) / 2;
    let overlay = Rect::new(x, y, text_width, text_height);

    frame.render_widget(Clear, overlay);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::ACCENT))
        .style(Style::default().bg(colors::BG_SECONDARY));
    let paragraph = Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(colors::ACCENT)
                .add_modifier(Modifier::BOLD),
        )
        .block(block);
    frame.render_widget(paragraph, overlay);
}
