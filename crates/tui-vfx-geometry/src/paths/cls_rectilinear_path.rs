// <FILE>tui-vfx-geometry/src/paths/cls_rectilinear_path.rs</FILE> - <DESC>Rectilinear (Manhattan) path implementation</DESC>
// <VERS>VERSION: 1.0.1 - 2025-12-18T13:20:00Z - 2025-12-18T13:14:46Z</VERS>
// <WCTX>Fixing clippy::collapsible_else_if</WCTX>
// <CLOG>Collapsed nested if statements</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
pub struct RectilinearPath {
    pub x_first: bool,
}
impl MotionPath for RectilinearPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let t = t as f32;
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;
        // Split time into two halves: 0.0-0.5 for first leg, 0.5-1.0 for second leg.
        if self.x_first {
            if t < 0.5 {
                let segment_t = t * 2.0;
                let x = sx + (ex - sx) * segment_t;
                (x, sy)
            } else {
                let segment_t = (t - 0.5) * 2.0;
                let y = sy + (ey - sy) * segment_t;
                (ex, y)
            }
        } else if t < 0.5 {
            let segment_t = t * 2.0;
            let y = sy + (ey - sy) * segment_t;
            (sx, y)
        } else {
            let segment_t = (t - 0.5) * 2.0;
            let x = sx + (ex - sx) * segment_t;
            (x, ey)
        }
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_rectilinear_path.rs</FILE> - <DESC>Rectilinear (Manhattan) path implementation</DESC>
// <VERS>END OF VERSION: 1.0.1 - 2025-12-18T13:20:00Z - 2025-12-18T13:14:46Z</VERS>
