// <FILE>tui-vfx-geometry/tests/test_slide_path.rs</FILE>
// <DESC>Tests for 3-point slide path semantics</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Support explicit start/dwell/end positions</WCTX>
// <CLOG>Added tests for toast-default path and custom left→center→right slide</CLOG>

use tui_vfx_types::Rect;

use tui_vfx_geometry::transitions::{
    SlidePath, SlidePhase, slide_calculate_rect_path, slide_path_offscreen_start_end,
};
use tui_vfx_geometry::types::{Anchor, SignedRect, SlideDirection};

#[test]
fn test_toast_default_path_is_static() {
    let frame = Rect::new(0, 0, 40, 10);
    let dwell = SignedRect::new(10, 3, 8, 3);
    let path = SlidePath::toast(dwell);

    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        assert_eq!(
            slide_calculate_rect_path(path, frame, t, SlidePhase::SlidingIn),
            Rect::new(10, 3, 8, 3)
        );
        assert_eq!(
            slide_calculate_rect_path(path, frame, t, SlidePhase::SlidingOut),
            Rect::new(10, 3, 8, 3)
        );
    }
}

#[test]
fn test_custom_left_center_right_slide() {
    let frame = Rect::new(0, 0, 40, 10);
    let start = SignedRect::new(0, 3, 6, 3);
    let dwell = SignedRect::new(17, 3, 6, 3);
    let end = SignedRect::new(34, 3, 6, 3);

    let path = SlidePath { start, dwell, end };

    // Enter: start -> dwell
    assert_eq!(
        slide_calculate_rect_path(path, frame, 0.0, SlidePhase::SlidingIn),
        Rect::new(0, 3, 6, 3)
    );
    assert_eq!(
        slide_calculate_rect_path(path, frame, 1.0, SlidePhase::SlidingIn),
        Rect::new(17, 3, 6, 3)
    );
    assert_eq!(
        slide_calculate_rect_path(path, frame, 0.5, SlidePhase::SlidingIn),
        Rect::new(9, 3, 6, 3)
    );

    // Exit: dwell -> end
    assert_eq!(
        slide_calculate_rect_path(path, frame, 0.0, SlidePhase::SlidingOut),
        Rect::new(17, 3, 6, 3)
    );
    assert_eq!(
        slide_calculate_rect_path(path, frame, 1.0, SlidePhase::SlidingOut),
        Rect::new(34, 3, 6, 3)
    );
    assert_eq!(
        slide_calculate_rect_path(path, frame, 0.5, SlidePhase::SlidingOut),
        Rect::new(26, 3, 6, 3)
    );
}

#[test]
fn test_offscreen_start_end_allows_exit_direction_override() {
    let frame = Rect::new(0, 0, 40, 10);
    let dwell = SignedRect::new(10, 3, 8, 3);

    let path = slide_path_offscreen_start_end(
        frame,
        Anchor::Center,
        SlideDirection::FromBottom,
        SlideDirection::FromTop,
        dwell,
    );

    assert!(path.start.y > dwell.y);
    assert!(path.end.y < dwell.y);
}
