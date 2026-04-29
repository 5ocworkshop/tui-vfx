// <FILE>crates/tui-vfx-contract/src/cls_clock_mode.rs</FILE> - <DESC>Lifecycle clock mode enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase I0: distinguish monotonic and looping recipe clock spaces.</WCTX>
// <CLOG>0.1.0: INIT — add clock mode vocabulary.</CLOG>

/// Time sample-space mode for recipe-level lifecycle evaluation.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ClockMode {
    /// Time advances monotonically for one recipe lifecycle pass.
    Monotonic,
    /// Time wraps by an explicit period for preview/repeating lifecycle semantics.
    Looping,
}

// <FILE>crates/tui-vfx-contract/src/cls_clock_mode.rs</FILE> - <DESC>Lifecycle clock mode enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
