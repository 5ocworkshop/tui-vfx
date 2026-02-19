// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_path_offscreen.rs</FILE>
// <DESC>Build an offscreen → dwell → offscreen SlidePath</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted SlidePath builder wrapper</CLOG>

use tui_vfx_types::Rect;

use crate::types::{Anchor, SignedRect, SlideDirection};

use super::fnc_slide_path_offscreen_start_end::slide_path_offscreen_start_end;
use super::types::SlidePath;

/// Convenience helper: create an offscreen → dwell → offscreen slide path.
///
/// This preserves the legacy notifications slide semantics.
pub fn slide_path_offscreen(
    frame_area: Rect,
    anchor: Anchor,
    slide_direction: SlideDirection,
    dwell: SignedRect,
) -> SlidePath {
    slide_path_offscreen_start_end(frame_area, anchor, slide_direction, slide_direction, dwell)
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_path_offscreen.rs</FILE>
// <DESC>Build an offscreen → dwell → offscreen SlidePath</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
