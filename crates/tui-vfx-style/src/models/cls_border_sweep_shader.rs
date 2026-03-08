// <FILE>tui-vfx-style/src/models/cls_border_sweep_shader.rs</FILE> - <DESC>Border sweep shader implementation</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Fix shader speed bug — speed field was truncating sweep range</WCTX>
// <CLOG>Remove self.speed from positional computation; caller controls sweep rate via loop_t</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct BorderSweepShader {
    #[config(default = 1.0)]
    pub speed: f32,
    #[config(default = 5)]
    pub length: u16,
    pub color: ColorConfig,
}
impl StyleShader for BorderSweepShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let (x, y, width, height) = (ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        if width < 2 || height < 2 {
            return base;
        }

        let t = ctx.t as f32;

        // Only affects border cells
        if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
            return base;
        }
        // Unwrap perimeter: Top -> Right -> Bottom -> Left
        let perimeter = 2_u32 * (u32::from(width) + u32::from(height)) - 4; // Approx
        let pos = if y == 0 {
            u32::from(x) // Top edge
        } else if x == width - 1 {
            u32::from(width) + u32::from(y) // Right edge
        } else if y == height - 1 {
            u32::from(width) + u32::from(height) + (u32::from(width - 1 - x))
        } else {
            u32::from(width) + u32::from(height) + u32::from(width) + (u32::from(height - 1 - y))
        } as f32;
        let sweep_pos = (t * perimeter as f32) % perimeter as f32;
        let dist = (sweep_pos - pos)
            .abs()
            .min(perimeter as f32 - (sweep_pos - pos).abs());
        let mut style = base;
        if dist < self.length as f32 {
            style.fg = Color::from(self.color);
        }
        style
    }
}

// <FILE>tui-vfx-style/src/models/cls_border_sweep_shader.rs</FILE> - <DESC>Border sweep shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
