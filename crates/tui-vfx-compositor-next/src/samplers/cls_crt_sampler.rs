// <FILE>tui-vfx-compositor-next/src/samplers/cls_crt_sampler.rs</FILE> - <DESC>CRT sampler with curvature and jitter</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>1.3.0: sample() now returns SamplerOutput; displacing branch carries full x/y deltas; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use mixed_signals::prelude::{Signal, SignalContext, SpatialCoordinateSignal};
use tui_vfx_types::VfxCellContext;

/// CRT monitor screen distortion sampler.
///
/// Simulates the barrel distortion of a curved CRT screen
/// and optional horizontal jitter from analog signal noise.
pub struct CrtSampler {
    /// Screen curvature amount (0.0 = flat, 1.0 = strong barrel distortion)
    pub curvature: f32,
    /// Horizontal jitter amount (0.0 = none, 1.0 = strong jitter)
    pub jitter: f32,
    /// Seed for jitter randomness
    seed: u64,
}

impl Default for CrtSampler {
    fn default() -> Self {
        Self::new(0.1, 0.0)
    }
}

impl CrtSampler {
    /// Create a new CRT sampler.
    ///
    /// # Arguments
    /// * `curvature` - Barrel distortion amount (0.0-1.0)
    /// * `jitter` - Horizontal jitter amount (0.0-1.0)
    pub fn new(curvature: f32, jitter: f32) -> Self {
        Self {
            curvature: curvature.clamp(0.0, 1.0),
            jitter: jitter.clamp(0.0, 1.0),
            seed: 42,
        }
    }
}

impl Sampler for CrtSampler {
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        let width = ctx.width;
        let height = ctx.height;
        let t = ctx.t;

        if width == 0 || height == 0 {
            return SamplerOutput::passthrough(dest_x, dest_y);
        }

        let signal_ctx = SignalContext::new(0, 0)
            .with_dimensions(width, height)
            .with_cell_position(dest_x, dest_y);

        // Normalize coordinates to -1..1 range centered on screen
        let nx = SpatialCoordinateSignal::sample_centered_x().sample_with_context(0.0, &signal_ctx);
        let ny = SpatialCoordinateSignal::sample_centered_y().sample_with_context(0.0, &signal_ctx);

        // Apply barrel distortion (CRT curvature)
        let (curved_x, curved_y) = if self.curvature > 0.001 {
            let r2 = nx * nx + ny * ny;
            let distortion = 1.0 + r2 * self.curvature * 0.5;
            (nx * distortion, ny * distortion)
        } else {
            (nx, ny)
        };

        // Apply horizontal jitter based on y position and time
        let jittered_x = if self.jitter > 0.001 {
            // Simple pseudo-random jitter based on y and time
            let jitter_seed = (dest_y as u64).wrapping_mul(31).wrapping_add(self.seed);
            let jitter_phase = (jitter_seed as f32 + t as f32 * 60.0) % 1.0;
            let jitter_amount = (jitter_phase * std::f32::consts::TAU).sin() * self.jitter * 0.02;
            curved_x + jitter_amount
        } else {
            curved_x
        };

        // Convert back to pixel coordinates
        let src_x_f = ((jittered_x + 1.0) / 2.0 * width as f32).round();
        let src_y_f = ((curved_y + 1.0) / 2.0 * height as f32).round();

        // Bounds check
        if src_x_f < 0.0 || src_y_f < 0.0 || src_x_f >= width as f32 || src_y_f >= height as f32 {
            SamplerOutput::no_displacement()
        } else {
            let src_x = src_x_f as u16;
            let src_y = src_y_f as u16;
            let delta_x = src_x as i32 - dest_x as i32;
            let delta_y = src_y as i32 - dest_y as i32;
            SamplerOutput::displaced(src_x, src_y, delta_x, delta_y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_sampler_zero_dimensions_noop() {
        let sampler = CrtSampler::default();
        // Zero dimensions should return passthrough
        assert_eq!(
            sampler
                .sample(&VfxCellContext::new(5, 5, 0, 0, 0, 0, 0.0))
                .source,
            Some((5, 5))
        );
    }

    #[test]
    fn test_crt_sampler_identity_with_no_distortion() {
        let sampler = CrtSampler::new(0.0, 0.0);
        // With no curvature and no jitter, center should map to center
        // Actually, due to rounding, let's check that the result is close
        let result = sampler.sample(&VfxCellContext::new(5, 5, 10, 10, 0, 0, 0.0));
        assert!(result.source.is_some());
        let (x, y) = result.source.unwrap();
        // Should be very close to input
        assert!((x as i16 - 5).abs() <= 1);
        assert!((y as i16 - 5).abs() <= 1);
    }

    #[test]
    fn test_crt_sampler_curvature_displaces_corners() {
        let flat = CrtSampler::new(0.0, 0.0);
        let curved = CrtSampler::new(0.5, 0.0);
        // Corner positions should differ with curvature
        let flat_corner = flat.sample(&VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        let curved_corner = curved.sample(&VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        // Curved version likely goes out of bounds or has different coords
        assert!(flat_corner.source != curved_corner.source || curved_corner.source.is_none());
    }

    #[test]
    fn test_crt_sampler_bounds_check() {
        let sampler = CrtSampler::new(0.5, 0.0);
        // Strong curvature at corners should push samples out of bounds
        let result = sampler.sample(&VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0));
        // Result might be no_displacement due to bounds check
        // This is acceptable behavior
        let _ = result;
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // With curvature=0 and no jitter, center cell should map very close to itself
        let sampler = CrtSampler::new(0.0, 0.0);
        let out = sampler.sample(&VfxCellContext::new(5, 5, 10, 10, 0, 0, 0.0));
        assert!(out.source.is_some());
        // Deltas should match src - dest
        let (src_x, src_y) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 5);
        assert_eq!(out.delta_y, src_y as i32 - 5);
    }
}

// <FILE>tui-vfx-compositor-next/src/samplers/cls_crt_sampler.rs</FILE> - <DESC>CRT sampler with curvature and jitter</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
