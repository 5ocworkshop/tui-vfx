// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_report.rs</FILE> - <DESC>Structured recipe validation report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: report per-file canonical RecipeDocument validation outcomes.</WCTX>
// <CLOG>0.1.0: INIT — add serializable CLI report.</CLOG>

use crate::cls_validation_error_report::ValidationErrorReport;

/// Validation result for one canonical v3.1 recipe file.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationReport {
    /// Recipe file path that was checked.
    pub path: String,
    /// Whether the file deserialized and passed contract validation.
    pub valid: bool,
    /// Validation errors. Empty when valid is true.
    pub errors: Vec<ValidationErrorReport>,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_report.rs</FILE> - <DESC>Structured recipe validation report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
