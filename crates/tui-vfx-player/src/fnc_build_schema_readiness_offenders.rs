// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_offenders.rs</FILE> - <DESC>Build schema-readiness offender rows</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.13 schema decision burn-down: resolve offenders to final schema dispositions.</WCTX>
// <CLOG>0.2.0: MINOR — populate disposition, schemaBlocking, holdback signoff, and next-action fields.
// 0.1.0: INIT — add opt-in offender ledger rows for schema-readiness.</CLOG>

use crate::{
    PlayerMigrationMappingRecord, PlayerSchemaReadinessOffender,
    classify_complex_schema_offender_path, schema_readiness_blocker_kind,
    schema_readiness_disposition, schema_readiness_exact_decision_required,
    schema_readiness_holdback_reason, schema_readiness_next_packet,
    schema_readiness_recommended_disposition, schema_readiness_recommended_next_action,
};

pub(crate) fn build_schema_readiness_offenders(
    records: &[PlayerMigrationMappingRecord],
) -> Vec<PlayerSchemaReadinessOffender> {
    records
        .iter()
        .filter(|record| should_emit_offender(record))
        .map(build_offender)
        .collect()
}

fn should_emit_offender(record: &PlayerMigrationMappingRecord) -> bool {
    !matches!(record.status.as_str(), "canonicalExists" | "candidateReady")
        && schema_readiness_offender_kind(record) != "duplicateOrVariant"
        && !is_existing_oracle_only(record)
}

fn build_offender(record: &PlayerMigrationMappingRecord) -> PlayerSchemaReadinessOffender {
    let kind = schema_readiness_offender_kind(record).to_string();
    let disposition = schema_readiness_disposition(&kind).to_string();
    let schema_blocking = disposition == "explicitOwnerDecisionNeeded";
    let holdback_signed_off = is_signed_off_disposition(&disposition);
    PlayerSchemaReadinessOffender {
        legacy_path: record.legacy_path.clone(),
        family: record.legacy_family.clone(),
        legacy_recipe_name: record.legacy_recipe_name.clone(),
        current_status: record.status.clone(),
        blocker_kind: kind.clone(),
        schema_readiness_blocking: schema_blocking,
        disposition,
        schema_blocking,
        holdback_signed_off,
        exact_decision_required: schema_readiness_exact_decision_required(record, &kind),
        recommended_next_action: schema_readiness_recommended_next_action(&kind).to_string(),
        recommended_disposition: schema_readiness_recommended_disposition(&kind).to_string(),
        recommended_next_packet: schema_readiness_next_packet(&kind).to_string(),
        confidence: record.confidence.clone(),
        candidate_canonical_path: record.candidate_canonical_path.clone(),
        canonical_exists: record.canonical_exists,
        required_descriptor_ids: record.required_descriptor_ids.clone(),
        missing_descriptor_ids: record.missing_descriptor_ids.clone(),
        required_source_ids: record.required_source_ids.clone(),
        missing_source_ids: record.missing_source_ids.clone(),
        unsupported_input_fields: record.unsupported_input_fields.clone(),
        holdback_reason: schema_readiness_holdback_reason(record, &kind),
        notes: record.notes.clone(),
    }
}

pub(crate) fn schema_readiness_offender_kind(
    record: &PlayerMigrationMappingRecord,
) -> &'static str {
    if is_command_capture_artifact(record) || is_existing_oracle_only(record) {
        "oracleOnly"
    } else if record.legacy_family == "styles" && record.status == "notYetClassified" {
        "contentDescriptor"
    } else if record.legacy_family == "complex" && record.status == "ownerAuditNeeded" {
        classify_complex_record(record)
    } else {
        schema_readiness_blocker_kind(record)
    }
}

fn classify_complex_record(record: &PlayerMigrationMappingRecord) -> &'static str {
    if let Some(kind) = classify_complex_schema_offender_path(&record.legacy_path) {
        kind
    } else if has_any(record, &["signal", "source", "emitsHint", "binds"]) {
        "valueSourceSemantics"
    } else if has_non_card_source(record) {
        "sourceDescriptor"
    } else if !record.required_descriptor_ids.is_empty() {
        "descriptorPack"
    } else {
        "ownerPolicyHoldback"
    }
}

fn is_signed_off_disposition(disposition: &str) -> bool {
    matches!(
        disposition,
        "backendHoldback"
            | "descriptorBacklog"
            | "adapterBacklog"
            | "guiHumanReviewHoldback"
            | "oracleOnly"
            | "duplicateVariant"
    )
}

fn is_existing_oracle_only(record: &PlayerMigrationMappingRecord) -> bool {
    record.status == "ownerAuditNeeded" && schema_readiness_blocker_kind(record) == "oracleOnly"
}

fn is_command_capture_artifact(record: &PlayerMigrationMappingRecord) -> bool {
    record.legacy_path.contains("command_capture") || record.legacy_path.contains(".capture")
}

fn has_non_card_source(record: &PlayerMigrationMappingRecord) -> bool {
    record
        .required_source_ids
        .iter()
        .any(|source| source != "source.card")
}

fn has_any(record: &PlayerMigrationMappingRecord, fields: &[&str]) -> bool {
    record
        .unsupported_input_fields
        .iter()
        .any(|field| fields.iter().any(|candidate| candidate == field))
}

// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_offenders.rs</FILE> - <DESC>Build schema-readiness offender rows</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
