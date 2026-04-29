// <FILE>crates/tui-vfx-contract/src/cls_trigger_action.rs</FILE> - <DESC>Lifecycle trigger action enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: distinguish trigger condition from lifecycle action.</WCTX>
// <CLOG>0.1.0: INIT — add minimal transition action vocabulary.</CLOG>

/// Lifecycle action requested when a trigger fires.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum TriggerAction {
    /// Advance from the current phase to the next lifecycle phase.
    AdvancePhase,
    /// Finish the recipe lifecycle immediately.
    FinishRecipe,
}

// <FILE>crates/tui-vfx-contract/src/cls_trigger_action.rs</FILE> - <DESC>Lifecycle trigger action enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
