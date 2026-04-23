// <FILE>tui-vfx-style/src/models/cls_radar_shader.rs</FILE> - <DESC>Radar sweep shader implementation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Refactor representative polar field math onto the shared mixed-signals spatial coordinate substrate now that angle sampling is available there.</WCTX>
// <CLOG>1.1.0: use mixed-signals `sample_angle` for the per-cell polar angle instead of open-coding local atan2 math inside the shader.
// 1.0.1: Remove self.speed from positional computation; caller controls sweep rate via loop_t</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use mixed_signals::prelude::{Signal, SignalContext, SpatialCoordinateSignal};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct RadarShader {
    #[config(default = 1.0)]
    pub speed: f32,
    #[config(default = 1.0)]
    pub tail_length: f32, // Radians of tail
    pub color: ColorConfig,
}
impl StyleShader for RadarShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;

        let signal_ctx = SignalContext::new(0, 0)
            .with_dimensions(ctx.width, ctx.height)
            .with_cell_position(ctx.local_x, ctx.local_y);
        let angle = SpatialCoordinateSignal::sample_angle().sample_with_context(0.0, &signal_ctx);
        // Current sweep angle
        let sweep = (t * std::f32::consts::TAU).rem_euclid(std::f32::consts::TAU);
        // Difference
        let diff = (sweep - angle).rem_euclid(std::f32::consts::TAU);
        let tail_length = if self.tail_length.is_finite() && self.tail_length > 0.0 {
            self.tail_length
        } else {
            return base;
        };
        let mut style = base;
        if diff < tail_length {
            // Inside the sweep tail. Intensity fades as diff increases.
            let intensity = 1.0 - (diff / tail_length);
            if intensity > 0.0 {
                // We assume base bg is dark, so we set FG to radar color
                // Ideally we'd blend, but setting FG works for text-based radar
                style.fg = Color::from(self.color);
                // If intensity is low, maybe dim?
                // For simplicity in v1, just set color if in tail.
            }
        }
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radar_shader_distinguishes_cells_by_angle() {
        let shader = RadarShader {
            speed: 1.0,
            tail_length: 0.5,
            color: ColorConfig::Red,
        };
        let base = Style {
            fg: Color::rgb(20, 20, 20),
            bg: Color::rgb(0, 0, 0),
            mods: Default::default(),
        };

        let lit_ctx = ShaderContext::new(4, 2, 9, 5, 0, 0, 0.0, None, None);
        let dark_ctx = ShaderContext::new(0, 2, 9, 5, 0, 0, 0.0, None, None);

        let lit = shader.style_at(&lit_ctx, base);
        let dark = shader.style_at(&dark_ctx, base);

        assert_ne!(lit, dark);
        assert_eq!(lit.fg, Color::from(shader.color));
        assert_eq!(dark.fg, base.fg);
    }
}

// <FILE>tui-vfx-style/src/models/cls_radar_shader.rs</FILE> - <DESC>Radar sweep shader implementation</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
