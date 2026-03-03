use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::data::models::{PartitionState, PartitionStatus};
use crate::theme::colors;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    partitions: &[PartitionState],
    show_detail: bool,
    show_throughput: bool,
) {
    let block = Block::default()
        .title(" Partitions ")
        .title_style(Style::default().fg(colors::TEXT_PRIMARY).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BG_ELEVATED))
        .style(Style::default().bg(colors::BG_PRIMARY));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height == 0 || inner.width < 20 {
        return;
    }

    let row_height: u16 = if show_detail { 2 } else { 1 };
    let max_rows = inner.height as usize / row_height as usize;
    let visible = partitions.iter().take(max_rows).collect::<Vec<_>>();

    let row_constraints: Vec<Constraint> = visible
        .iter()
        .map(|_| Constraint::Length(row_height))
        .collect();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(row_constraints)
        .split(inner);

    for (i, partition) in visible.iter().enumerate() {
        if show_detail {
            // Split each row into 2 lines
            let lines = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)])
                .split(rows[i]);
            render_partition_row(frame, lines[0], partition, show_throughput);
            render_detail_row(frame, lines[1], partition);
        } else {
            render_partition_row(frame, rows[i], partition, show_throughput);
        }
    }
}

fn render_partition_row(
    frame: &mut Frame,
    area: Rect,
    partition: &PartitionState,
    show_throughput: bool,
) {
    if area.width < 20 {
        return;
    }

    // Layout: Label(14) | Gauge(Fill) | Pct(6) | Throughput(11) | Sparkline(12)
    let has_sparkline = show_throughput && area.width >= 55;
    let constraints = if has_sparkline {
        vec![
            Constraint::Length(14),  // label
            Constraint::Min(10),     // gauge
            Constraint::Length(6),   // percentage
            Constraint::Length(11),  // throughput
            Constraint::Length(12),  // sparkline
        ]
    } else {
        vec![
            Constraint::Length(14),
            Constraint::Min(10),
            Constraint::Length(6),
            Constraint::Length(11),
        ]
    };

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    // Label
    let label_style = match partition.status {
        PartitionStatus::Complete => Style::default().fg(colors::SUCCESS),
        PartitionStatus::Error => Style::default().fg(colors::ERROR),
        PartitionStatus::Stalled => Style::default().fg(colors::WARNING),
        PartitionStatus::Active => Style::default().fg(colors::TEXT_PRIMARY),
    };
    let label = Paragraph::new(Line::from(Span::styled(
        format!("  {:>11}", partition.label()),
        label_style,
    )));
    frame.render_widget(label, cols[0]);

    // Gauge (progress bar)
    let gauge_color = colors::progress_color(partition.progress);
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(gauge_color).bg(colors::BG_ELEVATED))
        .ratio(partition.progress.clamp(0.0, 1.0))
        .label("");
    frame.render_widget(gauge, cols[1]);

    // Percentage
    let pct_text = format!("{:>4.0}%", partition.progress * 100.0);
    let pct_style = if partition.progress >= 1.0 {
        Style::default().fg(colors::SUCCESS).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(colors::TEXT_SECONDARY)
    };
    let pct = Paragraph::new(Span::styled(pct_text, pct_style));
    frame.render_widget(pct, cols[2]);

    // Throughput
    let throughput_text = format!("{:>6.0} MB/s", partition.throughput_mbps);
    let throughput = Paragraph::new(Span::styled(
        throughput_text,
        Style::default().fg(colors::TEXT_SECONDARY),
    ));
    frame.render_widget(throughput, cols[3]);

    // Sparkline (if space and toggled on)
    if has_sparkline && cols.len() > 4 {
        let history: Vec<u64> = partition.throughput_history.iter().copied().collect();
        let sparkline = Sparkline::default()
            .data(&history)
            .style(Style::default().fg(colors::INFO));
        frame.render_widget(sparkline, cols[4]);
    }
}

fn render_detail_row(frame: &mut Frame, area: Rect, partition: &PartitionState) {
    let status_text = match partition.status {
        PartitionStatus::Complete => "COMPLETE",
        PartitionStatus::Active => "ACTIVE",
        PartitionStatus::Stalled => "STALLED",
        PartitionStatus::Error => "ERROR",
    };
    let detail = format!(
        "    {:>11}  {} / {} records  [{}]",
        "",
        format_compact(partition.records_processed),
        format_compact(partition.total_records),
        status_text,
    );
    let style = Style::default().fg(colors::TEXT_MUTED);
    frame.render_widget(Paragraph::new(Span::styled(detail, style)), area);
}

fn format_compact(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
