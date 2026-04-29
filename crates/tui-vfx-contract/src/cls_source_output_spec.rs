// <FILE>crates/tui-vfx-contract/src/cls_source_output_spec.rs</FILE> - <DESC>Source output surface contract DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: describe source-produced surface semantics.</WCTX>
// <CLOG>0.1.0: INIT — add output kind, size behavior, and role behavior.</CLOG>

use crate::{SourceOutputSize, SourceRolePolicy};

/// Contract for the semantic surface produced by a source descriptor.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceOutputSpec {
    /// Size behavior of the produced surface.
    pub size: SourceOutputSize,
    /// Role assignment behavior of the produced surface.
    pub roles: SourceRolePolicy,
}

// <FILE>crates/tui-vfx-contract/src/cls_source_output_spec.rs</FILE> - <DESC>Source output surface contract DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
