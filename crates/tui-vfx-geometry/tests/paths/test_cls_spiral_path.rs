// <FILE>tui-vfx-geometry/tests/paths/test_cls_spiral_path.rs</FILE> - <DESC>Tests for SpiralPath</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T22:15:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Adding radius_cells to Spiral</WCTX>
// <CLOG>Initial tests for SpiralPath with radius_cells</CLOG>

use tui_vfx_geometry::paths::SpiralPath;
use tui_vfx_geometry::traits::MotionPath;
use tui_vfx_geometry::types::Position;

#[test]
fn test_spiral_at_t0_is_at_start() {
    let path = SpiralPath {
        rotations: 2.0,
        radius_cells: None,
    };
    let start = Position::new(0, 0);
    let end = Position::new(100, 0);

    let (x, y) = path.calculate(0.0, start, end);

    // At t=0, should be at start (with some initial radius offset)
    // Since radius = dist * 0.5 * (1.0 - 0.0) = 50 at t=0 with angle=0
    // x = 0 + 50 * cos(0) = 50, y = 0 + 50 * sin(0) = 0
    // Actually at t=0, angle = 0, cos(0) = 1, so x = start + radius
    assert!(
        (-10.0..=110.0).contains(&x),
        "x should be reasonable: {}",
        x
    );
    assert!((-60.0..=60.0).contains(&y), "y should be reasonable: {}", y);
}

#[test]
fn test_spiral_at_t1_is_at_end() {
    let path = SpiralPath {
        rotations: 2.0,
        radius_cells: None,
    };
    let start = Position::new(0, 0);
    let end = Position::new(100, 0);

    let (x, y) = path.calculate(1.0, start, end);

    // At t=1, radius = 0, so should be exactly at end
    assert!((x - 100.0).abs() < 0.01, "x should be at end: {}", x);
    assert!(y.abs() < 0.01, "y should be at end: {}", y);
}

#[test]
fn test_spiral_with_radius_cells() {
    // When radius_cells is specified, it should use that instead of distance-based
    let path = SpiralPath {
        rotations: 1.0,
        radius_cells: Some(10),
    };
    let start = Position::new(0, 0);
    let end = Position::new(100, 0);

    let (x, y) = path.calculate(0.0, start, end);

    // At t=0, angle=0, radius should be 10 (from radius_cells)
    // x = 0 + 10 * cos(0) = 10
    assert!((x - 10.0).abs() < 0.01, "x should use radius_cells: {}", x);
    assert!(y.abs() < 0.01, "y should be at start y: {}", y);
}

#[test]
fn test_spiral_radius_cells_decays_to_zero() {
    let path = SpiralPath {
        rotations: 1.0,
        radius_cells: Some(20),
    };
    let start = Position::new(50, 50);
    let end = Position::new(50, 50);

    // With same start and end, only the spiral offset matters
    let (x0, y0) = path.calculate(0.0, start, end);
    let (x1, y1) = path.calculate(1.0, start, end);

    // At t=0, radius = 20, angle=0, x = 50 + 20 = 70, y = 50
    assert!(
        (x0 - 70.0).abs() < 0.01,
        "x0 should be start + radius: {}",
        x0
    );
    assert!((y0 - 50.0).abs() < 0.01, "y0 should be start y: {}", y0);

    // At t=1, radius = 0, should be at end
    assert!((x1 - 50.0).abs() < 0.01, "x1 should be at end: {}", x1);
    assert!((y1 - 50.0).abs() < 0.01, "y1 should be at end: {}", y1);
}

#[test]
fn test_spiral_rotations() {
    let path = SpiralPath {
        rotations: 2.0,
        radius_cells: Some(10),
    };
    let start = Position::new(0, 0);
    let end = Position::new(0, 0);

    // At t=0.25, angle = 0.25 * 2.0 * TAU = TAU / 2 = PI
    let (x, _y) = path.calculate(0.25, start, end);
    // radius at t=0.25 = 10 * (1 - 0.25) = 7.5
    // x = 0 + 7.5 * cos(PI) = 0 + 7.5 * (-1) = -7.5
    assert!((x - (-7.5)).abs() < 0.1, "x at quarter rotation: {}", x);
}

// <FILE>tui-vfx-geometry/tests/paths/test_cls_spiral_path.rs</FILE> - <DESC>Tests for SpiralPath</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T22:15:00Z</VERS>
