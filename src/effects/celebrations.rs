use tachyonfx::fx;
use tachyonfx::{Effect, Interpolation};

use crate::theme::colors;

/// Glow effect when all partitions complete.
pub fn all_complete_glow() -> Effect {
    fx::sequence(&[
        fx::fade_to_fg(colors::SUCCESS, (300, Interpolation::CubicIn)),
        fx::fade_to_fg(colors::TEXT_PRIMARY, (700, Interpolation::CubicOut)),
    ])
}

/// Single partition completion celebration.
#[allow(dead_code)]
pub fn partition_complete() -> Effect {
    fx::fade_to_fg(colors::SUCCESS, (300, Interpolation::CubicIn))
}

/// CTA text materialisation.
pub fn cta_coalesce() -> Effect {
    fx::coalesce((600, Interpolation::CubicOut))
}
