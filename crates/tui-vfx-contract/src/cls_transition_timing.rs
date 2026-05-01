// <FILE>crates/tui-vfx-contract/src/cls_transition_timing.rs</FILE> - <DESC>Canonical transition timing DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: share timing between transition envelopes and tracks.</WCTX>
// <CLOG>0.1.0: INIT — add reusable timing contract.</CLOG>

use crate::{DurationSpec, EasingSpec, StructuredValue};

/// Reusable timing block for transition envelopes and per-track overrides.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionTiming {
    /// Optional duration; track timing inherits from the transition when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<DurationSpec>,
    /// Optional delay before the interval starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<DurationSpec>,
    /// Optional easing; track timing inherits from the transition when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub easing: Option<EasingSpec>,
    /// Optional per-cell, per-role, or author-defined stagger description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stagger: Option<StructuredValue>,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_timing.rs</FILE> - <DESC>Canonical transition timing DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
