// <FILE>tui-vfx-geometry/tests/test_fnc_vanishing_edge_trim_spec.rs</FILE>
// <DESC>Unit tests for fnc_vanishing_edge_trim_spec</DESC>

use tui_vfx_geometry::borders::{BorderSegment, vanishing_edge_trim_spec};
use tui_vfx_geometry::types::SlideDirection;
use tui_vfx_types::Rect;

#[test]
fn vanishing_edge_spec_from_right_blanks_right_edge_and_horizontalizes_right_corners() {
    let frame = Rect::new(0, 0, 10, 5);
    let dwell = Rect::new(2, 1, 8, 3);
    // Partially clipped on the right: right edge coincides with viewport.
    let visible = Rect::new(5, 1, 5, 3);

    let spec = vanishing_edge_trim_spec(SlideDirection::FromRight, frame, dwell, visible)
        .expect("expected trim spec");

    assert_eq!(spec.right, BorderSegment::Blank);
    assert_eq!(spec.top_right, BorderSegment::Horizontal);
    assert_eq!(spec.bottom_right, BorderSegment::Horizontal);

    // Unrelated edges remain untouched.
    assert_eq!(spec.left, BorderSegment::Keep);
    assert_eq!(spec.top, BorderSegment::Keep);
    assert_eq!(spec.bottom, BorderSegment::Keep);
}

#[test]
fn vanishing_edge_spec_returns_none_when_not_clipped() {
    let frame = Rect::new(0, 0, 20, 10);
    let dwell = Rect::new(2, 2, 8, 3);
    let visible = dwell;

    assert!(vanishing_edge_trim_spec(SlideDirection::FromRight, frame, dwell, visible).is_none());
    assert!(vanishing_edge_trim_spec(SlideDirection::FromTop, frame, dwell, visible).is_none());
}
