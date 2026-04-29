// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_report.rs</FILE> - <DESC>Build schema-readiness reports</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>K2.13 schema decision burn-down: derive declaration readiness from offender dispositions.</WCTX>
// <CLOG>0.4.0: MINOR — compute disposition counts and exact remaining owner decisions.
// 0.3.0: MINOR — include opt-in schema-readiness offender rows.</CLOG>

use std::{collections::BTreeMap, path::Path};

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerSchemaReadinessReport, build_migration_mapping_batch_report,
    build_schema_readiness_blockers, build_schema_readiness_families,
    build_schema_readiness_milestones, build_schema_readiness_offenders,
    schema_readiness_blocker_kind, summarize_schema_readiness,
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
    let offenders = build_schema_readiness_offenders(&mapping.records);
    let mut summary = summarize_schema_readiness(&mapping.records);
    apply_schema_decision_summary(&mut summary, &mapping.records, &offenders);
    Ok(PlayerSchemaReadinessReport {
        schema_version: "v3.1.player.schemaReadiness.1",
        legacy_root: mapping.legacy_root,
        v31_root: mapping.v31_root,
        descriptor_packs: mapping.descriptor_packs,
        summary: summary.clone(),
        families: build_schema_readiness_families(&mapping.records),
        blockers: build_schema_readiness_blockers(&mapping.records),
        offenders: if include_offenders {
            offenders
        } else {
            Vec::new()
        },
        readiness_milestones: build_schema_readiness_milestones(&summary),
        warnings: mapping.warnings,
        errors: mapping.errors,
    })
}

fn apply_schema_decision_summary(
    summary: &mut crate::PlayerSchemaReadinessSummary,
    records: &[crate::PlayerMigrationMappingRecord],
    offenders: &[crate::PlayerSchemaReadinessOffender],
) {
    let mut disposition_counts = BTreeMap::new();
    for record in records {
        if let Some(disposition) = non_offender_disposition(record) {
            increment_count(&mut disposition_counts, disposition);
        }
    }
    for offender in offenders {
        increment_count(&mut disposition_counts, &offender.disposition);
    }

    summary.unresolved_schema_blockers = offenders
        .iter()
        .filter(|offender| offender.schema_blocking)
        .count();
    summary.signed_off_holdbacks = offenders
        .iter()
        .filter(|offender| offender.holdback_signed_off)
        .count();
    summary.explicit_owner_decision_needed = offenders
        .iter()
        .filter(|offender| offender.disposition == "explicitOwnerDecisionNeeded")
        .count();
    summary.remaining_owner_decisions = offenders
        .iter()
        .filter(|offender| offender.disposition == "explicitOwnerDecisionNeeded")
        .map(|offender| crate::PlayerSchemaReadinessOwnerDecision {
            path: offender.legacy_path.clone(),
            family: offender.family.clone(),
            blocker_kind: offender.blocker_kind.clone(),
            exact_decision_required: offender.exact_decision_required.clone(),
        })
        .collect();
    summary.remaining_owner_decision_count = summary.remaining_owner_decisions.len();
    summary.disposition_counts = disposition_counts;
    summary.can_declare_schema_ready = summary.total_legacy_records > 0
        && summary.unresolved_schema_blockers == 0
        && summary.remaining_owner_decision_count == 0;
}

fn non_offender_disposition(record: &crate::PlayerMigrationMappingRecord) -> Option<&'static str> {
    match record.status.as_str() {
        "canonicalExists" | "candidateReady" => Some("acceptedSchema"),
        "duplicateOrVariant" => Some("duplicateVariant"),
        "ownerAuditNeeded" if schema_readiness_blocker_kind(record) == "oracleOnly" => {
            Some("oracleOnly")
        }
        _ => None,
    }
}

fn increment_count(counts: &mut BTreeMap<String, usize>, disposition: &str) {
    *counts.entry(disposition.to_string()).or_default() += 1;
}

// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_report.rs</FILE> - <DESC>Build schema-readiness reports</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
