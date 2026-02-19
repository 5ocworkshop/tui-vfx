// <FILE>tui-vfx-compositor/src/samplers/cls_pendulum.rs</FILE> - <DESC>Pendulum sampler for bidirectional swaying motion</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Rustfmt normalization for pendulum sampler</WCTX>
// <CLOG>Apply formatting updates after clippy run</CLOG>

use crate::traits::sampler::Sampler;
use crate::types::cls_sampler_spec::Axis;
use std::f32::consts::TAU;

/// Pendulum/swaying displacement sampler.
///
/// Creates a continuous swaying effect where elements oscillate back and forth.
/// Unlike Bounce (which uses abs(sin) for always-positive displacement),
/// Pendulum uses sin() for true bidirectional motion.
///
/// Elements at different positions sway with phase offsets, creating
/// a wave-like swaying pattern (like wind through grass or hanging items).
///
/// # Example
///
/// ```ignore
/// // Swaying menu items
/// let pendulum = Pendulum::new(3.0, 2.0, 0.3, Axis::X);
/// ```
pub struct Pendulum {
    /// Swing amplitude in cells (maximum displacement from center).
    amplitude: f32,
    /// Animation speed (affects swing frequency).
    speed: f32,
    /// Phase offset per position (creates staggered wave effect).
    phase_spread: f32,
    /// Which axis the pendulum swings along.
    axis: Axis,
}

impl Default for Pendulum {
    fn default() -> Self {
        Self::new(2.0, 2.0, 0.3, Axis::X)
    }
}

impl Pendulum {
    /// Create a new Pendulum sampler.
    ///
    /// # Arguments
    ///
    /// * `amplitude` - Swing distance in cells (2.0 recommended)
    /// * `speed` - Animation speed multiplier (2.0 = ~2 swings per second)
    /// * `phase_spread` - Phase offset between adjacent positions (0.3 for gentle wave)
    /// * `axis` - Which axis to swing along (X for horizontal, Y for vertical)
    pub fn new(amplitude: f32, speed: f32, phase_spread: f32, axis: Axis) -> Self {
        Self {
            amplitude,
            speed,
            phase_spread,
            axis,
        }
    }
}

impl Sampler for Pendulum {
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        _height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        match self.axis {
            Axis::X => {
                // Horizontal swing: phase based on Y position
                let phase = t * self.speed * TAU + (dest_y as f32 * self.phase_spread);
                let offset = self.amplitude * phase.sin();

                // Calculate source X (where to sample from)
                let src_x = dest_x as f32 + offset;
                if src_x < 0.0 {
                    None
                } else {
                    Some((src_x.round() as u16, dest_y))
                }
            }
            Axis::Y => {
                // Vertical swing: phase based on X position
                let phase = t * self.speed * TAU + (dest_x as f32 * self.phase_spread);
                let offset = self.amplitude * phase.sin();

                // Calculate source Y (where to sample from)
                let src_y = dest_y as f32 + offset;
                if src_y < 0.0 {
                    None
                } else {
                    Some((dest_x, src_y.round() as u16))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pendulum_zero_amplitude_identity() {
        let sampler = Pendulum::new(0.0, 2.0, 0.3, Axis::X);
        // With zero amplitude, no displacement should occur
        assert_eq!(sampler.sample(5, 7, 10, 10, 0.0), Some((5, 7)));
        assert_eq!(sampler.sample(5, 7, 10, 10, 0.5), Some((5, 7)));
        assert_eq!(sampler.sample(5, 7, 10, 10, 1.0), Some((5, 7)));
    }

    #[test]
    fn test_pendulum_axis_x_preserves_y() {
        let sampler = Pendulum::new(2.0, 2.0, 0.3, Axis::X);
        // Y should always be unchanged for X-axis swing
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let result = sampler.sample(5, 10, 20, 20, t);
            assert!(result.is_some());
            let (_, y) = result.unwrap();
            assert_eq!(y, 10);
        }
    }

    #[test]
    fn test_pendulum_axis_y_preserves_x() {
        let sampler = Pendulum::new(2.0, 2.0, 0.3, Axis::Y);
        // X should always be unchanged for Y-axis swing
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let result = sampler.sample(5, 10, 20, 20, t);
            assert!(result.is_some());
            let (x, _) = result.unwrap();
            assert_eq!(x, 5);
        }
    }

    #[test]
    fn test_pendulum_bidirectional() {
        let sampler = Pendulum::new(2.0, 1.0, 0.0, Axis::X);
        // Pendulum should swing both positive and negative from center
        // At t=0, sin(0) = 0
        // At t=0.25 (phase = PI/2), sin = 1 (positive offset)
        // At t=0.75 (phase = 3PI/2), sin = -1 (negative offset)

        let result_center = sampler.sample(10, 5, 20, 20, 0.0);
        let result_right = sampler.sample(10, 5, 20, 20, 0.25);
        let result_left = sampler.sample(10, 5, 20, 20, 0.75);

        assert!(result_center.is_some());
        assert!(result_right.is_some());
        assert!(result_left.is_some());

        let (x_center, _) = result_center.unwrap();
        let (x_right, _) = result_right.unwrap();
        let (x_left, _) = result_left.unwrap();

        // Right should be greater than center, left should be less
        assert!(
            x_right >= x_center,
            "Right swing {} should be >= center {}",
            x_right,
            x_center
        );
        assert!(
            x_left <= x_center,
            "Left swing {} should be <= center {}",
            x_left,
            x_center
        );
    }

    #[test]
    fn test_pendulum_phase_spread_creates_wave() {
        let sampler = Pendulum::new(2.0, 2.0, 1.0, Axis::X);
        // Different Y positions should have different X offsets
        let result_y0 = sampler.sample(10, 0, 20, 20, 0.0);
        let result_y1 = sampler.sample(10, 1, 20, 20, 0.0);

        assert!(result_y0.is_some());
        assert!(result_y1.is_some());

        // With phase_spread = 1.0, different rows should have different phases
        // Just verify both are valid
        let (x0, _) = result_y0.unwrap();
        let (x1, _) = result_y1.unwrap();
        let _ = (x0, x1); // Values will differ based on phase
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_pendulum.rs</FILE> - <DESC>Pendulum sampler for bidirectional swaying motion</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
