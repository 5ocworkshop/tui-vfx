// <FILE>tui-vfx-geometry/tests/paths/test_cls_linear_path.rs</FILE> - <DESC>Tests for LinearPath</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-16T20:22:32Z</VERS>
// <WCTX>Turn 10 Completeness</WCTX>
// <CLOG>Restored test_midpoint</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::paths::cls_linear_path::LinearPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;
#[test]
fn test_start_end() {
    let path = LinearPath;
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 10 };
    let (x0, y0) = path.calculate(0.0, start, end);
    assert_relative_eq!(x0, 0.0);
    assert_relative_eq!(y0, 0.0);
    let (x1, y1) = path.calculate(1.0, start, end);
    assert_relative_eq!(x1, 10.0);
    assert_relative_eq!(y1, 10.0);
}
#[test]
fn test_midpoint() {
    let path = LinearPath;
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 20 };
    let (x, y) = path.calculate(0.5, start, end);
    assert_relative_eq!(x, 5.0);
    assert_relative_eq!(y, 10.0);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_linear_path.rs</FILE> - <DESC>Tests for LinearPath</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-16T20:22:32Z</VERS>
