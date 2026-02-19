// <FILE>tui-vfx-style/src/models/cls_bevel_shader.rs</FILE> - <DESC>3D bevel edge effect with configurable light direction</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use shared style test helpers</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Light direction for bevel effect.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LightDirection {
    /// Light from top-left (default - standard UI convention)
    #[default]
    TopLeft,
    /// Light from top-right
    TopRight,
    /// Light from bottom-left
    BottomLeft,
    /// Light from bottom-right
    BottomRight,
    /// Light from directly above
    Top,
    /// Light from directly below
    Bottom,
    /// Light from the left
    Left,
    /// Light from the right
    Right,
}

/// Bevel shader that creates 3D raised/embossed edge effects.
///
/// Lightens edges facing the light source and darkens opposite edges,
/// creating the appearance of a raised or sunken surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct BevelShader {
    /// Direction of the simulated light source
    #[serde(default)]
    pub light_direction: LightDirection,

    /// Intensity of the highlight on lit edges (0.0 - 1.0)
    #[serde(default = "default_highlight_intensity")]
    pub highlight_intensity: f32,

    /// Intensity of the shadow on dark edges (0.0 - 1.0)
    #[serde(default = "default_shadow_intensity")]
    pub shadow_intensity: f32,

    /// Width of the bevel edge in cells (1-3)
    #[serde(default = "default_edge_width")]
    pub edge_width: u8,
}

fn default_highlight_intensity() -> f32 {
    0.3
}

fn default_shadow_intensity() -> f32 {
    0.3
}

fn default_edge_width() -> u8 {
    1
}

impl Default for BevelShader {
    fn default() -> Self {
        Self {
            light_direction: LightDirection::default(),
            highlight_intensity: default_highlight_intensity(),
            shadow_intensity: default_shadow_intensity(),
            edge_width: default_edge_width(),
        }
    }
}

impl BevelShader {
    /// Determine which edge zones this cell is in and whether they're lit or shadowed.
    /// Returns (horizontal_factor, vertical_factor) where:
    /// - positive = highlight (facing light)
    /// - negative = shadow (away from light)
    /// - 0 = not on edge / no effect
    fn edge_factors(&self, x: u16, y: u16, width: u16, height: u16) -> (f32, f32) {
        let edge_width = self.edge_width.max(1) as f32;
        let x = x as f32;
        let y = y as f32;
        let w = width.saturating_sub(1) as f32;
        let h = height.saturating_sub(1) as f32;

        // Calculate distance from each edge, normalized by edge_width
        let dist_left = x;
        let dist_right = w - x;
        let dist_top = y;
        let dist_bottom = h - y;

        // Horizontal factor: negative near left edge, positive near right edge
        let h_factor = if dist_left < edge_width {
            -(1.0 - dist_left / edge_width)
        } else if dist_right < edge_width {
            1.0 - dist_right / edge_width
        } else {
            0.0
        };

        // Vertical factor: negative near top edge, positive near bottom edge
        let v_factor = if dist_top < edge_width {
            -(1.0 - dist_top / edge_width)
        } else if dist_bottom < edge_width {
            1.0 - dist_bottom / edge_width
        } else {
            0.0
        };

        (h_factor, v_factor)
    }

    /// Calculate the final lighting factor based on light direction.
    /// Returns a value in [-1, 1] where:
    /// - positive = brighten (highlight)
    /// - negative = darken (shadow)
    fn lighting_factor(&self, h_factor: f32, v_factor: f32) -> f32 {
        // Map light direction to which edges should be highlighted vs shadowed
        // h_factor: negative=left edge, positive=right edge
        // v_factor: negative=top edge, positive=bottom edge
        match self.light_direction {
            LightDirection::TopLeft => {
                // Light from top-left: top and left edges are lit, bottom and right are shadowed
                // Left edge (h<0) = lit (+), Right edge (h>0) = shadow (-)
                // Top edge (v<0) = lit (+), Bottom edge (v>0) = shadow (-)
                -h_factor + (-v_factor)
            }
            LightDirection::TopRight => {
                // Top and right are lit
                h_factor + (-v_factor)
            }
            LightDirection::BottomLeft => {
                // Bottom and left are lit
                -h_factor + v_factor
            }
            LightDirection::BottomRight => {
                // Bottom and right are lit
                h_factor + v_factor
            }
            LightDirection::Top => {
                // Only top is lit
                -v_factor
            }
            LightDirection::Bottom => {
                // Only bottom is lit
                v_factor
            }
            LightDirection::Left => {
                // Only left is lit
                -h_factor
            }
            LightDirection::Right => {
                // Only right is lit
                h_factor
            }
        }
    }

    /// Brighten a color by a factor (0.0 - 1.0).
    fn brighten_color(&self, color: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        // Brighten by interpolating toward white
        Color::rgb(
            (color.r as f32 + (255.0 - color.r as f32) * factor).round() as u8,
            (color.g as f32 + (255.0 - color.g as f32) * factor).round() as u8,
            (color.b as f32 + (255.0 - color.b as f32) * factor).round() as u8,
        )
    }

    /// Darken a color by a factor (0.0 - 1.0).
    fn darken_color(&self, color: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color::rgb(
            (color.r as f32 * (1.0 - factor)).round() as u8,
            (color.g as f32 * (1.0 - factor)).round() as u8,
            (color.b as f32 * (1.0 - factor)).round() as u8,
        )
    }
}

