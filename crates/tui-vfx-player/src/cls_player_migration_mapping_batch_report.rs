// <FILE>crates/tui-vfx-player/src/cls_player_migration_mapping_batch_report.rs</FILE> - <DESC>Migration mapping batch report DTOs</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Migration mapping reports: expose conservative per-legacy-recipe records.</WCTX>
// <CLOG>0.3.0: MINOR — add corpus-wide legacy evidence fields without changing report schema.</CLOG>

use crate::DescriptorPackReport;

/// Stable machine-readable report for one migration mapping batch.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationMappingBatchReport {
    /// Stable report schema label.
    pub schema_version: &'static str,
    /// Legacy debug recipe root inspected as read-only evidence.
    pub legacy_root: String,
    /// Canonical v3.1 debug recipe root inspected for existing targets.
    pub v31_root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Families included in this batch.
    pub families: Vec<String>,
    /// Aggregate batch counts.
    pub summary: PlayerMigrationMappingBatchSummary,
    /// Per-legacy-recipe migration classification records.
    pub records: Vec<PlayerMigrationMappingRecord>,
    /// Conservative next actions derived from records.
    pub recommendation_queue: Vec<PlayerMigrationMappingQueueItem>,
    /// Non-fatal warnings.
    pub warnings: Vec<String>,
    /// Fatal report-building errors captured for stable JSON shape.
    pub errors: Vec<String>,
}

/// Aggregate counts for a migration mapping batch report.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationMappingBatchSummary {
    /// Number of families represented in the batch.
    pub families: usize,
    /// Number of per-recipe records emitted.
    pub records: usize,
    /// Records whose canonical fixture already exists.
    pub canonical_exists: usize,
    /// Records ready for bounded canonical fixture creation.
    pub candidate_ready: usize,
    /// Records needing descriptor decisions before migration.
    pub descriptor_decision_needed: usize,
    /// Records needing schema decisions before migration.
    pub schema_decision_needed: usize,
    /// Records needing owner audit before migration.
    pub owner_audit_needed: usize,
    /// Records needing adapter decisions before migration.
    pub adapter_decision_needed: usize,
    /// Records needing source descriptor decisions before migration.
    pub source_decision_needed: usize,
    /// Records blocked by unsupported source vocabulary.
    pub blocked_by_unsupported_source: usize,
    /// Records blocked by unsupported effect vocabulary.
    pub blocked_by_unsupported_effect: usize,
    /// Records blocked by incomplete field coverage.
    pub blocked_by_field_coverage: usize,
    /// Records blocked by ambiguous legacy intent.
    pub blocked_by_ambiguous_legacy_intent: usize,
    /// Records intentionally treated as variants rather than new fixtures.
    pub duplicate_or_variant: usize,
    /// Records that could not yet be classified.
    pub not_yet_classified: usize,
}

/// Conservative migration classification for one legacy recipe.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationMappingRecord {
    /// Path relative to the legacy root.
    pub legacy_path: String,
    /// Family bucket derived from the legacy path.
    pub legacy_family: String,
    /// Legacy recipe file stem.
    pub legacy_recipe_name: String,
    /// Candidate canonical path relative to the v3.1 root.
    pub candidate_canonical_path: String,
    /// Whether the candidate canonical fixture already exists.
    pub canonical_exists: bool,
    /// Stable migration status.
    pub status: String,
    /// Stable recommended next action.
    pub recommendation: String,
    /// Evidence summary used for the classification.
    pub evidence: String,
    /// Effect descriptors required by a faithful canonical fixture.
    pub required_descriptor_ids: Vec<String>,
    /// Required descriptors absent from the loaded descriptor catalog.
    pub missing_descriptor_ids: Vec<String>,
    /// Source descriptors expected by the canonical fixture.
    pub required_source_ids: Vec<String>,
    /// Required source descriptors absent from the loaded descriptor catalog.
    pub missing_source_ids: Vec<String>,
    /// Authored legacy inputs that need descriptor/player coverage.
    pub required_input_fields: Vec<String>,
    /// Legacy input fields intentionally not accepted in this batch.
    pub unsupported_input_fields: Vec<String>,
    /// Human-readable details and deferrals.
    pub notes: Vec<String>,
    /// Legacy signal or signal-like keys observed in the recipe.
    pub legacy_signals: Vec<String>,
    /// Legacy binding keys observed in the recipe.
    pub legacy_bindings: Vec<String>,
    /// Legacy source-kind candidates observed in the recipe.
    pub legacy_source_kinds: Vec<String>,
    /// Effect family kinds observed in the recipe.
    pub legacy_effect_families: Vec<String>,
    /// Candidate blockers that keep this record out of candidateReady.
    pub candidate_blockers: Vec<String>,
    /// Conservative confidence label for the classification.
    pub confidence: String,
}

/// Queued migration action derived from one or more records.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationMappingQueueItem {
    /// Legacy family that owns the action.
    pub legacy_family: String,
    /// Recommended action.
    pub recommendation: String,
    /// Short rationale.
    pub rationale: String,
}

// <FILE>crates/tui-vfx-player/src/cls_player_migration_mapping_batch_report.rs</FILE> - <DESC>Migration mapping batch report DTOs</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
