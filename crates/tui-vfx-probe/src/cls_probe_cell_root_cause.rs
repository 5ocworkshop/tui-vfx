// <FILE>crates/tui-vfx-probe/src/cls_probe_cell_root_cause.rs</FILE> - <DESC>Cell-centric root-cause explainer DTOs for probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Cell-centric root-cause explainers for probe debugging</WCTX>
// <CLOG>NEW: Add typed per-cell root-cause summaries so probe consumers can ask why one cell ended up wrong without manually reconstructing the whole trace</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_style::traits::cls_shader_context::ShaderRuntimeBindingResolution;

use crate::cls_probe_report::ProbePoint;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeCellStageCause {
    pub stage: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeCellRootCause {
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_stage: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub changed_stages: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub hidden_by_masks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampled_from: Option<ProbePoint>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub bindings: Vec<ShaderRuntimeBindingResolution>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub stage_causes: Vec<ProbeCellStageCause>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_cell_root_cause.rs</FILE> - <DESC>Cell-centric root-cause explainer DTOs for probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
