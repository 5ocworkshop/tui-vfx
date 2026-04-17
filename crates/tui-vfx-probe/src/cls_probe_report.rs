// <FILE>crates/tui-vfx-probe/src/cls_probe_report.rs</FILE> - <DESC>Top-level probe report DTOs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>First-class diagnostics in probe report output</WCTX>
// <CLOG>MINOR: Add a diagnostics array to ProbeReport so higher-level tooling can surface structured visual-integrity findings directly in emitted reports</CLOG>

use serde::{Deserialize, Serialize};

use crate::cls_probe_cell::ProbeCell;
use crate::cls_probe_diagnostic::ProbeDiagnostic;
use crate::cls_probe_pipeline_inventory::ProbePipelineInventory;
use crate::cls_probe_request::ProbeRequest;
use crate::cls_probe_runtime_context::ProbeRuntimeContext;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<ProbeRuntimeContext>,
    pub summary: ProbeSummary,
    #[serde(default)]
    pub diagnostics: Vec<ProbeDiagnostic>,
    pub cells: Vec<ProbeCell>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_report.rs</FILE> - <DESC>Top-level probe report DTOs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
