// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_report.rs</FILE> - <DESC>Build schema-readiness reports</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>K2.12 schema lock: optionally project offender-level readiness rows.</WCTX>
// <CLOG>0.3.0: MINOR — include opt-in schema-readiness offender rows.
// 0.2.0: REFACTOR — delegate summary, family, blocker, and milestone assembly.
// 0.1.0: INIT — build schema-readiness report from the conservative migration mapping batch.</CLOG>

use std::path::Path;

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerSchemaReadinessReport, build_migration_mapping_batch_report,
    build_schema_readiness_blockers, build_schema_readiness_families,
    build_schema_readiness_milestones, build_schema_readiness_offenders,
    summarize_schema_readiness,
};

/// Build a schema-readiness blocker ledger from legacy and v3.1 roots.
pub fn build_schema_readiness_report(
    legacy_root: &Path,
    v31_root: &Path,
    descriptor_packs: Vec<DescriptorPackReport>,
    catalog: &DescriptorCatalog,
    family: Option<&str>,
    recursive: bool,
    include_offenders: bool,
) -> Result<PlayerSchemaReadinessReport, String> {
    let mapping = build_migration_mapping_batch_report(
        legacy_root,
        v31_root,
        descriptor_packs,
        catalog,
        family,
        recursive,
    )?;
    let summary = summarize_schema_readiness(&mapping.records);
    Ok(PlayerSchemaReadinessReport {
        schema_version: "v3.1.player.schemaReadiness.1",
        legacy_root: mapping.legacy_root,
        v31_root: mapping.v31_root,
        descriptor_packs: mapping.descriptor_packs,
        summary: summary.clone(),
        families: build_schema_readiness_families(&mapping.records),
        blockers: build_schema_readiness_blockers(&mapping.records),
        offenders: if include_offenders {
            build_schema_readiness_offenders(&mapping.records)
        } else {
            Vec::new()
        },
        readiness_milestones: build_schema_readiness_milestones(&summary),
        warnings: mapping.warnings,
        errors: mapping.errors,
    })
}

// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_report.rs</FILE> - <DESC>Build schema-readiness reports</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
