//! Reflective traveling-band / sweep shader.
//!
//! ## V3 family note
//!
//! `ReflectShader` belongs to the broader traveling-band / sweep family
//! identified in the V3 capability catalog and style-model restructure
//! inventory. It is currently implemented as a direct flat variant, but it is
//! a likely candidate for future convergence with sibling sweep-style shaders
//! such as `BorderSweep`, `GlistenBand`, `TracePropagation`, and `TracePath`.
//!
// <FILE>tui-vfx-style/src/models/cls_reflect_shader.rs</FILE> - <DESC>Reflect (sheen) shader implementation</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Reflect shader remains the executable legacy target for V3 reflect traveling-band lowering while the wider family migration is in progress.</WCTX>
// <CLOG>Add optional head/tail colors so V3 reflect head_tail lowering stays lossless while solid-color reflect behavior remains unchanged.</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ReflectShader {
    /// Legacy playback speed scalar retained for compatibility.
    #[config(default = 2.0)]
    pub speed: f32,
    /// Solid-color fallback used when `head` / `tail` are absent.
    pub color: ColorConfig,
    /// Optional leading-edge color for V3 `head_tail` traveling-band lowering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head: Option<ColorConfig>,
    /// Optional trailing-tail color for V3 `head_tail` traveling-band lowering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<ColorConfig>,
    /// Horizontal gap appended to the active width when looping.
    #[serde(default = "default_gap")]
    #[config(default = 20.0)]
    pub gap: f32,
    /// Effective glint width in cells.
    #[serde(default = "default_width")]
    #[config(default = 2.0)]
    pub width: f32,
}

fn default_gap() -> f32 {
    20.0
}

fn default_width() -> f32 {
    2.0
}

impl Default for ReflectShader {
    fn default() -> Self {
        Self {
            speed: 2.0,
            color: ColorConfig::White,
            head: None,
            tail: None,
            gap: default_gap(),
            width: default_width(),
        }
    }
}

impl ReflectShader {
    fn has_head_tail_policy(&self) -> bool {
        self.head.is_some() || self.tail.is_some()
    }

    fn head_color(&self) -> &ColorConfig {
        self.head.as_ref().unwrap_or(&self.color)
    }

    fn tail_color(&self) -> &ColorConfig {
        self.tail.as_ref().unwrap_or(&self.color)
    }

    fn band_color(&self, intensity: f32) -> Color {
        blend_colors(
            Color::from(self.tail_color().clone()),
            Color::from(self.head_color().clone()),
            intensity.clamp(0.0, 1.0),
            ColorSpace::Rgb,
        )
    }
}

impl StyleShader for ReflectShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        // A band that moves across.
        // position = time % (width + gap); caller controls sweep rate via loop_t.
        let gap = f64::from(self.gap.max(0.0));
        let cycle_width = ctx.width as f64 + gap;
        let pos = (ctx.t * cycle_width) % cycle_width;
        let mut style = base;
        let width = f64::from(self.width.max(0.0));
        if self.has_head_tail_policy() {
            let behind_head = pos - ctx.local_x as f64;
            if behind_head >= 0.0 && behind_head < width {
                let intensity = 1.0 - (behind_head as f32 / self.width.max(0.001));
                style.fg = self.band_color(intensity);
            }
        } else {
            let dist = (ctx.local_x as f64 - pos).abs();
            if dist < width {
                style.fg = Color::from(self.color.clone());
            }
        }
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(local_x: u16, t: f64) -> ShaderContext {
        ShaderContext::new(local_x, 0, 10, 1, 0, 0, t, None, None)
    }

    #[test]
    fn width_controls_glint_extent() {
        let base = Style::default();
        let narrow = ReflectShader {
            width: 1.0,
            ..ReflectShader::default()
        };
        let wide = ReflectShader {
            width: 3.0,
            ..ReflectShader::default()
        };

        assert_eq!(narrow.style_at(&ctx_at(2, 0.0), base).fg, base.fg);
        assert_eq!(
            wide.style_at(&ctx_at(2, 0.0), base).fg,
            Color::from(ColorConfig::White)
        );
    }

    #[test]
    fn gap_controls_loop_distance() {
        let base = Style::default();
        let default_gap = ReflectShader::default();
        let no_gap = ReflectShader {
            gap: 0.0,
            ..ReflectShader::default()
        };

        assert_eq!(default_gap.style_at(&ctx_at(6, 0.5), base).fg, base.fg);
        assert_eq!(
            no_gap.style_at(&ctx_at(6, 0.5), base).fg,
            Color::from(ColorConfig::White)
        );
    }

    #[test]
    fn head_tail_policy_colors_head_and_trailing_cells() {
        let shader = ReflectShader {
            color: ColorConfig::Red,
            head: Some(ColorConfig::White),
            tail: Some(ColorConfig::Black),
            gap: 0.0,
            width: 3.0,
            ..ReflectShader::default()
        };
        let base = Style::default();

        assert_eq!(shader.style_at(&ctx_at(2, 0.2), base).fg, Color::WHITE);
        assert_eq!(
            shader.style_at(&ctx_at(1, 0.2), base).fg,
            Color::rgb(169, 169, 169)
        );
    }
}

// <FILE>tui-vfx-style/src/models/cls_reflect_shader.rs</FILE> - <DESC>Reflect (sheen) shader implementation</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
