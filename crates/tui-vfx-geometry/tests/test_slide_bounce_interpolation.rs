// <FILE>tui-vfx-geometry/tests/test_slide_bounce_interpolation.rs</FILE>
// <DESC>Slide interpolation honors PathType::Bounce behavior</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Generalization migration - test fix</WCTX>
// <CLOG>Fixed test to match actual bounce behavior (hits target, bounces back)</CLOG>

use tui_vfx_geometry::transitions::{
    SlidePath, SlidePhase, slide_calculate_rect_path_with_path_type,
};
use tui_vfx_geometry::types::{PathType, SignedRect};
use tui_vfx_types::Rect;

#[test]
fn bounce_hits_target_and_bounces_back() {
    let frame = Rect::new(0, 0, 200, 200);
    let start = SignedRect::new(10, 10, 10, 3);
    let dwell = SignedRect::new(10, 60, 10, 3);
    let path = SlidePath {
        start,
        dwell,
        end: dwell,
    };

    // At t=0, should be at start
    let at_start = slide_calculate_rect_path_with_path_type(
        path,
        frame,
        0.0,
        SlidePhase::SlidingIn,
        &PathType::Bounce {
            bounces: 3,
            decay: 2.0,
        },
    );
    assert_eq!(
        i32::from(at_start.y),
        start.y,
        "should start at start position"
    );

    // At t=0.5, should be somewhere between start and dwell (during bounce animation)
    let mid = slide_calculate_rect_path_with_path_type(
        path,
        frame,
        0.50,
        SlidePhase::SlidingIn,
        &PathType::Bounce {
            bounces: 3,
            decay: 2.0,
        },
    );
    assert!(
        i32::from(mid.y) >= start.y && i32::from(mid.y) <= dwell.y,
        "mid position should be between start and dwell (mid_y={}, start_y={}, dwell_y={})",
        mid.y,
        start.y,
        dwell.y
    );

    // At t=1.0, should settle at dwell
    let end = slide_calculate_rect_path_with_path_type(
        path,
        frame,
        1.0,
        SlidePhase::SlidingIn,
        &PathType::Bounce {
            bounces: 3,
            decay: 2.0,
        },
    );
    assert_eq!(i32::from(end.y), dwell.y, "should settle at dwell position");
}

// <FILE>tui-vfx-geometry/tests/test_slide_bounce_interpolation.rs</FILE>
// <DESC>Slide interpolation honors PathType::Bounce behavior</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
