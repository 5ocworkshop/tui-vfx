// <FILE>tui-vfx-geometry/src/transitions/col_interpolate_position.rs</FILE> - <DESC>Position interpolation with physics paths</DESC>
// <VERS>VERSION: 1.2.2 - 2026-01-01T00:13:20Z</VERS>
// <WCTX>Phase 6: Fixes</WCTX>
// <CLOG>Removed unused variable initialization</CLOG>

use super::col_arc_bezier_point::arc_bezier_point;
use super::col_lerp::lerp;
use crate::paths::cls_bezier_path::BezierPath;
use crate::paths::cls_rectilinear_path::RectilinearPath;
use crate::paths::cls_spiral_path::SpiralPath;
use crate::paths::cls_step_path::StepPath;
use crate::traits::MotionPath;
use crate::types::{PathType, Position};
pub fn interpolate_position(from: Position, to: Position, t: f64, path: &PathType) -> (f32, f32) {
    let t = t.clamp(0.0, 1.0);
    let t32 = t as f32;
    match *path {
        PathType::Linear | PathType::Squash | PathType::Hover => (
            lerp(from.x as f32, to.x as f32, t32),
            lerp(from.y as f32, to.y as f32, t32),
        ),
        PathType::Rectilinear { x_first } => {
            let p = RectilinearPath { x_first };
            p.calculate(t, from, to)
        }
        PathType::Spiral { rotations } => {
            let p = SpiralPath {
                rotations,
                radius_cells: None,
            };
            p.calculate(t, from, to)
        }
        PathType::Step { steps } => {
            let p = StepPath { steps };
            p.calculate(t, from, to)
        }
        PathType::Arc { bulge } => arc_bezier_point(
            from.x as f32,
            from.y as f32,
            to.x as f32,
            to.y as f32,
            t,
            bulge,
        ),
        PathType::Bezier {
            control_x,
            control_y,
        } => {
            let p = BezierPath {
                control_x,
                control_y,
            };
            p.calculate(t, from, to)
        }
        PathType::Spring { stiffness, damping } => {
            let decay = damping * 5.0;
            let freq = stiffness * 2.0;
            let factor = if t32 <= 0.0 {
                0.0
            } else if t32 >= 1.0 {
                1.0
            } else {
                1.0 - (-decay * t32).exp() * (freq * t32).cos()
            };
            (
                from.x as f32 + (to.x as f32 - from.x as f32) * factor,
                from.y as f32 + (to.y as f32 - from.y as f32) * factor,
            )
        }
        PathType::Bounce { bounces, decay } => {
            let factor = if t32 <= 0.0 {
                0.0
            } else if t32 >= 1.0 {
                1.0
            } else if bounces == 0 || decay <= 0.0 {
                t32
            } else {
                let decay_clamped = decay.clamp(0.01, 0.99);
                let sqrt_decay = decay_clamped.sqrt();
                let mut total_time = 1.0_f32;
                let mut bounce_time = sqrt_decay;
                for _ in 0..bounces {
                    total_time += 2.0 * bounce_time;
                    bounce_time *= sqrt_decay;
                }
                let scaled_t = t32 * total_time;
                let mut time_used;
                let mut current_bounce = 0_u8;
                let mut phase_duration: f32;
                let local_t: f32;
                let in_initial_fall;
                phase_duration = 1.0;
                if scaled_t <= phase_duration {
                    local_t = scaled_t / phase_duration;
                    in_initial_fall = true;
                } else {
                    time_used = phase_duration;
                    in_initial_fall = false;
                    bounce_time = sqrt_decay;
                    local_t = loop {
                        phase_duration = 2.0 * bounce_time;
                        if scaled_t <= time_used + phase_duration {
                            current_bounce += 1;
                            break (scaled_t - time_used) / phase_duration;
                        }
                        time_used += phase_duration;
                        bounce_time *= sqrt_decay;
                        current_bounce += 1;
                        if current_bounce > bounces {
                            break 1.0;
                        }
                    };
                }
                if in_initial_fall {
                    local_t * local_t
                } else if current_bounce > 0 {
                    let bounce_height = decay_clamped.powi(current_bounce as i32);
                    let arc_offset = 4.0 * bounce_height * local_t * (1.0 - local_t);
                    1.0 - arc_offset
                } else {
                    1.0
                }
            };
            (
                from.x as f32 + (to.x as f32 - from.x as f32) * factor,
                from.y as f32 + (to.y as f32 - from.y as f32) * factor,
            )
        }
        PathType::Projectile {
            arc_height,
            gravity,
        } => {
            let parabola_offset = 4.0 * arc_height * t32 * (1.0 - t32);
            let base_x = lerp(from.x as f32, to.x as f32, t32);
            let base_y = lerp(from.y as f32, to.y as f32, t32);
            let gravity_factor = (gravity / 500.0).max(0.1);
            (base_x, base_y + parabola_offset * gravity_factor)
        }
        PathType::Friction { drag } => {
            let factor = if t32 <= 0.0 {
                0.0
            } else if t32 >= 1.0 {
                1.0
            } else {
                let k = 5.0 / drag.max(0.1);
                1.0 - (-drag * t32 * k).exp()
            };
            (
                from.x as f32 + (to.x as f32 - from.x as f32) * factor,
                from.y as f32 + (to.y as f32 - from.y as f32) * factor,
            )
        }
        PathType::Orbit {
            revolutions,
            direction,
        } => {
            let cx = (from.x as f32 + to.x as f32) / 2.0;
            let cy = (from.y as f32 + to.y as f32) / 2.0;
            let dx = to.x as f32 - from.x as f32;
            let dy = to.y as f32 - from.y as f32;
            let radius = (dx * dx + dy * dy).sqrt() / 2.0;
            if radius < 0.001 {
                return (from.x as f32, from.y as f32);
            }
            let start_angle = (from.y as f32 - cy).atan2(from.x as f32 - cx);
            let extra_revs = revolutions.abs() * std::f32::consts::TAU;
            let base_arc = std::f32::consts::PI;
            let total_arc = (base_arc + extra_revs) * direction.signum();
            let current_angle = start_angle + total_arc * t32;
            let blend_start = 0.95_f32;
            if t32 > blend_start {
                let blend_t = (t32 - blend_start) / (1.0 - blend_start);
                let orbit_x = cx + radius * current_angle.cos();
                let orbit_y = cy + radius * current_angle.sin();
                (
                    orbit_x + (to.x as f32 - orbit_x) * blend_t,
                    orbit_y + (to.y as f32 - orbit_y) * blend_t,
                )
            } else {
                (
                    cx + radius * current_angle.cos(),
                    cy + radius * current_angle.sin(),
                )
            }
        }
        PathType::Pendulum {
            amplitude,
            oscillations,
            damping,
        } => {
            let base_x = lerp(from.x as f32, to.x as f32, t32);
            let base_y = lerp(from.y as f32, to.y as f32, t32);
            if t32 <= 0.0 || t32 >= 1.0 || amplitude <= 0.0 || oscillations <= 0.0 {
                return (base_x, base_y);
            }
            let omega = oscillations * std::f32::consts::TAU;
            let decay_factor = (-damping * t32 * 3.0).exp();
            let oscillation = (omega * t32).sin() * decay_factor;
            let dx = to.x as f32 - from.x as f32;
            let dy = to.y as f32 - from.y as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 0.001 {
                let offset = amplitude * oscillation * (1.0 - t32);
                return (base_x + offset, base_y);
            }
            let perp_x = -dy / dist;
            let perp_y = dx / dist;
            let offset = amplitude * oscillation * (1.0 - t32);
            (base_x + offset * perp_x, base_y + offset * perp_y)
        }
    }
}

// <FILE>tui-vfx-geometry/src/transitions/col_interpolate_position.rs</FILE> - <DESC>Position interpolation with physics paths</DESC>
// <VERS>END OF VERSION: 1.2.2 - 2026-01-01T00:13:20Z</VERS>
