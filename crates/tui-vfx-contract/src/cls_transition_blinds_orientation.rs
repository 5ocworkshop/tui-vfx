// <FILE>crates/tui-vfx-contract/src/cls_transition_blinds_orientation.rs</FILE> - <DESC>Visibility blinds transition orientation enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition recipe-oracle pass: model blinds as a visibility track, not a legacy mask chain.</WCTX>
// <CLOG>0.1.0: INIT — add horizontal/vertical blinds orientation vocabulary.</CLOG>

/// Orientation for `visibility.blinds` transition reveal bands.
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
pub enum TransitionBlindsOrientation {
    /// Bands open or close across horizontal rows.
    Horizontal,
    /// Bands open or close across vertical columns.
    Vertical,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_blinds_orientation.rs</FILE> - <DESC>Visibility blinds transition orientation enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
