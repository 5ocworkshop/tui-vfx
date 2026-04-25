// <FILE>crates/tui-vfx-content/src/cell_motion/enum_cell_placement.rs</FILE> - <DESC>Cell motion placement vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: authored/local-frame placement endpoints for per-cell motion.</WCTX>
// <CLOG>0.1.0: add authored, offset, absolute, origin, and offscreen placements.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_geometry::types::{Anchor, SlideDirection};

/// Basis used when resolving an anchored cell-motion origin.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CellPlacementBasis {
    /// Resolve anchor inside bounds of selected actors.
    #[default]
    SelectionBounds,
    /// Resolve anchor inside the full local motion frame.
    LocalFrame,
}

/// Source-space placement endpoint for one cell actor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CellPlacement {
    /// The actor's original source coordinate.
    Authored,
    /// The actor's original source coordinate plus a signed offset.
    AuthoredOffset { dx: i32, dy: i32 },
    /// One absolute local-frame coordinate. May be outside the frame.
    Absolute { x: i32, y: i32 },
    /// Anchor resolved against selected bounds or the local frame.
    Origin {
        anchor: Anchor,
        basis: CellPlacementBasis,
    },
    /// Just outside the local frame in the given direction.
    Offscreen {
        direction: SlideDirection,
        #[serde(default)]
        margin_cells: u16,
    },
}

// <FILE>crates/tui-vfx-content/src/cell_motion/enum_cell_placement.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
