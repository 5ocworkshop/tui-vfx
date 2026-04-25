// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_selected_cell_actor_bounds.rs</FILE> - <DESC>Resolve selected cell actor bounds</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: hoist selected actor bounds for cell-motion placement resolution.</WCTX>
// <CLOG>0.1.0: add frame-aware selected bounds helper.</CLOG>

use super::CellActor;
use tui_vfx_types::Rect;

pub(crate) fn selected_cell_actor_bounds(actors: &[CellActor], local_frame: Rect) -> Option<Rect> {
    let min_x = actors.iter().map(|a| a.authored_x).min()?;
    let max_x = actors.iter().map(|a| a.authored_x).max()?;
    let min_y = actors.iter().map(|a| a.authored_y).min()?;
    let max_y = actors.iter().map(|a| a.authored_y).max()?;
    Some(Rect::new(
        local_frame.x.saturating_add(min_x),
        local_frame.y.saturating_add(min_y),
        max_x - min_x + 1,
        max_y - min_y + 1,
    ))
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_selected_cell_actor_bounds.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
