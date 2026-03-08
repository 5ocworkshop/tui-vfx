// <FILE>tui-vfx-style/src/models/cls_reflect_shader.rs</FILE> - <DESC>Reflect (sheen) shader implementation</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Fix shader speed bug — speed field was truncating sweep range</WCTX>
// <CLOG>Remove self.speed from positional computation; caller controls sweep rate via loop_t</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ReflectShader {
    #[config(default = 2.0)]
    pub speed: f32,
    pub color: ColorConfig, // The color of the "glint"
}
impl StyleShader for ReflectShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        // A band that moves across.
        // position = (time * speed) % (width + gap)
        let gap = 20.0_f64;
        let cycle_width = ctx.width as f64 + gap;
        let pos = (ctx.t * cycle_width) % cycle_width;
        let dist = (ctx.local_x as f64 - pos).abs();
        let mut style = base;
        // Band width ~ 2.0
        if dist < 2.0 {
            // Apply glint
            style.fg = Color::from(self.color);
            // Optional: make it bold
            // style.add_modifier(Modifier::BOLD);
        }
        style
    }
}

// <FILE>tui-vfx-style/src/models/cls_reflect_shader.rs</FILE> - <DESC>Reflect (sheen) shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
