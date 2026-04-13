// <FILE>tui-vfx-style/src/models/cls_edge_sheen_shader.rs</FILE> - <DESC>EdgeSheen shader implementation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Introduce a calmer premium shell sheen distinct from Reflect</WCTX>
// <CLOG>Add EdgeSheen spatial shader with edge width, perimeter band, and corner emphasis</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Which color channel(s) to apply the edge sheen to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum EdgeSheenApplyTo {
    /// Apply to foreground only (default).
    #[default]
    Foreground,
    /// Apply to background only.
    Background,
    /// Apply to both foreground and background.
    Both,
}

/// A premium perimeter sheen that glides across a widget's outer shell.
///
/// Unlike `Reflect`, which behaves like a direct glint passing over all cells,
/// `EdgeSheen` stays constrained to the widget's edges and softly boosts
/// corners. It is designed for shells, cards, toasts, and overlays that need a
/// calmer "finished surface" feeling.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct EdgeSheenShader {
    /// Sheen tint.
    pub color: ColorConfig,

    /// Sweep speed multiplier.
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Band width along the perimeter in cells.
    #[serde(default = "default_band_width")]
    pub band_width: u16,

    /// Thickness of the effect measured inward from the edge.
    #[serde(default = "default_edge_width")]
    pub edge_width: u8,

    /// Blend strength.
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Extra highlight applied near corners.
    #[serde(default = "default_corner_boost")]
    pub corner_boost: f32,

    /// Which channel(s) to affect.
    #[serde(default)]
    pub apply_to: EdgeSheenApplyTo,
}

fn default_speed() -> f32 {
    0.8
}

fn default_band_width() -> u16 {
    10
}

fn default_edge_width() -> u8 {
    2
}

fn default_intensity() -> f32 {
    0.55
}

fn default_corner_boost() -> f32 {
    0.2
}

impl Default for EdgeSheenShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::White,
            speed: default_speed(),
            band_width: default_band_width(),
            edge_width: default_edge_width(),
            intensity: default_intensity(),
            corner_boost: default_corner_boost(),
            apply_to: EdgeSheenApplyTo::Foreground,
        }
    }
}

impl EdgeSheenShader {
    fn distance_to_edge(&self, x: u16, y: u16, width: u16, height: u16) -> u16 {
        let max_x = width.saturating_sub(1);
        let max_y = height.saturating_sub(1);
        x.min(max_x.saturating_sub(x))
            .min(y.min(max_y.saturating_sub(y)))
    }

    fn perimeter_position(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let max_x = width.saturating_sub(1);
        let max_y = height.saturating_sub(1);
        let top = y;
        let right = max_x.saturating_sub(x);
        let bottom = max_y.saturating_sub(y);
        let left = x;

        if top <= right && top <= bottom && top <= left {
            x as f32
        } else if right <= bottom && right <= left {
            max_x as f32 + y as f32
        } else if bottom <= left {
            max_x as f32 + max_y as f32 + (max_x.saturating_sub(x)) as f32
        } else {
            (2 * max_x + max_y) as f32 + (max_y.saturating_sub(y)) as f32
        }
    }

    fn nearest_corner_factor(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let max_x = width.saturating_sub(1) as f32;
        let max_y = height.saturating_sub(1) as f32;
        let x = x as f32;
        let y = y as f32;

        let distances = [
            x + y,
            (max_x - x) + y,
            x + (max_y - y),
            (max_x - x) + (max_y - y),
        ];
        let nearest = distances
            .iter()
            .copied()
            .fold(f32::INFINITY, |acc, value| acc.min(value));
        let window = (self.edge_width as f32 * 3.0).max(1.0);
        1.0 + self.corner_boost * (1.0 - (nearest / window).min(1.0))
    }

    fn blend_target(&self, base: Color, sheen: Color, alpha: f32) -> Color {
        if base == Color::TRANSPARENT {
            sheen
        } else {
            blend_colors(base, sheen, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
        }
    }
}

impl StyleShader for EdgeSheenShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 || self.band_width == 0 || self.edge_width == 0 {
            return base;
        }

        if ctx.width < 2 || ctx.height < 2 {
            return base;
        }

        let edge_distance = self.distance_to_edge(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        if edge_distance >= self.edge_width as u16 {
            return base;
        }

        let perimeter = (2 * (u32::from(ctx.width) + u32::from(ctx.height)) - 4) as f32;
        if perimeter <= 0.0 {
            return base;
        }

        let band_center = (ctx.t as f32 * self.speed).fract() * perimeter;
        let position = self.perimeter_position(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let distance = (band_center - position).abs();
        let wrapped_distance = distance.min(perimeter - distance);

        if wrapped_distance > self.band_width as f32 {
            return base;
        }

        let band_factor = 1.0 - (wrapped_distance / self.band_width as f32);
        let edge_factor = 1.0 - (edge_distance as f32 / self.edge_width as f32);
        let corner_factor =
            self.nearest_corner_factor(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let alpha = (band_factor * edge_factor * self.intensity * corner_factor).clamp(0.0, 1.0);
        let sheen_color: Color = self.color.into();

        let mut style = base;
        match self.apply_to {
            EdgeSheenApplyTo::Foreground => {
                style.fg = self.blend_target(base.fg, sheen_color, alpha);
            }
            EdgeSheenApplyTo::Background => {
                style.bg = self.blend_target(base.bg, sheen_color, alpha * 0.8);
            }
            EdgeSheenApplyTo::Both => {
                style.fg = self.blend_target(base.fg, sheen_color, alpha);
                style.bg = self.blend_target(base.bg, sheen_color, alpha * 0.65);
            }
        }

        style
    }

    fn name(&self) -> &'static str {
        "EdgeSheen"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::{make_ctx_at, make_style};

    #[test]
    fn default_values_are_valid() {
        let shader = EdgeSheenShader::default();
        assert_eq!(shader.speed, 0.8);
        assert_eq!(shader.band_width, 10);
        assert_eq!(shader.edge_width, 2);
        assert_eq!(shader.intensity, 0.55);
        assert_eq!(shader.corner_boost, 0.2);
    }

    #[test]
    fn center_cell_has_no_effect() {
        let shader = EdgeSheenShader::default();
        let ctx = make_ctx_at(5, 5, 12, 12, 0.25);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result, base);
    }

    #[test]
    fn edge_cell_changes_when_band_passes() {
        let shader = EdgeSheenShader {
            color: ColorConfig::Rgb {
                r: 255,
                g: 240,
                b: 180,
            },
            speed: 1.0,
            band_width: 6,
            edge_width: 2,
            intensity: 1.0,
            corner_boost: 0.0,
            apply_to: EdgeSheenApplyTo::Foreground,
        };
        let ctx = make_ctx_at(0, 0, 10, 6, 0.0);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_ne!(result.fg, base.fg);
    }

    #[test]
    fn serde_roundtrip_preserves_values() {
        let shader = EdgeSheenShader {
            color: ColorConfig::Magenta,
            speed: 0.6,
            band_width: 8,
            edge_width: 3,
            intensity: 0.7,
            corner_boost: 0.35,
            apply_to: EdgeSheenApplyTo::Both,
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: EdgeSheenShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }
}

// <FILE>tui-vfx-style/src/models/cls_edge_sheen_shader.rs</FILE> - <DESC>EdgeSheen shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
