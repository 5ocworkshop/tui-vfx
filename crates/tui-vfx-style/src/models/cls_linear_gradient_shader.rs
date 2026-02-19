// <FILE>tui-vfx-style/src/models/cls_linear_gradient_shader.rs</FILE> - <DESC>Concrete implementation of StyleShader</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-18T10:16:09Z</VERS>
// <WCTX>Fixing ConfigSchema gap for external recipes</WCTX>
// <CLOG>Derived ConfigSchema</CLOG>

use crate::models::Gradient;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct LinearGradientShader {
    pub gradient: Gradient,
    pub angle_deg: f32,
}
impl LinearGradientShader {
    pub fn new(gradient: Gradient) -> Self {
        Self {
            gradient,
            angle_deg: 0.0,
        }
    }
    pub fn vertical(gradient: Gradient) -> Self {
        Self {
            gradient,
            angle_deg: 90.0,
        }
    }
}
impl StyleShader for LinearGradientShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let u = if ctx.width > 1 {
            ctx.local_x as f32 / (ctx.width - 1) as f32
        } else {
            0.0
        };
        let v = if ctx.height > 1 {
            ctx.local_y as f32 / (ctx.height - 1) as f32
        } else {
            0.0
        };
        let sample_t = if self.angle_deg.abs() < 45.0 { u } else { v };
        let color = self.gradient.sample(sample_t);
        let mut result = base;
        result.fg = color;
        result
    }
}

// <FILE>tui-vfx-style/src/models/cls_linear_gradient_shader.rs</FILE> - <DESC>Concrete implementation of StyleShader</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-18T10:16:09Z</VERS>
