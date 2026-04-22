// <FILE>crates/tui-vfx-style/src/models/fnc_apply_style_effects_to_scene.rs</FILE> - <DESC>Apply non-spatial style effects to a semantic scene</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct V3 playback still needs to apply non-spatial style effects after compositor rendering. Keeping that pass in the style crate moves more runtime ownership inward instead of leaving scene/style mutation loops in recipes bridge code.</WCTX>
// <CLOG>0.1.0: add apply_style_effects_to_scene plus internal progress selection for modulation vs phase-bound effects.</CLOG>

use tui_vfx_types::{Cell, Grid, SemanticScene, Style as VfxStyle};

use crate::models::{StyleEffect, StyleRegion, VfxStyleEffectFamily};
use crate::traits::StyleInterpolator;

/// Apply non-spatial style effects to a rendered semantic scene.
pub fn apply_style_effects_to_scene(
    scene: &mut SemanticScene,
    effects: &[(StyleEffect, StyleRegion)],
    phase_t: f64,
    loop_t: Option<f64>,
) {
    if effects.is_empty() {
        return;
    }

    let area = scene.area();
    for y in 0..scene.grid().height() {
        for x in 0..scene.grid().width() {
            let role = scene.role((x as u16, y as u16));
            let should_apply = effects
                .iter()
                .any(|(_, region)| region.should_style(x as u16, y as u16, role.clone(), area));
            if !should_apply {
                continue;
            }
            if let Some(cell) = scene.grid().get(x, y).copied() {
                let mut style = VfxStyle::new(cell.fg, cell.bg, cell.mods);
                for (effect, region) in effects {
                    if region.should_style(x as u16, y as u16, role.clone(), area) {
                        style = effect.calculate(style_effect_progress(effect, phase_t, loop_t), style);
                    }
                }
                scene.grid_mut().set(
                    x,
                    y,
                    Cell::styled(cell.ch, style.fg, style.bg, style.mods)
                        .with_mod_alpha(cell.mod_alpha),
                );
            }
        }
    }
}

fn style_effect_progress(effect: &StyleEffect, phase_t: f64, loop_t: Option<f64>) -> f64 {
    match effect.v3_effect_family() {
        VfxStyleEffectFamily::StyleModulation | VfxStyleEffectFamily::StyleInstability => {
            loop_t.unwrap_or(phase_t)
        }
        _ => phase_t,
    }
}
