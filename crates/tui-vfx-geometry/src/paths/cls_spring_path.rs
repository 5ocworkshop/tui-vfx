// <FILE>tui-vfx-geometry/src/paths/cls_spring_path.rs</FILE> - <DESC>Spring physics path</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
// <WCTX>Turn 5 Restoration</WCTX>
// <CLOG>Re-emit</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
pub struct SpringPath {
    pub stiffness: f32,
    pub damping: f32,
}
impl MotionPath for SpringPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let t = t as f32;
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;
        let decay = self.damping * 5.0;
        let freq = self.stiffness * 2.0;
        let factor = if t <= 0.0 {
            0.0
        } else if t >= 1.0 {
            1.0
        } else {
            1.0 - (-decay * t).exp() * (freq * t).cos()
        };
        let x = sx + (ex - sx) * factor;
        let y = sy + (ey - sy) * factor;
        (x, y)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_spring_path.rs</FILE> - <DESC>Spring physics path</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
