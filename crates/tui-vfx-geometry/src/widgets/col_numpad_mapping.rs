// <FILE>tui-vfx-geometry/src/widgets/col_numpad_mapping.rs</FILE>
// <DESC>Numpad digit mapping helpers (Anchor/SlideDirection)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Share mapping logic across configurators</WCTX>
// <CLOG>Added digit mapping and direction cycle sets</CLOG>

use crate::types::{Anchor, SlideDirection};

pub fn anchor_from_numpad_digit(c: char) -> Option<Anchor> {
    Some(match c {
        '7' => Anchor::TopLeft,
        '8' => Anchor::TopCenter,
        '9' => Anchor::TopRight,
        '4' => Anchor::MiddleLeft,
        '5' => Anchor::Center,
        '6' => Anchor::MiddleRight,
        '1' => Anchor::BottomLeft,
        '2' => Anchor::BottomCenter,
        '3' => Anchor::BottomRight,
        _ => return None,
    })
}

pub fn numpad_digit_from_anchor(anchor: Anchor) -> char {
    #[allow(unreachable_patterns)]
    match anchor {
        Anchor::TopLeft => '7',
        Anchor::TopCenter => '8',
        Anchor::TopRight => '9',
        Anchor::MiddleLeft => '4',
        Anchor::Center => '5',
        Anchor::MiddleRight => '6',
        Anchor::BottomLeft => '1',
        Anchor::BottomCenter => '2',
        Anchor::BottomRight => '3',
        _ => '5',
    }
}

pub fn direction_from_numpad_digit(c: char) -> Option<SlideDirection> {
    match c {
        '7' => Some(SlideDirection::FromTopLeft),
        '8' => Some(SlideDirection::FromTop),
        '9' => Some(SlideDirection::FromTopRight),
        '4' => Some(SlideDirection::FromLeft),
        '5' => Some(SlideDirection::Default),
        '6' => Some(SlideDirection::FromRight),
        '1' => Some(SlideDirection::FromBottomLeft),
        '2' => Some(SlideDirection::FromBottom),
        '3' => Some(SlideDirection::FromBottomRight),
        _ => None,
    }
}

/// Returns the direction cycle list for the given digit.
///
/// This is the "richer" cycle (includes diagonals even on edge digits).
pub fn direction_cycle_for_digit(digit: char) -> &'static [SlideDirection] {
    use SlideDirection::*;
    match digit {
        // Corners: diagonal-in + along the edges.
        '7' => &[FromTopLeft, FromLeft, FromTop],
        '9' => &[FromTopRight, FromRight, FromTop],
        '1' => &[FromBottomLeft, FromLeft, FromBottom],
        '3' => &[FromBottomRight, FromRight, FromBottom],

        // Edges: straight-in + perpendicular "arc" hints.
        // (These are glyph directions; edge digits reinterpret the non-straight entries as arc hints.)
        '8' => &[FromTop, FromLeft, FromRight],
        '4' => &[FromLeft, FromBottom, FromTop],
        '6' => &[FromRight, FromBottom, FromTop],
        '2' => &[FromBottom, FromLeft, FromRight],

        '5' => &[Default],
        _ => &[],
    }
}

// <FILE>tui-vfx-geometry/src/widgets/col_numpad_mapping.rs</FILE>
// <DESC>Numpad digit mapping helpers (Anchor/SlideDirection)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
