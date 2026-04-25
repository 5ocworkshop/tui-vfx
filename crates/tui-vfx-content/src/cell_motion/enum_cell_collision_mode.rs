// <FILE>crates/tui-vfx-content/src/cell_motion/enum_cell_collision_mode.rs</FILE> - <DESC>Cell motion collision policy</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: deterministic per-destination collision semantics.</WCTX>
// <CLOG>0.1.0: add source-order, completion, and baseline-preserving collision modes.</CLOG>

use serde::{Deserialize, Serialize};

/// Deterministic winner policy when multiple moved cell actors target one cell.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum CellCollisionMode {
    /// Highest authored row-major index wins; may overwrite baseline cells.
    #[default]
    SourceOrder,
    /// Lowest authored row-major index wins; may overwrite baseline cells.
    ReverseSourceOrder,
    /// Actor nearest completion wins; ties use selected ordinal then authored index.
    NearestToCompletion,
    /// Preserve non-empty unselected baseline cells; otherwise use source-order.
    PreserveExisting,
}

// <FILE>crates/tui-vfx-content/src/cell_motion/enum_cell_collision_mode.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
