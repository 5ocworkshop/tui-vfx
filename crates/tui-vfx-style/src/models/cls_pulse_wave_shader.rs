// <FILE>tui-vfx-style/src/models/cls_pulse_wave_shader.rs</FILE> - <DESC>Spatial pulse wave with rippling color</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>Phase 0 P0.3 — add frequency_binding for runtime-parameter wave frequency override</WCTX>
// <CLOG>Add frequency_binding: Option<String> and thread an explicit frequency parameter through blend_at so style_at can resolve the binding once per frame</CLOG>

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
    /// Optional runtime parameter key that overrides `frequency` per frame.
    /// Missing or unresolvable bindings fall back to the static `frequency`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_binding: Option<String>,
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
            frequency_binding: None,
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

    /// Calculate blend factor at position and time, using the given wave
    /// frequency. Taking `frequency` explicitly (rather than reading
    /// `self.frequency`) lets `style_at` resolve a runtime-parameter
    /// binding once per frame and pass the result in.
    fn blend_at(&self, x: u16, y: u16, width: u16, height: u16, t: f32, frequency: f32) -> f32 {
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
        let wave_input = (t * self.speed * frequency + spatial_offset) * std::f32::consts::TAU;
        signal.sample(wave_input.into())
    }

    /// Resolve the effective frequency for the current frame, honoring
    /// `frequency_binding` when set.
    fn effective_frequency(&self, ctx: &ShaderContext) -> f32 {
        self.frequency_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_f32(binding))
            .unwrap_or(self.frequency)
    }
}

impl StyleShader for PulseWaveShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let frequency = self.effective_frequency(ctx);
        let blend_factor = self.blend_at(
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
            ctx.t as f32,
            frequency,
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
            frequency_binding: None,
            speed: 1.0,
            color: ColorConfig::Red,
            direction: WaveDirection::Horizontal,
            wavelength: 4.0,
        };

        let width = 20;
        let height = 5;
        let t = 0.25;

        // Different x positions should have different blend factors
        let blend_left = shader.blend_at(0, 2, width, height, t, shader.frequency);
        let blend_mid = shader.blend_at(10, 2, width, height, t, shader.frequency);
        let blend_right = shader.blend_at(19, 2, width, height, t, shader.frequency);

        // They shouldn't all be the same
        let all_same =
            (blend_left - blend_mid).abs() < 0.01 && (blend_mid - blend_right).abs() < 0.01;
        assert!(!all_same, "Wave should vary across positions");
    }

    #[test]
    fn test_radial_wave_from_center() {
        let shader = PulseWaveShader {
            frequency: 1.0,
            frequency_binding: None,
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

    // --- Phase 0 P0.3: frequency_binding tests ---------------------------

    use crate::traits::{ShaderContext, ShaderRuntimeParamValue, ShaderRuntimeParams};
    use std::sync::Arc;

    fn ctx_with_params(params: ShaderRuntimeParams) -> ShaderContext {
        ShaderContext::new(0, 0, 10, 5, 0, 0, 0.0, None, Some(Arc::new(params)))
    }

    #[test]
    fn effective_frequency_returns_static_when_binding_is_none() {
        let shader = PulseWaveShader {
            frequency: 3.5,
            frequency_binding: None,
            ..PulseWaveShader::default()
        };
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert_eq!(shader.effective_frequency(&ctx), 3.5);
    }

    #[test]
    fn effective_frequency_resolves_from_runtime_params() {
        let shader = PulseWaveShader {
            frequency: 2.0,
            frequency_binding: Some("freq".to_string()),
            ..PulseWaveShader::default()
        };
        let mut params = ShaderRuntimeParams::new();
        params.insert("freq", ShaderRuntimeParamValue::Float(7.0));
        let ctx = ctx_with_params(params);
        assert_eq!(shader.effective_frequency(&ctx), 7.0);
    }

    #[test]
    fn effective_frequency_missing_binding_falls_back_to_static() {
        let shader = PulseWaveShader {
            frequency: 4.2,
            frequency_binding: Some("missing".to_string()),
            ..PulseWaveShader::default()
        };
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert_eq!(shader.effective_frequency(&ctx), 4.2);
    }

    #[test]
    fn style_at_uses_bound_frequency_for_wave_math() {
        // Two PulseWaveShaders with different effective frequencies at the
        // same position must produce different blend factors. We exercise
        // this end-to-end through style_at rather than blend_at to prove
        // the binding is threaded through the public path.
        //
        // PulseWaveShader::style_at only writes into fg/bg when they are
        // non-transparent, so a concrete base Style is required.
        let base_shader = PulseWaveShader {
            frequency: 1.0,
            frequency_binding: Some("freq".to_string()),
            speed: 1.0,
            color: ColorConfig::Red,
            direction: WaveDirection::Horizontal,
            wavelength: 4.0,
        };
        // Frequencies 1 and 3 land at different sine phases at t=0.25
        // (fract 0.25 vs fract 0.75); integer-multiple pairs like 1 and 5
        // both land on the same phase point and would make this test vacuous.
        let mut params_a = ShaderRuntimeParams::new();
        params_a.insert("freq", ShaderRuntimeParamValue::Float(1.0));
        let mut params_b = ShaderRuntimeParams::new();
        params_b.insert("freq", ShaderRuntimeParamValue::Float(3.0));

        let t = 0.25;
        let ctx_a = ShaderContext::new(8, 2, 20, 5, 0, 0, t, None, Some(Arc::new(params_a)));
        let ctx_b = ShaderContext::new(8, 2, 20, 5, 0, 0, t, None, Some(Arc::new(params_b)));

        let base = tui_vfx_types::Style {
            fg: tui_vfx_types::Color::rgb(10, 20, 30),
            bg: tui_vfx_types::Color::rgb(60, 70, 80),
            mods: Default::default(),
        };
        let out_a = base_shader.style_at(&ctx_a, base);
        let out_b = base_shader.style_at(&ctx_b, base);
        // Higher frequency shifts the wave phase, so the resulting styles
        // should differ at this fixed position.
        assert_ne!(out_a, out_b);
    }
}

// <FILE>tui-vfx-style/src/models/cls_pulse_wave_shader.rs</FILE> - <DESC>Spatial pulse wave with rippling color</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
