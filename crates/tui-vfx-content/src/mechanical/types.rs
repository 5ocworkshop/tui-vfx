// <FILE>crates/tui-vfx-content/src/mechanical/types.rs</FILE> - <DESC>Shared internal mechanical display data types</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 mechanical helpers for grid-first odometer tile roll.</WCTX>
// <CLOG>Add source, sizing, and tile structs used by private roll helpers.</CLOG>

use tui_vfx_types::OwnedGrid;

pub(crate) struct MechanicalSource {
    pub(crate) from: OwnedGrid,
    pub(crate) to: OwnedGrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MechanicalSizing {
    PadToMax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MechanicalTile {
    pub(crate) width: u16,
    pub(crate) height: u16,
}

impl MechanicalTile {
    pub(crate) fn new(width: u16, height: u16) -> Option<Self> {
        if width == 0 || height == 0 {
            None
        } else {
            Some(Self { width, height })
        }
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/types.rs</FILE> - <DESC>Shared internal mechanical display data types</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
