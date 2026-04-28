// <FILE>crates/tui-vfx-contract/src/cls_runtime_mutability.rs</FILE> - <DESC>Effect input runtime mutability vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase F1: describe how declared inputs may change over effect lifetime.</WCTX>
// <CLOG>0.1.0: INIT — add compile-time, phase-start, reset-only, and runtime mutability tags.</CLOG>

/// Vocabulary describing when an effect input may change.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum RuntimeMutability {
    /// Value is fixed when the effect contract is compiled or instantiated.
    CompileTime,
    /// Value may be set at the start of a phase.
    PhaseStart,
    /// Value may change only when the effect is reset.
    ResetOnly,
    /// Value may change while the effect is running.
    Runtime,
}

// <FILE>crates/tui-vfx-contract/src/cls_runtime_mutability.rs</FILE> - <DESC>Effect input runtime mutability vocabulary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
