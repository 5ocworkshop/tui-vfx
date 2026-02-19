// <FILE>tui-vfx-style/src/models/cls_glow_shader.rs</FILE> - <DESC>Multi-cell bloom/halo glow effect around widget edges</DESC>
// <VERS>VERSION: 1.0.3</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Fix time-aware ShaderContext helper import</CLOG>

use crate::models::{ColorConfig, FalloffType};
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Glow shader that adds a bloom/halo effect around widget edges.
///
/// Creates an outer glow effect by blending the glow color into cells
/// near the edge, with intensity fading based on distance and falloff curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GlowShader {
    /// The glow color
    pub color: ColorConfig,

    /// Radius of the glow effect in cells (1-5)
    #[serde(default = "default_radius")]
    pub radius: u8,

    /// Falloff curve for intensity over distance
    #[serde(default)]
    pub falloff: FalloffType,

    /// Overall intensity multiplier (0.0 - 1.0)
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Optional pulse speed in Hz (cycles per second). 0 = no pulse.
    #[serde(default)]
    pub pulse_speed: f32,
}

fn default_radius() -> u8 {
    2
}

fn default_intensity() -> f32 {
    0.6
}

impl Default for GlowShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::Cyan,
            radius: default_radius(),
            falloff: FalloffType::default(),
            intensity: default_intensity(),
            pulse_speed: 0.0,
        }
    }
}

impl GlowShader {
    /// Calculate the minimum distance from the cell to any edge.
    fn distance_to_edge(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let w = (width.saturating_sub(1)) as f32;
        let h = (height.saturating_sub(1)) as f32;

        let dist_top = y;
        let dist_bottom = h - y;
        let dist_left = x;
        let dist_right = w - x;

        dist_top.min(dist_bottom).min(dist_left).min(dist_right)
    }

    /// Calculate pulse-modulated intensity based on animation time.
    fn pulsed_intensity(&self, t: f64) -> f32 {
        if self.pulse_speed <= 0.0 {
            return self.intensity;
        }

        // Sinusoidal pulse: intensity varies from 0.5*base to base
        let phase = (t as f32 * self.pulse_speed * std::f32::consts::TAU).sin();
        let pulse_factor = 0.75 + 0.25 * phase; // Range: 0.5 to 1.0
        self.intensity * pulse_factor
    }

    /// Blend glow color into base color with given alpha.
    fn blend_glow(&self, base: Color, glow_color: Color, alpha: f32) -> Color {
        let alpha = alpha.clamp(0.0, 1.0);
        Color::rgb(
            (base.r as f32 * (1.0 - alpha) + glow_color.r as f32 * alpha).round() as u8,
            (base.g as f32 * (1.0 - alpha) + glow_color.g as f32 * alpha).round() as u8,
            (base.b as f32 * (1.0 - alpha) + glow_color.b as f32 * alpha).round() as u8,
        )
    }
}

