// <FILE>crates/tui-vfx-contract/src/cls_transition_motion_path.rs</FILE> - <DESC>Transition motion path DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition recipe-oracle pass: motion paths are transition tracks when they express state-change movement.</WCTX>
// <CLOG>0.1.0: INIT — add canonical motion path variants for transition tracks.</CLOG>

use crate::ValueSource;

/// Grid-native path followed by a `motion.path` transition track.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TransitionMotionPath {
    /// Straight-line path between the transition's resolved start and end placements.
    Linear,
    /// Arc path with a signed bulge value; positive and negative values bend opposite directions.
    Arc {
        /// Signed arc bulge amount.
        bulge: ValueSource,
    },
    /// Spring-like path controlled by transition timing/easing and optional amplitude.
    Spring {
        /// Optional overshoot/amplitude source.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amplitude: Option<ValueSource>,
    },
    /// Bezier-like path reserved for authored control-point motion.
    Bezier {
        /// Control points or descriptor-owned payload encoded as a structured value source.
        control_points: ValueSource,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_motion_path.rs</FILE> - <DESC>Transition motion path DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
