// <FILE>tui-vfx-style/src/models/cls_stochastic_sparkle_shader.rs</FILE> - <DESC>Film grain / frosted glass sparkle effect using stochastic per-cell brightening</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implement premium stochastic sparkle effect from IDEAS.md</WCTX>
// <CLOG>Initial implementation with configurable density, brightness, and noise type</CLOG>

use crate::models::NoiseType;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Stochastic sparkle shader for film grain / frosted glass effects.
///
/// Instead of flickering all cells, this shader randomly brightens a small
/// percentage of cells per frame, creating a subtle "alive" texture that
/// looks like film grain, sensor noise, or diamond dust on a surface.
///
/// The effect is deterministic per-frame (same frame + same position = same
/// sparkle state), but evolves over time as the frame seed changes.
///
/// # Use Cases
///
/// - **Frosted glass**: Low brightness boost, moderate density
/// - **Diamond dust**: Higher brightness, lower density
/// - **Film grain**: Very low brightness, higher density
/// - **Premium buttons**: Subtle sparkle on hover/active states
///
/// # Example
///
/// ```ignore
/// let sparkle = StochasticSparkleShader {
///     sparkle_density: 0.05,    // 5% of cells sparkle
///     brightness_boost: 1.2,    // 20% brighter
///     speed: 0.25,              // Slow shimmer (update every 4 frames at 60fps)
///     seed: 42,
///     apply_to: SparkleTarget::Background,
///     noise_type: NoiseType::Uniform,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct StochasticSparkleShader {
    /// Fraction of cells that sparkle per frame (0.0 - 1.0).
    /// 0.05 = 5% of cells sparkle, creating subtle texture.
    /// Higher values create more active/noisy appearance.
    #[config(default = 0.05)]
    pub sparkle_density: f32,

    /// How much to brighten sparkling cells (multiplier).
    /// 1.0 = no change, 1.2 = 20% brighter, 1.5 = 50% brighter.
    #[config(default = 1.2)]
    pub brightness_boost: f32,

    /// Speed of sparkle animation (lower = slower shimmer).
    /// 1.0 = update every frame, 0.25 = update every 4 frames.
    /// Lower values create "shimmering sand" vs "static" look.
    #[config(default = 0.25)]
    pub speed: f32,

    /// Seed for deterministic randomness.
    #[config(default = 42)]
    pub seed: u64,

    /// Which color channel(s) to apply sparkle to.
    #[serde(default)]
    #[config(default = "background")]
    pub apply_to: SparkleTarget,

    /// Noise distribution type.
    /// Uniform gives even distribution, Gaussian clusters sparkles.
    #[serde(default)]
    #[config(default = "uniform")]
    pub noise_type: NoiseType,
}

/// Which color channel(s) to apply the sparkle effect to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SparkleTarget {
    /// Apply sparkle to foreground color only
    Foreground,
    /// Apply sparkle to background color only
    #[default]
    Background,
    /// Apply sparkle to both foreground and background
    Both,
}

impl Default for StochasticSparkleShader {
    fn default() -> Self {
        Self {
            sparkle_density: 0.05,
            brightness_boost: 1.2,
            speed: 0.25,
            seed: 42,
            apply_to: SparkleTarget::Background,
            noise_type: NoiseType::Uniform,
        }
    }
}

impl StochasticSparkleShader {
    /// Create a new sparkle shader with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a "frosted glass" preset - subtle, slow shimmer.
    pub fn frosted_glass() -> Self {
        Self {
            sparkle_density: 0.03,
            brightness_boost: 1.15,
            speed: 0.2,
            seed: 42,
            apply_to: SparkleTarget::Background,
            noise_type: NoiseType::Uniform,
        }
    }

    /// Create a "diamond dust" preset - sparse but bright sparkles.
    pub fn diamond_dust() -> Self {
        Self {
            sparkle_density: 0.02,
            brightness_boost: 1.4,
            speed: 0.15,
            seed: 42,
            apply_to: SparkleTarget::Both,
            noise_type: NoiseType::Gaussian,
        }
    }

    /// Create a "film grain" preset - dense but subtle texture.
    pub fn film_grain() -> Self {
        Self {
            sparkle_density: 0.08,
            brightness_boost: 1.1,
            speed: 0.5,
            seed: 42,
            apply_to: SparkleTarget::Background,
            noise_type: NoiseType::Gaussian,
        }
    }

    /// Generate deterministic noise for a cell position and time.
    ///
    /// Combines screen position with time-based seed for animation.
    fn cell_noise(&self, x: u16, y: u16, t: f32) -> f32 {
        // Slow down animation by speed factor
        // At speed=0.25, we update pattern every 4 frames (at 60fps effective ~15fps)
        let time_component = (t * 1000.0 * self.speed).floor() as u64;

        // Pack x,y into a single value with good bit distribution
        // Using prime multipliers to avoid patterns
        let position_hash = (x as u64).wrapping_mul(374761393) ^ (y as u64).wrapping_mul(668265263);

        // Combine with seed and time
        let input = self
            .seed
            .wrapping_add(position_hash)
            .wrapping_add(time_component.wrapping_mul(3935559000370003845));

        self.noise_type.sample(input)
    }

    /// Check if a cell should sparkle at the current time.
    #[inline]
    fn should_sparkle(&self, x: u16, y: u16, t: f32) -> bool {
        let noise = self.cell_noise(x, y, t);
        // Threshold: if density is 0.05, then noise > 0.95 means sparkle
        noise > (1.0 - self.sparkle_density)
    }

