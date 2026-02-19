// <FILE>tui-vfx-style/src/models/cls_barber_pole_shader.rs</FILE> - <DESC>BarberPole shader implementation</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T11:45:00Z - 2025-12-18T12:24:24Z</VERS>
// <WCTX>New primitive</WCTX>
// <CLOG>Initial implementation</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct BarberPoleShader {
    #[config(default = 1)]
    pub speed: f32,
    #[config(default = 2)]
    pub stripe_width: u16,
    #[config(default = 2)]
    pub gap_width: u16,
    pub color: ColorConfig,
}
impl StyleShader for BarberPoleShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let cycle_len = self.stripe_width + self.gap_width;
        if cycle_len == 0 {
            return base;
        }

        let t = ctx.t as f32;

        // Diagonal movement: (x + y)
        // Time offset: t * speed * 10.0 (arbitrary scaling for feel)
        let offset = t * self.speed * 10.0;
        let pos = (ctx.local_x as f32 + ctx.local_y as f32 + offset) % cycle_len as f32;
        let mut style = base;
        if pos < self.stripe_width as f32 {
            // In stripe
            style.bg = Color::from(self.color);
        }
        style
    }
}

// <FILE>tui-vfx-style/src/models/cls_barber_pole_shader.rs</FILE> - <DESC>BarberPole shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T11:45:00Z - 2025-12-18T12:24:24Z</VERS>
