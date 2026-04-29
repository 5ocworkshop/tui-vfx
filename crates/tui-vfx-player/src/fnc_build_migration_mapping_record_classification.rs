// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record_classification.rs</FILE> - <DESC>Build migration mapping record classifications</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: share classification construction across classifier helpers.</WCTX>
// <CLOG>0.1.0: INIT — add focused classification constructor.</CLOG>

use crate::MigrationMappingRecordClassification;

/// Build a conservative migration mapping record classification.
pub(crate) fn build_migration_mapping_record_classification(
    status: &str,
    recommendation: &str,
    unsupported_input_fields: &[String],
    candidate_blockers: &[String],
    confidence: &str,
    note: &str,
) -> MigrationMappingRecordClassification {
    MigrationMappingRecordClassification {
        status: status.to_string(),
        recommendation: recommendation.to_string(),
        unsupported_input_fields: unsupported_input_fields.to_vec(),
        notes: vec![note.to_string()],
        candidate_blockers: candidate_blockers.to_vec(),
        confidence: confidence.to_string(),
    }
}

/// Build a single-item blocker list.
pub(crate) fn migration_mapping_blocker(value: &str) -> Vec<String> {
    vec![value.to_string()]
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record_classification.rs</FILE> - <DESC>Build migration mapping record classifications</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
