// <FILE>crates/tui-vfx-contract/src/cls_trigger_reset_boundary.rs</FILE> - <DESC>Lifecycle trigger reset boundary enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: make trigger reset semantics explicit.</WCTX>
// <CLOG>0.1.0: INIT — add reset boundary vocabulary.</CLOG>

/// Boundary that resets a lifecycle trigger's sampled/latch state.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum TriggerResetBoundary {
    /// Reset when entering the phase that owns the trigger.
    PhaseStart,
    /// Reset when the whole recipe lifecycle restarts.
    RecipeStart,
}

// <FILE>crates/tui-vfx-contract/src/cls_trigger_reset_boundary.rs</FILE> - <DESC>Lifecycle trigger reset boundary enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
