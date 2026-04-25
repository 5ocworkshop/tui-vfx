// <FILE>crates/tui-vfx-content/src/cell_motion/enum_cell_stagger.rs</FILE> - <DESC>Cell motion stagger vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: deterministic actor-local stagger offsets.</WCTX>
// <CLOG>0.1.0: add none/index/position/distance/random stagger specs.</CLOG>

use super::CellPlacement;
use serde::{Deserialize, Serialize};

/// Axis used by position-ranked cell stagger.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CellStaggerAxis {
    #[default]
    X,
    Y,
}

/// Rank direction used by position-ranked cell stagger.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CellStaggerDirection {
    #[default]
    Ascending,
    Descending,
}

/// Actor-specific start-delay policy for one cell-motion phase.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CellStagger {
    /// No actor-specific delay.
    #[default]
    None,
    /// Delay by selected row-major ordinal.
    ByIndex { stride_ms: u64 },
    /// Delay by rank along an authored coordinate axis.
    ByPosition {
        axis: CellStaggerAxis,
        direction: CellStaggerDirection,
        stride_ms: u64,
    },
    /// Delay by Manhattan distance from a resolved origin placement.
    ByDistance {
        origin: CellPlacement,
        stride_ms: u64,
    },
    /// Deterministic pseudo-random delay in `0..=max_offset_ms`.
    Random { seed: u64, max_offset_ms: u64 },
}

// <FILE>crates/tui-vfx-content/src/cell_motion/enum_cell_stagger.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
