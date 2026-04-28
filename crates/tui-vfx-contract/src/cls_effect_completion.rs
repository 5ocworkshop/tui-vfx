// <FILE>crates/tui-vfx-contract/src/cls_effect_completion.rs</FILE> - <DESC>Effect completion category enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase E1: add minimal descriptor lifecycle vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — add completion categories without full runtime event semantics.</CLOG>

/// Minimal completion category for descriptor lifecycle metadata.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum EffectCompletion {
    /// The effect does not naturally complete on its own.
    Never,
    /// The effect completes during the same evaluation step.
    Instant,
    /// The effect completes after a bounded duration.
    TimeBound,
    /// The effect eventually completes, but the bound is not part of E1.
    Eventual,
    /// Completion is controlled by an external host signal.
    External,
}

// <FILE>crates/tui-vfx-contract/src/cls_effect_completion.rs</FILE> - <DESC>Effect completion category enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
