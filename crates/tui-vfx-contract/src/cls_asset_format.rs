// <FILE>crates/tui-vfx-contract/src/cls_asset_format.rs</FILE> - <DESC>Canonical asset format identifier newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: identify source asset formats.</WCTX>
// <CLOG>0.1.0: INIT — add transparent asset format DTO.</CLOG>

/// Stable asset format identifier, such as `tui-vfx.braille_flag_asset.v1`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(transparent)]
pub struct AssetFormat(
    /// Format identifier string.
    pub String,
);

impl AssetFormat {
    /// Build an asset format from a string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Borrow the format as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// <FILE>crates/tui-vfx-contract/src/cls_asset_format.rs</FILE> - <DESC>Canonical asset format identifier newtype</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
