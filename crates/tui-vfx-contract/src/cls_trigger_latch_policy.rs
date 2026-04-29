// <FILE>crates/tui-vfx-contract/src/cls_trigger_latch_policy.rs</FILE> - <DESC>Lifecycle trigger latch policy enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: make trigger latch semantics explicit.</WCTX>
// <CLOG>0.1.0: INIT — add latch policy vocabulary.</CLOG>

/// Whether a lifecycle trigger remains fired after its condition first passes.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum TriggerLatchPolicy {
    /// Trigger is sampled each time and does not latch.
    None,
    /// Trigger remains fired until the current lifecycle phase resets.
    UntilPhaseReset,
    /// Trigger remains fired until the whole recipe lifecycle resets.
    UntilRecipeReset,
}

// <FILE>crates/tui-vfx-contract/src/cls_trigger_latch_policy.rs</FILE> - <DESC>Lifecycle trigger latch policy enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
