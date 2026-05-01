// <FILE>crates/tui-vfx-contract/src/cls_named_easing.rs</FILE> - <DESC>Named transition easing vocabulary</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>V3.1 transition timing: avoid magic easing strings in canonical schema.</WCTX>
// <CLOG>0.2.0: MINOR — add sine easing names used by current recipe corpus.
// 0.1.0: INIT — add named easing enum for transition timing.</CLOG>

/// Closed named easing vocabulary for canonical transition timing.
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
pub enum NamedEasing {
    /// Constant progress rate.
    Linear,
    /// Quadratic ease-in.
    InQuad,
    /// Quadratic ease-out.
    OutQuad,
    /// Quadratic ease-in-out.
    InOutQuad,
    /// Cubic ease-in.
    InCubic,
    /// Cubic ease-out.
    OutCubic,
    /// Cubic ease-in-out.
    InOutCubic,
    /// Sine ease-in.
    InSine,
    /// Sine ease-out.
    OutSine,
    /// Sine ease-in-out.
    InOutSine,
    /// Backtracking ease-in.
    InBack,
    /// Backtracking ease-out.
    OutBack,
    /// Backtracking ease-in-out.
    InOutBack,
}

// <FILE>crates/tui-vfx-contract/src/cls_named_easing.rs</FILE> - <DESC>Named transition easing vocabulary</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
