// <FILE>crates/tui-vfx-contract/src/cls_transition_variant.rs</FILE> - <DESC>Generic transition variant selection DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: support reduced-motion and capability fallbacks without app semantics.</WCTX>
// <CLOG>0.1.0: INIT — add transition variant reference contract.</CLOG>

use crate::{TransitionId, TransitionVariantCondition};

/// Conditional transition replacement used for reduced motion, capability fallback, or host-selected variants.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionVariant {
    /// Engine-neutral condition that selects this variant.
    pub when: TransitionVariantCondition,
    /// Replacement transition id from the same recipe transition map.
    pub use_transition: TransitionId,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_variant.rs</FILE> - <DESC>Generic transition variant selection DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
