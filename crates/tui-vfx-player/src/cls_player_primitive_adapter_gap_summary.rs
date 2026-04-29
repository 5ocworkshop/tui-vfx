// <FILE>crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_summary.rs</FILE> - <DESC>Primitive adapter gap summary DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: summarize support outcomes.</WCTX>
// <CLOG>0.1.0: INIT — add adapter gap aggregate counts.</CLOG>

/// Aggregate counts for one primitive adapter gap invocation.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveAdapterGapSummary {
    /// Number of represented effect ids classified.
    pub total_effects: usize,
    /// Number of represented effect ids that produce honest player evidence.
    pub rendered: usize,
    /// Number of represented effect ids still lacking support without a sharper blocker.
    pub still_unsupported: usize,
    /// Number of represented effect ids blocked by missing styled-cell evidence.
    pub blocked_by_styled_cell_substrate: usize,
    /// Number of represented effect ids blocked by semantic or descriptor decisions.
    pub blocked_by_semantic_decision: usize,
    /// Number of represented effect ids missing descriptor coverage.
    pub missing_descriptor: usize,
}

// <FILE>crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_summary.rs</FILE> - <DESC>Primitive adapter gap summary DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
