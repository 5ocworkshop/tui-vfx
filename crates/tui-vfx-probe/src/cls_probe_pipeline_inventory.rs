// <FILE>crates/tui-vfx-probe/src/cls_probe_pipeline_inventory.rs</FILE> - <DESC>Configured pipeline inventory DTO for probe reports</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>0.3.0: add grouped V3 style-effect family labels so probe reports can surface overall style categories alongside concrete style effect labels.
// MINOR: Add configured pipeline element name lists so analysis can report success/failure per concrete sampler, mask, shader, filter, style, and content effect instead of only per stage</CLOG>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbePipelineInventory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sampler_effects: Vec<String>,
    pub mask_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mask_effects: Vec<String>,
    pub filter_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter_effects: Vec<String>,
    pub shader_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shader_effects: Vec<String>,
    pub style_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style_effects: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// Grouped V3 family labels for active non-spatial style effects.
    pub style_effect_families: Vec<String>,
    pub content_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_effects: Vec<String>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_pipeline_inventory.rs</FILE> - <DESC>Configured pipeline inventory DTO for probe reports</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
