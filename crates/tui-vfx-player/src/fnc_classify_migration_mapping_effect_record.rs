// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_effect_record.rs</FILE> - <DESC>Classify effect-backed migration mapping records</DESC>
// <VERS>VERSION: 0.2.3</VERS>
// <WCTX>K2.10 corpus mapping: keep effect-backed migration records conservative.</WCTX>
// <CLOG>0.2.3: PATCH — add current work context metadata.
// 0.2.2: PATCH — route direct source/field blockers through a helper.</CLOG>

use crate::{
    MigrationMappingRecordClassification,
    fnc_build_migration_mapping_record_classification::{
        build_migration_mapping_record_classification, migration_mapping_blocker,
    },
    fnc_classify_migration_mapping_effect_blocker::classify_migration_mapping_effect_blocker,
};

pub(crate) fn classify_migration_mapping_effect_record(
    family: &str,
    descriptor_ids: &[String],
    input_fields: &[String],
    missing_descriptor_ids: &[String],
    missing_source_ids: &[String],
    unsupported_by_descriptor: &[String],
    value_source_decision_fields: &[String],
) -> MigrationMappingRecordClassification {
    if descriptor_ids.is_empty() {
        return with_blocker(
            "notYetClassified",
            "deferForOwnerAudit",
            &[],
            "no effect payload was found",
            "low",
            "no effect payload was found",
        );
    }
    if !missing_descriptor_ids.is_empty() {
        return build_migration_mapping_record_classification(
            "descriptorDecisionNeeded",
            "deferForDescriptorDecision",
            input_fields,
            missing_descriptor_ids,
            "high",
            "effect vocabulary is not accepted in this batch",
        );
    }
    if let Some(classification) = classify_migration_mapping_effect_blocker(
        input_fields,
        missing_source_ids,
        unsupported_by_descriptor,
    ) {
        return classification;
    }
    if !candidate_ready_family(family) {
        return with_blocker(
            "notYetClassified",
            "deferForOwnerAudit",
            input_fields,
            "unknown family requires owner audit",
            "medium",
            "descriptor-shaped records outside candidate-ready families remain owner gated",
        );
    }
    build_migration_mapping_record_classification(
        "candidateReady",
        "createCanonicalFixture",
        &[],
        &accepted_runtime_blockers(value_source_decision_fields),
        "medium",
        candidate_ready_note(value_source_decision_fields),
    )
}

fn accepted_runtime_blockers(value_source_decision_fields: &[String]) -> Vec<String> {
    if value_source_decision_fields.is_empty() {
        vec![]
    } else {
        migration_mapping_blocker("valueSourceOrSignalAccepted")
    }
}

fn candidate_ready_note(value_source_decision_fields: &[String]) -> &'static str {
    if value_source_decision_fields.is_empty() {
        "descriptor-backed legacy recipe appears ready for bounded fixture authoring"
    } else {
        "legacy dynamic input values map to accepted ValueSource signal, parameter, map, graph-value, or literal semantics"
    }
}

fn candidate_ready_family(family: &str) -> bool {
    matches!(
        family,
        "content" | "filters" | "masks" | "samplers" | "shaders" | "styles"
    )
}
fn with_blocker(
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

// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_effect_record.rs</FILE> - <DESC>Classify effect-backed migration mapping records</DESC>
// <VERS>END OF VERSION: 0.2.3</VERS>
