// <FILE>crates/tui-vfx-contract/src/cls_transition_cascade_order.rs</FILE> - <DESC>Content transition cascade order enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition recipe-oracle pass: content reveal tracks need deterministic glyph/cell ordering.</WCTX>
// <CLOG>0.1.0: INIT — add cascade order vocabulary for content reveal tracks.</CLOG>

/// Deterministic ordering for content reveal cascades.
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
pub enum TransitionCascadeOrder {
    /// Reveal from the leftmost visible content toward the right.
    LeftToRight,
    /// Reveal from the rightmost visible content toward the left.
    RightToLeft,
    /// Reveal from top rows toward bottom rows.
    TopToBottom,
    /// Reveal from bottom rows toward top rows.
    BottomToTop,
    /// Reveal outward from the center.
    CenterOut,
    /// Reveal inward toward the center.
    EdgesIn,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_cascade_order.rs</FILE> - <DESC>Content transition cascade order enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
