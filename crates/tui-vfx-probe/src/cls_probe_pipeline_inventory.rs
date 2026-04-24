// <FILE>crates/tui-vfx-probe/src/cls_probe_pipeline_inventory.rs</FILE> - <DESC>Configured pipeline inventory DTO for probe reports</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Phase-1 pipeline probe scaffolding</WCTX>
// <CLOG>0.5.0: track ordered sampler-chain inventory explicitly so probe reports can describe multi-sampler specs without collapsing them to the legacy single-sampler mirror.</CLOG>

use serde::{Deserialize, Serialize};

/// Configured pipeline inventory captured alongside one probe report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbePipelineInventory {
    /// Legacy single-sampler compatibility mirror, when one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<String>,
    /// Number of active configured samplers in authored order.
    pub sampler_count: usize,
    /// Ordered sampler labels mirrored from the effective sampler chain.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sampler_effects: Vec<String>,
    /// Number of configured masks.
    pub mask_count: usize,
    /// Ordered mask labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mask_effects: Vec<String>,
    /// Number of configured filters.
    pub filter_count: usize,
    /// Ordered filter labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filter_effects: Vec<String>,
    /// Number of configured shader layers.
    pub shader_count: usize,
    /// Ordered shader labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shader_effects: Vec<String>,
    /// Ordered grouped V3 shader family labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shader_families: Vec<String>,
    /// Number of configured non-spatial style effects.
    pub style_count: usize,
    /// Ordered non-spatial style effect labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style_effects: Vec<String>,
    /// Grouped V3 family labels for active non-spatial style effects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub style_effect_families: Vec<String>,
    /// Number of configured content effects.
    pub content_count: usize,
    /// Ordered content effect labels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_effects: Vec<String>,
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_pipeline_inventory.rs</FILE> - <DESC>Configured pipeline inventory DTO for probe reports</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