impl StyleShader for BevelShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if ctx.width < 2 || ctx.height < 2 {
            return base;
        }

        let (h_factor, v_factor) =
            self.edge_factors(ctx.local_x, ctx.local_y, ctx.width, ctx.height);

        // Not on any edge
        if h_factor == 0.0 && v_factor == 0.0 {
            return base;
        }

        let lighting = self.lighting_factor(h_factor, v_factor);

        let mut style = base;
        if lighting > 0.0 {
            // Highlight
            let strength = lighting.abs().min(1.0) * self.highlight_intensity;
            style.fg = self.brighten_color(base.fg, strength);
            style.bg = self.brighten_color(base.bg, strength);
        } else if lighting < 0.0 {
            // Shadow
            let strength = lighting.abs().min(1.0) * self.shadow_intensity;
            style.fg = self.darken_color(base.fg, strength);
            style.bg = self.darken_color(base.bg, strength);
        }

        style
    }

    fn name(&self) -> &'static str {
        "Bevel"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::{make_ctx, make_style};

    #[test]
    fn default_values() {
        let shader = BevelShader::default();
        assert_eq!(shader.light_direction, LightDirection::TopLeft);
        assert_eq!(shader.highlight_intensity, 0.3);
        assert_eq!(shader.shadow_intensity, 0.3);
        assert_eq!(shader.edge_width, 1);
    }

    #[test]
    fn center_no_effect() {
        let shader = BevelShader::default();
        let ctx = make_ctx(5, 5, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
        assert_eq!(result.bg, base.bg);
    }

    #[test]
    fn top_left_light_brightens_top_and_left() {
        let shader = BevelShader {
            light_direction: LightDirection::TopLeft,
            highlight_intensity: 1.0,
            shadow_intensity: 1.0,
            edge_width: 1,
        };

        // Top-left corner: should be highlighted
        let ctx = make_ctx(0, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        // Highlight brightens toward white
        assert!(result.fg.r > base.fg.r);
        assert!(result.fg.g > base.fg.g);
        assert!(result.fg.b > base.fg.b);
    }

    #[test]
    fn top_left_light_darkens_bottom_and_right() {
        let shader = BevelShader {
            light_direction: LightDirection::TopLeft,
            highlight_intensity: 1.0,
            shadow_intensity: 1.0,
            edge_width: 1,
        };

        // Bottom-right corner: should be shadowed
        let ctx = make_ctx(9, 9, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        // Shadow darkens
        assert!(result.fg.r < base.fg.r);
        assert!(result.fg.g < base.fg.g);
        assert!(result.fg.b < base.fg.b);
    }

    #[test]
    fn bottom_right_light_reverses_effect() {
        let shader = BevelShader {
            light_direction: LightDirection::BottomRight,
            highlight_intensity: 1.0,
            shadow_intensity: 1.0,
            edge_width: 1,
        };

        // Top-left corner: should be shadowed (opposite of TopLeft light)
        let ctx = make_ctx(0, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert!(result.fg.r < base.fg.r);

        // Bottom-right corner: should be highlighted
        let ctx = make_ctx(9, 9, 10, 10);
        let result = shader.style_at(&ctx, base);
        assert!(result.fg.r > base.fg.r);
    }

    #[test]
    fn wider_edge_affects_more_cells() {
        let shader = BevelShader {
            edge_width: 3,
            highlight_intensity: 0.5,
            ..Default::default()
        };

        // Cell at x=2 should still be affected with edge_width=3
        let ctx = make_ctx(2, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_ne!(result.fg, base.fg);

        // Cell at x=4 should not be affected
        let ctx = make_ctx(4, 5, 10, 10);
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn small_widget_returns_base() {
        let shader = BevelShader::default();
        // 1x1 widget is too small
        let ctx = make_ctx(0, 0, 1, 1);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn directional_light_top() {
        let shader = BevelShader {
            light_direction: LightDirection::Top,
            highlight_intensity: 1.0,
            shadow_intensity: 1.0,
            edge_width: 1,
        };

        // Top edge: lit
        let ctx = make_ctx(5, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert!(result.fg.r > base.fg.r);

        // Bottom edge: shadowed
        let ctx = make_ctx(5, 9, 10, 10);
        let result = shader.style_at(&ctx, base);
        assert!(result.fg.r < base.fg.r);

        // Left edge (middle): no effect from purely top light
        let ctx = make_ctx(0, 5, 10, 10);
        let result = shader.style_at(&ctx, base);
        // Should be unchanged since only vertical component matters for Top light
        assert_eq!(result.fg.r, base.fg.r);
    }

    #[test]
    fn serde_roundtrip() {
        let shader = BevelShader {
            light_direction: LightDirection::BottomLeft,
            highlight_intensity: 0.5,
            shadow_intensity: 0.7,
            edge_width: 2,
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: BevelShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }

    #[test]
    fn name_is_correct() {
        let shader = BevelShader::default();
        assert_eq!(shader.name(), "Bevel");
    }
}

// <FILE>tui-vfx-style/src/models/cls_bevel_shader.rs</FILE> - <DESC>3D bevel edge effect with configurable light direction</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
