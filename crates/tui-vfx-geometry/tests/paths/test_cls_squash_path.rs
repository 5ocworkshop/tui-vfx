// <FILE>tui-vfx-geometry/tests/paths/test_cls_squash_path.rs</FILE> - <DESC>Tests for SquashPath</DESC>
// <VERS>VERSION: 1.0.1 - 2025-12-16T20:17:32Z</VERS>
// <WCTX>Turn 8 Fixes</WCTX>
// <CLOG>Updated import to ratatui::layout::Position</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::easing::EasingType;
use tui_vfx_geometry::paths::cls_squash_path::SquashPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;
#[test]
fn test_independent_axes() {
    let path = SquashPath {
        h_curve: EasingType::Linear,
        v_curve: EasingType::QuadIn,
    };
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 10 };
    let (x, y) = path.calculate(0.5, start, end);
    assert_relative_eq!(x, 5.0);
    assert_relative_eq!(y, 2.5);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_squash_path.rs</FILE> - <DESC>Tests for SquashPath</DESC>
// <VERS>END OF VERSION: 1.0.1 - 2025-12-16T20:17:32Z</VERS>
