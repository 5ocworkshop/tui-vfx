// <FILE>crates/tui-vfx-contract/src/cls_transition_materialize_pattern.rs</FILE> - <DESC>Visibility materialize pattern enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 transition recipe-oracle pass: materialize masks are visibility reveal tracks.</WCTX>
// <CLOG>0.1.0: INIT — add materialize reveal pattern vocabulary.</CLOG>

/// Pattern used by `visibility.materialize` reveal tracks.
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
pub enum TransitionMaterializePattern {
    /// Seeded noise/chunk reveal.
    Noise,
    /// Ordered chunk reveal.
    Chunks,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_materialize_pattern.rs</FILE> - <DESC>Visibility materialize pattern enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
