// <FILE>crates/tui-vfx-player/src/cls_player_summary.rs</FILE> - <DESC>Recursive player run summary DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: summarize recursive smoke render outcomes.</WCTX>
// <CLOG>0.1.0: INIT — add rendered/unsupported/error aggregate counts.</CLOG>

/// Aggregate counts for one player CLI invocation.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSummary {
    /// Number of recipe files sampled.
    pub total: usize,
    /// Number of recipe files that rendered within the K0 supported subset.
    pub rendered: usize,
    /// Number of valid recipe files requiring unsupported adapters.
    pub unsupported: usize,
    /// Number of recipe files with hard load/validation errors.
    pub errors: usize,
}

// <FILE>crates/tui-vfx-player/src/cls_player_summary.rs</FILE> - <DESC>Recursive player run summary DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
