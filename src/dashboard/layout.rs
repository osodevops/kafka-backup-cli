use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Pre-computed layout areas for all dashboard zones.
#[derive(Debug, Clone)]
pub struct LayoutAreas {
    pub header: Rect,
    pub pipeline: Rect,
    pub partitions: Rect,
    pub bottom: Rect,
    pub footer: Rect,
}

impl LayoutAreas {
    /// Compute layout areas from the terminal area.
    /// Vertical zones: Header(3) | Pipeline(5) | Partitions(Fill) | Bottom(8) | Footer(1)
    /// Bottom split is handled by the render orchestrator based on view toggles.
    pub fn compute(area: Rect) -> Self {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(5),  // Pipeline
                Constraint::Min(4),    // Partitions (fill remaining)
                Constraint::Length(8),  // Bottom (summary + log)
                Constraint::Length(1),  // Footer
            ])
            .split(area);

        Self {
            header: vertical[0],
            pipeline: vertical[1],
            partitions: vertical[2],
            bottom: vertical[3],
            footer: vertical[4],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_compute_120x40() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = LayoutAreas::compute(area);

        assert_eq!(layout.header.height, 3);
        assert_eq!(layout.pipeline.height, 5);
        assert_eq!(layout.footer.height, 1);
        assert!(layout.partitions.height >= 4);
        assert_eq!(layout.bottom.height, 8);
        assert_eq!(layout.bottom.width, area.width);
    }

    #[test]
    fn test_layout_compute_80x24() {
        let area = Rect::new(0, 0, 80, 24);
        let layout = LayoutAreas::compute(area);

        // Should not panic at minimum size
        assert_eq!(layout.header.height, 3);
        assert_eq!(layout.footer.height, 1);
    }
}
