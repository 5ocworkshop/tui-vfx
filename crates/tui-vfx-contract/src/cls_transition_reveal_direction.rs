// <FILE>crates/tui-vfx-contract/src/cls_transition_reveal_direction.rs</FILE> - <DESC>Qualified wipe reveal direction enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 field normalization: avoid ambiguous generic direction fields for wipes.</WCTX>
// <CLOG>0.1.0: INIT — add revealDirection vocabulary.</CLOG>

/// Direction of coverage progression for a visibility wipe.
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
pub enum TransitionRevealDirection {
    /// Reveal progresses from left to right.
    LeftToRight,
    /// Reveal progresses from right to left.
    RightToLeft,
    /// Reveal progresses from top to bottom.
    TopToBottom,
    /// Reveal progresses from bottom to top.
    BottomToTop,
    /// Reveal progresses along a configured angle.
    Angle,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_reveal_direction.rs</FILE> - <DESC>Qualified wipe reveal direction enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
