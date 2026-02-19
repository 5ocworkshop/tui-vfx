// <FILE>tui-vfx-geometry/src/paths/cls_step_path.rs</FILE> - <DESC>Step path implementation</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T12:00:00Z - 2025-12-18T12:25:30Z</VERS>
// <WCTX>New primitive</WCTX>
// <CLOG>Initial implementation</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
pub struct StepPath {
    pub steps: u8,
}
impl MotionPath for StepPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let steps = self.steps.max(1) as f64;
        let stepped_t = ((t * steps).floor() / steps) as f32;
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;
        let x = sx + (ex - sx) * stepped_t;
        let y = sy + (ey - sy) * stepped_t;
        (x, y)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_step_path.rs</FILE> - <DESC>Step path implementation</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T12:00:00Z - 2025-12-18T12:25:30Z</VERS>
