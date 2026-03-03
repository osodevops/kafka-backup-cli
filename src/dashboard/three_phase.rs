use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::data::models::ThreePhaseState;
use crate::theme::colors;

pub fn render(frame: &mut Frame, area: Rect, state: &ThreePhaseState) {
    let block = Block::default()
        .title(" Phases ")
        .title_style(Style::default().fg(colors::TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BG_ELEVATED))
        .style(Style::default().bg(colors::BG_PRIMARY));

    let phases = vec![
        phase_span(1, "Collect Headers", state),
        Span::raw("  "),
        phase_span(2, "Restore Data", state),
        Span::raw("  "),
        phase_span(3, "Reset Offsets", state),
    ];

    let line = Line::from(phases);
    let paragraph = Paragraph::new(vec![Line::raw(""), line]).block(block);
    frame.render_widget(paragraph, area);
}

fn phase_span<'a>(phase_num: u8, label: &'a str, state: &ThreePhaseState) -> Span<'a> {
    let is_complete = match phase_num {
        1 => state.phase1_complete,
        2 => state.phase2_complete,
        3 => state.phase3_complete,
        _ => false,
    };
    let is_active = state.active_phase == phase_num;

    let (prefix, style) = if is_complete {
        (
            "✓",
            Style::default()
                .fg(colors::SUCCESS)
                .add_modifier(Modifier::BOLD),
        )
    } else if is_active {
        (
            "▸",
            Style::default()
                .fg(colors::STAGE_ACTIVE)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            "○",
            Style::default().fg(colors::STAGE_PENDING),
        )
    };

    Span::styled(
        format!("  [{} Phase {}: {}]", prefix, phase_num, label),
        style,
    )
}
