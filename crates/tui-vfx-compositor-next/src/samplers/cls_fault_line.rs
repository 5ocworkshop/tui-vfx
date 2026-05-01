// <FILE>tui-vfx-compositor-next/src/samplers/cls_fault_line.rs</FILE> - <DESC>FaultLine sampler implementation</DESC>
// <VERS>VERSION: 2.3.1</VERS>
// <WCTX>v3.1 native debug-recipes closure: support fixed lower-half horizontal offsets authored by debug sampler fixtures.</WCTX>
// <CLOG>2.3.1: crop dynamic right-edge displacement instead of sampling outside the source width.
// 2.3.0: add optional fixed_offset mode while preserving the existing dynamic fault-line default.
// 2.2.0: sample() now returns SamplerOutput; displacing branch carries delta_x; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use std::hash::{Hash, Hasher};
use tui_vfx_types::VfxCellContext;

/// Fault line displacement effect - splits content horizontally with offset.
///
/// Creates an earthquake-like effect where the top and bottom halves
/// of the content slide in opposite directions.
pub struct FaultLine {
    /// Seed for deterministic split position
    pub seed: u64,
    /// Intensity of the displacement (multiplier for offset)
    pub intensity: f32,
    /// Bias toward upper (negative) or lower (positive) split position
    pub split_bias: f32,
    /// Optional fixed offset for lower-half player-authored fixtures.
    pub fixed_offset: Option<i16>,
}

impl Default for FaultLine {
    fn default() -> Self {
        Self::new(42, 1.0, 0.0)
    }
}

impl FaultLine {
    /// Create a new FaultLine sampler.
    ///
    /// # Arguments
    /// * `seed` - Seed for deterministic split position variation
    /// * `intensity` - Displacement intensity multiplier
    /// * `split_bias` - Bias for split position (-1.0 to 1.0)
    pub fn new(seed: u64, intensity: f32, split_bias: f32) -> Self {
        Self {
            seed,
            intensity,
            split_bias: split_bias.clamp(-1.0, 1.0),
            fixed_offset: None,
        }
    }

    /// Use a fixed lower-half horizontal offset instead of dynamic split motion.
    pub fn with_fixed_offset(mut self, offset: i16) -> Self {
        self.fixed_offset = Some(offset);
        self
    }

    /// Compute split position based on seed and bias
    fn split_y(&self, height: u16) -> u16 {
        // Need at least 3 rows to have a meaningful split (top, split, bottom)
        if height < 3 {
            return height / 2;
        }
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.seed.hash(&mut hasher);
        let hash = hasher.finish();
        let base_split = (hash % height as u64) as f32;
        let biased = base_split + (self.split_bias * height as f32 * 0.3);
        biased.clamp(1.0, (height - 1) as f32) as u16
    }
}

impl Sampler for FaultLine {
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        let height = ctx.height;

        if let Some(offset) = self.fixed_offset {
            if dest_y < height / 2 {
                return SamplerOutput::passthrough(dest_x, dest_y);
            }
            let src_x_i = dest_x as i32 - offset as i32;
            if src_x_i < 0 || src_x_i >= ctx.width as i32 {
                return SamplerOutput::no_displacement();
            }
            let src_x = src_x_i as u16;
            return SamplerOutput::displaced(src_x, dest_y, src_x as i32 - dest_x as i32, 0);
        }

        let t = ctx.t as f32;

        // Calculate split position using actual widget height
        let split_y = self.split_y(height);

        // At t=0, offset is large. At t=1, offset is 0 (content comes together).
        let base_offset = (1.0 - t) * 20.0 * self.intensity;
        let offset = base_offset.round() as i32;

        let src_x_i = if dest_y < split_y {
            dest_x as i32 - offset
        } else {
            dest_x as i32 + offset
        };

        if src_x_i < 0 || src_x_i >= ctx.width as i32 {
            SamplerOutput::no_displacement()
        } else {
            let src_x = src_x_i as u16;
            let delta_x = src_x as i32 - dest_x as i32;
            SamplerOutput::displaced(src_x, dest_y, delta_x, 0)
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
    fn test_fault_line_small_height_no_panic() {
        // Regression test: height < 3 should not panic
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // Should not panic with small heights
        let _ = sampler.sample(&ctx_at(5, 0, 10, 1, 0.5));
        let _ = sampler.sample(&ctx_at(5, 0, 10, 2, 0.5));
    }

    #[test]
    fn test_fault_line_identity_at_t1() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // At t=1.0, offset should be 0 (content comes together)
        let result = sampler.sample(&ctx_at(5, 0, 10, 10, 1.0));
        assert_eq!(result.source, Some((5, 0)));
    }

    #[test]
    fn test_fault_line_displacement_at_t0() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // At t=0, there should be displacement
        let result = sampler.sample(&ctx_at(50, 0, 100, 10, 0.0));
        // Above split: x - offset
        // offset = (1-0) * 20 * 1.0 = 20
        // src_x = 50 - 20 = 30
        assert!(result.source.is_some());
        let (x, _) = result.source.unwrap();
        assert_ne!(x, 50); // Should be displaced
    }

    #[test]
    fn test_fault_line_opposite_directions() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        let _split_y = sampler.split_y(10);

        // Above split moves one direction, below split moves opposite
        let above = sampler.sample(&ctx_at(50, 0, 100, 10, 0.0));
        let below = sampler.sample(&ctx_at(50, 9, 100, 10, 0.0));

        if let (Some((ax, _)), Some((bx, _))) = (above.source, below.source) {
            // One should be < 50, other should be > 50 (or at different offsets)
            assert!(ax != bx);
        }
    }

    #[test]
    fn test_fault_line_negative_x_returns_none() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // Small x with large offset should return no_displacement
        let result = sampler.sample(&ctx_at(5, 0, 100, 10, 0.0));
        // offset = 20, src_x = 5 - 20 = -15 < 0
        assert_eq!(result.source, None);
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // At t=0.5: offset = (1-0.5) * 20 * 1.0 = 10; row 0 is above split -> src_x = 50 - 10 = 40
        let sampler = FaultLine::new(1, 1.0, 0.0);
        let out = sampler.sample(&ctx_at(50, 0, 100, 10, 0.5));
        assert!(out.source.is_some());
        assert_eq!(out.delta_y, 0);
        let (src_x, _) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 50);
        assert!(out.delta_x != 0);
    }
}

// <FILE>tui-vfx-compositor-next/src/samplers/cls_fault_line.rs</FILE> - <DESC>FaultLine sampler implementation</DESC>
// <VERS>END OF VERSION: 2.3.1</VERS>
