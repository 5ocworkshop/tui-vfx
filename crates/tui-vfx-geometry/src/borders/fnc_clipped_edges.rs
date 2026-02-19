// <FILE>tui-vfx-geometry/src/borders/fnc_clipped_edges.rs</FILE>
// <DESC>Compute which viewport edges a rect is clipped against</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy effects parity: vanishing-edge border behavior</WCTX>
// <CLOG>Extracted clipped-edges decision logic</CLOG>

use tui_vfx_types::Rect;

use super::border_trim_spec::ClippedEdges;

/// Returns which viewport edges the `visible_area` is clipped against.
pub fn clipped_edges(frame_area: Rect, dwell_rect: Rect, visible_area: Rect) -> ClippedEdges {
    let clipped_horiz = visible_area.width < dwell_rect.width;
    let clipped_vert = visible_area.height < dwell_rect.height;

    let left = clipped_horiz && visible_area.x == frame_area.x;
    let right =
        clipped_horiz && visible_area.right() == frame_area.right() && visible_area.width > 0;
    let top = clipped_vert && visible_area.y == frame_area.y;
    let bottom =
        clipped_vert && visible_area.bottom() == frame_area.bottom() && visible_area.height > 0;

    ClippedEdges {
        top,
        right,
        bottom,
        left,
    }
}

// <FILE>tui-vfx-geometry/src/borders/fnc_clipped_edges.rs</FILE>
// <DESC>Compute which viewport edges a rect is clipped against</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
