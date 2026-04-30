// <FILE>crates/tui-vfx-player/src/cls_player_implementation_readiness_report.rs</FILE> - <DESC>Player implementation-readiness report DTOs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Implementation readiness: expose disposition-first backlog reporting.</WCTX>
// <CLOG>0.1.0: INIT — add disposition-normalized implementation-readiness DTOs.</CLOG>

use std::collections::BTreeMap;

/// Disposition-first report that separates implementation backlog from schema readiness.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerImplementationReadinessReport {
    /// Stable report schema label.
    pub schema_version: &'static str,
    /// Legacy debug recipe root inspected as read-only evidence.
    pub legacy_root: String,
    /// Canonical v3.1 debug recipe root inspected for existing targets.
    pub v31_root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<crate::DescriptorPackReport>,
    /// Disposition-first summary.
    pub summary: PlayerImplementationReadinessSummary,
    /// Families included in this report.
    pub families: Vec<String>,
    /// Per-legacy-recipe implementation-readiness records.
    pub records: Vec<PlayerImplementationReadinessRecord>,
    /// Disposition-prioritized work queues.
    pub priority_queues: Vec<PlayerImplementationReadinessQueue>,
    /// Signed-off holdback groups.
    pub holdbacks: Vec<PlayerImplementationReadinessHoldback>,
}

/// Aggregate implementation-readiness counts.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerImplementationReadinessSummary {
    /// Total legacy mapping records inspected.
    pub records: usize,
    /// Existing canonical fixtures.
    pub canonical_exists: usize,
    /// Records ready for fixture creation.
    pub candidate_ready: usize,
    /// Records still requiring explicit owner decisions.
    pub explicit_owner_decision_needed: usize,
    /// Records blocking implementation work.
    pub implementation_blocking: usize,
    /// Disposition counts keyed by stable disposition vocabulary.
    pub disposition_counts: BTreeMap<String, usize>,
    /// Blocking-kind counts keyed by stable implementation category.
    pub implementation_blocking_counts: BTreeMap<String, usize>,
}

/// Per-legacy-recipe implementation-readiness record.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerImplementationReadinessRecord {
    pub legacy_path: String,
    pub legacy_family: String,
    pub family: String,
    pub legacy_recipe_id: String,
    pub legacy_recipe_name: String,
    pub canonical_path: String,
    pub canonical_exists: bool,
    pub raw_migration_status: String,
    pub raw_status: String,
    pub disposition: String,
    pub implementation_disposition: String,
    pub assigned_lane: String,
    pub schema_blocking: bool,
    pub implementation_blocking: bool,
    pub blocker_kind: String,
    pub blocking_kind: String,
    pub recommended_action: String,
    pub recommended_next_action: String,
    pub required_descriptors: Vec<String>,
    pub missing_descriptors: Vec<String>,
    pub required_sources: Vec<String>,
    pub missing_sources: Vec<String>,
    pub required_content_descriptors: Vec<String>,
    pub missing_content_descriptors: Vec<String>,
    pub required_player_adapters: Vec<String>,
    pub required_runtime_features: Vec<String>,
    pub field_coverage_issues: Vec<String>,
    pub player_adapter_status: String,
    pub backend_status: String,
    pub holdback_reason: String,
    pub holdback_signed_off: bool,
    pub owner_decision_required: bool,
    pub confidence: String,
    pub notes: Vec<String>,
}

/// Disposition work queue entry.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerImplementationReadinessQueue {
    pub disposition: String,
    pub count: usize,
    pub representative_legacy_paths: Vec<String>,
    pub recommended_next_action: String,
}

/// Signed-off holdback summary.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerImplementationReadinessHoldback {
    pub disposition: String,
    pub count: usize,
    pub signed_off: bool,
    pub representative_legacy_paths: Vec<String>,
}

// <FILE>crates/tui-vfx-player/src/cls_player_implementation_readiness_report.rs</FILE> - <DESC>Player implementation-readiness report DTOs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
