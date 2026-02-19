// <FILE>tui-vfx-style/src/models/cls_glitch_lines_shader.rs</FILE> - <DESC>Spatial glitch with random interference lines</DESC>
// <VERS>VERSION: 1.8.0</VERS>
// <WCTX>Adding screen coordinate context to shaders</WCTX>
// <CLOG>Updated to use ShaderContext for screen-space effects</CLOG>

use crate::models::{ColorConfig, NoiseType};

fn default_speed() -> f32 {
    1.0
}

fn default_pulse_speed() -> f32 {
    0.5
}

fn default_flash_hold() -> u32 {
    1
}
use crate::models::ColorSpace;
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Spatial glitch shader that creates random horizontal interference lines.
///
/// Unlike the temporal `StyleEffect::Glitch`, this shader varies by row position,
/// creating thin horizontal "interference" lines at random positions that change
/// each time the effect fires.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GlitchLinesShader {
    /// Seed for deterministic randomness
    #[config(default = 42)]
    pub seed: u64,
    /// Probability of any line appearing (0.0 - 1.0)
    #[config(default = 0.5)]
    pub intensity: f32,
    /// Maximum number of interference lines
    #[config(default = 6)]
    pub max_lines: u16,
    /// Speed multiplier for pattern changes (higher = faster flicker)
    #[serde(default = "default_speed")]
    #[config(default = 1.0)]
    pub speed: f32,
    /// Chance of a full-row REVERSED flash (0.0 - 1.0)
    #[serde(default)]
    #[config(default = 0.0)]
    pub flash_chance: f32,
    /// Optional color to pulse towards (e.g., white for cyan→white→cyan)
    #[serde(default)]
    pub pulse_color: Option<ColorConfig>,
    /// Speed of the color pulse cycle (cycles per animation)
    #[serde(default = "default_pulse_speed")]
    #[config(default = 0.5)]
    pub pulse_speed: f32,
    /// Apply ITALIC modifier during flash moments
    #[serde(default)]
    #[config(default = false)]
    pub italic_on_flash: bool,
    /// How many time slots the flash persists (1 = instant, 5 = noticeable, 10+ = elongated)
    #[serde(default = "default_flash_hold")]
    #[config(default = 1)]
    pub flash_hold: u32,
    /// Noise distribution type (uniform or gaussian).
    /// Gaussian produces more natural-looking variation.
    #[serde(default)]
    #[config(default = "uniform")]
    pub noise_type: NoiseType,
}

impl Default for GlitchLinesShader {
    fn default() -> Self {
        Self {
            seed: 42,
            intensity: 0.5,
            max_lines: 6,
            speed: 1.0,
            flash_chance: 0.0,
            pulse_color: None,
            pulse_speed: 0.5,
            italic_on_flash: false,
            flash_hold: 1,
            noise_type: NoiseType::Uniform,
        }
    }
}

impl GlitchLinesShader {
    /// Pseudo-random number generator using the configured noise type.
    fn noise(&self, input: u32) -> f32 {
        self.noise_type.sample(input as u64)
    }

    /// Check if a row should have an interference line at time t
    fn row_has_line(&self, y: u16, height: u16, t: f32) -> bool {
        if height == 0 {
            return false;
        }

        // Create a time-varying seed that changes the pattern periodically
        // Base rate is 30 changes per animation cycle, multiplied by speed
        let time_slot = (t * 30.0 * self.speed).floor() as u32;
        let base_seed = self.seed as u32;

        // First, determine how many lines this frame (varies 1 to max_lines)
        let line_count_noise = self.noise(base_seed.wrapping_add(time_slot).wrapping_mul(7919));
        let line_count = 1 + (line_count_noise * self.max_lines as f32) as u16;

        // Check if this row is one of the selected lines
        for i in 0..line_count {
            // Generate a random row for this line slot
            let row_noise = self.noise(
                base_seed
                    .wrapping_add(time_slot)
                    .wrapping_mul(1000 + i as u32)
                    .wrapping_add(i as u32 * 31337),
            );
            let affected_row = (row_noise * height as f32) as u16;

            if y == affected_row {
                // Additional intensity check - not every potential line fires
                let fire_noise = self.noise(
                    base_seed
                        .wrapping_add(time_slot)
                        .wrapping_mul(2000 + i as u32),
                );
                if fire_noise < self.intensity {
                    return true;
                }
            }
        }
        false
    }

