// <FILE>crates/tui-vfx-probe/src/cls_probe_summary.rs</FILE> - <DESC>Frame summary DTO for probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add summary counts for total, non-empty, and modified cells</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeSummary {
    pub total_cells: usize,
    pub non_empty_cells: usize,
    pub modified_cells: usize,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_summary.rs</FILE> - <DESC>Frame summary DTO for probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
