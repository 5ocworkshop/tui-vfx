// <FILE>tui-vfx-geometry/src/transitions/fnc_resolve_slide_direction.rs</FILE>
// <DESC>Resolve SlideDirection::Default based on anchor</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted slide direction resolver</CLOG>

use crate::types::{Anchor, SlideDirection};

/// Resolves `SlideDirection::Default` into a concrete direction based on the anchor.
///
/// Matches legacy notifications.
pub fn resolve_slide_direction(direction: SlideDirection, anchor: Anchor) -> SlideDirection {
    if direction != SlideDirection::Default {
        return direction;
    }

    match anchor {
        Anchor::TopLeft => SlideDirection::FromTopLeft,
        Anchor::TopCenter => SlideDirection::FromTop,
        Anchor::TopRight => SlideDirection::FromTopRight,
        Anchor::MiddleLeft => SlideDirection::FromLeft,
        // Legacy notifications treat Default at the center as a vertical slide.
        Anchor::Center => SlideDirection::FromTop,
        Anchor::MiddleRight => SlideDirection::FromRight,
        Anchor::BottomLeft => SlideDirection::FromBottomLeft,
        Anchor::BottomCenter => SlideDirection::FromBottom,
        Anchor::BottomRight => SlideDirection::FromBottomRight,
        Anchor::Absolute(_, _) => SlideDirection::FromTop,
    }
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_resolve_slide_direction.rs</FILE>
// <DESC>Resolve SlideDirection::Default based on anchor</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
