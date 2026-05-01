// <FILE>crates/tui-vfx-contract/src/cls_transition_motion_sampling.rs</FILE> - <DESC>Grid motion sampling policy enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition recipe-oracle pass: preserve grid-native motion quantization semantics.</WCTX>
// <CLOG>0.1.0: INIT — add motion path sampling policies.</CLOG>

/// Policy for sampling continuous motion paths onto a cell grid.
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
pub enum TransitionMotionSampling {
    /// Preserve continuous coordinates for consumers that support them.
    None,
    /// Round coordinates to the nearest cell.
    RoundToCell,
    /// Floor coordinates to the lower cell.
    FloorToCell,
    /// Ceil coordinates to the higher cell.
    CeilToCell,
    /// Dither between neighboring cells over time.
    TemporalDither,
    /// Preserve sub-cell motion using braille cell density where supported.
    BrailleSubcell,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_motion_sampling.rs</FILE> - <DESC>Grid motion sampling policy enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
