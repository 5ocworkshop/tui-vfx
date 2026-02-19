// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_offscreen_position.rs</FILE>
// <DESC>Offscreen positioning helper for slide transitions</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted offscreen start/end calculation</CLOG>

use tui_vfx_types::Rect;

use crate::types::{Position, SignedRect, SlideDirection};

/// Calculates the offscreen start/end position for a slide transition.
///
/// The returned position is fully outside `frame_area`, with a 1-cell margin.
pub fn slide_offscreen_position(
    slide_direction: SlideDirection,
    full_rect: SignedRect,
    frame_area: Rect,
) -> Position {
    const EDGE_MARGIN: i32 = 1;
    let width = full_rect.width as i32;
    let height = full_rect.height as i32;

    let frame_x = frame_area.x as i32;
    let frame_y = frame_area.y as i32;
    let frame_right = frame_area.right() as i32;
    let frame_bottom = frame_area.bottom() as i32;

    let start_x = match slide_direction {
        SlideDirection::FromLeft | SlideDirection::FromTopLeft | SlideDirection::FromBottomLeft => {
            frame_x - width - EDGE_MARGIN
        }
        SlideDirection::FromRight
        | SlideDirection::FromTopRight
        | SlideDirection::FromBottomRight => frame_right + EDGE_MARGIN,
        _ => full_rect.x,
    };

    let start_y = match slide_direction {
        SlideDirection::FromTop | SlideDirection::FromTopLeft | SlideDirection::FromTopRight => {
            frame_y - height - EDGE_MARGIN
        }
        SlideDirection::FromBottom
        | SlideDirection::FromBottomLeft
        | SlideDirection::FromBottomRight => frame_bottom + EDGE_MARGIN,
        _ => full_rect.y,
    };

    Position::new(start_x, start_y)
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_offscreen_position.rs</FILE>
// <DESC>Offscreen positioning helper for slide transitions</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
