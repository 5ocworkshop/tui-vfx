// <FILE>tui-vfx-style/src/models/cls_highlighter_shader.rs</FILE> - <DESC>Highlighter shader implementation</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T11:45:00Z - 2025-12-18T12:24:24Z</VERS>
// <WCTX>New primitive</WCTX>
// <CLOG>Initial implementation</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct HighlighterShader {
    pub color: ColorConfig,
}
impl StyleShader for HighlighterShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let limit = (ctx.width as f64 * ctx.t) as u16;
        let mut style = base;
        if ctx.local_x < limit {
            style.bg = Color::from(self.color);
            // Ensure text remains readable (usually black on yellow/cyan)
            style.fg = Color::BLACK;
        }
        style
    }
}

// <FILE>tui-vfx-style/src/models/cls_highlighter_shader.rs</FILE> - <DESC>Highlighter shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T11:45:00Z - 2025-12-18T12:24:24Z</VERS>
