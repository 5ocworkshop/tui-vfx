// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_calculate_rect_path.rs</FILE>
// <DESC>Slide rect calculation helper for an explicit path</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted slide rect calculation wrapper</CLOG>

use tui_vfx_types::Rect;

use crate::types::PathType;

use super::fnc_slide_calculate_rect_path_with_path_type::slide_calculate_rect_path_with_path_type;
use super::types::{SlidePath, SlidePhase};

/// Calculates the visible rectangle during a slide animation along an explicit 3-point path.
///
/// - `SlidingIn`: interpolates `start → dwell`
/// - `SlidingOut`: interpolates `dwell → end`
///
/// The resulting rect is clipped to `frame_area`.
pub fn slide_calculate_rect_path(
    path: SlidePath,
    frame_area: Rect,
    progress: f64,
    phase: SlidePhase,
) -> Rect {
    slide_calculate_rect_path_with_path_type(path, frame_area, progress, phase, &PathType::Linear)
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_calculate_rect_path.rs</FILE>
// <DESC>Slide rect calculation helper for an explicit path</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