    /// Apply brightness boost to a color.
    #[inline]
    fn brighten_color(&self, color: Color) -> Color {
        color.brighten(self.brightness_boost)
    }
}

impl StyleShader for StochasticSparkleShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;

        // Use screen coordinates for screen-space noise pattern
        // This makes the sparkle pattern consistent across widget boundaries
        let x = ctx.screen_cell_x();
        let y = ctx.screen_cell_y();

        if self.should_sparkle(x, y, t) {
            let mut result = base;

            match self.apply_to {
                SparkleTarget::Foreground => {
                    if base.fg != Color::TRANSPARENT {
                        result.fg = self.brighten_color(base.fg);
                    }
                }
                SparkleTarget::Background => {
                    if base.bg != Color::TRANSPARENT {
                        result.bg = self.brighten_color(base.bg);
                    }
                }
                SparkleTarget::Both => {
                    if base.fg != Color::TRANSPARENT {
                        result.fg = self.brighten_color(base.fg);
                    }
                    if base.bg != Color::TRANSPARENT {
                        result.bg = self.brighten_color(base.bg);
                    }
                }
            }

            result
        } else {
            base
        }
    }

    fn name(&self) -> &'static str {
        "StochasticSparkle"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creates_valid_shader() {
        let shader = StochasticSparkleShader::default();
        assert_eq!(shader.sparkle_density, 0.05);
        assert_eq!(shader.brightness_boost, 1.2);
        assert_eq!(shader.speed, 0.25);
    }

    #[test]
    fn test_presets_have_valid_values() {
        let frosted = StochasticSparkleShader::frosted_glass();
        assert!(frosted.sparkle_density > 0.0 && frosted.sparkle_density < 1.0);
        assert!(frosted.brightness_boost >= 1.0);

        let diamond = StochasticSparkleShader::diamond_dust();
        assert!(diamond.sparkle_density > 0.0 && diamond.sparkle_density < 1.0);
        assert!(diamond.brightness_boost >= 1.0);

        let grain = StochasticSparkleShader::film_grain();
        assert!(grain.sparkle_density > 0.0 && grain.sparkle_density < 1.0);
        assert!(grain.brightness_boost >= 1.0);
    }

    #[test]
    fn test_sparkle_density_affects_frequency() {
        let low_density = StochasticSparkleShader {
            sparkle_density: 0.01,
            ..Default::default()
        };
        let high_density = StochasticSparkleShader {
            sparkle_density: 0.20,
            ..Default::default()
        };

        // Count sparkles over a grid
        let mut low_count = 0;
        let mut high_count = 0;
        for y in 0..100 {
            for x in 0..100 {
                if low_density.should_sparkle(x, y, 0.5) {
                    low_count += 1;
                }
                if high_density.should_sparkle(x, y, 0.5) {
                    high_count += 1;
                }
            }
        }

        // High density should have significantly more sparkles
        assert!(
            high_count > low_count * 5,
            "High density ({}) should have much more sparkles than low density ({})",
            high_count,
            low_count
        );
    }

    #[test]
    fn test_deterministic_noise() {
        let shader = StochasticSparkleShader::default();

        // Same inputs should give same results
        let noise1 = shader.cell_noise(10, 20, 0.5);
        let noise2 = shader.cell_noise(10, 20, 0.5);
        assert_eq!(noise1, noise2, "Noise should be deterministic");

        // Different positions should give different results (usually)
        let noise3 = shader.cell_noise(11, 20, 0.5);
        assert_ne!(
            noise1, noise3,
            "Different positions should give different noise"
        );
    }

    #[test]
    fn test_different_seeds_produce_different_patterns() {
        let shader1 = StochasticSparkleShader {
            seed: 42,
            ..Default::default()
        };
        let shader2 = StochasticSparkleShader {
            seed: 123,
            ..Default::default()
        };

        // Count matching sparkle states
        let mut matches = 0;
        for y in 0..50 {
            for x in 0..50 {
                if shader1.should_sparkle(x, y, 0.5) == shader2.should_sparkle(x, y, 0.5) {
                    matches += 1;
                }
            }
        }

        // With different seeds, patterns should differ significantly
        // (not all cells should match)
        let total = 50 * 50;
        assert!(
            matches < total - 100,
            "Different seeds should produce different patterns (matches: {}/{})",
            matches,
            total
        );
    }

    #[test]
    fn test_style_shader_impl() {
        let shader = StochasticSparkleShader {
            sparkle_density: 1.0, // All cells sparkle for testing
            brightness_boost: 1.5,
            apply_to: SparkleTarget::Background,
            ..Default::default()
        };

        let ctx = ShaderContext::new(5, 5, 10, 10, 0, 0, 0.5, None);
        let base = Style {
            fg: Color::WHITE,
            bg: Color::new(100, 100, 100, 255),
            ..Default::default()
        };

        let result = shader.style_at(&ctx, base);

        // Background should be brightened
        assert!(
            result.bg.r > base.bg.r,
            "Background should be brightened: {} > {}",
            result.bg.r,
            base.bg.r
        );

        // Foreground should remain unchanged (only bg target)
        assert_eq!(result.fg, base.fg, "Foreground should remain unchanged");
    }
}

// <FILE>tui-vfx-style/src/models/cls_stochastic_sparkle_shader.rs</FILE> - <DESC>Film grain / frosted glass sparkle effect using stochastic per-cell brightening</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
