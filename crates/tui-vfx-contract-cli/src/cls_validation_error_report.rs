// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_error_report.rs</FILE> - <DESC>Structured validation error report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: report deserialize and contract-validation failures.</WCTX>
// <CLOG>0.1.0: INIT — add serializable CLI error report.</CLOG>

/// One structured validation failure emitted by the contract CLI.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationErrorReport {
    /// Validation stage that produced the error.
    pub stage: &'static str,
    /// Human-readable error summary.
    pub message: String,
    /// Structured details when available.
    pub details: serde_json::Value,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_error_report.rs</FILE> - <DESC>Structured validation error report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
