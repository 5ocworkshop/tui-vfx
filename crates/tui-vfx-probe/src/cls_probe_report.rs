// <FILE>crates/tui-vfx-probe/src/cls_probe_report.rs</FILE> - <DESC>Top-level probe report DTOs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add top-level frame dump report DTOs plus shared point/size metadata types</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_cell::ProbeCell;
use crate::cls_probe_pipeline_inventory::ProbePipelineInventory;
use crate::cls_probe_request::ProbeRequest;
use crate::cls_probe_summary::ProbeSummary;
use crate::cls_probe_timing::ProbeTiming;
use crate::cls_probe_widget::ProbeWidget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbePoint {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeFrame {
    pub size: ProbeSize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeReportSource {
    pub input_kind: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeReport {
    pub schema_version: String,
    pub kind: String,
    pub source: ProbeReportSource,
    pub request: ProbeRequest,
    pub timing: ProbeTiming,
    pub frame: ProbeFrame,
    pub widget: ProbeWidget,
    pub pipeline: ProbePipelineInventory,
    pub summary: ProbeSummary,
    pub cells: Vec<ProbeCell>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_report.rs</FILE> - <DESC>Top-level probe report DTOs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
