// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_calculate_rect.rs</FILE>
// <DESC>Slide rect calculation helper (offscreen path default)</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted slide rect calculation wrapper</CLOG>

use tui_vfx_types::Rect;

use crate::types::{Anchor, SignedRect, SlideDirection};

use super::fnc_slide_calculate_rect_path::slide_calculate_rect_path;
use super::fnc_slide_path_offscreen::slide_path_offscreen;
use super::types::SlidePhase;

/// Calculates the visible rectangle during a slide animation, clipped to the frame.
///
/// Returns `Rect::default()` when fully offscreen.
pub fn slide_calculate_rect(
    full_rect: SignedRect,
    frame_area: Rect,
    progress: f64,
    phase: SlidePhase,
    anchor: Anchor,
    slide_direction: SlideDirection,
) -> Rect {
    let path = slide_path_offscreen(frame_area, anchor, slide_direction, full_rect);
    slide_calculate_rect_path(path, frame_area, progress, phase)
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_calculate_rect.rs</FILE>
// <DESC>Slide rect calculation helper (offscreen path default)</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
