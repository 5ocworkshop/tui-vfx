// <FILE>crates/tui-vfx-contract/src/cls_reduced_motion_kind.rs</FILE> - <DESC>Reduced-motion policy kind enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: name reduced-motion policies without ambiguous mode fields.</WCTX>
// <CLOG>0.1.0: INIT — add reduced-motion policy kind vocabulary.</CLOG>

/// Accessibility policy kind for reducing or substituting motion-heavy transitions.
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
pub enum ReducedMotionKind {
    /// Do not alter this transition for reduced-motion contexts.
    None,
    /// Replace the transition with an instant state change.
    Instant,
    /// Preserve only opacity/fade tracks when possible.
    KeepOpacityOnly,
    /// Substitute another named transition from the same recipe.
    Substitute,
}

// <FILE>crates/tui-vfx-contract/src/cls_reduced_motion_kind.rs</FILE> - <DESC>Reduced-motion policy kind enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
