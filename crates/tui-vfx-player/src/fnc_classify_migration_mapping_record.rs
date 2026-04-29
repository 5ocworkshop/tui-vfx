// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_record.rs</FILE> - <DESC>Classify one legacy migration mapping record</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>K2.10 corpus mapping: keep the top-level classifier within OFPF limits.</WCTX>
// <CLOG>0.4.0: MINOR — route nested value-source evidence into effect classification.</CLOG>

use crate::{
    MigrationMappingRecordClassification,
    fnc_build_migration_mapping_record_classification::{
        build_migration_mapping_record_classification, migration_mapping_blocker,
    },
    fnc_classify_migration_mapping_effect_record::classify_migration_mapping_effect_record,
    fnc_classify_migration_mapping_record_family::classify_migration_mapping_record_family,
};

/// Classify one legacy recipe record for migration planning.
pub(crate) fn classify_migration_mapping_record(
    family: &str,
    legacy_path: &str,
    canonical_exists: bool,
    descriptor_ids: &[String],
    input_fields: &[String],
    missing_descriptor_ids: &[String],
    missing_source_ids: &[String],
    unsupported_by_descriptor: &[String],
    value_source_decision_fields: &[String],
) -> MigrationMappingRecordClassification {
    if legacy_path.contains("_DEPRECATED_") {
        return classify_with_blocker(
            "ownerAuditNeeded",
            "useAsOracleOnly",
            input_fields,
            "deprecated legacy recipe",
            "medium",
            "deprecated legacy fixture retained as oracle evidence only",
        );
    }
    if canonical_exists {
        return build_migration_mapping_record_classification(
            "canonicalExists",
            "skipAsDuplicateVariant",
            &[],
            &[],
            "high",
            "canonical fixture already exists",
        );
    }
    if legacy_path.contains("_square") {
        return classify_with_blocker(
            "duplicateOrVariant",
            "skipAsDuplicateVariant",
            input_fields,
            "variant fixture",
            "high",
            "geometry-clarity variant of a simple mask candidate",
        );
    }
    if let Some(result) = classify_migration_mapping_record_family(family, input_fields) {
        return result;
    }
    classify_migration_mapping_effect_record(
        family,
        descriptor_ids,
        input_fields,
        missing_descriptor_ids,
        missing_source_ids,
        unsupported_by_descriptor,
        value_source_decision_fields,
    )
}

fn classify_with_blocker(
    status: &str,
    recommendation: &str,
    unsupported_input_fields: &[String],
    blocker: &str,
    confidence: &str,
    note: &str,
) -> MigrationMappingRecordClassification {
    build_migration_mapping_record_classification(
        status,
        recommendation,
        unsupported_input_fields,
        &migration_mapping_blocker(blocker),
        confidence,
        note,
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_record.rs</FILE> - <DESC>Classify one legacy migration mapping record</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
