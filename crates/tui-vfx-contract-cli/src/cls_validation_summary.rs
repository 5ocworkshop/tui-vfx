// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_summary.rs</FILE> - <DESC>Validation report summary DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J1: summarize recursive validator report results.</WCTX>
// <CLOG>0.1.0: INIT — add total/valid/invalid summary DTO.</CLOG>

/// Aggregate counts for one validation command invocation.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationSummary {
    /// Number of recipes checked.
    pub total: usize,
    /// Number of valid recipes.
    pub valid: usize,
    /// Number of invalid recipes.
    pub invalid: usize,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_summary.rs</FILE> - <DESC>Validation report summary DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
