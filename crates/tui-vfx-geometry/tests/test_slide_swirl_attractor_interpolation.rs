// <FILE>tui-vfx-geometry/tests/test_slide_swirl_attractor_interpolation.rs</FILE> - <DESC>Slide interpolation honors vortex and attraction path dynamics</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 edge-lane pattern/path pass — add executable primitives for swirl/vortex and gravity-well style motion discovered during animation-pattern review.</WCTX>
// <CLOG>0.1.0: add regression coverage for PathType::Swirl and PathType::Attractor endpoint safety and mid-route deviation.</CLOG>

use tui_vfx_geometry::transitions::interpolate_position;
use tui_vfx_geometry::types::{PathType, Position};

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.001,
        "expected {actual} ~= {expected}"
    );
}

#[test]
fn swirl_preserves_endpoints_and_bows_around_route() {
    let from = Position::new(0, 0);
    let to = Position::new(10, 0);
    let path = PathType::Swirl {
        rotations: 0.5,
        radius: 4.0,
        direction: 1.0,
    };

    let start = interpolate_position(from, to, 0.0, &path);
    let middle = interpolate_position(from, to, 0.5, &path);
    let end = interpolate_position(from, to, 1.0, &path);

    assert_close(start.0, 0.0);
    assert_close(start.1, 0.0);
    assert_close(end.0, 10.0);
    assert_close(end.1, 0.0);
    assert_close(middle.0, 5.0);
    assert!(
        middle.1 > 3.9,
        "quarter-turn swirl should lift away from the linear route, got {middle:?}"
    );
}

#[test]
fn swirl_direction_flips_the_vortex_side() {
    let from = Position::new(0, 0);
    let to = Position::new(10, 0);
    let clockwise = PathType::Swirl {
        rotations: 0.5,
        radius: 4.0,
        direction: -1.0,
    };
    let counter_clockwise = PathType::Swirl {
        rotations: 0.5,
        radius: 4.0,
        direction: 1.0,
    };

    let cw = interpolate_position(from, to, 0.5, &clockwise);
    let ccw = interpolate_position(from, to, 0.5, &counter_clockwise);

    assert_close(cw.0, ccw.0);
    assert!(cw.1 < -3.9, "clockwise swirl should go below route: {cw:?}");
    assert!(
        ccw.1 > 3.9,
        "counter-clockwise swirl should go above route: {ccw:?}"
    );
}

#[test]
fn attractor_preserves_endpoints_and_pulls_midpoint_to_target() {
    let from = Position::new(0, 0);
    let to = Position::new(10, 0);
    let path = PathType::Attractor {
        target_x: 5.0,
        target_y: 6.0,
        strength: 1.0,
    };

    let start = interpolate_position(from, to, 0.0, &path);
    let middle = interpolate_position(from, to, 0.5, &path);
    let end = interpolate_position(from, to, 1.0, &path);

    assert_close(start.0, 0.0);
    assert_close(start.1, 0.0);
    assert_close(middle.0, 5.0);
    assert_close(middle.1, 6.0);
    assert_close(end.0, 10.0);
    assert_close(end.1, 0.0);
}

// <FILE>tui-vfx-geometry/tests/test_slide_swirl_attractor_interpolation.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
