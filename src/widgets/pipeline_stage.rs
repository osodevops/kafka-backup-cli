use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::data::models::StageStatus;
use crate::theme::colors;

/// A single pipeline stage box widget with status colouring.
#[allow(dead_code)]
pub struct PipelineStageWidget<'a> {
    pub name: &'a str,
    pub status: &'a StageStatus,
}

impl<'a> Widget for PipelineStageWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 3 || area.height < 1 {
            return;
        }

        let (style, prefix) = match self.status {
            StageStatus::Complete => (
                Style::default().fg(colors::STAGE_COMPLETE).add_modifier(Modifier::BOLD),
                "✓",
            ),
            StageStatus::Active => (
                Style::default().fg(colors::STAGE_ACTIVE).add_modifier(Modifier::BOLD),
                "▸",
            ),
            StageStatus::Error => (
                Style::default().fg(colors::ERROR).add_modifier(Modifier::BOLD),
                "✗",
            ),
            StageStatus::Pending => (
                Style::default().fg(colors::STAGE_PENDING),
                "○",
            ),
        };

        let text = format!("[{} {}]", prefix, self.name.to_uppercase());
        let line = Line::from(Span::styled(text, style));
        buf.set_line(area.x, area.y, &line, area.width);
    }
}
