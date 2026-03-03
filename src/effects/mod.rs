pub mod celebrations;
pub mod continuous;
pub mod disaster;
pub mod transitions;

use std::time::Duration;
use tachyonfx::EffectManager;

/// Keys for unique effects in the effect manager.
/// Using String for simplicity — the K in EffectManager<K> is just for unique effect identification.
pub type AppEffectManager = EffectManager<String>;

/// Create a new effect manager.
pub fn new_effect_manager() -> AppEffectManager {
    AppEffectManager::default()
}

/// Dispatch a named effect trigger from the demo engine.
pub fn trigger_named_effect(
    manager: &mut AppEffectManager,
    effect_name: &str,
    _area: ratatui::layout::Rect,
) {
    let fx = match effect_name {
        "fade_from_black" => transitions::fade_from_black(),
        "slide_in_pipeline" => transitions::slide_in_from_top(),
        "celebration" | "celebration_restore" => celebrations::all_complete_glow(),
        "screen_shake_and_red" => disaster::red_flash(),
        "dim_screen" => transitions::dim_screen(),
        "fade_to_restore_theme" => transitions::fade_from_black(),
        "dissolve" => disaster::partition_dissolve(),
        "explode" => disaster::pipeline_explode(),
        "coalesce" => celebrations::cta_coalesce(),
        _ => return, // Unknown effect — silently ignore
    };

    // Use add_unique_effect so the same named effect replaces the previous one
    manager.add_unique_effect(effect_name.to_string(), fx);
}

/// Process all active effects, applying them to the frame buffer.
pub fn process_effects(
    manager: &mut AppEffectManager,
    delta: Duration,
    buf: &mut ratatui::buffer::Buffer,
    area: ratatui::layout::Rect,
) {
    manager.process_effects(delta, buf, area);
}
