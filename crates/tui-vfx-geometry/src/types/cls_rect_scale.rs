// <FILE>tui-vfx-geometry/src/types/cls_rect_scale.rs</FILE> - <DESC>RectScale types for expand/collapse animations</DESC>
// <VERS>VERSION: 2.0.0 - 2025-12-31</VERS>
// <WCTX>V2.2 schema standardization - snake_case serialization</WCTX>
// <CLOG>BREAKING: Added snake_case serde serialization for all variants</CLOG>

use crate::types::Origin;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Rect;

/// Specification for rectangle scaling animations.
///
/// Used for expand/collapse effects like `center_expand`, `bottom_blinds_collapse`,
/// and `spring_disclosure`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RectScaleSpec {
    /// Linear scaling from min dimensions to target dimensions.
    RectScale {
        /// Origin point that stays fixed during scaling
        origin: Origin,
        /// Minimum width at t=0 (0 = collapse to point)
        min_width: u16,
        /// Minimum height at t=0 (0 = collapse to point)
        min_height: u16,
    },
    /// Spring-based scaling with overshoot and settle.
    RectScaleSpring {
        /// Origin point that stays fixed during scaling
        origin: Origin,
        /// Minimum width at t=0
        min_width: u16,
        /// Minimum height at t=0
        min_height: u16,
        /// Spring stiffness (higher = faster oscillation)
        stiffness: f32,
        /// Damping factor (higher = less bounce)
        damping: f32,
    },
    /// Two-phase scaling where width and height animate at different times.
    ///
    /// Useful for CRT power-on (horizontal line first, then expand) or
    /// typewriter effects (width first, height follows).
    ///
    /// # Example: CRT Power-On
    /// - `width_start=0.0, width_end=0.4`: Width reaches full at 40%
    /// - `height_start=0.3, height_end=1.0`: Height starts at 30%, full at end
    /// - Result: Thin horizontal line expands, then grows vertically
    Squash {
        /// Origin point that stays fixed during scaling
        origin: Origin,
        /// Minimum width at animation start
        min_width: u16,
        /// Minimum height at animation start
        min_height: u16,
        /// Progress (0.0-1.0) when width animation starts
        width_start: f32,
        /// Progress (0.0-1.0) when width animation ends (full width)
        width_end: f32,
        /// Progress (0.0-1.0) when height animation starts
        height_start: f32,
        /// Progress (0.0-1.0) when height animation ends (full height)
        height_end: f32,
    },
}

impl Default for RectScaleSpec {
    fn default() -> Self {
        Self::RectScale {
            origin: Origin::Center,
            min_width: 0,
            min_height: 0,
        }
    }
}

impl RectScaleSpec {
    /// Returns the origin for this scale spec.
    pub fn origin(&self) -> Origin {
        match self {
            RectScaleSpec::RectScale { origin, .. } => *origin,
            RectScaleSpec::RectScaleSpring { origin, .. } => *origin,
            RectScaleSpec::Squash { origin, .. } => *origin,
        }
    }

    /// Returns the minimum width for this scale spec.
    pub fn min_width(&self) -> u16 {
        match self {
            RectScaleSpec::RectScale { min_width, .. } => *min_width,
            RectScaleSpec::RectScaleSpring { min_width, .. } => *min_width,
            RectScaleSpec::Squash { min_width, .. } => *min_width,
        }
    }

    /// Returns the minimum height for this scale spec.
    pub fn min_height(&self) -> u16 {
        match self {
            RectScaleSpec::RectScale { min_height, .. } => *min_height,
            RectScaleSpec::RectScaleSpring { min_height, .. } => *min_height,
            RectScaleSpec::Squash { min_height, .. } => *min_height,
        }
    }

