// <FILE>crates/tui-vfx-style/src/models/cls_color_fade_shader.rs</FILE> - <DESC>Timeline-driven color fade style shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-owned v3.1 style.colorFade lowering needs color interpolation as a style shader instead of backend renderer emulation.</WCTX>
// <CLOG>0.1.0: INIT — add color-fade shader for foreground/background interpolation toward a target color.</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_style_to_color_in_space;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Fades non-transparent foreground and background channels toward a target.
///
/// Progress comes from normalized timeline `phase_t` (`ctx.t`), matching the
/// V2/V3.1 `style.colorFade` recipe surface without player/backend stages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ColorFadeShader {
    /// Target color reached when timeline progress is `1.0`.
    pub target: ColorConfig,
    /// Interpolation color space.
    #[serde(default)]
    pub color_space: ColorSpace,
}

impl Default for ColorFadeShader {
    fn default() -> Self {
        Self {
            target: ColorConfig::Rgb {
                r: 255,
                g: 200,
                b: 50,
            },
            color_space: ColorSpace::Rgb,
        }
    }
}

impl StyleShader for ColorFadeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        blend_style_to_color_in_space(
            base,
            Color::from(self.target),
            (ctx.t as f32).clamp(0.0, 1.0),
            self.color_space,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(t: f64) -> ShaderContext {
        ShaderContext::new(0, 0, 8, 4, 0, 0, t, None, None)
    }

    #[test]
    fn fades_foreground_and_background_toward_target() {
        let shader = ColorFadeShader::default();
        let base = Style {
            fg: Color::WHITE,
            bg: Color::rgb(60, 20, 80),
            ..Style::default()
        };

        let out = shader.style_at(&ctx(0.5), base);

        assert_eq!(out.fg, Color::rgb(255, 227, 152));
        assert_eq!(out.bg, Color::rgb(157, 110, 65));
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_color_fade_shader.rs</FILE> - <DESC>Timeline-driven color fade style shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
