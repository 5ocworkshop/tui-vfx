// <FILE>tui-vfx-style/src/models/cls_sub_cell_shake_shader.rs</FILE> - <DESC>Micro-jitter visual effect through rapid color oscillation</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use time-aware ShaderContext helper</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Axis for the shake effect.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ShakeAxis {
    /// Horizontal shake only (affects x-based calculations)
    Horizontal,
    /// Vertical shake only (affects y-based calculations)
    Vertical,
    /// Both axes (default)
    #[default]
    Both,
}

/// Sub-cell shake shader that creates a micro-jitter visual effect.
///
/// Creates a vibrating/shaking appearance through rapid color oscillations
/// and optional chromatic aberration. Works by modulating brightness and
/// color channels based on position, time, and noise.
///
/// Note: This is a style-based shader that creates visual shake through
/// color manipulation. For character-based displacement using partial
/// block characters, use a filter implementation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct SubCellShakeShader {
    /// Shake amplitude as brightness variation (0.0 - 0.5 typical)
    #[serde(default = "default_amplitude")]
    pub amplitude: f32,

    /// Shake frequency in Hz (oscillations per second)
    #[serde(default = "default_frequency")]
    pub frequency: f32,

    /// Which axis to apply shake to
    #[serde(default)]
    pub axis: ShakeAxis,

    /// Enable chromatic aberration (RGB channel offset)
    #[serde(default)]
    pub chromatic: bool,

    /// Random seed for deterministic noise
    #[serde(default = "default_seed")]
    pub seed: u64,

    /// Target edges only (shake strongest at widget borders)
    #[serde(default)]
    pub edge_only: bool,

    /// Edge width in cells (when edge_only is true)
    #[serde(default = "default_edge_width")]
    pub edge_width: u8,
}

fn default_amplitude() -> f32 {
    0.15
}

fn default_frequency() -> f32 {
    12.0
}

fn default_seed() -> u64 {
    42
}

fn default_edge_width() -> u8 {
    1
}

impl Default for SubCellShakeShader {
    fn default() -> Self {
        Self {
            amplitude: default_amplitude(),
            frequency: default_frequency(),
            axis: ShakeAxis::default(),
            chromatic: false,
            seed: default_seed(),
            edge_only: false,
            edge_width: default_edge_width(),
        }
    }
}

impl SubCellShakeShader {
    /// Simple hash function for deterministic pseudo-random values.
    fn hash(&self, x: u32, y: u32, frame: u32) -> f32 {
        let mut h = self.seed.wrapping_add(x as u64 * 374761393);
        h = h.wrapping_add(y as u64 * 668265263);
        h = h.wrapping_add(frame as u64 * 2147483647);
        h ^= h >> 13;
        h = h.wrapping_mul(1274126177);
        h ^= h >> 16;
        // Convert to 0.0-1.0 range
        (h & 0xFFFF) as f32 / 65535.0
    }

    /// Calculate distance to nearest edge.
    fn edge_distance(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let w = (width.saturating_sub(1)) as f32;
        let h = (height.saturating_sub(1)) as f32;

        x.min(w - x).min(y).min(h - y)
    }

