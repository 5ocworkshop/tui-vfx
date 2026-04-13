// <FILE>crates/tui-vfx-probe/src/cls_probe_grid_spec.rs</FILE> - <DESC>Serializable grid specification for probe scenes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add a row-major serializable grid DTO for source and destination frames</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::Cell;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeGridSpec {
    pub width: u16,
    pub height: u16,
    pub cells: Vec<Cell>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_grid_spec.rs</FILE> - <DESC>Serializable grid specification for probe scenes</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
