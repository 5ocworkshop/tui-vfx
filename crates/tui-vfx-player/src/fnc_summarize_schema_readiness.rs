// <FILE>crates/tui-vfx-player/src/fnc_summarize_schema_readiness.rs</FILE> - <DESC>Summarize schema-readiness counts</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: compute conservative readiness summary from migration mapping records.</WCTX>
// <CLOG>0.1.0: INIT — move summary counting out of report orchestration.</CLOG>

use crate::{
    PlayerMigrationMappingRecord, PlayerSchemaReadinessSummary, schema_readiness_blocker_kind,
};

pub(crate) fn summarize_schema_readiness(
    records: &[PlayerMigrationMappingRecord],
) -> PlayerSchemaReadinessSummary {
    let total = records.len();
    let schema_blocked = count_status(records, "schemaDecisionNeeded");
    let source_blocked = count_status(records, "sourceDecisionNeeded")
        + count_status(records, "blockedByUnsupportedSource");
    let descriptor_blocked = count_status(records, "descriptorDecisionNeeded")
        + count_status(records, "blockedByUnsupportedEffect");
    let adapter_blocked = count_status(records, "adapterDecisionNeeded");
    let field_blocked = count_status(records, "blockedByFieldCoverage");
    let owner_audit = count_status(records, "ownerAuditNeeded");
    let duplicate_or_variant = count_status(records, "duplicateOrVariant");
    let oracle_only = records
        .iter()
        .filter(|record| schema_readiness_blocker_kind(record) == "oracleOnly")
        .count();
    let unknown = count_status(records, "notYetClassified")
        + count_status(records, "blockedByAmbiguousLegacyIntent");
    let schema_ready = count_status(records, "canonicalExists")
        + count_status(records, "candidateReady")
        + duplicate_or_variant
        + oracle_only;
    let hard_blockers = schema_blocked + source_blocked + field_blocked + owner_audit + unknown;

    PlayerSchemaReadinessSummary {
        total_legacy_records: total,
        schema_ready_records: schema_ready,
        schema_blocked_records: schema_blocked,
        source_blocked_records: source_blocked,
        descriptor_blocked_records: descriptor_blocked,
        adapter_blocked_records: adapter_blocked,
        field_coverage_blocked_records: field_blocked,
        owner_audit_records: owner_audit,
        oracle_only_records: oracle_only,
        duplicate_or_variant_records: duplicate_or_variant,
        unknown_records: unknown,
        estimated_schema_readiness_percent: readiness_percent(schema_ready, total),
        can_declare_schema_ready: total > 0 && hard_blockers == 0,
    }
}

fn count_status(records: &[PlayerMigrationMappingRecord], status: &str) -> usize {
    records
        .iter()
        .filter(|record| record.status == status)
        .count()
}

fn readiness_percent(schema_ready: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        ((schema_ready as f64 / total as f64) * 1000.0).round() / 10.0
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_schema_readiness.rs</FILE> - <DESC>Summarize schema-readiness counts</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