    /// Calculate shake offset for this position and time.
    fn shake_offset(&self, ctx: &ShaderContext) -> f32 {
        let t = ctx.t as f32;
        let frame = (t * self.frequency * 10.0) as u32;

        // Base oscillation
        let phase = t * self.frequency * std::f32::consts::TAU;

        let (h_component, v_component) = match self.axis {
            ShakeAxis::Horizontal => ((phase + ctx.local_x as f32 * 0.5).sin(), 0.0),
            ShakeAxis::Vertical => (0.0, (phase + ctx.local_y as f32 * 0.5).sin()),
            ShakeAxis::Both => (
                (phase + ctx.local_x as f32 * 0.5).sin(),
                (phase * 1.3 + ctx.local_y as f32 * 0.7).sin(),
            ),
        };

        // Add noise for natural variation
        let noise = self.hash(ctx.local_x as u32, ctx.local_y as u32, frame) * 2.0 - 1.0;

        let raw_offset = (h_component + v_component) * 0.5 + noise * 0.3;

        // Apply edge falloff if enabled
        if self.edge_only {
            let dist = self.edge_distance(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
            let edge_factor = (1.0 - dist / self.edge_width as f32).max(0.0);
            raw_offset * edge_factor
        } else {
            raw_offset
        }
    }

    /// Apply brightness modulation to a color.
    fn modulate_color(&self, color: Color, offset: f32) -> Color {
        let brightness_delta = offset * self.amplitude;

        if brightness_delta >= 0.0 {
            // Brighten
            Color::rgb(
                (color.r as f32 + (255.0 - color.r as f32) * brightness_delta).round() as u8,
                (color.g as f32 + (255.0 - color.g as f32) * brightness_delta).round() as u8,
                (color.b as f32 + (255.0 - color.b as f32) * brightness_delta).round() as u8,
            )
        } else {
            // Darken
            let factor = 1.0 + brightness_delta; // brightness_delta is negative
            Color::rgb(
                (color.r as f32 * factor).round() as u8,
                (color.g as f32 * factor).round() as u8,
                (color.b as f32 * factor).round() as u8,
            )
        }
    }

    /// Apply chromatic aberration - offset RGB channels differently.
    fn chromatic_color(&self, color: Color, offset: f32) -> Color {
        let shift = (offset * self.amplitude * 30.0) as i16;
        Color::rgb(
            (color.r as i16 + shift).clamp(0, 255) as u8,
            color.g,
            (color.b as i16 - shift).clamp(0, 255) as u8,
        )
    }
}

impl StyleShader for SubCellShakeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.amplitude <= 0.0 || self.frequency <= 0.0 {
            return base;
        }

        let offset = self.shake_offset(ctx);

        let mut style = base;
        if self.chromatic {
            style.fg = self.chromatic_color(base.fg, offset);
            style.bg = self.chromatic_color(base.bg, offset * 0.5); // Subtler on bg
        } else {
            style.fg = self.modulate_color(base.fg, offset);
            style.bg = self.modulate_color(base.bg, offset);
        }