    /// Check if we should do a full REVERSED flash this frame.
    /// When flash_hold > 1, checks if ANY of the last N time slots triggered a flash,
    /// creating an elongated flash effect.
    fn should_flash(&self, t: f32) -> bool {
        if self.flash_chance <= 0.0 {
            return false;
        }
        let current_slot = (t * 50.0 * self.speed).floor() as u32;
        let hold = self.flash_hold.max(1);

        // Check current slot and previous (hold-1) slots
        for offset in 0..hold {
            let time_slot = current_slot.saturating_sub(offset);
            let flash_noise = self.noise(
                (self.seed as u32)
                    .wrapping_add(time_slot)
                    .wrapping_mul(77777),
            );
            if flash_noise < self.flash_chance {
                return true;
            }
        }
        false
    }

    /// Calculate color pulse blend factor (0.0 = base color, 1.0 = pulse color)
    fn pulse_blend(&self, t: f32) -> f32 {
        if self.pulse_color.is_none() {
            return 0.0;
        }
        // Sine wave from 0 to 1 and back
        let wave = (t * self.pulse_speed * std::f32::consts::TAU).sin();
        // Normalize from [-1, 1] to [0, 1]
        (wave + 1.0) / 2.0
    }
}

impl StyleShader for GlitchLinesShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;
        let (y, height) = (ctx.local_y, ctx.height);
        let mut result = base;

        // Apply color pulse if configured
        if let Some(ref pulse_cfg) = self.pulse_color {
            let blend = self.pulse_blend(t);
            if blend > 0.0 {
                let pulse_color: Color = (*pulse_cfg).into();
                if base.fg != Color::TRANSPARENT {
                    result.fg = blend_colors(base.fg, pulse_color, blend, ColorSpace::Rgb);
                }
            }
        }

        // Check for full REVERSED flash (with optional ITALIC)
        if self.should_flash(t) {
            let mut new_mods = result.mods;
            new_mods.reverse = true;
            if self.italic_on_flash {
                new_mods.italic = true;
            }
            result = result.with_mods(new_mods);
            return result;
        }

        // Apply underline to interference lines
        if self.row_has_line(y, height, t) {
            result = result.underline();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creates_valid_shader() {
        let shader = GlitchLinesShader::default();
        assert_eq!(shader.seed, 42);
        assert!(shader.intensity > 0.0);
        assert!(shader.max_lines > 0);
    }

    #[test]
    fn test_different_times_produce_different_patterns() {
        let shader = GlitchLinesShader {
            seed: 123,
            intensity: 1.0, // Always fire when selected
            max_lines: 4,
            speed: 1.0,
            flash_chance: 0.0,
            pulse_color: None,
            pulse_speed: 0.5,
            italic_on_flash: false,
            flash_hold: 1,
            noise_type: NoiseType::Uniform,
        };

        let height = 10;
        let mut patterns = Vec::new();

        // Sample at different times
        for t_idx in 0..5 {
            let t = t_idx as f32 * 0.1;
            let mut pattern = Vec::new();
            for y in 0..height {
                pattern.push(shader.row_has_line(y, height, t));
            }
            patterns.push(pattern);
        }

        // At least some patterns should differ
        let all_same = patterns.windows(2).all(|w| w[0] == w[1]);
        assert!(!all_same, "Patterns should vary across time");
    }

    #[test]
    fn test_zero_intensity_no_lines() {
        let shader = GlitchLinesShader {
            seed: 42,
            intensity: 0.0,
            max_lines: 10,
            speed: 1.0,
            flash_chance: 0.0,
            pulse_color: None,
            pulse_speed: 0.5,
            italic_on_flash: false,
            flash_hold: 1,
            noise_type: NoiseType::Uniform,
        };

        let height = 20;
        for y in 0..height {
            assert!(
                !shader.row_has_line(y, height, 0.5),
                "Zero intensity should produce no lines"
            );
        }
    }
}

// <FILE>tui-vfx-style/src/models/cls_glitch_lines_shader.rs</FILE> - <DESC>Spatial glitch with random interference lines</DESC>
// <VERS>END OF VERSION: 1.8.0</VERS>
