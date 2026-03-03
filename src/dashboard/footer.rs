use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::theme::colors;

pub fn render(frame: &mut Frame, area: Rect, is_demo: bool) {
    let mut spans = vec![
        Span::styled(" [q]", colors::footer_key_style()),
        Span::styled(" Quit  ", colors::footer_desc_style()),
    ];

    if is_demo {
        spans.extend([
            Span::styled("[Space]", colors::footer_key_style()),
            Span::styled(" Pause  ", colors::footer_desc_style()),
        ]);
    }

    spans.extend([
        Span::styled("[p]", colors::footer_key_style()),
        Span::styled(" Detail  ", colors::footer_desc_style()),
        Span::styled("[t]", colors::footer_key_style()),
        Span::styled(" Throughput  ", colors::footer_desc_style()),
        Span::styled("[l]", colors::footer_key_style()),
        Span::styled(" Log expand", colors::footer_desc_style()),
    ]);

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(colors::BG_SECONDARY));
    frame.render_widget(paragraph, area);
}
