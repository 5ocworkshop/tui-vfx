// <FILE>crates/tui-vfx-next/src/cls_role_write_policy.rs</FILE> - <DESC>Semantic role write policy enum</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>New kernel Phase D0 verifier fix: make role-write enum wire shape strict and fully described.</WCTX>
// <CLOG>0.4.1: PATCH — switch explicit-role payload to a named field for strict schema descriptions.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract RoleWritePolicy enum.</CLOG>

use tui_vfx_types::RoleTag;

/// Policy for how a cell write updates the semantic role channel.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum RoleWritePolicy {
    /// Preserve the destination role.
    PreserveDestination,
    /// Copy the sampled-source role.
    CopySampledSource,
    /// Set one explicit role, used by procedural/shadow-style writers.
    SetExplicit {
        /// Role to write into the destination role channel.
        role: RoleTag,
    },
}

// <FILE>crates/tui-vfx-next/src/cls_role_write_policy.rs</FILE> - <DESC>Semantic role write policy enum</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
