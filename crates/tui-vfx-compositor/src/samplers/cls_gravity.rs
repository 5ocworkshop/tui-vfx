// <FILE>tui-vfx-compositor/src/samplers/cls_gravity.rs</FILE> - <DESC>Gravity sampler for parabolic acceleration displacement</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>New gravity sampler for falling/rising content effects</WCTX>
// <CLOG>Initial creation: parabolic displacement with acceleration, terminal velocity cap, and axis selection</CLOG>

use crate::traits::sampler::Sampler;
use crate::types::cls_sampler_spec::Axis;

/// Gravity displacement sampler.
///
/// Applies parabolic acceleration to content, making it fall (or rise)
/// with increasing speed over time. Unlike pendulum (oscillating) or
/// bounce (abs-sin), gravity accelerates in one direction and stays there.
///
/// The displacement follows `0.5 * acceleration * t²`, capped at
/// `terminal_velocity` to prevent content from flying off screen.
///
/// # Use Cases
///
/// - Falling text/debris during exit transitions
/// - Rising smoke or bubbles (negative acceleration)
/// - Avalanche wipes (content falls away from top to bottom)
/// - Drop-in entrances (reverse time: content decelerates into place)
///
/// # Example
///
/// ```ignore
/// let gravity = Gravity::new(4.0, 8.0, Axis::Y); // fall down, cap at 8 cells
/// ```
pub struct Gravity {
    /// Acceleration in cells per t² unit. Positive = down/right, negative = up/left.
    acceleration: f32,
    /// Maximum displacement in cells (prevents content from flying off screen).
    terminal_velocity: f32,
    /// Which axis gravity pulls along.
    axis: Axis,
}

impl Default for Gravity {
    fn default() -> Self {
        Self::new(4.0, 10.0, Axis::Y)
    }
}

impl Gravity {
    /// Create a new Gravity sampler.
    ///
    /// # Arguments
    ///
    /// * `acceleration` - Cells per t² (4.0 = gentle, 10.0+ = dramatic). Positive = down/right.
    /// * `terminal_velocity` - Maximum displacement cap in cells.
    /// * `axis` - Which axis to apply gravity along.
    pub fn new(acceleration: f32, terminal_velocity: f32, axis: Axis) -> Self {
        Self {
            acceleration,
            terminal_velocity: terminal_velocity.abs(),
            axis,
        }
    }
}

impl Sampler for Gravity {
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        _height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        // Parabolic displacement: d = 0.5 * a * t², capped at terminal velocity
        let raw_displacement = 0.5 * self.acceleration * t * t;
        let displacement = if self.acceleration >= 0.0 {
            raw_displacement.min(self.terminal_velocity)
        } else {
            raw_displacement.max(-self.terminal_velocity)
        };

        match self.axis {
            Axis::X => {
                let src_x = dest_x as f32 + displacement;
                if src_x < 0.0 {
                    None
                } else {
                    Some((src_x.round() as u16, dest_y))
                }
            }
            Axis::Y => {
                let src_y = dest_y as f32 + displacement;
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
    fn zero_time_no_displacement() {
        let sampler = Gravity::new(10.0, 20.0, Axis::Y);
        assert_eq!(sampler.sample(5, 10, 20, 20, 0.0), Some((5, 10)));
    }

    #[test]
    fn positive_acceleration_increases_y() {
        let sampler = Gravity::new(8.0, 20.0, Axis::Y);
        let (_, y0) = sampler.sample(5, 10, 20, 20, 0.0).unwrap();
        let (_, y1) = sampler.sample(5, 10, 20, 20, 0.5).unwrap();
        let (_, y2) = sampler.sample(5, 10, 20, 20, 1.0).unwrap();
        assert!(y1 >= y0, "Should move downward over time");
        assert!(y2 >= y1, "Should accelerate");
    }

    #[test]
    fn negative_acceleration_decreases_y() {
        let sampler = Gravity::new(-8.0, 20.0, Axis::Y);
        let (_, y0) = sampler.sample(5, 10, 20, 20, 0.0).unwrap();
        let (_, y1) = sampler.sample(5, 10, 20, 20, 0.5).unwrap();
        assert!(y1 <= y0, "Negative accel should move upward");
    }

    #[test]
    fn terminal_velocity_caps_displacement() {
        let sampler = Gravity::new(100.0, 3.0, Axis::Y);
        // Even with huge acceleration, displacement capped at 3 cells
        let (_, y) = sampler.sample(5, 10, 20, 20, 10.0).unwrap();
        assert!(y <= 10 + 3, "Should be capped at terminal velocity");
    }

    #[test]
    fn x_axis_preserves_y() {
        let sampler = Gravity::new(4.0, 10.0, Axis::X);
        let (_, y) = sampler.sample(5, 10, 20, 20, 1.0).unwrap();
        assert_eq!(y, 10);
    }

    #[test]
    fn y_axis_preserves_x() {
        let sampler = Gravity::new(4.0, 10.0, Axis::Y);
        let (x, _) = sampler.sample(5, 10, 20, 20, 1.0).unwrap();
        assert_eq!(x, 5);
    }

    #[test]
    fn returns_none_when_source_negative() {
        let sampler = Gravity::new(-100.0, 50.0, Axis::Y);
        // Large negative acceleration at high t should push source above 0
        assert_eq!(sampler.sample(5, 2, 20, 20, 5.0), None);
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_gravity.rs</FILE> - <DESC>Gravity sampler for parabolic acceleration displacement</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
