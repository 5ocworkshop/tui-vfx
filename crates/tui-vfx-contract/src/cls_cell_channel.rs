// <FILE>crates/tui-vfx-contract/src/cls_cell_channel.rs</FILE> - <DESC>Canonical surface channel enum</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase G3: allow deterministic channel ordering in proof deltas.</WCTX>
// <CLOG>0.5.0: MINOR — derive ordering traits for channel-aware merge maps.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract CellChannel enum.</CLOG>

/// Canonical channels that make up a cell write.
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
pub enum CellChannel {
    /// Glyph character channel.
    Glyph,
    /// Foreground color channel.
    Foreground,
    /// Background color channel.
    Background,
    /// Text modifier channel.
    Modifiers,
    /// Modifier alpha channel.
    ModifierAlpha,
    /// Semantic role channel. Role writes are governed by role policy.
    Role,
}

// <FILE>crates/tui-vfx-contract/src/cls_cell_channel.rs</FILE> - <DESC>Canonical surface channel enum</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
