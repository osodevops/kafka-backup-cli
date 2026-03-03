use tachyonfx::fx;
use tachyonfx::{Effect, Interpolation};

use crate::theme::colors;

/// Pulsing effect for the active pipeline stage.
#[allow(dead_code)]
pub fn active_stage_pulse() -> Effect {
    fx::ping_pong(fx::fade_to_fg(
        colors::STAGE_ACTIVE,
        (800, Interpolation::SineInOut),
    ))
}

/// Error state pulsing.
#[allow(dead_code)]
pub fn error_pulse() -> Effect {
    fx::ping_pong(fx::fade_to_fg(
        colors::ERROR,
        (500, Interpolation::SineInOut),
    ))
}

/// New log entry fade in.
#[allow(dead_code)]
pub fn new_log_entry_fade() -> Effect {
    fx::fade_from_fg(colors::BG_PRIMARY, (200, Interpolation::CubicOut))
}
