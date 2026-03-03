use tachyonfx::fx;
use tachyonfx::{Effect, Interpolation};

use crate::theme::colors;

/// Red flash for disaster scene.
pub fn red_flash() -> Effect {
    fx::sequence(&[
        fx::fade_to_fg(colors::ERROR, (150, Interpolation::Linear)),
        fx::fade_to_fg(colors::TEXT_PRIMARY, (400, Interpolation::CubicOut)),
    ])
}

/// Pipeline explode effect.
pub fn pipeline_explode() -> Effect {
    fx::dissolve((500, Interpolation::QuadOut))
}

/// Partition dissolve effect.
pub fn partition_dissolve() -> Effect {
    fx::dissolve((400, Interpolation::CubicOut))
}
