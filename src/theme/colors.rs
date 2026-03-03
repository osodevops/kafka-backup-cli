use ratatui::style::{Color, Modifier, Style};

// Backgrounds
pub const BG_PRIMARY: Color = Color::Rgb(15, 23, 42); // #0F172A slate-900
pub const BG_SECONDARY: Color = Color::Rgb(30, 41, 59); // #1E293B slate-800
pub const BG_ELEVATED: Color = Color::Rgb(51, 65, 85); // #334155 slate-700

// Text
pub const TEXT_PRIMARY: Color = Color::Rgb(248, 250, 252); // #F8FAFC
pub const TEXT_SECONDARY: Color = Color::Rgb(148, 163, 184); // #94A3B8
pub const TEXT_MUTED: Color = Color::Rgb(100, 116, 139); // #64748B

// Status colours
pub const SUCCESS: Color = Color::Rgb(16, 185, 129); // #10B981 emerald
pub const WARNING: Color = Color::Rgb(245, 158, 11); // #F59E0B amber
pub const ERROR: Color = Color::Rgb(239, 68, 68); // #EF4444 red
pub const INFO: Color = Color::Rgb(56, 189, 248); // #38BDF8 sky

// Brand
pub const PRIMARY: Color = Color::Rgb(37, 99, 235); // #2563EB blue
pub const ACCENT: Color = Color::Rgb(139, 92, 246); // #8B5CF6 violet

// Pipeline stages
pub const STAGE_PENDING: Color = Color::Rgb(71, 85, 105); // #475569
pub const STAGE_ACTIVE: Color = Color::Rgb(6, 182, 212); // #06B6D4 cyan
pub const STAGE_COMPLETE: Color = Color::Rgb(16, 185, 129); // #10B981

// Progress bar gradient endpoints
pub const PROGRESS_START: Color = Color::Rgb(37, 99, 235); // #2563EB blue
pub const PROGRESS_END: Color = Color::Rgb(16, 185, 129); // #10B981 green

// Kafka-specific
#[allow(dead_code)]
pub const KAFKA_ORANGE: Color = Color::Rgb(255, 107, 53); // #FF6B35
#[allow(dead_code)]
pub const S3_ORANGE: Color = Color::Rgb(255, 153, 0); // #FF9900

/// Interpolate between two RGB colours based on ratio (0.0 - 1.0).
pub fn lerp_color(from: Color, to: Color, ratio: f64) -> Color {
    let ratio = ratio.clamp(0.0, 1.0);
    match (from, to) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * ratio) as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * ratio) as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * ratio) as u8;
            Color::Rgb(r, g, b)
        }
        _ => to,
    }
}

/// Get progress bar colour based on completion ratio.
pub fn progress_color(ratio: f64) -> Color {
    lerp_color(PROGRESS_START, PROGRESS_END, ratio)
}

/// Style helpers
#[allow(dead_code)]
pub fn header_style() -> Style {
    Style::default().fg(TEXT_PRIMARY).bg(BG_SECONDARY)
}

#[allow(dead_code)]
pub fn title_style() -> Style {
    Style::default()
        .fg(PRIMARY)
        .add_modifier(Modifier::BOLD)
}

pub fn phase_style(phase: &str) -> Style {
    let color = match phase {
        "BACKUP" => INFO,
        "RESTORE" => ACCENT,
        "COMPLETE" | "RECOVERED" => SUCCESS,
        "ERROR" => ERROR,
        "STORE" | "CONSUME" | "COMPRESS" => STAGE_ACTIVE,
        _ => TEXT_SECONDARY,
    };
    Style::default()
        .fg(color)
        .add_modifier(Modifier::BOLD)
}

pub fn footer_key_style() -> Style {
    Style::default()
        .fg(TEXT_PRIMARY)
        .add_modifier(Modifier::BOLD)
}

pub fn footer_desc_style() -> Style {
    Style::default().fg(TEXT_MUTED)
}

pub fn log_timestamp_style() -> Style {
    Style::default().fg(TEXT_MUTED)
}

pub fn log_level_style(level: &str) -> Style {
    match level {
        "ERROR" => Style::default().fg(ERROR).add_modifier(Modifier::BOLD),
        "WARN" => Style::default().fg(WARNING),
        "INFO" => Style::default().fg(TEXT_SECONDARY),
        _ => Style::default().fg(TEXT_SECONDARY),
    }
}
