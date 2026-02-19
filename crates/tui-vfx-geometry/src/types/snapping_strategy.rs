// <FILE>tui-vfx-geometry/src/types/snapping_strategy.rs</FILE> - <DESC>Enum for grid rounding</DESC>
// <VERS>VERSION: 2.0.0 - 2025-12-31</VERS>
// <WCTX>V2.2 schema standardization - snake_case serialization</WCTX>
// <CLOG>BREAKING: Added snake_case serde serialization for all variants</CLOG>

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SnappingStrategy {
    Floor,
    Round,
    Stochastic { seed: u64 },
}

// <FILE>tui-vfx-geometry/src/types/snapping_strategy.rs</FILE> - <DESC>Enum for grid rounding</DESC>
// <VERS>END OF VERSION: 2.0.0 - 2025-12-31</VERS>
