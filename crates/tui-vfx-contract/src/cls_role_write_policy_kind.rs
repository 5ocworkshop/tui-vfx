// <FILE>crates/tui-vfx-contract/src/cls_role_write_policy_kind.rs</FILE> - <DESC>Role write policy support kind enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: describe supported role write policies without payloads.</WCTX>
// <CLOG>0.1.0: INIT — add payload-free role write policy kind for descriptor capabilities.</CLOG>

use crate::RoleWritePolicy;

/// Payload-free role write policy kind used by descriptor support declarations.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum RoleWritePolicyKind {
    /// Descriptor may preserve destination roles.
    PreserveDestination,
    /// Descriptor may copy sampled-source roles.
    CopySampledSource,
    /// Descriptor may set an explicit role chosen by the operation.
    SetExplicit,
}

impl From<&RoleWritePolicy> for RoleWritePolicyKind {
    fn from(policy: &RoleWritePolicy) -> Self {
        match policy {
            RoleWritePolicy::PreserveDestination => Self::PreserveDestination,
            RoleWritePolicy::CopySampledSource => Self::CopySampledSource,
            RoleWritePolicy::SetExplicit { .. } => Self::SetExplicit,
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_role_write_policy_kind.rs</FILE> - <DESC>Role write policy support kind enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
