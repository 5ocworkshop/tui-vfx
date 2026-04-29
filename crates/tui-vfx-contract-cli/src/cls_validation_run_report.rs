// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_run_report.rs</FILE> - <DESC>Top-level validation run report DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J2: include descriptor-pack context in validator reports.</WCTX>
// <CLOG>0.2.0: MINOR — add loaded descriptor pack report entries.
// 0.1.0: INIT — add schema version, root, summary, and recipe reports.</CLOG>

use crate::{
    cls_descriptor_pack_report::DescriptorPackReport, cls_validation_report::ValidationReport,
    cls_validation_summary::ValidationSummary,
};

/// Stable JSON output for one validate-recipe invocation.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationRunReport {
    /// Validator report schema version.
    pub schema_version: &'static str,
    /// Common root path or invocation label for the validated set.
    pub root: String,
    /// Descriptor packs loaded for this validation run.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate validation counts.
    pub summary: ValidationSummary,
    /// Per-recipe validation reports.
    pub recipes: Vec<ValidationReport>,
}

// <FILE>crates/tui-vfx-contract-cli/src/cls_validation_run_report.rs</FILE> - <DESC>Top-level validation run report DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
