use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};

use crate::theme::colors;

/// PITR time window slider for restore mode.
/// Displays a timeline with start/end timestamps and a marker for the target point.
#[allow(dead_code)]
pub struct PitrTimelineWidget<'a> {
    pub start_time: &'a str,
    pub end_time: &'a str,
    pub target_position: f64, // 0.0 - 1.0 within the window
}

impl<'a> Widget for PitrTimelineWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 20 || area.height < 1 {
            return;
        }

        let start_label = format!("  {} ", self.start_time);
        let end_label = format!(" {}", self.end_time);

        let track_width = area.width as usize
            - start_label.len()
            - end_label.len();

        if track_width < 5 {
            return;
        }

        // Build the track
        let marker_pos = (self.target_position * track_width as f64).clamp(0.0, (track_width - 1) as f64) as usize;

        let mut track = String::with_capacity(track_width);
        for i in 0..track_width {
            if i == marker_pos {
                track.push('●');
            } else {
                track.push('─');
            }
        }

        let line = Line::from(vec![
            Span::styled(
                "  PITR Window: ",
                Style::default().fg(colors::TEXT_MUTED),
            ),
            Span::styled(
                &start_label,
                Style::default().fg(colors::TEXT_SECONDARY),
            ),
            Span::styled(
                track,
                Style::default().fg(colors::ACCENT),
            ),
            Span::styled(
                end_label,
                Style::default().fg(colors::TEXT_SECONDARY),
            ),
        ]);

        buf.set_line(area.x, area.y, &line, area.width);
    }
}
