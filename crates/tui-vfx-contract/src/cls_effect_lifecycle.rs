// <FILE>crates/tui-vfx-contract/src/cls_effect_lifecycle.rs</FILE> - <DESC>Minimal effect lifecycle descriptor DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: add lightweight lifecycle metadata for descriptors.</WCTX>
// <CLOG>0.1.0: INIT — add completion/reset/seek/determinism lifecycle fields.</CLOG>

use crate::EffectCompletion;

/// Minimal lifecycle metadata declared by an effect descriptor.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EffectLifecycle {
    /// Completion category exposed to recipe and runtime planning.
    pub completion: EffectCompletion,
    /// Whether the effect can be reset to its initial state.
    pub resettable: bool,
    /// Whether the effect can be sampled at an arbitrary timeline position.
    pub seekable: bool,
    /// Whether seeded execution is deterministic for the same inputs.
    pub deterministic_with_seed: bool,
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_lifecycle.rs</FILE> - <DESC>Minimal effect lifecycle descriptor DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
