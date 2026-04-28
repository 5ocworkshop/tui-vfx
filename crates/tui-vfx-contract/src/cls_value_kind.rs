// <FILE>crates/tui-vfx-contract/src/cls_value_kind.rs</FILE> - <DESC>Closed effect input value kind vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: declare the first typed effect input value vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — add closed ValueKind enum for schema-backed effect inputs.</CLOG>

/// Closed v3.1 vocabulary for typed effect input values.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ValueKind {
    /// Explicit absence of a value.
    Null,
    /// Boolean true/false value.
    Boolean,
    /// Signed integer value without a fractional component.
    Integer,
    /// Floating-point numeric value.
    Number,
    /// Short machine-facing string value.
    String,
    /// Human-facing text value.
    Text,
    /// RGBA color value.
    Color,
    /// Duration value expressed as seconds.
    Duration,
    /// One value selected from a declared closed string set.
    Enum,
    /// Semantic role tag value.
    Role,
    /// Surface scope value.
    Scope,
    /// Cell rectangle value.
    Rect,
}

// <FILE>crates/tui-vfx-contract/src/cls_value_kind.rs</FILE> - <DESC>Closed effect input value kind vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
