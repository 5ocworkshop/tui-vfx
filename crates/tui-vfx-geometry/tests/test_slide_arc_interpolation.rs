// <FILE>tui-vfx-geometry/tests/test_slide_arc_interpolation.rs</FILE>
// <DESC>Slide interpolation honors PathType::Arc bulge</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Arc semantics for slide (numpad arc hints)</WCTX>
// <CLOG>Added focused tests for arc bulge direction</CLOG>

use tui_vfx_geometry::transitions::{
    SlidePath, SlidePhase, slide_calculate_rect_path_with_path_type,
};
use tui_vfx_geometry::types::{PathType, SignedRect};
use tui_vfx_types::Rect;

#[test]
fn arc_bulge_positive_bends_down_for_left_to_right_segment() {
    let frame = Rect::new(0, 0, 200, 200);
    let start = SignedRect::new(10, 50, 10, 3);
    let dwell = SignedRect::new(30, 50, 10, 3);
    let path = SlidePath {
        start,
        dwell,
        end: dwell,
    };

    let rect = slide_calculate_rect_path_with_path_type(
        path,
        frame,
        0.5,
        SlidePhase::SlidingIn,
        &PathType::Arc { bulge: 0.50 },
    );

    // For a left->right segment, the Arc normal points "down" for positive bulge.
    assert!(
        rect.y > 50,
        "expected arc to bend down (y increases), got y={}",
        rect.y
    );
}

#[test]
fn arc_bulge_negative_bends_up_for_left_to_right_segment() {
    let frame = Rect::new(0, 0, 200, 200);
    let start = SignedRect::new(10, 50, 10, 3);
    let dwell = SignedRect::new(30, 50, 10, 3);
    let path = SlidePath {
        start,
        dwell,
        end: dwell,
    };

    let rect = slide_calculate_rect_path_with_path_type(
        path,
        frame,
        0.5,
        SlidePhase::SlidingIn,
        &PathType::Arc { bulge: -0.50 },
    );

    assert!(
        rect.y < 50,
        "expected arc to bend up (y decreases), got y={}",
        rect.y
    );
}

// <FILE>tui-vfx-geometry/tests/test_slide_arc_interpolation.rs</FILE>
// <DESC>Slide interpolation honors PathType::Arc bulge</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
