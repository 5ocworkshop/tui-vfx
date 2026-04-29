// <FILE>crates/tui-vfx-contract-cli/src/fnc_build_run_report.rs</FILE> - <DESC>Build top-level validation run report</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J1: aggregate per-recipe reports into stable JSON output.</WCTX>
// <CLOG>0.1.0: INIT — add report builder with summary counts.</CLOG>

use crate::{
    cls_validation_report::ValidationReport, cls_validation_run_report::ValidationRunReport,
    cls_validation_summary::ValidationSummary,
};

/// Build one validation run report from per-recipe reports.
pub fn build_run_report(root: String, recipes: Vec<ValidationReport>) -> ValidationRunReport {
    let valid = recipes.iter().filter(|recipe| recipe.valid).count();
    let total = recipes.len();
    ValidationRunReport {
        schema_version: "v3.1.validator.report.1",
        root,
        summary: ValidationSummary {
            total,
            valid,
            invalid: total - valid,
        },
        recipes,
    }
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_build_run_report.rs</FILE> - <DESC>Build top-level validation run report</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
