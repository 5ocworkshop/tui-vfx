// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_clip_cell_motion_position.rs</FILE> - <DESC>Clip signed cell-motion positions to local output coordinates</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: support non-zero local frames without double-translating returned scenes.</WCTX>
// <CLOG>0.1.0: add frame-aware clip-to-local helper.</CLOG>

use tui_vfx_geometry::types::Position;
use tui_vfx_types::Rect;

pub(crate) fn clip_cell_motion_position(pos: Position, frame: Rect) -> Option<(u16, u16)> {
    if pos.x < frame.x as i32
        || pos.y < frame.y as i32
        || pos.x >= frame.right() as i32
        || pos.y >= frame.bottom() as i32
    {
        None
    } else {
        Some((
            (pos.x - frame.x as i32) as u16,
            (pos.y - frame.y as i32) as u16,
        ))
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_clip_cell_motion_position.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
