// <FILE>tui-vfx-style/src/models/cls_ambient_occlusion_shader.rs</FILE> - <DESC>Contact shadow/AO effect darkening cells near widget edges</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use shared style test helpers</CLOG>

use crate::models::{ColorConfig, FalloffType};
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Which edges to apply ambient occlusion to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AOEdges {
    /// Bottom and right edges (default - simulates light from top-left)
    #[default]
    BottomRight,
    /// Top and left edges
    TopLeft,
    /// All four edges
    All,
    /// Inner contact shadow (from all edges toward center)
    Inner,
}

/// Ambient occlusion shader that darkens cells near widget edges.
///
/// Creates a subtle "contact shadow" effect that adds depth and makes
/// widgets appear to sit on a surface rather than float.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct AmbientOcclusionShader {
    /// Maximum darkening intensity (0.0 = no effect, 0.5 = 50% darker at edge)
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Effect radius in cells from edge
    #[serde(default = "default_radius")]
    pub radius: u8,

    /// Which edges to apply the effect to
    #[serde(default)]
    pub edges: AOEdges,

    /// Falloff curve type
    #[serde(default)]
    pub falloff: FalloffType,

    /// Shadow color (mixed with base color). Default is black.
    #[serde(default = "default_shadow_color")]
    pub shadow_color: ColorConfig,
}

fn default_intensity() -> f32 {
    0.3
}

fn default_radius() -> u8 {
    2
}

fn default_shadow_color() -> ColorConfig {
    ColorConfig::Black
}

impl Default for AmbientOcclusionShader {
    fn default() -> Self {
        Self {
            intensity: default_intensity(),
            radius: default_radius(),
            edges: AOEdges::default(),
            falloff: FalloffType::default(),
            shadow_color: default_shadow_color(),
        }
    }
}

impl AmbientOcclusionShader {
    /// Calculate the minimum distance from the cell to any active edge.
    fn distance_to_active_edge(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let w = (width.saturating_sub(1)) as f32;
        let h = (height.saturating_sub(1)) as f32;

        match self.edges {
            AOEdges::BottomRight => {
                // Distance to bottom edge or right edge
                let dist_bottom = h - y;
                let dist_right = w - x;
                dist_bottom.min(dist_right)
            }
            AOEdges::TopLeft => {
                // Distance to top edge or left edge
                let dist_top = y;
                let dist_left = x;
                dist_top.min(dist_left)
            }
            AOEdges::All => {
                // Distance to nearest edge
                let dist_top = y;
                let dist_bottom = h - y;
                let dist_left = x;
                let dist_right = w - x;
                dist_top.min(dist_bottom).min(dist_left).min(dist_right)
            }
            AOEdges::Inner => {
                // Same as All - shadows from all edges inward
                let dist_top = y;
                let dist_bottom = h - y;
                let dist_left = x;
                let dist_right = w - x;
                dist_top.min(dist_bottom).min(dist_left).min(dist_right)
            }
        }
    }

    /// Mix a color toward the shadow color by a given factor.
    fn darken_color(&self, color: Color, factor: f32) -> Color {
        let shadow: Color = self.shadow_color.into();
        let factor = factor.clamp(0.0, 1.0);
        Color::rgb(
            ((color.r as f32) * (1.0 - factor) + (shadow.r as f32) * factor).round() as u8,
            ((color.g as f32) * (1.0 - factor) + (shadow.g as f32) * factor).round() as u8,
            ((color.b as f32) * (1.0 - factor) + (shadow.b as f32) * factor).round() as u8,
        )
    }
}

