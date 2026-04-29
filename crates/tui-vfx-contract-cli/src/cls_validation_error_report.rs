// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_error_report.rs</FILE> - <DESC>Structured validation error report DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J1: add stable diagnostic code/path/hint fields for migration smoke validation.</WCTX>
// <CLOG>0.2.0: MINOR — replace stage-only failures with code/path/message/hint diagnostics.
// 0.1.0: INIT — add serializable CLI error report.</CLOG>

/// One structured validation failure emitted by the contract CLI.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationErrorReport {
    /// Stable machine-facing diagnostic code.
    pub code: String,
    /// JSON or file-system path associated with the failure.
    pub path: String,
    /// Human-readable error summary.
    pub message: String,
    /// Actionable remediation hint when known.
    pub hint: Option<String>,
    /// Structured details when available.
    pub details: serde_json::Value,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_error_report.rs</FILE> - <DESC>Structured validation error report DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
