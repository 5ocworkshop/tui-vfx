// <FILE>crates/tui-vfx-contract/src/cls_transition_visibility_geometry.rs</FILE> - <DESC>Structured visibility transition geometry DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 transition recipe-oracle pass: avoid long opaque wipe direction strings for corner geometry.</WCTX>
// <CLOG>0.1.0: INIT — add structured corner-arc visibility geometry.</CLOG>

use crate::SceneAnchor;

/// Structured geometry for visibility transition tracks.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TransitionVisibilityGeometry {
    /// Corner arc reveal/hide geometry.
    CornerArc {
        /// Corner arc reveal/hide mode.
        corner_arc_mode: TransitionCornerArcMode,
        /// Corner used as the reveal destination or origin.
        corner: SceneAnchor,
        /// Distance metric used to calculate corner coverage.
        metric: TransitionDistanceMetric,
    },
}

/// Directional mode for corner-arc visibility geometry.
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
pub enum TransitionCornerArcMode {
    /// Reveal converges inward toward the selected corner.
    InToCorner,
    /// Reveal expands outward from the selected corner.
    OutFromCorner,
}

/// Distance metric used by structured visibility geometry.
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
pub enum TransitionDistanceMetric {
    /// Euclidean distance.
    Euclidean,
    /// Manhattan grid distance.
    Manhattan,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_visibility_geometry.rs</FILE> - <DESC>Structured visibility transition geometry DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
