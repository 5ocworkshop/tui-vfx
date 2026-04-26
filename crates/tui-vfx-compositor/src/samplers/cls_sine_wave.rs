// <FILE>tui-vfx-compositor/src/samplers/cls_sine_wave.rs</FILE> - <DESC>SineWave sampler with axis and phase support</DESC>
// <VERS>VERSION: 3.2.0</VERS>
// <WCTX>Slice 6.6 §F.4 — migrate Sampler trait to take &VfxCellContext</WCTX>
// <CLOG>3.2.0: sample() now takes &VfxCellContext; dest_x/dest_y/t reach via ctx.local_x/local_y/t.

use crate::traits::sampler::Sampler;
use tui_vfx_types::VfxCellContext;
use crate::types::cls_sampler_spec::Axis;
use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine};

/// Sinusoidal wave distortion sampler.
///
/// Uses mixed_signals::Sine with `.normalized()` for 0-1 output, then Remap for bidirectional displacement.
/// The normalized sine (0 to 1) is remapped to (-amplitude, +amplitude).
pub struct SineWave {
    /// The remapped sine signal outputting -amplitude to +amplitude
    signal: Remap<Normalized<Sine>>,
    /// Which axis the wave displacement affects
    axis: Axis,
    /// Spatial frequency (waves per cell)
    spatial_freq: f32,
    /// Temporal speed multiplier
    speed: f32,
    /// Phase offset in radians
    phase_offset: f32,
}

impl Default for SineWave {
    fn default() -> Self {
        Self::new(2.0, 0.5, 10.0, Axis::X, 0.0)
    }
}

impl SineWave {
    /// Create a new SineWave sampler.
    ///
    /// # Arguments
    /// * `amplitude` - Wave amplitude in cells
    /// * `spatial_freq` - Spatial frequency (waves per cell)
    /// * `speed` - Temporal animation speed
    /// * `axis` - Which axis the wave displacement affects
    /// * `phase_offset` - Phase offset in radians
    pub fn new(
        amplitude: f32,
        spatial_freq: f32,
        speed: f32,
        axis: Axis,
        phase_offset: f32,
    ) -> Self {
        // Create normalized sine (0-1)
        // Use frequency = 1/(2*PI) so sample(phase) follows sin(phase) timing
        let base_sine = Sine::new(1.0 / std::f32::consts::TAU, 1.0, 0.0, 0.0).normalized();
        // Remap 0..1 to -amplitude..+amplitude for bidirectional displacement
        let signal = Remap::new(base_sine, 0.0, 1.0, -amplitude, amplitude);
        Self {
            signal,
            axis,
            spatial_freq,
            speed,
            phase_offset,
        }
    }
}

impl Sampler for SineWave {
    fn sample(&self, ctx: &VfxCellContext) -> Option<(u16, u16)> {
        let t = ctx.t as f32;
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        match self.axis {
            Axis::X => {
                // Wave along Y, displaces X (horizontal wave motion)
                let phase = dest_y as f32 * self.spatial_freq + t * self.speed + self.phase_offset;
                let offset = self.signal.sample(phase.into());
                let src_x = (dest_x as f32 + offset).round();
                if src_x < 0.0 {
                    None
                } else {
                    Some((src_x as u16, dest_y))
                }
            }
            Axis::Y => {
                // Wave along X, displaces Y (vertical wave motion)
                let phase = dest_x as f32 * self.spatial_freq + t * self.speed + self.phase_offset;
                let offset = self.signal.sample(phase.into());
                let src_y = (dest_y as f32 + offset).round();
                if src_y < 0.0 {
                    None
                } else {
                    Some((dest_x, src_y as u16))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_sine_wave_zero_amplitude_identity() {
        let sampler = SineWave::new(0.0, 1.0, 1.0, Axis::X, 0.0);
        // With zero amplitude, no displacement should occur
        assert_eq!(sampler.sample(&ctx_at(5, 7, 10, 10, 0.0)), Some((5, 7)));
        assert_eq!(sampler.sample(&ctx_at(5, 7, 10, 10, 0.5)), Some((5, 7)));
        assert_eq!(sampler.sample(&ctx_at(5, 7, 10, 10, 1.0)), Some((5, 7)));
    }

    #[test]
    fn test_sine_wave_axis_x_displaces_x() {
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::X, 0.0);
        // Should return same y, but potentially different x
        let result = sampler.sample(&ctx_at(5, 5, 10, 10, 0.0));
        assert!(result.is_some());
        let (_, y) = result.unwrap();
        assert_eq!(y, 5); // Y should be unchanged
    }

    #[test]
    fn test_sine_wave_axis_y_displaces_y() {
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::Y, 0.0);
        // Should return same x, but potentially different y
        let result = sampler.sample(&ctx_at(5, 5, 10, 10, 0.0));
        assert!(result.is_some());
        let (x, _) = result.unwrap();
        assert_eq!(x, 5); // X should be unchanged
    }

    #[test]
    fn test_sine_wave_handles_edge_positions() {
        // Test that the sampler handles edge positions gracefully
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::X, 0.0);
        // Sampling at x=0 should either return a valid position or None
        let result = sampler.sample(&ctx_at(0, 5, 10, 10, 0.0));
        // Either valid Some(...) or None is acceptable - no panic
        if let Some((x, y)) = result {
            assert_eq!(y, 5); // Y should still be unchanged
            // X might be 0 or nearby
            let _ = x;
        }
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_sine_wave.rs</FILE> - <DESC>SineWave sampler with axis and phase support</DESC>
// <VERS>END OF VERSION: 3.2.0</VERS>
