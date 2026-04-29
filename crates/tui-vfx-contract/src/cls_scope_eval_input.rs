// <FILE>crates/tui-vfx-contract/src/cls_scope_eval_input.rs</FILE> - <DESC>Scope evaluation input DTO</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>K2.13 schema decision burn-down: carry optional content and dimension context for built-in scopes.</WCTX>
// <CLOG>0.5.0: MINOR — add optional glyph and dimensions for non-empty, outer-band, and inner scope evaluation.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract ScopeEvalInput DTO.</CLOG>

use tui_vfx_types::RoleTag;

/// Input bundle used to evaluate a scope for one destination cell.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScopeEvalInput {
    /// Destination-local x coordinate.
    pub destination_x: usize,
    /// Destination-local y coordinate.
    pub destination_y: usize,
    /// Sampled-source x coordinate.
    pub sampled_source_x: usize,
    /// Sampled-source y coordinate.
    pub sampled_source_y: usize,
    /// Role observed at the sampled source coordinate.
    pub sampled_source_role: RoleTag,
    /// Role observed at the destination coordinate before writing.
    pub destination_role: RoleTag,
    /// Optional destination-local surface width.
    #[serde(default)]
    pub destination_width: Option<usize>,
    /// Optional destination-local surface height.
    #[serde(default)]
    pub destination_height: Option<usize>,
    /// Optional sampled-source surface width.
    #[serde(default)]
    pub sampled_source_width: Option<usize>,
    /// Optional sampled-source surface height.
    #[serde(default)]
    pub sampled_source_height: Option<usize>,
    /// Optional glyph at the destination coordinate before writing.
    #[serde(default)]
    pub destination_glyph: Option<String>,
    /// Optional glyph at the sampled-source coordinate.
    #[serde(default)]
    pub sampled_source_glyph: Option<String>,
}

// <FILE>crates/tui-vfx-contract/src/cls_scope_eval_input.rs</FILE> - <DESC>Scope evaluation input DTO</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
