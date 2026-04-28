// <FILE>crates/tui-vfx-contract/src/cls_scene_outcome.rs</FILE> - <DESC>Scene composition outcome DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: report final composed surface and deterministic scene diagnostics.</WCTX>
// <CLOG>0.1.0: ADD — introduce schema-ready scene composition outcome.</CLOG>

use crate::{Surface, SurfaceDiagnostic};

/// Result of composing scene elements into one final semantic surface.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SceneOutcome {
    /// Final composed semantic surface.
    pub surface: Surface,
    /// Number of in-bounds element cells that were considered for writing.
    pub matched_cells: usize,
    /// Number of element cells actually written after cell write policy.
    pub written_cells: usize,
    /// Number of element-local cells clipped by final scene bounds.
    pub clipped_cells: usize,
    /// Element-aware diagnostics in deterministic composition order.
    pub diagnostics: Vec<SurfaceDiagnostic>,
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_outcome.rs</FILE> - <DESC>Scene composition outcome DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
