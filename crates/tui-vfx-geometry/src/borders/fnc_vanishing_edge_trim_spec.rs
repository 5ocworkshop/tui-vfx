// <FILE>tui-vfx-geometry/src/borders/fnc_vanishing_edge_trim_spec.rs</FILE>
// <DESC>Vanishing-edge trim spec (legacy illusion)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy effects parity: vanishing-edge border behavior</WCTX>
// <CLOG>Extracted vanishing edge trim spec computation</CLOG>

use tui_vfx_types::Rect;

use crate::types::SlideDirection;

use super::border_trim_spec::{BorderSegment, BorderTrimSpec};
use super::fnc_clipped_edges::clipped_edges;

/// Computes the legacy-inspired “vanishing edge” trim spec.
///
/// The returned spec expresses *what should vanish* (edges/corners), without choosing concrete
/// glyph sets or colors. Callers apply this spec to their chosen border appearance.
///
/// Returns `None` when the rect is not clipped against the relevant viewport edge.
pub fn vanishing_edge_trim_spec(
    direction: SlideDirection,
    frame_area: Rect,
    dwell_rect: Rect,
    visible_area: Rect,
) -> Option<BorderTrimSpec> {
    let clip = clipped_edges(frame_area, dwell_rect, visible_area);

    let should_apply = match direction {
        SlideDirection::FromRight => clip.right,
        SlideDirection::FromLeft => clip.left,
        SlideDirection::FromTop => clip.top,
        SlideDirection::FromBottom => clip.bottom,
        SlideDirection::FromTopLeft => clip.top || clip.left,
        SlideDirection::FromTopRight => clip.top || clip.right,
        SlideDirection::FromBottomLeft => clip.bottom || clip.left,
        SlideDirection::FromBottomRight => clip.bottom || clip.right,
        SlideDirection::Default => false,
    };

    if !should_apply {
        return None;
    }

    let mut spec = BorderTrimSpec::none();

    match direction {
        SlideDirection::FromRight => {
            spec.right = BorderSegment::Blank;
            spec.top_right = BorderSegment::Horizontal;
            spec.bottom_right = BorderSegment::Horizontal;
        }
        SlideDirection::FromLeft => {
            spec.left = BorderSegment::Blank;
            spec.top_left = BorderSegment::Horizontal;
            spec.bottom_left = BorderSegment::Horizontal;
        }
        SlideDirection::FromTop => {
            spec.top = BorderSegment::Blank;
            spec.top_left = BorderSegment::Vertical;
            spec.top_right = BorderSegment::Vertical;
        }
        SlideDirection::FromBottom => {
            spec.bottom = BorderSegment::Blank;
            spec.bottom_left = BorderSegment::Vertical;
            spec.bottom_right = BorderSegment::Vertical;
        }
        // Diagonals: combine two simple cases.
        SlideDirection::FromTopLeft => {
            spec.left = BorderSegment::Blank;
            spec.top = BorderSegment::Blank;
            spec.top_left = BorderSegment::Horizontal;
            spec.top_right = BorderSegment::Vertical;
            spec.bottom_left = BorderSegment::Horizontal;
        }
        SlideDirection::FromTopRight => {
            spec.right = BorderSegment::Blank;
            spec.top = BorderSegment::Blank;
            spec.top_left = BorderSegment::Vertical;
            spec.top_right = BorderSegment::Horizontal;
            spec.bottom_right = BorderSegment::Horizontal;
        }
        SlideDirection::FromBottomLeft => {
            spec.left = BorderSegment::Blank;
            spec.bottom = BorderSegment::Blank;
            spec.top_left = BorderSegment::Horizontal;
            spec.bottom_left = BorderSegment::Vertical;
            spec.bottom_right = BorderSegment::Vertical;
        }
        SlideDirection::FromBottomRight => {
            spec.right = BorderSegment::Blank;
            spec.bottom = BorderSegment::Blank;
            spec.top_right = BorderSegment::Horizontal;
            spec.bottom_left = BorderSegment::Vertical;
            spec.bottom_right = BorderSegment::Horizontal;
        }
        SlideDirection::Default => return None,
    }

    Some(spec)
}

// <FILE>tui-vfx-geometry/src/borders/fnc_vanishing_edge_trim_spec.rs</FILE>
// <DESC>Vanishing-edge trim spec (legacy illusion)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
