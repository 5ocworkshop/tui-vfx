// <FILE>tui-vfx-style/src/models/cls_orbit_shader.rs</FILE> - <DESC>Orbiting dot shader implementation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Introduce orbit shader for spatial effects</WCTX>
// <CLOG>Initial OrbitShader implementation</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Orbiting dots around the widget center.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct OrbitShader {
    /// Animation speed (cycles per second).
    #[config(default = 1.0)]
    pub speed: f32,
    /// Number of dots in the orbit.
    #[config(default = 3)]
    pub dot_count: u8,
    /// Color of the orbiting dots.
    pub color: ColorConfig,
}

impl StyleShader for OrbitShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.dot_count == 0 {
            return base;
        }

        let width = ctx.width as f32;
        let height = ctx.height as f32;
        if width <= 1.0 || height <= 1.0 {
            return base;
        }

        let cx = (width - 1.0) / 2.0;
        let cy = (height - 1.0) / 2.0;
        let radius = cx.min(cy);
        if !radius.is_finite() || radius <= 0.0 {
            return base;
        }

        let mut style = base;
        let base_angle = ctx.t as f32 * self.speed * std::f32::consts::TAU;
        let dot_count = self.dot_count as f32;

        for i in 0..self.dot_count {
            let angle = base_angle + (i as f32) * std::f32::consts::TAU / dot_count;
            let x = (cx + radius * angle.cos()).round() as i32;
            let y = (cy + radius * angle.sin()).round() as i32;

            if x == ctx.local_x as i32 && y == ctx.local_y as i32 {
                style.fg = Color::from(self.color);
                break;
            }
        }

        style
    }
}

// <FILE>tui-vfx-style/src/models/cls_orbit_shader.rs</FILE> - <DESC>Orbiting dot shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
