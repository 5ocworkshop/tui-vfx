// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_report.rs</FILE> - <DESC>Structured recipe validation report DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J1: expose status and warnings while preserving valid bool for smoke tests.</WCTX>
// <CLOG>0.2.0: MINOR — add status and warnings fields to validation reports.
// 0.1.0: INIT — add serializable CLI report.</CLOG>

use crate::{
    cls_validation_error_report::ValidationErrorReport,
    cls_validation_warning_report::ValidationWarningReport,
};

/// Validation result for one canonical v3.1 recipe file.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationReport {
    /// Recipe file path that was checked.
    pub path: String,
    /// Stable status string: `ok` or `error`.
    pub status: &'static str,
    /// Whether the file deserialized and passed contract validation.
    pub valid: bool,
    /// Validation errors. Empty when valid is true.
    pub errors: Vec<ValidationErrorReport>,
    /// Non-fatal validation warnings. Empty for J1.
    pub warnings: Vec<ValidationWarningReport>,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_report.rs</FILE> - <DESC>Structured recipe validation report DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
