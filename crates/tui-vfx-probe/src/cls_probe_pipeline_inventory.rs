// <FILE>crates/tui-vfx-probe/src/cls_probe_pipeline_inventory.rs</FILE> - <DESC>Configured pipeline inventory DTO for probe reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>NEW: Add configured pipeline inventory counts for frame dump metadata</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbePipelineInventory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<String>,
    pub mask_count: usize,
    pub filter_count: usize,
    pub shader_count: usize,
    pub style_count: usize,
    pub content_count: usize,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_pipeline_inventory.rs</FILE> - <DESC>Configured pipeline inventory DTO for probe reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
