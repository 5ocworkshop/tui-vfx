// <FILE>crates/tui-vfx-contract/src/cls_asset_spec.rs</FILE> - <DESC>Canonical asset declaration DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: declare assets available to source specs.</WCTX>
// <CLOG>0.1.0: INIT — add asset identity, kind, format, locator, and validation.</CLOG>

use crate::{AssetFormat, AssetId, AssetKind, AssetLocator, DescriptorValidationError};

/// Canonical asset declaration available for structural source asset refs.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AssetSpec {
    /// Stable asset id referenced by source specs.
    pub id: AssetId,
    /// Broad asset kind used for compatibility validation.
    pub kind: AssetKind,
    /// Exact asset format expected by the source adapter family.
    pub format: AssetFormat,
    /// Structural locator for the asset material.
    pub locator: AssetLocator,
    /// Optional human-facing description for catalogs and generated reference docs.
    pub description: Option<String>,
}

impl AssetSpec {
    /// Validate asset identity and locator structure.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if !self.id.is_valid() {
            return Err(DescriptorValidationError::InvalidAssetId {
                id: self.id.clone(),
            });
        }
        self.locator.validate()
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_asset_spec.rs</FILE> - <DESC>Canonical asset declaration DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
