// <FILE>crates/tui-vfx-contract/src/cls_descriptor_pack_ref.rs</FILE> - <DESC>Recipe reference to an external descriptor pack</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: let canonical recipes name external primitive descriptor packs.</WCTX>
// <CLOG>0.1.0: INIT — add minimal descriptor pack reference DTO.</CLOG>

use crate::DescriptorPackId;

/// Recipe-local declaration that an external descriptor pack is required.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DescriptorPackRef {
    /// Required descriptor pack id.
    pub id: DescriptorPackId,
}

// <FILE>crates/tui-vfx-contract/src/cls_descriptor_pack_ref.rs</FILE> - <DESC>Recipe reference to an external descriptor pack</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
