// <FILE>tui-vfx-geometry/src/paths/cls_linear_path.rs</FILE> - <DESC>Linear interpolation path</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
// <WCTX>Turn 5 Restoration</WCTX>
// <CLOG>Re-emit</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
pub struct LinearPath;
impl MotionPath for LinearPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let t = t as f32;
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;
        let x = sx + (ex - sx) * t;
        let y = sy + (ey - sy) * t;
        (x, y)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_linear_path.rs</FILE> - <DESC>Linear interpolation path</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
