// <FILE>crates/tui-vfx-contract/src/cls_transition_track_subject.rs</FILE> - <DESC>Transition track subject selector</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: distinguish single-subject tracks from from/to relations.</WCTX>
// <CLOG>0.1.0: INIT — add track subject enum.</CLOG>

/// Subject selector for a transition track.
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
pub enum TransitionTrackSubject {
    /// Prior surface or state.
    From,
    /// Next surface or state.
    To,
    /// Both prior and next surfaces.
    Both,
    /// Shared or matched element pair.
    Shared,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_track_subject.rs</FILE> - <DESC>Transition track subject selector</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
