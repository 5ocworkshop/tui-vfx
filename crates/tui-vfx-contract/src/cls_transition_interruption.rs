// <FILE>crates/tui-vfx-contract/src/cls_transition_interruption.rs</FILE> - <DESC>Transition interruption policy enum</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>V3.1 transition schema: reserve interactive interruption semantics early.</WCTX>
// <CLOG>0.2.0: MINOR — add preserveCurrentFrame and snapToEndThenStartNext policies from recipe-oracle mapping.
// 0.1.0: INIT — add interruption policy vocabulary.</CLOG>

/// Policy used when a transition is superseded before completion.
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
pub enum TransitionInterruption {
    /// Restart the new transition from its beginning.
    Restart,
    /// Reverse from the current visual state.
    ReverseFromCurrent,
    /// Continue toward the new target from the current visual state.
    ContinueToNewTarget,
    /// Snap to the ending state.
    SnapToEnd,
    /// Snap the current transition to its end before starting the next transition.
    SnapToEndThenStartNext,
    /// Snap to the starting state.
    SnapToStart,
    /// Preserve the current rendered frame until superseded by the next stable state.
    PreserveCurrentFrame,
    /// Cancel without applying a replacement transition.
    Cancel,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_interruption.rs</FILE> - <DESC>Transition interruption policy enum</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
