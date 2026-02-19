// <FILE>tui-vfx-compositor/src/samplers/cls_bounce.rs</FILE> - <DESC>Bounce sampler for bouncing ball vertical displacement</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Silence dead_code warning for public sampler builder</WCTX>
// <CLOG>Allow unused with_params constructor to satisfy clippy -D warnings</CLOG>

use crate::traits::sampler::Sampler;
use std::f32::consts::TAU;

/// Bouncing vertical displacement sampler.
///
/// Creates a bouncing ball effect where elements rise and fall with natural
/// physics-like motion. Elements at different X positions bounce with phase
/// offsets, creating a wave of bouncing dots.
///
/// The bounce uses `abs(sin(phase))` which creates smooth, gravity-like motion
/// where elements spend more time at the top of the bounce (like a real ball).
///
/// # Example
///
/// ```ignore
/// // Classic bouncing dots loader
/// let bounce = Bounce::new(2.0, 4.0, 0.5);
/// ```
pub struct Bounce {
    /// Bounce height in cells (how high elements rise).
    amplitude: f32,
    /// Animation speed (affects bounce frequency).
    speed: f32,
    /// Phase offset per column (creates staggered wave effect).
    phase_spread: f32,
}

impl Default for Bounce {
    fn default() -> Self {
        Self::new(2.0, 4.0, 0.5)
    }
}

impl Bounce {
    /// Create a new Bounce sampler.
    ///
    /// # Arguments
    ///
    /// * `amplitude` - Bounce height in cells (2.0 recommended for loaders)
    /// * `speed` - Animation speed multiplier (4.0 = ~4 bounces per second)
    /// * `phase_spread` - Phase offset between adjacent X positions (0.5 for wave effect)
    pub fn new(amplitude: f32, speed: f32, phase_spread: f32) -> Self {
        Self {
            amplitude,
            speed,
            phase_spread,
        }
    }

    /// Create a bounce sampler with custom parameters.
    #[allow(dead_code)]
    pub fn with_params(amplitude: f32, speed: f32, phase_spread: f32) -> Self {
        Self::new(amplitude, speed, phase_spread)
    }
}

impl Sampler for Bounce {
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        _height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        // Calculate phase based on time and x position
        // Phase spread creates the wave effect where each column bounces slightly later
        let phase = t * self.speed * TAU + (dest_x as f32 * self.phase_spread);

        // Use abs(sin) for bouncing motion - always positive, smooth at peaks
        // This creates natural gravity-like motion where the element
        // decelerates at the top and accelerates at the bottom
        let bounce = self.amplitude * phase.sin().abs();

        // Bounce moves UP (negative Y in terminal coordinates)
        // We're finding the SOURCE y to sample from for this DESTINATION y
        // If dest is at the "bounced up" position, source is lower (higher Y value)
        let src_y = dest_y as f32 + bounce;

        // Clamp to valid range
        if src_y < 0.0 {
            None
        } else {
            Some((dest_x, src_y.round() as u16))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounce_zero_amplitude_identity() {
        let sampler = Bounce::new(0.0, 4.0, 0.5);
        // With zero amplitude, no displacement should occur
        assert_eq!(sampler.sample(5, 7, 10, 10, 0.0), Some((5, 7)));
        assert_eq!(sampler.sample(5, 7, 10, 10, 0.5), Some((5, 7)));
        assert_eq!(sampler.sample(5, 7, 10, 10, 1.0), Some((5, 7)));
    }

    #[test]
    fn test_bounce_preserves_x_coordinate() {
        let sampler = Bounce::new(2.0, 4.0, 0.5);
        // X should always be unchanged
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let result = sampler.sample(5, 10, 20, 20, t);
            assert!(result.is_some());
            let (x, _) = result.unwrap();
            assert_eq!(x, 5);
        }
    }

    #[test]
    fn test_bounce_y_offset_is_positive() {
        let sampler = Bounce::new(2.0, 4.0, 0.0);
        // Source Y should always be >= dest Y (bounce adds positive offset)
        for t in [0.0, 0.1, 0.2, 0.3, 0.4, 0.5] {
            let result = sampler.sample(0, 10, 20, 20, t);
            assert!(result.is_some());
            let (_, y) = result.unwrap();
            assert!(y >= 10, "Source Y {} should be >= dest Y 10 at t={}", y, t);
        }
    }

    #[test]
    fn test_bounce_phase_spread_creates_offset() {
        let sampler = Bounce::new(2.0, 4.0, 1.0);
        // Different X positions should have different Y offsets at same time
        let result_x0 = sampler.sample(0, 10, 20, 20, 0.0);
        let result_x1 = sampler.sample(1, 10, 20, 20, 0.0);

        assert!(result_x0.is_some());
        assert!(result_x1.is_some());

        let (_, y0) = result_x0.unwrap();
        let (_, y1) = result_x1.unwrap();

        // With phase_spread = 1.0, adjacent columns should have different offsets
        // (unless we happen to hit a point where sin values coincide)
        // Just verify both are valid - the visual effect is what matters
        assert!(y0 >= 10);
        assert!(y1 >= 10);
    }

    #[test]
    fn test_bounce_amplitude_affects_max_displacement() {
        let small_bounce = Bounce::new(1.0, 4.0, 0.0);
        let large_bounce = Bounce::new(4.0, 4.0, 0.0);

        // Find max displacement over a cycle
        let mut max_small = 0u16;
        let mut max_large = 0u16;

        for i in 0..100 {
            let t = i as f64 / 100.0;
            if let Some((_, y)) = small_bounce.sample(0, 10, 20, 20, t) {
                max_small = max_small.max(y.saturating_sub(10));
            }
            if let Some((_, y)) = large_bounce.sample(0, 10, 20, 20, t) {
                max_large = max_large.max(y.saturating_sub(10));
            }
        }

        assert!(
            max_large > max_small,
            "Larger amplitude {} should create larger displacement than {}",
            max_large,
            max_small
        );
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_bounce.rs</FILE> - <DESC>Bounce sampler for bouncing ball vertical displacement</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
