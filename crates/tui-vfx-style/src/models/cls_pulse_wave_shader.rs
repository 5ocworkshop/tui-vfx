// <FILE>tui-vfx-style/src/models/cls_pulse_wave_shader.rs</FILE> - <DESC>Spatial pulse wave with rippling color</DESC>
// <VERS>VERSION: 1.3.1</VERS>
// <WCTX>mixed-signals v2 bipolar migration</WCTX>
// <CLOG>Add .normalized() to Sine for 0-1 blend factor after mixed-signals v2 bipolar change</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use mixed_signals::prelude::{Signal, SignalExt, Sine};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Spatial pulse wave shader with rippling color.
///
/// Unlike the temporal `StyleEffect::Pulse` which pulses the entire
/// notification uniformly, this shader creates waves of color that
/// ripple across the widget based on position. Uses mixed_signals::Sine
/// for consistent wave generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct PulseWaveShader {
    /// Wave frequency (waves per animation cycle)
    #[config(default = 2.0)]
    pub frequency: f32,
    /// Wave speed multiplier
    #[config(default = 1.0)]
    pub speed: f32,
    /// Color to pulse towards
    pub color: ColorConfig,
    /// Wave direction
    #[config(default = "horizontal")]
    pub direction: WaveDirection,
    /// Wavelength in cells (how spread out the wave is)
    #[config(default = 8.0)]
    pub wavelength: f32,
}

/// Direction the wave travels.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WaveDirection {
    /// Wave moves left to right
    #[default]
    Horizontal,
    /// Wave moves top to bottom
    Vertical,
    /// Wave radiates from center
    Radial,
    /// Wave moves diagonally
    Diagonal,
}

impl Default for PulseWaveShader {
    fn default() -> Self {
        Self {
            frequency: 2.0,
            speed: 1.0,
            color: ColorConfig::Magenta,
            direction: WaveDirection::Horizontal,
            wavelength: 8.0,
        }
    }
}

impl PulseWaveShader {
    /// Calculate wave phase at a position.
    fn wave_phase(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let nx = if width > 0 {
            x as f32 / width as f32
        } else {
            0.0
        };
        let ny = if height > 0 {
            y as f32 / height as f32
        } else {
            0.0
        };

        match self.direction {
            WaveDirection::Horizontal => nx,
            WaveDirection::Vertical => ny,
            WaveDirection::Radial => {
                // Distance from center, normalized
                let cx = nx - 0.5;
                let cy = ny - 0.5;
                (cx * cx + cy * cy).sqrt() * 2.0
            }
            WaveDirection::Diagonal => (nx + ny) / 2.0,
        }
    }

    /// Calculate blend factor at position and time.
    fn blend_at(&self, x: u16, y: u16, width: u16, height: u16, t: f32) -> f32 {
        let phase = self.wave_phase(x, y, width, height);

        let wavelength = if self.wavelength.is_finite() && self.wavelength > 0.0 {
            self.wavelength
        } else {
            1.0
        };

        // Spatial phase offset based on wavelength
        let spatial_offset = phase * (width.max(height) as f32 / wavelength);

        // Combined wave using mixed_signals::Sine
        // Use frequency = 1/(2*PI) so sample(phase) gives sin(phase)
        // Note: mixed-signals v2 outputs bipolar [-1,1], use .normalized() for 0-1
        let signal = Sine::new(1.0 / std::f32::consts::TAU, 1.0, 0.0, 0.0).normalized();
        let wave_input = (t * self.speed * self.frequency + spatial_offset) * std::f32::consts::TAU;
        signal.sample(wave_input.into())
    }
}

impl StyleShader for PulseWaveShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let blend_factor = self.blend_at(
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
            ctx.t as f32,
        );
        let pulse_color: Color = self.color.into();

        let mut result = base;
        if base.fg != Color::TRANSPARENT {
            result.fg = blend_colors(base.fg, pulse_color, blend_factor, ColorSpace::Rgb);
        }
        if base.bg != Color::TRANSPARENT {
            result.bg = blend_colors(base.bg, pulse_color, blend_factor * 0.3, ColorSpace::Rgb);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creates_valid_shader() {
        let shader = PulseWaveShader::default();
        assert_eq!(shader.frequency, 2.0);
        assert_eq!(shader.direction, WaveDirection::Horizontal);
    }

    #[test]
    fn test_wave_varies_by_position() {
        let shader = PulseWaveShader {
            frequency: 1.0,
            speed: 1.0,
            color: ColorConfig::Red,
            direction: WaveDirection::Horizontal,
            wavelength: 4.0,
        };

        let width = 20;
        let height = 5;
        let t = 0.25;

        // Different x positions should have different blend factors
        let blend_left = shader.blend_at(0, 2, width, height, t);
        let blend_mid = shader.blend_at(10, 2, width, height, t);
        let blend_right = shader.blend_at(19, 2, width, height, t);

        // They shouldn't all be the same
        let all_same =
            (blend_left - blend_mid).abs() < 0.01 && (blend_mid - blend_right).abs() < 0.01;
        assert!(!all_same, "Wave should vary across positions");
    }

    #[test]
    fn test_radial_wave_from_center() {
        let shader = PulseWaveShader {
            frequency: 1.0,
            speed: 1.0,
            color: ColorConfig::Cyan,
            direction: WaveDirection::Radial,
            wavelength: 10.0,
        };

        let width = 20;
        let height = 10;

        // Center should have phase ~0, corners should have higher phase
        let center_phase = shader.wave_phase(10, 5, width, height);
        let corner_phase = shader.wave_phase(0, 0, width, height);

        assert!(
            corner_phase > center_phase,
            "Corners should have higher phase than center for radial wave"
        );
    }
}

// <FILE>tui-vfx-style/src/models/cls_pulse_wave_shader.rs</FILE> - <DESC>Spatial pulse wave with rippling color</DESC>
// <VERS>END OF VERSION: 1.3.1</VERS>