    /// Calculates the scaled rectangle at time t.
    ///
    /// # Arguments
    /// * `target` - The target (final) rectangle at t=1.0
    /// * `t` - Progress from 0.0 to 1.0 (may overshoot for spring)
    ///
    /// # Returns
    /// The scaled rectangle at the given time.
    pub fn scale_rect(&self, target: Rect, t: f64) -> Rect {
        let t = t as f32;
        match self {
            RectScaleSpec::RectScale {
                origin,
                min_width,
                min_height,
            } => scale_rect_uniform(*origin, *min_width, *min_height, t, target),
            RectScaleSpec::RectScaleSpring {
                origin,
                min_width,
                min_height,
                stiffness,
                damping,
            } => {
                let scale = spring_factor(t, *stiffness, *damping);
                scale_rect_uniform(*origin, *min_width, *min_height, scale, target)
            }
            RectScaleSpec::Squash {
                origin,
                min_width,
                min_height,
                width_start,
                width_end,
                height_start,
                height_end,
            } => {
                // Calculate independent progress for width and height
                let width_progress = remap_progress(t, *width_start, *width_end);
                let height_progress = remap_progress(t, *height_start, *height_end);

                scale_rect_independent(
                    *origin,
                    *min_width,
                    *min_height,
                    width_progress,
                    height_progress,
                    target,
                )
            }
        }
    }

    /// Create a CRT power-on squash: horizontal line first, then expand vertically.
    pub fn crt_power_on(origin: Origin) -> Self {
        Self::Squash {
            origin,
            min_width: 0,
            min_height: 1, // Start as thin horizontal line
            width_start: 0.0,
            width_end: 0.4, // Width reaches full at 40%
            height_start: 0.3,
            height_end: 1.0, // Height expands from 30% to end
        }
    }

    /// Create a typewriter squash: width first, then height.
    pub fn typewriter(origin: Origin) -> Self {
        Self::Squash {
            origin,
            min_width: 0,
            min_height: 0,
            width_start: 0.0,
            width_end: 0.6, // Width reaches full at 60%
            height_start: 0.4,
            height_end: 1.0, // Height expands from 40% to end
        }
    }
}

/// Scale rectangle with uniform progress for width and height.
fn scale_rect_uniform(origin: Origin, min_w: u16, min_h: u16, t: f32, target: Rect) -> Rect {
    scale_rect_independent(origin, min_w, min_h, t, t, target)
}

/// Scale rectangle with independent progress for width and height.
fn scale_rect_independent(
    origin: Origin,
    min_w: u16,
    min_h: u16,
    width_progress: f32,
    height_progress: f32,
    target: Rect,
) -> Rect {
    let target_w = target.width as f32;
    let target_h = target.height as f32;
    let min_w = min_w as f32;
    let min_h = min_h as f32;

    let width = (min_w + (target_w - min_w) * width_progress.clamp(0.0, 1.0)).round() as u16;
    let height = (min_h + (target_h - min_h) * height_progress.clamp(0.0, 1.0)).round() as u16;

    // Calculate position based on origin
    let (fx, fy) = origin.offset_factors();

    // Anchor point in target rect
    let anchor_x = target.x as f32 + target_w * fx;
    let anchor_y = target.y as f32 + target_h * fy;

    // Position scaled rect so anchor point stays fixed
    let x = (anchor_x - width as f32 * fx).round() as u16;
    let y = (anchor_y - height as f32 * fy).round() as u16;

    Rect::new(x, y, width, height)
}

/// Remap global progress to a local range.
///
/// Returns 0.0 before start, 1.0 after end, and linear interpolation between.
fn remap_progress(t: f32, start: f32, end: f32) -> f32 {
    if t <= start {
        0.0
    } else if t >= end {
        1.0
    } else {
        let range = end - start;
        if range.abs() <= f32::EPSILON {
            1.0
        } else {
            (t - start) / range
        }
    }
}

/// Calculate spring scale factor at time t.
///
/// Uses a damped harmonic oscillator model:
/// - At t=0: returns 0.0
/// - At t=1: returns ~1.0 (spring curve may overshoot; rect sizing clamps to target)
fn spring_factor(t: f32, stiffness: f32, damping: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    // Simple damped spring formula
    // y = 1 - e^(-damping*t) * cos(stiffness * t * PI)
    let decay = (-damping * t * 5.0).exp();
    let oscillation = (stiffness * t * std::f32::consts::PI * 2.0).cos();
    1.0 - decay * oscillation
}

// <FILE>tui-vfx-geometry/src/types/cls_rect_scale.rs</FILE> - <DESC>RectScale types for expand/collapse animations</DESC>
// <VERS>END OF VERSION: 2.0.0 - 2025-12-31</VERS>
