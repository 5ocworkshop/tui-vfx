// <FILE>crates/tui-vfx-contract/src/cls_transition_reveal_direction.rs</FILE> - <DESC>Qualified wipe reveal direction enum</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Add horizontal/vertical centerOut and edgesIn directions for symmetric two-sided wipes.</WCTX>
// <CLOG>0.2.0: MINOR — add four symmetric reveal directions (horizontal/vertical × centerOut/edgesIn).</CLOG>

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
    /// Symmetric horizontal reveal expanding outward from the vertical centerline.
    HorizontalCenterOut,
    /// Symmetric horizontal reveal collapsing inward from both vertical edges.
    HorizontalEdgesIn,
    /// Symmetric vertical reveal expanding outward from the horizontal centerline.
    VerticalCenterOut,
    /// Symmetric vertical reveal collapsing inward from both horizontal edges.
    VerticalEdgesIn,
    /// Reveal progresses along a configured angle.
    Angle,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_reveal_direction.rs</FILE> - <DESC>Qualified wipe reveal direction enum</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
