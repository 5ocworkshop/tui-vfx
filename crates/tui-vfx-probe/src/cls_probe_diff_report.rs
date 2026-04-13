// <FILE>crates/tui-vfx-probe/src/cls_probe_diff_report.rs</FILE> - <DESC>Top-level frame diff report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe diff support</WCTX>
// <CLOG>NEW: Add a structured diff report for comparing one phase-local sample against another</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_diff_cell::ProbeDiffCell;
use crate::cls_probe_pipeline_inventory::ProbePipelineInventory;
use crate::cls_probe_report::{ProbeFrame, ProbeReportSource};
use crate::cls_probe_request::ProbePhase;
use crate::cls_probe_widget::ProbeWidget;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeDiffReport {
    pub schema_version: String,
    pub kind: String,
    pub source: ProbeReportSource,
    pub phase: ProbePhase,
    pub from_t: f64,
    pub to_t: f64,
    pub frame: ProbeFrame,
    pub widget: ProbeWidget,
    pub pipeline: ProbePipelineInventory,
    pub changed_cells_count: usize,
    pub cells: Vec<ProbeDiffCell>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_diff_report.rs</FILE> - <DESC>Top-level frame diff report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
