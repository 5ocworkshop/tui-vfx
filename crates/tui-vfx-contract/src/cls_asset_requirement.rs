// <FILE>crates/tui-vfx-contract/src/cls_asset_requirement.rs</FILE> - <DESC>Source descriptor asset slot requirement DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: declare source asset slots.</WCTX>
// <CLOG>0.1.0: INIT — add required/optional asset slot compatibility contract.</CLOG>

use crate::{AssetFormat, AssetKind};

/// Asset slot declared by a source descriptor.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AssetRequirement {
    /// Broad asset kind accepted by this source asset slot.
    pub kind: AssetKind,
    /// Exact asset format accepted by this source asset slot.
    pub format: AssetFormat,
    /// Whether a source spec must supply this asset slot.
    pub required: bool,
    /// Optional human-facing slot description.
    pub description: Option<String>,
}

// <FILE>crates/tui-vfx-contract/src/cls_asset_requirement.rs</FILE> - <DESC>Source descriptor asset slot requirement DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
