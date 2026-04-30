// <FILE>tui-vfx-compositor/src/samplers/cls_gravity.rs</FILE> - <DESC>Gravity sampler for parabolic acceleration displacement</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>1.2.0: sample() now returns SamplerOutput; displacing branches carry axis delta; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use crate::types::cls_sampler_spec::Axis;
use tui_vfx_types::VfxCellContext;

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
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let t = ctx.t as f32;
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;

        // Parabolic displacement: d = 0.5 * a * t², capped at terminal velocity
        let raw_displacement = 0.5 * self.acceleration * t * t;
        let displacement = if self.acceleration >= 0.0 {
            raw_displacement.min(self.terminal_velocity)
        } else {
            raw_displacement.max(-self.terminal_velocity)
        };

        match self.axis {
            Axis::X => {
                let src_x_f = dest_x as f32 + displacement;
                if src_x_f < 0.0 {
                    SamplerOutput::no_displacement()
                } else {
                    let src_x = src_x_f.round() as u16;
                    let delta_x = src_x as i32 - dest_x as i32;
                    SamplerOutput::displaced(src_x, dest_y, delta_x, 0)
                }
            }
            Axis::Y => {
                let src_y_f = dest_y as f32 + displacement;
                if src_y_f < 0.0 {
                    SamplerOutput::no_displacement()
                } else {
                    let src_y = src_y_f.round() as u16;
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

    fn ctx_at(x: u16, y: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, 20, 20, 0, 0, t)
    }

    #[test]
    fn zero_time_no_displacement() {
        let sampler = Gravity::new(10.0, 20.0, Axis::Y);
        assert_eq!(sampler.sample(&ctx_at(5, 10, 0.0)).source, Some((5, 10)));
    }

    #[test]
    fn positive_acceleration_increases_y() {
        let sampler = Gravity::new(8.0, 20.0, Axis::Y);
        let (_, y0) = sampler.sample(&ctx_at(5, 10, 0.0)).source.unwrap();
        let (_, y1) = sampler.sample(&ctx_at(5, 10, 0.5)).source.unwrap();
        let (_, y2) = sampler.sample(&ctx_at(5, 10, 1.0)).source.unwrap();
        assert!(y1 >= y0, "Should move downward over time");
        assert!(y2 >= y1, "Should accelerate");
    }

    #[test]
    fn negative_acceleration_decreases_y() {
        let sampler = Gravity::new(-8.0, 20.0, Axis::Y);
        let (_, y0) = sampler.sample(&ctx_at(5, 10, 0.0)).source.unwrap();
        let (_, y1) = sampler.sample(&ctx_at(5, 10, 0.5)).source.unwrap();
        assert!(y1 <= y0, "Negative accel should move upward");
    }

    #[test]
    fn terminal_velocity_caps_displacement() {
        let sampler = Gravity::new(100.0, 3.0, Axis::Y);
        // Even with huge acceleration, displacement capped at 3 cells
        let (_, y) = sampler.sample(&ctx_at(5, 10, 10.0)).source.unwrap();
        assert!(y <= 10 + 3, "Should be capped at terminal velocity");
    }

    #[test]
    fn x_axis_preserves_y() {
        let sampler = Gravity::new(4.0, 10.0, Axis::X);
        let (_, y) = sampler.sample(&ctx_at(5, 10, 1.0)).source.unwrap();
        assert_eq!(y, 10);
    }

    #[test]
    fn y_axis_preserves_x() {
        let sampler = Gravity::new(4.0, 10.0, Axis::Y);
        let (x, _) = sampler.sample(&ctx_at(5, 10, 1.0)).source.unwrap();
        assert_eq!(x, 5);
    }

    #[test]
    fn returns_none_when_source_negative() {
        let sampler = Gravity::new(-100.0, 50.0, Axis::Y);
        // Large negative acceleration at high t should push source above 0
        assert_eq!(sampler.sample(&ctx_at(5, 2, 5.0)).source, None);
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // At t=1.0: displacement = 0.5 * 8.0 * 1.0 = 4.0; src_y = 10 + 4 = 14; delta_y = 4
        let sampler = Gravity::new(8.0, 20.0, Axis::Y);
        let out = sampler.sample(&ctx_at(5, 10, 1.0));
        assert!(out.source.is_some());
        assert_eq!(out.delta_x, 0);
        assert!(out.delta_y > 0);
        let (_, src_y) = out.source.unwrap();
        assert_eq!(out.delta_y, src_y as i32 - 10);
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_gravity.rs</FILE> - <DESC>Gravity sampler for parabolic acceleration displacement</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
