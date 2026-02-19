// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_path_offscreen_start_end.rs</FILE>
// <DESC>Build an offscreen(enter) → dwell → offscreen(exit) SlidePath</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy notifications parity</WCTX>
// <CLOG>Extracted SlidePath constructor helper</CLOG>

use tui_vfx_types::Rect;

use crate::types::{Anchor, SignedRect, SlideDirection};

use super::fnc_resolve_slide_direction::resolve_slide_direction;
use super::fnc_slide_offscreen_position::slide_offscreen_position;
use super::types::SlidePath;

/// Convenience helper: create an offscreen(enter) → dwell → offscreen(exit) slide path.
///
/// Allows enter and exit directions to differ (e.g. enter from bottom, exit to top).
pub fn slide_path_offscreen_start_end(
    frame_area: Rect,
    anchor: Anchor,
    enter_direction: SlideDirection,
    exit_direction: SlideDirection,
    dwell: SignedRect,
) -> SlidePath {
    let enter_dir = resolve_slide_direction(enter_direction, anchor);
    let exit_dir = resolve_slide_direction(exit_direction, anchor);

    let enter_pos = slide_offscreen_position(enter_dir, dwell, frame_area);
    let exit_pos = slide_offscreen_position(exit_dir, dwell, frame_area);

    let start = SignedRect::new(enter_pos.x, enter_pos.y, dwell.width, dwell.height);
    let end = SignedRect::new(exit_pos.x, exit_pos.y, dwell.width, dwell.height);

    SlidePath { start, dwell, end }
}

// <FILE>tui-vfx-geometry/src/transitions/fnc_slide_path_offscreen_start_end.rs</FILE>
// <DESC>Build an offscreen(enter) → dwell → offscreen(exit) SlidePath</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
