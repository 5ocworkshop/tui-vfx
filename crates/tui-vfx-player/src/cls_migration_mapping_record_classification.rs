// <FILE>crates/tui-vfx-player/src/cls_migration_mapping_record_classification.rs</FILE> - <DESC>Migration mapping record classification DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: keep record classification data separate from record construction.</WCTX>
// <CLOG>0.1.0: INIT — add internal classification carrier for migration mapping records.</CLOG>

/// Conservative migration classification for one legacy record.
pub(crate) struct MigrationMappingRecordClassification {
    /// Stable migration status.
    pub status: String,
    /// Stable recommended next action.
    pub recommendation: String,
    /// Input fields not yet accepted by descriptor/player coverage.
    pub unsupported_input_fields: Vec<String>,
    /// Human-readable classification notes.
    pub notes: Vec<String>,
    /// Blockers that keep this record out of candidateReady.
    pub candidate_blockers: Vec<String>,
    /// Conservative confidence label.
    pub confidence: String,
}

// <FILE>crates/tui-vfx-player/src/cls_migration_mapping_record_classification.rs</FILE> - <DESC>Migration mapping record classification DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
