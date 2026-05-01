// <FILE>crates/tui-vfx-contract/src/cls_transition_travel_direction.rs</FILE> - <DESC>Qualified transition travel direction enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 field normalization: separate motion travel from wipe reveal direction.</WCTX>
// <CLOG>0.1.0: INIT — add travelDirection vocabulary.</CLOG>

/// Movement direction for motion and relation tracks.
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
pub enum TransitionTravelDirection {
    /// Travel left.
    Left,
    /// Travel right.
    Right,
    /// Travel upward.
    Up,
    /// Travel downward.
    Down,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_travel_direction.rs</FILE> - <DESC>Qualified transition travel direction enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
