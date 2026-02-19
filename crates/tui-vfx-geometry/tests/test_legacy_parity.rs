// <FILE>tui-vfx-geometry/tests/test_legacy_parity.rs</FILE>
// <DESC>Legacy notifications parity tests for anchors + slide + expand</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Encode legacy semantics without runtime dependency on legacy repo</WCTX>
// <CLOG>Added integration tests for Batch C parity primitives</CLOG>

use tui_vfx_types::{Point, Rect};

use tui_vfx_geometry::anchors::calculate_anchor_position;
use tui_vfx_geometry::transitions::{
    ExpandPhase, SlidePath, SlidePhase, expand_collapse_calculate_rect, resolve_slide_direction,
    slide_calculate_rect_path, slide_offscreen_position,
};
use tui_vfx_geometry::types::{Anchor, SignedRect, SlideDirection};

#[test]
fn test_anchors_exist_and_map_deterministically() {
    let frame = Rect::new(0, 0, 100, 50);

    assert_eq!(
        calculate_anchor_position(Anchor::TopLeft, frame),
        Point::new(0, 0)
    );
    assert_eq!(
        calculate_anchor_position(Anchor::TopCenter, frame),
        Point::new(50, 0)
    );
    assert_eq!(
        calculate_anchor_position(Anchor::TopRight, frame),
        Point::new(99, 0)
    );

    // Middle row uses a 45% visual center offset: 50 * 45 / 100 = 22.
    assert_eq!(
        calculate_anchor_position(Anchor::MiddleLeft, frame),
        Point::new(0, 22)
    );
    assert_eq!(
        calculate_anchor_position(Anchor::Center, frame),
        Point::new(50, 22)
    );
    assert_eq!(
        calculate_anchor_position(Anchor::MiddleRight, frame),
        Point::new(99, 22)
    );

    assert_eq!(
        calculate_anchor_position(Anchor::BottomLeft, frame),
        Point::new(0, 49)
    );
    assert_eq!(
        calculate_anchor_position(Anchor::BottomCenter, frame),
        Point::new(50, 49)
    );
    assert_eq!(
        calculate_anchor_position(Anchor::BottomRight, frame),
        Point::new(99, 49)
    );
}

#[test]
fn test_slide_direction_defaults_by_anchor() {
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::TopLeft),
        SlideDirection::FromTopLeft
    );
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::TopCenter),
        SlideDirection::FromTop
    );
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::TopRight),
        SlideDirection::FromTopRight
    );

    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::MiddleLeft),
        SlideDirection::FromLeft
    );
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::Center),
        SlideDirection::FromTop
    );
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::MiddleRight),
        SlideDirection::FromRight
    );

    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::BottomLeft),
        SlideDirection::FromBottomLeft
    );
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::BottomCenter),
        SlideDirection::FromBottom
    );
    assert_eq!(
        resolve_slide_direction(SlideDirection::Default, Anchor::BottomRight),
        SlideDirection::FromBottomRight
    );
}

#[test]
fn test_slide_offscreen_positions_have_one_cell_margin() {
    let full = SignedRect::new(50, 25, 20, 10);
    let frame = Rect::new(0, 0, 100, 50);

    // Left: x = frame_x - width - 1 = -21
    assert_eq!(
        slide_offscreen_position(SlideDirection::FromLeft, full, frame),
        tui_vfx_geometry::types::Position::new(-21, 25)
    );

    // Right: x = frame_right + 1 = 101
    assert_eq!(
        slide_offscreen_position(SlideDirection::FromRight, full, frame),
        tui_vfx_geometry::types::Position::new(101, 25)
    );

    // Top: y = frame_y - height - 1 = -11
    assert_eq!(
        slide_offscreen_position(SlideDirection::FromTop, full, frame),
        tui_vfx_geometry::types::Position::new(50, -11)
    );

    // Bottom: y = frame_bottom + 1 = 51
    assert_eq!(
        slide_offscreen_position(SlideDirection::FromBottom, full, frame),
        tui_vfx_geometry::types::Position::new(50, 51)
    );
}

#[test]
fn test_slide_interpolation_and_clipping_to_frame() {
    let frame = Rect::new(5, 5, 10, 5); // right=15
    let full = SignedRect::new(11, 6, 4, 3); // fully visible at end

    let path = SlidePath {
        start: SignedRect::new(16, 6, 4, 3),
        dwell: full,
        end: SignedRect::new(16, 6, 4, 3),
    };

    // At t=1.0 sliding in, should equal full rect (clipped = identity).
    assert_eq!(
        slide_calculate_rect_path(path, frame, 1.0, SlidePhase::SlidingIn),
        Rect::new(11, 6, 4, 3)
    );

    // At t=0.0 sliding in, fully offscreen => default.
    assert_eq!(
        slide_calculate_rect_path(path, frame, 0.0, SlidePhase::SlidingIn),
        Rect::default()
    );

    // Halfway-ish: choose progress=0.6 to land on an integer x.
    // start_x = frame_right + 1 = 16; end_x = 11; lerp(16,11,0.6)=13.
    // Visible portion is [13,15) => width=2.
    assert_eq!(
        slide_calculate_rect_path(path, frame, 0.6, SlidePhase::SlidingIn),
        Rect::new(13, 6, 2, 3)
    );
}

#[test]
fn test_expand_collapse_min_size_is_3x3_centered_on_full_rect_center() {
    let full = SignedRect::new(10, 20, 33, 13);

    // Expand at t=0: min size (3x3), centered on full rect center.
    assert_eq!(
        expand_collapse_calculate_rect(full, ExpandPhase::Expanding, 0.0),
        Rect::new(25, 25, 3, 3)
    );

    // Expand at t=1: full rect.
    assert_eq!(
        expand_collapse_calculate_rect(full, ExpandPhase::Expanding, 1.0),
        Rect::new(10, 20, 33, 13)
    );

    // Collapse at t=1: min size (3x3), same center.
    assert_eq!(
        expand_collapse_calculate_rect(full, ExpandPhase::Collapsing, 1.0),
        Rect::new(25, 25, 3, 3)
    );
}
