// <FILE>tui-vfx-geometry/tests/paths/test_cls_spring_path.rs</FILE> - <DESC>Tests for SpringPath</DESC>
// <VERS>VERSION: 1.2.0 - 2025-12-16T20:22:32Z</VERS>
// <WCTX>Turn 10 Completeness</WCTX>
// <CLOG>Added test_settling</CLOG>

use approx::assert_relative_eq;
use tui_vfx_geometry::paths::cls_spring_path::SpringPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;
#[test]
fn test_spring_ends() {
    let path = SpringPath {
        stiffness: 10.0,
        damping: 0.5,
    };
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 100, y: 0 };
    let (x0, _) = path.calculate(0.0, start, end);
    assert_relative_eq!(x0, 0.0);
}
#[test]
fn test_overshoot() {
    let path = SpringPath {
        stiffness: 15.0,
        damping: 2.0,
    };
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 0 };
    let mut max_x = 0.0;
    for i in 0..100 {
        let t = i as f64 / 100.0;
        let (x, _) = path.calculate(t, start, end);
        if x > max_x {
            max_x = x;
        }
    }
    assert!(max_x > 10.0, "Spring should overshoot target");
}
#[test]
fn test_settling() {
    // High damping should settle close to target at t=1.0
    let path = SpringPath {
        stiffness: 10.0,
        damping: 5.0,
    };
    let start = Position { x: 0, y: 0 };
    let end = Position { x: 10, y: 0 };
    let (x, _) = path.calculate(1.0, start, end);
    // Should be very close to 10.0
    assert_relative_eq!(x, 10.0, epsilon = 0.1);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_spring_path.rs</FILE> - <DESC>Tests for SpringPath</DESC>
// <VERS>END OF VERSION: 1.2.0 - 2025-12-16T20:22:32Z</VERS>
