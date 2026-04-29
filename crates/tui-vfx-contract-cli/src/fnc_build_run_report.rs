// <FILE>crates/tui-vfx-contract-cli/src/fnc_build_run_report.rs</FILE> - <DESC>Build top-level validation run report</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J2: include loaded descriptor packs in run reports.</WCTX>
// <CLOG>0.2.0: MINOR — add descriptor pack report entries to validation output.
// 0.1.0: INIT — add report builder with summary counts.</CLOG>

use crate::{
    cls_descriptor_pack_report::DescriptorPackReport, cls_validation_report::ValidationReport,
    cls_validation_run_report::ValidationRunReport, cls_validation_summary::ValidationSummary,
};

/// Build one validation run report from per-recipe reports.
pub fn build_run_report(
    root: String,
    descriptor_packs: Vec<DescriptorPackReport>,
    recipes: Vec<ValidationReport>,
) -> ValidationRunReport {
    let valid = recipes.iter().filter(|recipe| recipe.valid).count();
    let total = recipes.len();
    ValidationRunReport {
        schema_version: "v3.1.validator.report.1",
        root,
        descriptor_packs,
        summary: ValidationSummary {
            total,
            valid,
            invalid: total - valid,
        },
        recipes,
    }
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_build_run_report.rs</FILE> - <DESC>Build top-level validation run report</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
