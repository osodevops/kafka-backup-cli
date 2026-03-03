use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::Widget,
};

use crate::theme::colors;

/// Animated data flow connector between pipeline stages.
/// Renders `══▸` with animation offset for flowing effect.
#[allow(dead_code)]
pub struct DataFlowWidget {
    pub active: bool,
    pub tick: u64,
}

impl Widget for DataFlowWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 3 || area.height < 1 {
            return;
        }

        let style = if self.active {
            Style::default().fg(colors::STAGE_ACTIVE)
        } else {
            Style::default().fg(colors::STAGE_PENDING)
        };

        // Simple connector — animation handled by tachyonfx effects
        let connector = " ══▸ ";
        let x = area.x;
        let y = area.y;

        for (i, ch) in connector.chars().enumerate() {
            if x + i as u16 >= area.x + area.width {
                break;
            }
            buf[(x + i as u16, y)].set_char(ch).set_style(style);
        }
    }
}
