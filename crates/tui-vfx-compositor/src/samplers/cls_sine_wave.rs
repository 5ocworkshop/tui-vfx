// <FILE>tui-vfx-compositor/src/samplers/cls_sine_wave.rs</FILE> - <DESC>SineWave sampler with axis and phase support</DESC>
// <VERS>VERSION: 3.3.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>3.3.0: sample() now returns SamplerOutput; displacing branches carry axis delta; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use crate::types::cls_sampler_spec::Axis;
use mixed_signals::prelude::{Normalized, Remap, Signal, SignalExt, Sine};
use tui_vfx_types::VfxCellContext;

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
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let t = ctx.t as f32;
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        match self.axis {
            Axis::X => {
                // Wave along Y, displaces X (horizontal wave motion)
                let phase = dest_y as f32 * self.spatial_freq + t * self.speed + self.phase_offset;
                let offset = self.signal.sample(phase.into());
                let src_x_f = (dest_x as f32 + offset).round();
                if src_x_f < 0.0 {
                    SamplerOutput::no_displacement()
                } else {
                    let src_x = src_x_f as u16;
                    let delta_x = src_x as i32 - dest_x as i32;
                    SamplerOutput::displaced(src_x, dest_y, delta_x, 0)
                }
            }
            Axis::Y => {
                // Wave along X, displaces Y (vertical wave motion)
                let phase = dest_x as f32 * self.spatial_freq + t * self.speed + self.phase_offset;
                let offset = self.signal.sample(phase.into());
                let src_y_f = (dest_y as f32 + offset).round();
                if src_y_f < 0.0 {
                    SamplerOutput::no_displacement()
                } else {
                    let src_y = src_y_f as u16;
                    let delta_y = src_y as i32 - dest_y as i32;
                    SamplerOutput::displaced(dest_x, src_y, 0, delta_y)
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
        assert_eq!(
            sampler.sample(&ctx_at(5, 7, 10, 10, 0.0)).source,
            Some((5, 7))
        );
        assert_eq!(
            sampler.sample(&ctx_at(5, 7, 10, 10, 0.5)).source,
            Some((5, 7))
        );
        assert_eq!(
            sampler.sample(&ctx_at(5, 7, 10, 10, 1.0)).source,
            Some((5, 7))
        );
    }

    #[test]
    fn test_sine_wave_axis_x_displaces_x() {
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::X, 0.0);
        // Should return same y, but potentially different x
        let result = sampler.sample(&ctx_at(5, 5, 10, 10, 0.0));
        assert!(result.source.is_some());
        let (_, y) = result.source.unwrap();
        assert_eq!(y, 5); // Y should be unchanged
    }

    #[test]
    fn test_sine_wave_axis_y_displaces_y() {
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::Y, 0.0);
        // Should return same x, but potentially different y
        let result = sampler.sample(&ctx_at(5, 5, 10, 10, 0.0));
        assert!(result.source.is_some());
        let (x, _) = result.source.unwrap();
        assert_eq!(x, 5); // X should be unchanged
    }

    #[test]
    fn test_sine_wave_handles_edge_positions() {
        // Test that the sampler handles edge positions gracefully
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::X, 0.0);
        // Sampling at x=0 should either return a valid position or no_displacement
        let result = sampler.sample(&ctx_at(0, 5, 10, 10, 0.0));
        // Either valid Some(...) or None is acceptable - no panic
        if let Some((x, y)) = result.source {
            assert_eq!(y, 5); // Y should still be unchanged
            // X might be 0 or nearby
            let _ = x;
        }
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // amplitude=2, spatial_freq=0.5, speed=10, Axis::X, phase_offset=0
        // At dest_y=5, t=0: phase = 5*0.5 + 0 + 0 = 2.5; offset is non-zero for non-integer phase
        // We just verify structure: delta_y=0 and delta_x == src_x - dest_x
        let sampler = SineWave::new(2.0, 0.5, 10.0, Axis::X, 0.0);
        let out = sampler.sample(&ctx_at(10, 5, 20, 20, 0.0));
        assert!(out.source.is_some());
        assert_eq!(out.delta_y, 0);
        let (src_x, _) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 10);
        // With amplitude=2 and phase=2.5, displacement should be non-zero
        assert!(out.delta_x != 0);
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_sine_wave.rs</FILE> - <DESC>SineWave sampler with axis and phase support</DESC>
// <VERS>END OF VERSION: 3.3.0</VERS>
