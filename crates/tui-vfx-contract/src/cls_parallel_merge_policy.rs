// <FILE>crates/tui-vfx-contract/src/cls_parallel_merge_policy.rs</FILE> - <DESC>Canonical graph parallel merge policy DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: define deterministic channel-aware parallel merge policies.</WCTX>
// <CLOG>0.1.0: INIT — add child-order and error-on-conflict merge policy vocabulary.</CLOG>

/// Policy for resolving channel writes produced by parallel graph branches.
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
pub enum ParallelMergePolicy {
    /// Merge branches in authored child order; later same-channel writes win.
    #[default]
    ChildOrderLastWriterWins,
    /// Reject the merge when multiple branches write the same cell channel.
    ErrorOnSameChannelConflict,
}

// <FILE>crates/tui-vfx-contract/src/cls_parallel_merge_policy.rs</FILE> - <DESC>Canonical graph parallel merge policy DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
