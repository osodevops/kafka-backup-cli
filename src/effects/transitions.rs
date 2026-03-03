use tachyonfx::fx;
use tachyonfx::{Effect, Interpolation, Motion};

use crate::theme::colors;

/// Fade from black — used at app start and scene transitions.
pub fn fade_from_black() -> Effect {
    fx::fade_from(colors::BG_PRIMARY, colors::BG_PRIMARY, (500, Interpolation::CubicOut))
}

/// Slide in from top — used when pipeline first appears.
pub fn slide_in_from_top() -> Effect {
    fx::slide_in(
        Motion::UpToDown,
        15,
        0,
        colors::BG_PRIMARY,
        (400, Interpolation::CubicOut),
    )
}

/// Dim the screen — used before disaster scene.
pub fn dim_screen() -> Effect {
    fx::fade_to(colors::BG_PRIMARY, colors::BG_PRIMARY, (800, Interpolation::Linear))
}

/// Phase change sweep — used during phase transitions.
#[allow(dead_code)]
pub fn phase_change_sweep() -> Effect {
    fx::sweep_in(
        Motion::LeftToRight,
        15,
        0,
        colors::BG_PRIMARY,
        (600, Interpolation::CubicInOut),
    )
}

/// Partition slide in from left.
#[allow(dead_code)]
pub fn partition_slide_in(delay_ms: u32) -> Effect {
    fx::sequence(&[
        fx::sleep(delay_ms),
        fx::slide_in(
            Motion::LeftToRight,
            15,
            0,
            colors::BG_PRIMARY,
            (300, Interpolation::CubicOut),
        ),
    ])
}
