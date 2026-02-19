// <FILE>tui-vfx-style/src/models/cls_radar_shader.rs</FILE> - <DESC>Radar sweep shader implementation</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T11:45:00Z - 2025-12-18T12:24:24Z</VERS>
// <WCTX>New primitive</WCTX>
// <CLOG>Initial implementation</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
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

        let cx = ctx.width as f32 / 2.0;
        let cy = ctx.height as f32 / 2.0;
        let dx = ctx.local_x as f32 - cx;
        let dy = ctx.local_y as f32 - cy;
        // Angle in 0..2PI
        let angle = dy.atan2(dx).rem_euclid(std::f32::consts::TAU);
        // Current sweep angle
        let sweep = (t * self.speed * std::f32::consts::TAU).rem_euclid(std::f32::consts::TAU);
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

// <FILE>tui-vfx-style/src/models/cls_radar_shader.rs</FILE> - <DESC>Radar sweep shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T11:45:00Z - 2025-12-18T12:24:24Z</VERS>
