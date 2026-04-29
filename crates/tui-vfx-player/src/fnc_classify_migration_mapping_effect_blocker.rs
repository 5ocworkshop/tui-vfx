// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_effect_blocker.rs</FILE> - <DESC>Classify direct blockers for effect-backed migration mapping records</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: keep direct source/field blockers out of candidate-ready routing.</WCTX>
// <CLOG>0.1.0: INIT — extract direct effect blocker classification.</CLOG>

use crate::{
    MigrationMappingRecordClassification,
    fnc_build_migration_mapping_record_classification::build_migration_mapping_record_classification,
};

pub(crate) fn classify_migration_mapping_effect_blocker(
    input_fields: &[String],
    missing_source_ids: &[String],
    unsupported_by_descriptor: &[String],
) -> Option<MigrationMappingRecordClassification> {
    if !missing_source_ids.is_empty() {
        return Some(build_migration_mapping_record_classification(
            "sourceDecisionNeeded",
            "addSourceDescriptor",
            input_fields,
            missing_source_ids,
            "high",
            "source vocabulary is not accepted in this batch",
        ));
    }

    if !unsupported_by_descriptor.is_empty() {
        return Some(build_migration_mapping_record_classification(
            "blockedByFieldCoverage",
            "addFieldHandling",
            unsupported_by_descriptor,
            unsupported_by_descriptor,
            "medium",
            "legacy input fields need descriptor/player handling before migration",
        ));
    }

    None
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_effect_blocker.rs</FILE> - <DESC>Classify direct blockers for effect-backed migration mapping records</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
