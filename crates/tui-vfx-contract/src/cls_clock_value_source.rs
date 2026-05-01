// <FILE>crates/tui-vfx-contract/src/cls_clock_value_source.rs</FILE> - <DESC>Clock-derived numeric value source enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 recipe-oracle pass: persistent dwell effects need time without pretending to be transitions.</WCTX>
// <CLOG>0.1.0: INIT — add clock value source vocabulary.</CLOG>

/// Numeric clock source exposed to canonical `ValueSource::Clock`.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ClockValueSource {
    /// Absolute recipe sample time in seconds.
    RecipeSeconds,
    /// Current lifecycle phase sample time in seconds.
    PhaseSeconds,
    /// Current loop-local sample time in seconds when a loop exists.
    LoopSeconds,
}

// <FILE>crates/tui-vfx-contract/src/cls_clock_value_source.rs</FILE> - <DESC>Clock-derived numeric value source enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
