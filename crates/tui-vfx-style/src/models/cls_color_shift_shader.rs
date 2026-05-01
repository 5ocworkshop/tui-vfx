// <FILE>crates/tui-vfx-style/src/models/cls_color_shift_shader.rs</FILE> - <DESC>Timeline-driven HSL color shift style shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-owned v3.1 style.colorShift lowering needs HSL channel shifts as a style shader instead of backend renderer emulation.</WCTX>
// <CLOG>0.1.0: INIT — add color-shift shader for foreground/background HSL adjustment.</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use crate::utils::shift_style_hsl;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;

/// Shifts non-transparent foreground and background channels in HSL space.
///
/// Authored shifts are multiplied by normalized timeline `phase_t` (`ctx.t`),
/// matching the V2/V3.1 `style.colorShift` recipe surface without backend
/// renderer stages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ColorShiftShader {
    /// Hue shift in degrees applied at timeline progress `1.0`.
    #[config(default = 0.0)]
    pub hue_shift: f32,
    /// Saturation delta applied at timeline progress `1.0`.
    #[config(default = 0.0)]
    pub saturation_shift: f32,
    /// Lightness delta applied at timeline progress `1.0`.
    #[config(default = 0.0)]
    pub lightness_shift: f32,
}

impl Default for ColorShiftShader {
    fn default() -> Self {
        Self {
            hue_shift: 0.0,
            saturation_shift: 0.0,
            lightness_shift: 0.0,
        }
    }
}

impl StyleShader for ColorShiftShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let progress = (ctx.t as f32).clamp(0.0, 1.0);
        shift_style_hsl(
            base,
            self.hue_shift * progress,
            self.saturation_shift * progress,
            self.lightness_shift * progress,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Color;

    fn ctx(t: f64) -> ShaderContext {
        ShaderContext::new(0, 0, 8, 4, 0, 0, t, None, None)
    }

    #[test]
    fn shifts_foreground_and_background_by_timeline_progress() {
        let shader = ColorShiftShader {
            hue_shift: 90.0,
            saturation_shift: 0.2,
            lightness_shift: 0.1,
        };
        let base = Style {
            fg: Color::rgb(0, 205, 205),
            bg: Color::rgb(20, 60, 80),
            ..Style::default()
        };

        let out = shader.style_at(&ctx(0.5), base);

        assert_eq!(out.fg, Color::rgb(0, 57, 230));
        assert_eq!(out.bg, Color::rgb(26, 18, 106));
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_color_shift_shader.rs</FILE> - <DESC>Timeline-driven HSL color shift style shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