impl StyleShader for GlowShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.radius == 0 || self.intensity <= 0.0 {
            return base;
        }

        let distance = self.distance_to_edge(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let radius = self.radius as f32;

        // No effect beyond radius
        if distance >= radius {
            return base;
        }

        // Calculate intensity based on falloff and distance
        let falloff_value = self.falloff.apply(distance, radius);
        let pulsed = self.pulsed_intensity(ctx.t);
        let glow_strength = falloff_value * pulsed;

        let glow_color: Color = self.color.into();

        let mut style = base;
        style.fg = self.blend_glow(base.fg, glow_color, glow_strength);
        style.bg = self.blend_glow(base.bg, glow_color, glow_strength);
        style
    }

    fn name(&self) -> &'static str {
        "Glow"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::{make_ctx_at, make_style};

    #[test]
    fn default_values() {
        let shader = GlowShader::default();
        assert_eq!(shader.color, ColorConfig::Cyan);
        assert_eq!(shader.radius, 2);
        assert_eq!(shader.falloff, FalloffType::Quadratic);
        assert_eq!(shader.intensity, 0.6);
        assert_eq!(shader.pulse_speed, 0.0);
    }

    #[test]
    fn center_no_effect_with_small_radius() {
        let shader = GlowShader {
            radius: 2,
            intensity: 1.0,
            ..Default::default()
        };
        // Center of 10x10: distance to all edges is 4-5, beyond radius=2
        let ctx = make_ctx_at(5, 5, 10, 10, 0.0);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn edge_has_glow() {
        let shader = GlowShader {
            color: ColorConfig::Rgb { r: 0, g: 255, b: 0 }, // Green glow
            radius: 2,
            intensity: 1.0,
            falloff: FalloffType::Linear,
            pulse_speed: 0.0,
        };

        // At edge (distance=0), full glow
        let ctx = make_ctx_at(0, 5, 10, 10, 0.0);
        let base = make_style();
        let result = shader.style_at(&ctx, base);

        // At distance=0, falloff=1.0, intensity=1.0, blend fully to green
        assert_eq!(result.fg, Color::rgb(0, 255, 0));
        assert_eq!(result.bg, Color::rgb(0, 255, 0));
    }

    #[test]
    fn partial_glow_at_mid_distance() {
        let shader = GlowShader {
            color: ColorConfig::Rgb { r: 0, g: 0, b: 200 }, // Blue glow
            radius: 2,
            intensity: 1.0,
            falloff: FalloffType::Linear,
            pulse_speed: 0.0,
        };

        // At distance=1 from edge with radius=2, linear falloff = 0.5
        let ctx = make_ctx_at(1, 5, 10, 10, 0.0);
        let base = make_style();
        let result = shader.style_at(&ctx, base);

        // fg: (100, 100, 100) blended with (0, 0, 200) at 0.5
        // r: 100 * 0.5 + 0 * 0.5 = 50
        // g: 100 * 0.5 + 0 * 0.5 = 50
        // b: 100 * 0.5 + 200 * 0.5 = 150
        assert_eq!(result.fg, Color::rgb(50, 50, 150));
    }

    #[test]
    fn zero_radius_no_effect() {
        let shader = GlowShader {
            radius: 0,
            ..Default::default()
        };
        let ctx = make_ctx_at(0, 0, 10, 10, 0.0);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn zero_intensity_no_effect() {
        let shader = GlowShader {
            intensity: 0.0,
            ..Default::default()
        };
        let ctx = make_ctx_at(0, 0, 10, 10, 0.0);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn pulse_modulates_intensity() {
        let shader = GlowShader {
            color: ColorConfig::Rgb { r: 255, g: 0, b: 0 },
            radius: 2,
            intensity: 1.0,
            falloff: FalloffType::Linear,
            pulse_speed: 1.0, // 1 Hz
        };

        // At t=0, pulse should be at mid-level (sin(0) = 0, factor = 0.75)
        let ctx = make_ctx_at(0, 5, 10, 10, 0.0);
        let base = make_style();
        let result_t0 = shader.style_at(&ctx, base);

        // At t=0.25 (quarter cycle), sin(PI/2) = 1, factor = 1.0 (max)
        let ctx = make_ctx_at(0, 5, 10, 10, 0.25);
        let result_t025 = shader.style_at(&ctx, base);

        // At max pulse, glow should be stronger (more red)
        assert!(result_t025.fg.r > result_t0.fg.r);
    }

    #[test]
    fn affects_all_edges() {
        let shader = GlowShader {
            color: ColorConfig::Yellow,
            radius: 2,
            intensity: 0.5,
            ..Default::default()
        };
        let base = make_style();

        // All edge positions should have glow effect
        for (x, y) in [(0, 5), (9, 5), (5, 0), (5, 9)] {
            let ctx = make_ctx_at(x, y, 10, 10, 0.0);
            let result = shader.style_at(&ctx, base);
            assert_ne!(
                result.fg, base.fg,
                "Edge at ({}, {}) should have glow",
                x, y
            );
        }
    }

    #[test]
    fn serde_roundtrip() {
        let shader = GlowShader {
            color: ColorConfig::Magenta,
            radius: 3,
            falloff: FalloffType::Exponential,
            intensity: 0.8,
            pulse_speed: 2.5,
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: GlowShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }

    #[test]
    fn name_is_correct() {
        let shader = GlowShader::default();
        assert_eq!(shader.name(), "Glow");
    }
}

// <FILE>tui-vfx-style/src/models/cls_glow_shader.rs</FILE> - <DESC>Multi-cell bloom/halo glow effect around widget edges</DESC>
// <VERS>END OF VERSION: 1.0.3</VERS>