impl StyleShader for AmbientOcclusionShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.radius == 0 || self.intensity <= 0.0 {
            return base;
        }

        let distance =
            self.distance_to_active_edge(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let radius = self.radius as f32;

        // No effect if beyond radius
        if distance >= radius {
            return base;
        }

        // Apply falloff: at distance=0, falloff returns 1.0 (full effect)
        // This is what we want: maximum shadow at edges
        let falloff_value = self.falloff.apply(distance, radius);
        let shadow_strength = falloff_value * self.intensity;

        let mut style = base;
        style.fg = self.darken_color(base.fg, shadow_strength);
        style.bg = self.darken_color(base.bg, shadow_strength);
        style
    }

    fn name(&self) -> &'static str {
        "AmbientOcclusion"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::{make_ctx, make_style};

    #[test]
    fn default_values() {
        let shader = AmbientOcclusionShader::default();
        assert_eq!(shader.intensity, 0.3);
        assert_eq!(shader.radius, 2);
        assert_eq!(shader.edges, AOEdges::BottomRight);
        assert_eq!(shader.falloff, FalloffType::Quadratic);
    }

    #[test]
    fn zero_intensity_no_change() {
        let shader = AmbientOcclusionShader {
            intensity: 0.0,
            ..Default::default()
        };
        let ctx = make_ctx(5, 5, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
        assert_eq!(result.bg, base.bg);
    }

    #[test]
    fn zero_radius_no_change() {
        let shader = AmbientOcclusionShader {
            radius: 0,
            ..Default::default()
        };
        let ctx = make_ctx(5, 5, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn center_no_effect_with_small_radius() {
        let shader = AmbientOcclusionShader {
            radius: 2,
            intensity: 0.5,
            edges: AOEdges::All,
            ..Default::default()
        };
        // Center of 10x10: distance to all edges is 4-5, beyond radius=2
        let ctx = make_ctx(5, 5, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn bottom_right_edge_darkened() {
        let shader = AmbientOcclusionShader {
            radius: 2,
            intensity: 1.0, // Full darkening for visibility
            edges: AOEdges::BottomRight,
            falloff: FalloffType::Linear,
            shadow_color: ColorConfig::Black,
        };
        // Bottom-right corner: should be fully darkened (distance=0)
        let ctx = make_ctx(9, 9, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        // At distance 0, shadow_strength = 1.0 * intensity = 1.0
        // Color should be black
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
        assert_eq!(result.bg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn top_left_unaffected_by_bottom_right_mode() {
        let shader = AmbientOcclusionShader {
            radius: 3,
            intensity: 1.0,
            edges: AOEdges::BottomRight,
            ..Default::default()
        };
        // Top-left corner: distance to bottom and right is large
        let ctx = make_ctx(0, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        // Distance to bottom = 9, distance to right = 9, both > radius
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn top_left_edge_darkened_in_top_left_mode() {
        let shader = AmbientOcclusionShader {
            radius: 2,
            intensity: 1.0,
            edges: AOEdges::TopLeft,
            falloff: FalloffType::Linear,
            shadow_color: ColorConfig::Black,
        };
        // Top-left corner
        let ctx = make_ctx(0, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn all_edges_affects_any_edge() {
        let shader = AmbientOcclusionShader {
            radius: 2,
            intensity: 1.0,
            edges: AOEdges::All,
            falloff: FalloffType::Linear,
            shadow_color: ColorConfig::Black,
        };

        // Top edge
        let ctx = make_ctx(5, 0, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));

        // Bottom edge
        let ctx = make_ctx(5, 9, 10, 10);
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn falloff_affects_gradient() {
        // Linear: at distance 1 with radius 2, falloff = 0.5, shadow = 0.5 * intensity
        let linear = AmbientOcclusionShader {
            radius: 2,
            intensity: 1.0,
            edges: AOEdges::BottomRight,
            falloff: FalloffType::Linear,
            shadow_color: ColorConfig::Black,
        };

        // One cell from bottom-right corner: (8,8) in 10x10
        // dist_right = 9 - 8 = 1, dist_bottom = 9 - 8 = 1, min = 1
        let ctx = make_ctx(8, 8, 10, 10);
        let base = make_style();
        let result = linear.style_at(&ctx, base);
        // Linear falloff at d=1, r=2: 1 - 0.5 = 0.5
        // shadow_strength = 0.5 * 1.0 = 0.5
        // fg: 100 * (1 - 0.5) + 0 * 0.5 = 50
        assert_eq!(result.fg, Color::rgb(50, 50, 50));
    }

    #[test]
    fn custom_shadow_color() {
        let shader = AmbientOcclusionShader {
            radius: 2,
            intensity: 1.0,
            edges: AOEdges::BottomRight,
            falloff: FalloffType::Linear,
            shadow_color: ColorConfig::Rgb { r: 50, g: 0, b: 0 }, // Dark red
        };
        // At edge (distance=0), full shadow
        let ctx = make_ctx(9, 9, 10, 10);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        // Should blend to dark red
        assert_eq!(result.fg, Color::rgb(50, 0, 0));
    }

    #[test]
    fn serde_roundtrip() {
        let shader = AmbientOcclusionShader {
            intensity: 0.4,
            radius: 3,
            edges: AOEdges::All,
            falloff: FalloffType::Exponential,
            shadow_color: ColorConfig::Gray,
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: AmbientOcclusionShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }

    #[test]
    fn name_is_correct() {
        let shader = AmbientOcclusionShader::default();
        assert_eq!(shader.name(), "AmbientOcclusion");
    }
}

// <FILE>tui-vfx-style/src/models/cls_ambient_occlusion_shader.rs</FILE> - <DESC>Contact shadow/AO effect darkening cells near widget edges</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
