// <FILE>crates/tui-vfx-contract/src/cls_transition_variant_condition.rs</FILE> - <DESC>Generic transition variant selection condition DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: keep variant selection engine-neutral and signal/capability based.</WCTX>
// <CLOG>0.1.0: INIT — add generic transition variant condition vocabulary.</CLOG>

use crate::{ParameterId, SignalId};

/// Engine-neutral condition that may select an alternate transition variant.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TransitionVariantCondition {
    /// Host/runtime signal is truthy according to the loader or player policy.
    Signal {
        /// Referenced signal id.
        id: SignalId,
    },
    /// Public parameter is truthy according to the loader or player policy.
    Parameter {
        /// Referenced parameter id.
        id: ParameterId,
    },
    /// Host/runtime requests reduced motion.
    ReducedMotionRequested,
    /// A named grid/backend capability is unavailable.
    CapabilityUnavailable {
        /// Stable capability name, such as `glyphSet.braille` or `color.truecolor`.
        capability: String,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_variant_condition.rs</FILE> - <DESC>Generic transition variant selection condition DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
