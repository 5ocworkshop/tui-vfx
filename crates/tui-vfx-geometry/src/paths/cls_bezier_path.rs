// <FILE>tui-vfx-geometry/src/paths/cls_bezier_path.rs</FILE> - <DESC>Quadratic Bezier with explicit control point</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-25</VERS>
// <WCTX>Mixed-signals migration Phase 5 - WP3</WCTX>
// <CLOG>Migrated quadratic Bezier formula to mixed_signals::math module</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
use mixed_signals::math::quadratic_bezier;

/// A quadratic Bezier path with an explicit control point.
///
/// Unlike `ArcPath` which computes the control point from a bulge factor,
/// `BezierPath` uses a user-specified control point for precise curve shaping.
///
/// This enables animations that pass through arbitrary intermediate positions,
/// such as an arc from mid-left to mid-right via a top-center waypoint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BezierPath {
    /// X coordinate of the control point (in screen coordinates).
    pub control_x: f32,
    /// Y coordinate of the control point (in screen coordinates).
    pub control_y: f32,
}

impl MotionPath for BezierPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let t = t.clamp(0.0, 1.0);

        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;

        let x = quadratic_bezier(t, sx, self.control_x, ex);
        let y = quadratic_bezier(t, sy, self.control_y, ey);

        (x, y)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_bezier_path.rs</FILE> - <DESC>Quadratic Bezier with explicit control point</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-25</VERS>
