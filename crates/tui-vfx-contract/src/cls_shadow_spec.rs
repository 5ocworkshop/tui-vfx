// <FILE>crates/tui-vfx-contract/src/cls_shadow_spec.rs</FILE> - <DESC>Typed scene-element shadow attachment DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Scene shadow contract: shadows are surface attachments that may expand paint bounds and coordinate with viewport edge crossing.</WCTX>
// <CLOG>0.2.0: MINOR — add explicit shadow edge-crossing policy for viewport transit.
// 0.1.0: INIT — add typed shadow geometry and compositing vocabulary.</CLOG>

use tui_vfx_types::Color;

use crate::ScopeSpec;

/// Typed surface-attached shadow specification for scene elements.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShadowSpec {
    /// Optional source cells that cast the shadow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_region: Option<ScopeSpec>,
    /// Edges that contribute to the shadow.
    pub edges: Vec<ShadowEdge>,
    /// Shadow offset in cells.
    pub offset: ShadowOffset,
    /// Optional inset along the selected edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inset: Option<ShadowInset>,
    /// Optional shadow falloff in cells.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub falloff: Option<ShadowFalloff>,
    /// Shadow color.
    pub shadow_color: Color,
    /// Whether the shadow edge should be softened.
    pub soft_edges: bool,
    /// Where the shadow composites relative to the element.
    pub composite_mode: ShadowCompositeMode,
    /// Blend operation used when compositing the shadow.
    pub blend_mode: ShadowBlendMode,
    /// Optional policy for shadows while the source surface crosses a viewport edge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge_crossing_policy: Option<ShadowEdgeCrossingPolicy>,
    /// Optional glyph material policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glyph_material: Option<ShadowGlyphMaterial>,
    /// Optional paint expansion needed beyond the source element bounds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paint_outset: Option<ShadowOutset>,
}

/// Shadow edge selector.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ShadowEdge {
    Top,
    Right,
    Bottom,
    Left,
}

/// Shadow offset in cells.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShadowOffset {
    /// Horizontal shadow offset in cells.
    pub x: i16,
    /// Vertical shadow offset in cells.
    pub y: i16,
}

/// Shadow inset along an edge.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShadowInset {
    /// Inset from the start of the selected edge.
    pub start: u16,
    /// Inset from the end of the selected edge.
    pub end: u16,
}

/// Shadow falloff in cells.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShadowFalloff {
    /// Horizontal falloff in cells.
    pub x: u16,
    /// Vertical falloff in cells.
    pub y: u16,
}

/// Optional paint expansion needed for support surfaces such as shadows.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShadowOutset {
    /// Paint expansion to the left of the source bounds.
    pub left: u16,
    /// Paint expansion to the right of the source bounds.
    pub right: u16,
    /// Paint expansion above the source bounds.
    pub top: u16,
    /// Paint expansion below the source bounds.
    pub bottom: u16,
}

/// Relative compositing layer for a shadow.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ShadowCompositeMode {
    Under,
    Over,
}

/// Shadow blend operation.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ShadowBlendMode {
    SourceOver,
    Multiply,
}

/// Shadow behavior while a moving source surface crosses a viewport edge.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ShadowEdgeCrossingPolicy {
    /// Use the renderer's default edge-crossing behavior.
    Default,
    /// Fade shadow coverage at the crossed edge.
    Fade,
    /// Preserve shadow coverage even while the source crosses the edge.
    Preserve,
}

/// Glyph material used by shadow support cells.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ShadowGlyphMaterial {
    Solid,
    PreserveDestination,
}

// <FILE>crates/tui-vfx-contract/src/cls_shadow_spec.rs</FILE> - <DESC>Typed scene-element shadow attachment DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
