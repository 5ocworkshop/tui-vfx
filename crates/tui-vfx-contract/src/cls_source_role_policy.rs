// <FILE>crates/tui-vfx-contract/src/cls_source_role_policy.rs</FILE> - <DESC>Source output role behavior DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: describe role assignment for produced surfaces.</WCTX>
// <CLOG>0.1.0: INIT — add explicit, default-role, and generated role policies.</CLOG>

use tui_vfx_types::RoleTag;

/// Declares how a source assigns roles to its produced semantic surface.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum SourceRolePolicy {
    /// Source writes roles explicitly per produced cell.
    Explicit,
    /// Source applies one default role to produced cells unless overridden by a later rule.
    DefaultRole {
        /// Default role assigned to produced cells.
        role: RoleTag,
    },
    /// Source generates roles structurally, such as card border/background/text roles.
    Generated,
}

// <FILE>crates/tui-vfx-contract/src/cls_source_role_policy.rs</FILE> - <DESC>Source output role behavior DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
