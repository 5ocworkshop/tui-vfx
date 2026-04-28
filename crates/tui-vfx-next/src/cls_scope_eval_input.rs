// <FILE>crates/tui-vfx-next/src/cls_scope_eval_input.rs</FILE> - <DESC>Scope evaluation input DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
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
}

// <FILE>crates/tui-vfx-next/src/cls_scope_eval_input.rs</FILE> - <DESC>Scope evaluation input DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
