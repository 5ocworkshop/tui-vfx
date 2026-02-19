// <FILE>tui-vfx-geometry/tests/paths/test_cls_hover_path.rs</FILE> - <DESC>Tests for HoverPath</DESC>
// <VERS>VERSION: 1.0.2 - 2025-12-22</VERS>
// <WCTX>Signal generator integration</WCTX>
// <CLOG>Updated to use HoverPath::new() constructor</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::paths::cls_hover_path::HoverPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;
#[test]
fn test_oscillation() {
    let path = HoverPath::new(5.0, 1.0);
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 10 };
    let (x0, y0) = path.calculate(0.0, start, end);
    assert_relative_eq!(x0, 10.0);
    assert_relative_eq!(y0, 10.0);
    // Using TAU to map 0.25 -> PI/2 for easy sine check
    let path_fast = HoverPath::new(5.0, std::f32::consts::TAU);
    let (_x, y) = path_fast.calculate(0.25, start, end);
    assert_relative_eq!(y, 15.0);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_hover_path.rs</FILE> - <DESC>Tests for HoverPath</DESC>
// <VERS>END OF VERSION: 1.0.2 - 2025-12-22</VERS>
