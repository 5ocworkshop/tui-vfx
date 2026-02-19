// <FILE>tui-vfx-style/src/models/cls_neon_flicker_shader.rs</FILE> - <DESC>Spatial neon flicker with independent segments</DESC>
// <VERS>VERSION: 1.6.0</VERS>
// <WCTX>Adding screen coordinate context to shaders</WCTX>
// <CLOG>Updated to use ShaderContext for screen-space effects</CLOG>

use crate::models::NoiseType;
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::darken;
use mixed_signals::envelopes::Impact;
use mixed_signals::traits::Signal;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

fn default_speed() -> f32 {
    1.0
}

/// Spatial neon flicker shader with independently flickering segments.
///
/// Unlike the temporal `StyleEffect::NeonFlicker` which dims the entire
/// notification uniformly, this shader makes different segments flicker
/// at different times - like a damaged neon sign where different tubes
/// have varying levels of stability.
///
/// When `decay_rate` is set, flickers have a smooth decay tail (like a tube
/// warming back up) instead of instant on/off behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct NeonFlickerShader {
    /// Base stability (0.0 = always flickering, 1.0 = never flickers)
    #[config(default = 0.7)]
    pub stability: f32,
    /// Seed for deterministic randomness
    #[config(default = 42)]
    pub seed: u64,
    /// Segment mode: "cell", "row", or "column"
    #[config(default = "row")]
    pub segment: SegmentMode,
    /// How much the flicker dims the color (0.0 - 1.0)
    #[config(default = 0.8)]
    pub dim_amount: f32,
    /// Speed multiplier (lower = slower flicker)
    #[serde(default = "default_speed")]
    #[config(default = 1.0)]
    pub speed: f32,
    /// Chance of a white flash instead of dim (0.0 - 1.0)
    #[serde(default)]
    #[config(default = 0.0)]
    pub flash_chance: f32,
    /// Optional decay rate for smooth flicker recovery.
    /// When present, flickers fade out smoothly using Impact envelope.
    /// Higher values = faster recovery (3.0 is quick, 0.5 is slow fade).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[config(opaque)]
    pub decay_rate: Option<f32>,
    /// Noise distribution type (uniform or gaussian).
    /// Gaussian produces more natural-looking variation.
    #[serde(default)]
    #[config(default = "uniform")]
    pub noise_type: NoiseType,
}

/// How to segment the flickering regions.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SegmentMode {
    /// Each cell flickers independently
    Cell,
    /// Each row flickers as a unit
    #[default]
    Row,
    /// Each column flickers as a unit
    Column,
}

impl Default for NeonFlickerShader {
    fn default() -> Self {
        Self {
            stability: 0.7,
            seed: 42,
            segment: SegmentMode::Row,
            dim_amount: 0.8,
            speed: 1.0,
            flash_chance: 0.0,
            decay_rate: None,
            noise_type: NoiseType::Uniform,
        }
    }
}

impl NeonFlickerShader {
    /// Pseudo-random number generator using the configured noise type.
    fn noise(&self, input: u32) -> f32 {
        self.noise_type.sample(input as u64)
    }

    /// Get the segment ID for a position.
    fn segment_id(&self, x: u16, y: u16) -> u32 {
        match self.segment {
            SegmentMode::Cell => ((y as u32) << 16) | (x as u32),
            SegmentMode::Row => y as u32,
            SegmentMode::Column => x as u32,
        }
    }

    /// Calculate flicker for a segment at time t.
    /// Returns (dim_amount, is_flash) - dim_amount > 0 means flicker active
    fn flicker_state(&self, segment_id: u32, t: f32) -> (f32, bool) {
        // Each segment has its own noise pattern based on seed + segment_id + time
        // Base rate of 5000, scaled by speed
        let time_component = (t * 5000.0 * self.speed).floor() as u32;
        let input = (self.seed as u32)
            .wrapping_mul(segment_id.wrapping_add(1))
            .wrapping_add(time_component);

        let noise = self.noise(input);

        // If noise exceeds stability, this segment flickers
        if noise > self.stability {
            // Scale the dim amount based on how far over stability we are
            let overage = (noise - self.stability) / (1.0 - self.stability + 0.001);
            let mut dim = overage * self.dim_amount;

            // Apply decay envelope if decay_rate is set
            if let Some(decay_rate) = self.decay_rate {
                // Calculate time within current flicker frame (0.0-1.0)
                let frame_progress = (t * 5000.0 * self.speed).fract();

                // Use Impact envelope for smooth decay
                let decay_env = Impact::new(1.0, decay_rate);
                dim *= decay_env.sample(frame_progress.into());
            }

            // Check if this should be a white flash instead
            let flash_noise = self.noise(input.wrapping_add(99991));
            let is_flash = flash_noise < self.flash_chance;

            (dim, is_flash)
        } else {
            (0.0, false)
        }
    }
}

impl StyleShader for NeonFlickerShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;
        let segment_id = self.segment_id(ctx.local_x, ctx.local_y);
        let (dim, is_flash) = self.flicker_state(segment_id, t);

        if dim > 0.0 {
            let mut result = base;
            if is_flash {
                // White flash - briefly go bright white
                result.fg = Color::WHITE;
                if base.bg != Color::TRANSPARENT {
                    result.bg = Color::WHITE;
                }
            } else {
                // Normal dim flicker
                if base.fg != Color::TRANSPARENT {
                    result.fg = darken(base.fg, dim);
                }
                if base.bg != Color::TRANSPARENT {
                    result.bg = darken(base.bg, dim);
                }
            }
            result
        } else {
            base
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creates_valid_shader() {
        let shader = NeonFlickerShader::default();
        assert_eq!(shader.stability, 0.7);
        assert_eq!(shader.segment, SegmentMode::Row);
    }

    #[test]
    fn test_different_segments_flicker_independently() {
        let shader = NeonFlickerShader {
            stability: 0.5, // 50% chance of flicker
            seed: 123,
            segment: SegmentMode::Row,
            dim_amount: 0.8,
            speed: 1.0,
            flash_chance: 0.0,
            decay_rate: None,
            noise_type: NoiseType::Uniform,
        };

        // Check multiple rows at same time - they should have different flicker states
        let mut flicker_amounts = Vec::new();
        for y in 0..10 {
            let (dim, _) = shader.flicker_state(y, 0.5);
            flicker_amounts.push(dim);
        }

        // Not all should be the same (statistically very unlikely with 50% stability)
        let all_same = flicker_amounts
            .windows(2)
            .all(|w| (w[0] - w[1]).abs() < 0.001);
        assert!(
            !all_same,
            "Different segments should have different flicker states"
        );
    }

    #[test]
    fn test_high_stability_rarely_flickers() {
        let shader = NeonFlickerShader {
            stability: 0.99,
            seed: 42,
            segment: SegmentMode::Cell,
            dim_amount: 0.8,
            speed: 1.0,
            flash_chance: 0.0,
            decay_rate: None,
            noise_type: NoiseType::Uniform,
        };

        // With 99% stability, most segments shouldn't flicker
        let mut flicker_count = 0;
        for segment in 0..100 {
            let (dim, _) = shader.flicker_state(segment, 0.5);
            if dim > 0.0 {
                flicker_count += 1;
            }
        }
        assert!(
            flicker_count < 20,
            "High stability should mean rare flickers"
        );
    }
}

// <FILE>tui-vfx-style/src/models/cls_neon_flicker_shader.rs</FILE> - <DESC>Spatial neon flicker with independent segments</DESC>
// <VERS>END OF VERSION: 1.6.0</VERS>
