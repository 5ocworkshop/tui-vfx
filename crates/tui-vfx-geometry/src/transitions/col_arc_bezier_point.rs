// <FILE>tui-vfx-geometry/src/transitions/col_arc_bezier_point.rs</FILE>
// <DESC>Quadratic bezier point for arc slide paths</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>mixed-signals migration cleanup</WCTX>
// <CLOG>Replaced inline bezier formula with mixed_signals::math::quadratic_bezier</CLOG>

use mixed_signals::math::quadratic_bezier;

/// Calculate a point on an arc slide path using quadratic bezier curve.
///
/// The control point is calculated based on bulge and motion direction:
/// - For mostly-horizontal motion, bulge affects Y (up/down)
/// - For mostly-vertical motion, bulge affects X (left/right)
///
/// This matches the legacy "numpad arc hint" mental model for edges.
pub(super) fn arc_bezier_point(
    sx: f32,
    sy: f32,
    ex: f32,
    ey: f32,
    t: f64,
    bulge: f32,
) -> (f32, f32) {
    let mx = (sx + ex) / 2.0;
    let my = (sy + ey) / 2.0;
    let dx = ex - sx;
    let dy = ey - sy;
    let major = dx.abs().max(dy.abs());
    if major < 0.001 {
        return (sx, sy);
    }

    // Calculate control point based on direction
    let (p1x, p1y) = if dx.abs() >= dy.abs() {
        (mx, my + bulge * major)
    } else {
        (mx + bulge * major, my)
    };

    // Use mixed_signals::math for bezier calculation
    let x = quadratic_bezier(t, sx, p1x, ex);
    let y = quadratic_bezier(t, sy, p1y, ey);
    (x, y)
}

// <FILE>tui-vfx-geometry/src/transitions/col_arc_bezier_point.rs</FILE>
// <DESC>Quadratic bezier point for arc slide paths</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
