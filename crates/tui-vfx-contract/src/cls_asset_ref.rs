// <FILE>crates/tui-vfx-contract/src/cls_asset_ref.rs</FILE> - <DESC>Structural source asset reference DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: replace interpolation tokens with asset refs.</WCTX>
// <CLOG>0.1.0: INIT — add canonical asset ref by id.</CLOG>

use crate::AssetId;

/// Structural reference from a source asset slot to a declared asset.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AssetRef {
    /// Referenced asset id from the containing catalog/document.
    pub id: AssetId,
}

// <FILE>crates/tui-vfx-contract/src/cls_asset_ref.rs</FILE> - <DESC>Structural source asset reference DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
