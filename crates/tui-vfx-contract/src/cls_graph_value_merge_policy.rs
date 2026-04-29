// <FILE>crates/tui-vfx-contract/src/cls_graph_value_merge_policy.rs</FILE> - <DESC>Parallel graph value merge policy DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: make parallel value-bus conflicts explicit.</WCTX>
// <CLOG>0.1.0: INIT — add child-order and error-on-conflict value merge policies.</CLOG>

/// Policy for resolving graph value outputs produced by parallel branches.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum GraphValueMergePolicy {
    /// Merge branch value outputs in authored child order; later outputs win.
    #[default]
    ChildOrderLastWriterWins,
    /// Reject the merge when multiple branches publish the same graph value id.
    ErrorOnSameValueConflict,
}

// <FILE>crates/tui-vfx-contract/src/cls_graph_value_merge_policy.rs</FILE> - <DESC>Parallel graph value merge policy DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
