// <FILE>crates/tui-vfx-contract/src/cls_cell_write_policy.rs</FILE> - <DESC>Cell channel write policy enum</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract CellWritePolicy enum.</CLOG>

/// Policy for how a cell write updates cell channels.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum CellWritePolicy {
    /// Write the cell exactly, including transparent empty cells.
    WriteCell,
    /// Skip only transparent empty cell writes, preserving destination cell and role.
    SkipTransparentEmpty,
}

// <FILE>crates/tui-vfx-contract/src/cls_cell_write_policy.rs</FILE> - <DESC>Cell channel write policy enum</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
