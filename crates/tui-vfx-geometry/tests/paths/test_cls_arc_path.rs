// <FILE>tui-vfx-geometry/tests/paths/test_cls_arc_path.rs</FILE> - <DESC>Tests for ArcPath</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-16T20:22:32Z</VERS>
// <WCTX>Turn 10 Completeness</WCTX>
// <CLOG>Restored test_bulge_offset</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::paths::cls_arc_path::ArcPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;
#[test]
fn test_zero_bulge_is_linear() {
    let path = ArcPath { bulge: 0.0 };
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 0 };
    let (x, y) = path.calculate(0.5, start, end);
    assert_relative_eq!(x, 5.0);
    assert_relative_eq!(y, 0.0);
}
#[test]
fn test_bulge_offset() {
    let path = ArcPath { bulge: 0.5 };
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 0 };
    let (x, y) = path.calculate(0.5, start, end);
    assert_relative_eq!(x, 5.0);
    // Based on Turn 3 logic: P1y = 5.0. B(0.5) = 2.5
    assert_relative_eq!(y, 2.5);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_arc_path.rs</FILE> - <DESC>Tests for ArcPath</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-16T20:22:32Z</VERS>
