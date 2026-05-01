// <FILE>crates/tui-vfx-contract/src/cls_transition_focal.rs</FILE> - <DESC>Transition focal point DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition tracks: represent focal coordinates structurally.</WCTX>
// <CLOG>0.1.0: INIT — add focal coordinate value sources.</CLOG>

use crate::ValueSource;

/// Focal point used by aperture-style visibility tracks.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionFocal {
    /// Focal x coordinate source.
    pub x: ValueSource,
    /// Focal y coordinate source.
    pub y: ValueSource,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_focal.rs</FILE> - <DESC>Transition focal point DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
