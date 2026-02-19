// <FILE>tui-vfx-geometry/src/traits/motion_path.rs</FILE> - <DESC>Contract for motion logic</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
// <WCTX>Turn 5 Restoration</WCTX>
// <CLOG>Re-emit</CLOG>

use crate::types::Position;
pub trait MotionPath {
    /// Calculate the position at normalized time `t` (0.0 to 1.0).
    ///
    /// Returns (x, y) as f32 coordinates.
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32);
}

// <FILE>tui-vfx-geometry/src/traits/motion_path.rs</FILE> - <DESC>Contract for motion logic</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:10:03Z</VERS>
