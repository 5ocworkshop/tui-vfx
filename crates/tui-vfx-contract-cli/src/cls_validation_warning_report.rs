// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_warning_report.rs</FILE> - <DESC>Structured validation warning report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J1: reserve warning channel in stable validation diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — add serializable warning report DTO for future non-fatal findings.</CLOG>

/// One structured non-fatal validation warning emitted by the contract CLI.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationWarningReport {
    /// Stable machine-facing diagnostic code.
    pub code: String,
    /// JSON or file-system path associated with the warning.
    pub path: String,
    /// Human-readable warning summary.
    pub message: String,
    /// Actionable remediation hint when known.
    pub hint: Option<String>,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_warning_report.rs</FILE> - <DESC>Structured validation warning report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
