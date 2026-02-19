// <FILE>tui-vfx-geometry/tests/widgets/test_col_numpad_mapping.rs</FILE>
// <DESC>Tests for numpad mapping and direction cycles</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Lock down mapping used by shared grid widget</WCTX>
// <CLOG>Added mapping tests</CLOG>

use tui_vfx_geometry::types::{Anchor, SlideDirection};
use tui_vfx_geometry::widgets::col_numpad_mapping::{
    anchor_from_numpad_digit, direction_cycle_for_digit, direction_from_numpad_digit,
    numpad_digit_from_anchor,
};

#[test]
fn anchor_roundtrip_numpad_digit() {
    for (digit, anchor) in [
        ('7', Anchor::TopLeft),
        ('8', Anchor::TopCenter),
        ('9', Anchor::TopRight),
        ('4', Anchor::MiddleLeft),
        ('5', Anchor::Center),
        ('6', Anchor::MiddleRight),
        ('1', Anchor::BottomLeft),
        ('2', Anchor::BottomCenter),
        ('3', Anchor::BottomRight),
    ] {
        assert_eq!(anchor_from_numpad_digit(digit), Some(anchor));
        assert_eq!(numpad_digit_from_anchor(anchor), digit);
    }
}

#[test]
fn direction_cycle_is_richer_on_edges() {
    assert_eq!(
        direction_cycle_for_digit('8'),
        &[
            SlideDirection::FromTop,
            SlideDirection::FromLeft,
            SlideDirection::FromRight
        ]
    );
    assert_eq!(
        direction_cycle_for_digit('2'),
        &[
            SlideDirection::FromBottom,
            SlideDirection::FromLeft,
            SlideDirection::FromRight
        ]
    );
    assert_eq!(direction_cycle_for_digit('5'), &[SlideDirection::Default]);
}

#[test]
fn base_direction_mapping_exists_for_all_digits() {
    for d in ['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
        assert!(direction_from_numpad_digit(d).is_some());
    }
}

// <FILE>tui-vfx-geometry/tests/widgets/test_col_numpad_mapping.rs</FILE>
// <DESC>Tests for numpad mapping and direction cycles</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
