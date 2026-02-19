// <FILE>tui-vfx-geometry/src/paths/cls_squash_path.rs</FILE> - <DESC>Independent axis easing path</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
// <WCTX>Turn 5 Restoration</WCTX>
// <CLOG>Re-emit</CLOG>

use crate::easing::ease;
use crate::traits::MotionPath;
use crate::types::Position;
pub struct SquashPath {
    pub h_curve: crate::easing::EasingType,
    pub v_curve: crate::easing::EasingType,
}
impl MotionPath for SquashPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;
        let t = t.clamp(0.0, 1.0);
        let t_x = ease(t, self.h_curve);
        let t_y = ease(t, self.v_curve);
        let x = sx + (ex - sx) * t_x;
        let y = sy + (ey - sy) * t_y;
        (x, y)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_squash_path.rs</FILE> - <DESC>Independent axis easing path</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
