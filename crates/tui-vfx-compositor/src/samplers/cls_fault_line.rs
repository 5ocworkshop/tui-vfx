// <FILE>tui-vfx-compositor/src/samplers/cls_fault_line.rs</FILE> - <DESC>FaultLine sampler implementation</DESC>
// <VERS>VERSION: 2.0.1</VERS>
// <WCTX>FaultLine edge case fix</WCTX>
// <CLOG>BUG FIX: Handle height < 3 to prevent clamp panic when min > max</CLOG>

use crate::traits::sampler::Sampler;
use std::hash::{Hash, Hasher};

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
        }
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
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        // Calculate split position using actual widget height
        let split_y = self.split_y(height);

        // At t=0, offset is large. At t=1, offset is 0 (content comes together).
        let base_offset = (1.0 - t) * 20.0 * self.intensity;
        let offset = base_offset.round() as i32;

        let src_x = if dest_y < split_y {
            dest_x as i32 - offset
        } else {
            dest_x as i32 + offset
        };

        if src_x < 0 {
            None
        } else {
            Some((src_x as u16, dest_y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fault_line_small_height_no_panic() {
        // Regression test: height < 3 should not panic
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // Should not panic with small heights
        let _ = sampler.sample(5, 0, 10, 1, 0.5);
        let _ = sampler.sample(5, 0, 10, 2, 0.5);
    }

    #[test]
    fn test_fault_line_identity_at_t1() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // At t=1.0, offset should be 0 (content comes together)
        let result = sampler.sample(5, 0, 10, 10, 1.0);
        assert_eq!(result, Some((5, 0)));
    }

    #[test]
    fn test_fault_line_displacement_at_t0() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // At t=0, there should be displacement
        let result = sampler.sample(50, 0, 100, 10, 0.0);
        // Above split: x - offset
        // offset = (1-0) * 20 * 1.0 = 20
        // src_x = 50 - 20 = 30
        assert!(result.is_some());
        let (x, _) = result.unwrap();
        assert_ne!(x, 50); // Should be displaced
    }

    #[test]
    fn test_fault_line_opposite_directions() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        let _split_y = sampler.split_y(10);

        // Above split moves one direction, below split moves opposite
        let above = sampler.sample(50, 0, 100, 10, 0.0);
        let below = sampler.sample(50, 9, 100, 10, 0.0);

        if let (Some((ax, _)), Some((bx, _))) = (above, below) {
            // One should be < 50, other should be > 50 (or at different offsets)
            assert!(ax != bx);
        }
    }

    #[test]
    fn test_fault_line_negative_x_returns_none() {
        let sampler = FaultLine::new(1, 1.0, 0.0);
        // Small x with large offset should return None
        let result = sampler.sample(5, 0, 100, 10, 0.0);
        // offset = 20, src_x = 5 - 20 = -15 < 0
        assert_eq!(result, None);
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_fault_line.rs</FILE> - <DESC>FaultLine sampler implementation</DESC>
// <VERS>END OF VERSION: 2.0.1</VERS>
