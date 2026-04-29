// <FILE>crates/tui-vfx-contract/src/cls_asset_kind.rs</FILE> - <DESC>Source asset kind vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H0: classify assets consumed by source descriptors.</WCTX>
// <CLOG>0.1.0: INIT — add closed built-in asset kind vocabulary plus custom escape hatch.</CLOG>

/// Broad source asset kind used for validation before runtime loading exists.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum AssetKind {
    /// Raster or vector image-like source material.
    Image,
    /// ANSI/terminal capture source material.
    Ansi,
    /// Braille dotfield source material.
    BrailleDotfield,
    /// JSON-encoded structured source data.
    Json,
    /// Pre-shaped cell-grid source material.
    CellGrid,
    /// Opaque binary source material.
    Binary,
    /// Project-defined asset kind with explicit stable name.
    Custom {
        /// Stable custom asset kind name.
        name: String,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_asset_kind.rs</FILE> - <DESC>Source asset kind vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
