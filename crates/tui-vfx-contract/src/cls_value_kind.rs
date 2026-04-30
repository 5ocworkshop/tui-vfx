// <FILE>crates/tui-vfx-contract/src/cls_value_kind.rs</FILE> - <DESC>Closed effect input value kind vocabulary</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Vertical recipe parity: carry structured primitive payload details through migration.</WCTX>
// <CLOG>0.3.0: MINOR — add structured value kind for descriptor-owned JSON payload fields.
// 0.2.0: MINOR — add gradient value kind.
// 0.1.0: INIT — add closed ValueKind enum for schema-backed effect inputs.</CLOG>

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
    /// Ordered gradient stop value.
    Gradient,
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
    /// Descriptor-owned structured JSON payload for fields whose shape is primitive-specific.
    Structured,
}

// <FILE>crates/tui-vfx-contract/src/cls_value_kind.rs</FILE> - <DESC>Closed effect input value kind vocabulary</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
