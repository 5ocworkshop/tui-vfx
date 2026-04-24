// <FILE>tui-vfx-style/src/models/cls_radial_spiral_shader.rs</FILE> - <DESC>Radial spiral field shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>whoa primitive review — expose wavy spiral screensaver texture through tui-vfx substrate naming and mixed-signals field math.</WCTX>
// <CLOG>0.1.0: add RadialSpiralShader backed by mixed_signals::math::radial_spiral_field with blend-strength and color controls.</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use mixed_signals::math::radial_spiral_field;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Procedural radial spiral density shader.
///
/// This is the substrate-named version of the wavy spiral screensaver pattern:
/// each cell samples a radial/angle field and blends toward `color` by the
/// resulting density. It is useful for portals, loading fields, attention
/// halos, and procedural recipe backgrounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct RadialSpiralShader {
    /// Angular arm count / repetition factor.
    #[config(default = 1.5)]
    pub arms: f32,
    /// Radial ring frequency.
    #[config(default = 12.0)]
    pub radial_frequency: f32,
    /// Radius falloff power.
    #[config(default = 0.6)]
    pub radial_power: f32,
    /// Animation speed multiplier.
    #[config(default = 1.0)]
    pub speed: f32,
    /// Maximum blend strength.
    #[config(default = 0.5)]
    pub blend_strength: f32,
    /// Color blended into non-transparent cells.
    pub color: ColorConfig,
}

impl Default for RadialSpiralShader {
    fn default() -> Self {
        Self {
            arms: 1.5,
            radial_frequency: 12.0,
            radial_power: 0.6,
            speed: 1.0,
            blend_strength: 0.5,
            color: ColorConfig::Cyan,
        }
    }
}

impl RadialSpiralShader {
    fn field_at(&self, ctx: &ShaderContext) -> f32 {
        if ctx.width == 0 || ctx.height == 0 {
            return 0.0;
        }
        let min_dim = ctx.width.min(ctx.height).max(1) as f32;
        let aspect = ctx.width.max(1) as f32 / ctx.height.max(1) as f32;
        let x = (2.0 * (ctx.local_x as f32 - ctx.width as f32 / 2.0) / min_dim) * aspect;
        let y = 2.0 * (ctx.local_y as f32 - ctx.height as f32 / 2.0) / min_dim;
        radial_spiral_field(
            x,
            y,
            ctx.t as f32 * self.speed,
            self.arms,
            self.radial_frequency,
            self.radial_power,
        )
    }
}

impl StyleShader for RadialSpiralShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let factor = (self.field_at(ctx) * self.blend_strength).clamp(0.0, 1.0);
        let color: Color = self.color.into();
        let mut result = base;
        if base.fg != Color::TRANSPARENT {
            result.fg = blend_colors(base.fg, color, factor, ColorSpace::Rgb);
        }
        if base.bg != Color::TRANSPARENT {
            result.bg = blend_colors(base.bg, color, factor * 0.5, ColorSpace::Rgb);
        }
        result
    }

    fn name(&self) -> &'static str {
        "RadialSpiral"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_field_is_bounded() {
        let shader = RadialSpiralShader::default();
        let ctx = ShaderContext::new(4, 3, 12, 8, 0, 0, 0.25, None, None);
        let value = shader.field_at(&ctx);
        assert!((0.0..=1.0).contains(&value));
    }

    #[test]
    fn shader_changes_visible_foreground() {
        let shader = RadialSpiralShader {
            blend_strength: 1.0,
            color: ColorConfig::Red,
            ..Default::default()
        };
        let ctx = ShaderContext::new(4, 3, 12, 8, 0, 0, 0.25, None, None);
        let base = Style::fg(Color::WHITE);
        let styled = shader.style_at(&ctx, base);
        assert_ne!(styled.fg, Color::WHITE);
    }
}

// <FILE>tui-vfx-style/src/models/cls_radial_spiral_shader.rs</FILE> - <DESC>Radial spiral field shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
