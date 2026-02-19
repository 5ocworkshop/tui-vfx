// <FILE>tui-vfx-geometry/src/paths/cls_arc_path.rs</FILE> - <DESC>Quadratic Bezier path</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-25</VERS>
// <WCTX>Mixed-signals migration Phase 5 - WP3</WCTX>
// <CLOG>Migrated quadratic Bezier formula to mixed_signals::math module</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
use mixed_signals::math::quadratic_bezier;
pub struct ArcPath {
    pub bulge: f32,
}
impl MotionPath for ArcPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let t = t.clamp(0.0, 1.0);
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;
        let mx = (sx + ex) / 2.0;
        let my = (sy + ey) / 2.0;
        let dx = ex - sx;
        let dy = ey - sy;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 0.001 {
            return (sx, sy);
        }
        let nx = -dy / dist;
        let ny = dx / dist;
        let p1x = mx + nx * self.bulge * dist;
        let p1y = my + ny * self.bulge * dist;
        let x = quadratic_bezier(t, sx, p1x, ex);
        let y = quadratic_bezier(t, sy, p1y, ey);
        (x, y)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_arc_path.rs</FILE> - <DESC>Quadratic Bezier path</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-25</VERS>
