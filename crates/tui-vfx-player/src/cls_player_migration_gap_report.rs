// <FILE>crates/tui-vfx-player/src/cls_player_migration_gap_report.rs</FILE> - <DESC>Debug recipe migration gap report DTOs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: expose old/new debug recipe family gap reports.</WCTX>
// <CLOG>0.2.0: PATCH — include descriptor-pack provenance in migration-gap reports.</CLOG>

use crate::DescriptorPackReport;

/// Stable machine-readable report for debug recipe migration planning.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationGapReport {
    /// Stable migration gap report schema label.
    pub schema_version: &'static str,
    /// Legacy debug recipe corpus root inspected as JSON/path inventory only.
    pub legacy_root: String,
    /// Canonical v3.1 debug recipe corpus root inspected as canonical JSON.
    pub v31_root: String,
    /// Descriptor packs validated for this migration-gap invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate corpus and family status counts.
    pub summary: PlayerMigrationGapSummary,
    /// Per-family path-count and planning status entries.
    pub families: Vec<PlayerMigrationGapFamily>,
    /// Conservative next migration/adaptation queue.
    pub recommended_queue: Vec<PlayerMigrationQueueItem>,
}

/// Aggregate counts for a migration gap report.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationGapSummary {
    /// Number of legacy debug recipe JSON files found.
    pub legacy_recipes: usize,
    /// Number of canonical v3.1 debug recipe JSON files found.
    pub v31_recipes: usize,
    /// Number of families with at least one v3.1 recipe.
    pub represented_families: usize,
    /// Number of legacy-backed families with no v3.1 representative.
    pub unrepresented_families: usize,
    /// Number of legacy-backed families with some but incomplete v3.1 coverage.
    pub partially_represented_families: usize,
    /// Number of families currently suitable for recipe/adaptor follow-up.
    pub ready_families: usize,
    /// Number of families needing schema, descriptor, or owner decisions first.
    pub blocked_families: usize,
}

/// Per-family migration inventory and recommendation entry.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationGapFamily {
    /// Stable family bucket name.
    pub family: String,
    /// Legacy debug recipe JSON count for this family.
    pub legacy_count: usize,
    /// Canonical v3.1 debug recipe JSON count for this family.
    pub v31_count: usize,
    /// Stable coverage classification: none, partial, represented, or notApplicable.
    pub coverage: String,
    /// Effect ids observed in canonical v3.1 recipes for this family.
    pub known_v31_effect_ids: Vec<String>,
    /// Stable planning status for this family.
    pub status: String,
    /// Conservative blockers to resolve before broad migration.
    pub blockers: Vec<String>,
    /// Legacy recipe paths that look like near-term candidates for human review.
    pub recommended_next_candidates: Vec<String>,
}

/// One conservative queue item for future migration/adaptation work.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMigrationQueueItem {
    /// One-based priority rank.
    pub rank: usize,
    /// Family or workstream label.
    pub family: String,
    /// Short objective for the queued workstream.
    pub objective: String,
    /// Evidence-backed reason for this ordering.
    pub rationale: String,
}

// <FILE>crates/tui-vfx-player/src/cls_player_migration_gap_report.rs</FILE> - <DESC>Debug recipe migration gap report DTOs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
