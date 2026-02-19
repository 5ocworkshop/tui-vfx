// <FILE>tui-vfx-geometry/tests/test_slide_direction_defaults.rs</FILE>
// <DESC>Lock legacy default slide-direction mapping</DESC>

use tui_vfx_geometry::transitions::resolve_slide_direction;
use tui_vfx_geometry::types::{Anchor, SlideDirection};

#[test]
fn default_slide_direction_matches_legacy_for_all_anchors() {
    let cases = [
        (Anchor::TopLeft, SlideDirection::FromTopLeft),
        (Anchor::TopCenter, SlideDirection::FromTop),
        (Anchor::TopRight, SlideDirection::FromTopRight),
        (Anchor::MiddleLeft, SlideDirection::FromLeft),
        (Anchor::Center, SlideDirection::FromTop),
        (Anchor::MiddleRight, SlideDirection::FromRight),
        (Anchor::BottomLeft, SlideDirection::FromBottomLeft),
        (Anchor::BottomCenter, SlideDirection::FromBottom),
        (Anchor::BottomRight, SlideDirection::FromBottomRight),
    ];

    for (anchor, expected) in cases {
        assert_eq!(
            resolve_slide_direction(SlideDirection::Default, anchor),
            expected,
            "unexpected default slide direction for {anchor:?}"
        );
    }
}
