// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_actor.rs</FILE> - <DESC>Cell motion actor identity</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: stable per-cell actor identity for content-local motion.</WCTX>
// <CLOG>0.1.0: add row-major CellActor for deterministic cell-motion scheduling.</CLOG>

use tui_vfx_types::{Cell, RoleTag};

/// Stable row-major actor extracted from a source [`tui_vfx_types::SemanticScene`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellActor {
    /// Row-major index over the full source surface, including unselected cells.
    pub authored_index: u32,
    /// Row-major ordinal among selected actors only.
    pub selected_ordinal: u32,
    /// Source x coordinate in authored/source space.
    pub authored_x: u16,
    /// Source y coordinate in authored/source space.
    pub authored_y: u16,
    /// Full source cell payload moved by the scheduler.
    pub cell: Cell,
    /// Semantic role moved with the cell payload.
    pub role: RoleTag,
}

// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_actor.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
