// <FILE>crates/tui-vfx-contract/src/cls_asset_locator.rs</FILE> - <DESC>Canonical source asset locator DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: describe assets without loading them.</WCTX>
// <CLOG>0.1.0: INIT — add structural path/logical locator variants and interpolation guard.</CLOG>

use crate::DescriptorValidationError;

/// Structural locator for an asset declaration; not a string interpolation token.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum AssetLocator {
    /// Repository- or package-relative path to asset material.
    Path {
        /// Canonical path string. Must not contain legacy interpolation markers.
        path: String,
    },
    /// Logical locator resolved by a later host/runtime resolver.
    Logical {
        /// Resolver-owned logical locator string.
        locator: String,
    },
}

impl AssetLocator {
    /// Validate that locator strings do not encode legacy interpolation tokens.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        let value = match self {
            Self::Path { path } => path,
            Self::Logical { locator } => locator,
        };
        if value.contains("{{") || value.contains("}}") {
            Err(DescriptorValidationError::InterpolatedAssetLocator {
                locator: value.clone(),
            })
        } else {
            Ok(())
        }
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_asset_locator.rs</FILE> - <DESC>Canonical source asset locator DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
