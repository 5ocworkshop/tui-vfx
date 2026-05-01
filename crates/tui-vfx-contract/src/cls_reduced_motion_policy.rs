// <FILE>crates/tui-vfx-contract/src/cls_reduced_motion_policy.rs</FILE> - <DESC>Reduced-motion transition policy DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: make reduced-motion behavior explicit.</WCTX>
// <CLOG>0.1.0: INIT — add reduced-motion policy contract.</CLOG>

use crate::{ReducedMotionKind, TransitionId};

/// Accessibility policy for reducing or substituting motion-heavy transitions.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReducedMotionPolicy {
    /// Selected reduced-motion policy.
    pub policy: ReducedMotionKind,
    /// Optional replacement transition id used when `policy` is `substitute`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition: Option<TransitionId>,
}

// <FILE>crates/tui-vfx-contract/src/cls_reduced_motion_policy.rs</FILE> - <DESC>Reduced-motion transition policy DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