        style
    }

    fn name(&self) -> &'static str {
        "SubCellShake"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::make_ctx_at;
    use tui_vfx_types::Modifiers;

    fn make_style() -> Style {
        Style {
            fg: Color::rgb(128, 128, 128),
            bg: Color::rgb(64, 64, 64),
            mods: Modifiers::NONE,
        }
    }

    #[test]
    fn default_values() {
        let shader = SubCellShakeShader::default();
        assert_eq!(shader.amplitude, 0.15);
        assert_eq!(shader.frequency, 12.0);
        assert_eq!(shader.axis, ShakeAxis::Both);
        assert!(!shader.chromatic);
        assert!(!shader.edge_only);
    }

    #[test]
    fn zero_amplitude_no_change() {
        let shader = SubCellShakeShader {
            amplitude: 0.0,
            ..Default::default()
        };
        let ctx = make_ctx_at(5, 5, 10, 10, 0.5);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn zero_frequency_no_change() {
        let shader = SubCellShakeShader {
            frequency: 0.0,
            ..Default::default()
        };
        let ctx = make_ctx_at(5, 5, 10, 10, 0.5);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result.fg, base.fg);
    }

    #[test]
    fn different_times_produce_different_colors() {
        let shader = SubCellShakeShader {
            amplitude: 0.3,
            frequency: 10.0,
            ..Default::default()
        };
        let base = make_style();

        let ctx1 = make_ctx_at(5, 5, 10, 10, 0.0);
        let ctx2 = make_ctx_at(5, 5, 10, 10, 0.05); // Different time

        let result1 = shader.style_at(&ctx1, base);
        let result2 = shader.style_at(&ctx2, base);

        // Colors should differ due to animation
        assert_ne!(result1.fg, result2.fg);
    }

    #[test]
    fn different_positions_produce_different_colors() {
        let shader = SubCellShakeShader {
            amplitude: 0.3,
            frequency: 10.0,
            ..Default::default()
        };
        let base = make_style();
        let t = 0.5;

        let ctx1 = make_ctx_at(0, 0, 10, 10, t);
        let ctx2 = make_ctx_at(5, 5, 10, 10, t);

        let result1 = shader.style_at(&ctx1, base);
        let result2 = shader.style_at(&ctx2, base);

        // Different positions should have different shake phases
        assert_ne!(result1.fg, result2.fg);
    }

    #[test]
    fn edge_only_affects_edges() {
        let shader = SubCellShakeShader {
            amplitude: 0.5,
            frequency: 10.0,
            edge_only: true,
            edge_width: 2,
            ..Default::default()
        };
        let base = make_style();

        // Center should be unchanged
        let ctx_center = make_ctx_at(5, 5, 10, 10, 0.5);
        let result_center = shader.style_at(&ctx_center, base);

        // Edge should be changed
        let ctx_edge = make_ctx_at(0, 5, 10, 10, 0.5);
        let result_edge = shader.style_at(&ctx_edge, base);

        // Center should match base (no shake outside edge_width)
        assert_eq!(result_center.fg, base.fg);
        // Edge should differ
        assert_ne!(result_edge.fg, base.fg);
    }

    #[test]
    fn chromatic_shifts_red_and_blue() {
        let shader = SubCellShakeShader {
            amplitude: 0.5,
            frequency: 10.0,
            chromatic: true,
            ..Default::default()
        };

        let base = Style {
            fg: Color::rgb(128, 128, 128),
            bg: Color::rgb(64, 64, 64),
            mods: Modifiers::NONE,
        };

        let ctx = make_ctx_at(0, 0, 10, 10, 0.25);
        let result = shader.style_at(&ctx, base);

        // Green should stay the same, R and B should shift
        assert_eq!(result.fg.g, base.fg.g);
        // R and B should be different from each other (opposite shifts)
        assert_ne!(result.fg.r, result.fg.b);
    }

    #[test]
    fn deterministic_with_same_seed() {
        let shader1 = SubCellShakeShader {
            seed: 12345,
            amplitude: 0.3,
            ..Default::default()
        };
        let shader2 = SubCellShakeShader {
            seed: 12345,
            amplitude: 0.3,
            ..Default::default()
        };

        let ctx = make_ctx_at(3, 7, 10, 10, 0.5);
        let base = make_style();

        let result1 = shader1.style_at(&ctx, base);
        let result2 = shader2.style_at(&ctx, base);

        assert_eq!(result1.fg, result2.fg);
    }

    #[test]
    fn different_seeds_produce_different_results() {
        let shader1 = SubCellShakeShader {
            seed: 111,
            amplitude: 0.3,
            ..Default::default()
        };
        let shader2 = SubCellShakeShader {
            seed: 222,
            amplitude: 0.3,
            ..Default::default()
        };

        let ctx = make_ctx_at(3, 7, 10, 10, 0.5);
        let base = make_style();

        let result1 = shader1.style_at(&ctx, base);
        let result2 = shader2.style_at(&ctx, base);

        assert_ne!(result1.fg, result2.fg);
    }

    #[test]
    fn serde_roundtrip() {
        let shader = SubCellShakeShader {
            amplitude: 0.2,
            frequency: 8.0,
            axis: ShakeAxis::Horizontal,
            chromatic: true,
            seed: 999,
            edge_only: true,
            edge_width: 3,
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: SubCellShakeShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }

    #[test]
    fn name_is_correct() {
        let shader = SubCellShakeShader::default();
        assert_eq!(shader.name(), "SubCellShake");
    }
}

// <FILE>tui-vfx-style/src/models/cls_sub_cell_shake_shader.rs</FILE> - <DESC>Micro-jitter visual effect through rapid color oscillation</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
