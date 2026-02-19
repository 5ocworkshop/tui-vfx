// <FILE>tui-vfx-geometry/tests/paths/test_cls_bezier_path.rs</FILE> - <DESC>Tests for BezierPath</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T23:30:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Arbitrary waypoint support</WCTX>
// <CLOG>Initial tests for explicit control point Bezier path</CLOG>

use tui_vfx_geometry::paths::BezierPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;

#[test]
fn test_bezier_at_t0_returns_start() {
    let path = BezierPath {
        control_x: 50.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (x, y) = path.calculate(0.0, start, end);
    assert_eq!(x, 0.0);
    assert_eq!(y, 50.0);
}

#[test]
fn test_bezier_at_t1_returns_end() {
    let path = BezierPath {
        control_x: 50.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (x, y) = path.calculate(1.0, start, end);
    assert_eq!(x, 100.0);
    assert_eq!(y, 50.0);
}

#[test]
fn test_bezier_at_t05_passes_through_control_influence() {
    // Control point at (50, 0), start at (0, 50), end at (100, 50)
    // At t=0.5, quadratic bezier: B(0.5) = 0.25*P0 + 0.5*P1 + 0.25*P2
    // X: 0.25*0 + 0.5*50 + 0.25*100 = 0 + 25 + 25 = 50
    // Y: 0.25*50 + 0.5*0 + 0.25*50 = 12.5 + 0 + 12.5 = 25
    let path = BezierPath {
        control_x: 50.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (x, y) = path.calculate(0.5, start, end);
    assert!((x - 50.0).abs() < 0.001);
    assert!((y - 25.0).abs() < 0.001);
}

#[test]
fn test_bezier_with_control_below_creates_downward_arc() {
    // Control point below the line creates downward bulge
    let path = BezierPath {
        control_x: 50.0,
        control_y: 100.0, // below the line at y=50
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (_, y) = path.calculate(0.5, start, end);
    // Should be pulled toward control point (y=100)
    // Y: 0.25*50 + 0.5*100 + 0.25*50 = 12.5 + 50 + 12.5 = 75
    assert!((y - 75.0).abs() < 0.001);
}

#[test]
fn test_bezier_symmetric_horizontal() {
    // Symmetric control point should produce symmetric curve
    let path = BezierPath {
        control_x: 50.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (x1, y1) = path.calculate(0.25, start, end);
    let (x2, y2) = path.calculate(0.75, start, end);

    // At t=0.25 and t=0.75, should be equidistant from center (symmetric)
    assert!((x1 + x2 - 100.0).abs() < 0.001); // x1 + x2 = 100 (symmetric about x=50)
    assert!((y1 - y2).abs() < 0.001); // same y due to symmetry
}

#[test]
fn test_bezier_diagonal_path() {
    // Diagonal from (0,0) to (100,100) with control at (100, 0)
    // Creates a curve that starts horizontal then curves up
    let path = BezierPath {
        control_x: 100.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 0);
    let end = Position::new(100, 100);

    let (x, y) = path.calculate(0.5, start, end);
    // At t=0.5:
    // X: 0.25*0 + 0.5*100 + 0.25*100 = 0 + 50 + 25 = 75
    // Y: 0.25*0 + 0.5*0 + 0.25*100 = 0 + 0 + 25 = 25
    assert!((x - 75.0).abs() < 0.001);
    assert!((y - 25.0).abs() < 0.001);
}

#[test]
fn test_bezier_clamps_t_below_zero() {
    let path = BezierPath {
        control_x: 50.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (x, y) = path.calculate(-0.5, start, end);
    assert_eq!(x, 0.0);
    assert_eq!(y, 50.0);
}

#[test]
fn test_bezier_clamps_t_above_one() {
    let path = BezierPath {
        control_x: 50.0,
        control_y: 0.0,
    };
    let start = Position::new(0, 50);
    let end = Position::new(100, 50);

    let (x, y) = path.calculate(1.5, start, end);
    assert_eq!(x, 100.0);
    assert_eq!(y, 50.0);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_bezier_path.rs</FILE> - <DESC>Tests for BezierPath</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T23:30:00Z</VERS>
