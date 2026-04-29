// <FILE>crates/tui-vfx-player/src/cls_player_schema_readiness_report.rs</FILE> - <DESC>Schema-readiness report DTOs</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>K2.13 schema decision burn-down: expose disposition-based readiness declaration fields.</WCTX>
// <CLOG>0.3.0: MINOR — add disposition counts and remaining owner decision DTOs.
// 0.2.0: MINOR — add schema-readiness offender DTOs.
// 0.1.0: INIT — add schema-readiness summary, family, blocker, and milestone DTOs.</CLOG>

use std::collections::BTreeMap;

use crate::DescriptorPackReport;

/// Stable machine-readable report for debug recipe schema-readiness planning.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessReport {
    /// Stable report schema label.
    pub schema_version: &'static str,
    /// Legacy debug recipe root inspected as read-only evidence.
    pub legacy_root: String,
    /// Canonical v3.1 debug recipe root inspected for existing targets.
    pub v31_root: String,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<DescriptorPackReport>,
    /// Aggregate schema-readiness counts.
    pub summary: PlayerSchemaReadinessSummary,
    /// Per-family blocker counts derived from migration mapping records.
    pub families: Vec<PlayerSchemaReadinessFamily>,
    /// Grouped outstanding blockers with representative legacy paths.
    pub blockers: Vec<PlayerSchemaReadinessBlocker>,
    /// Optional per-record offender rows for schema-lock planning.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub offenders: Vec<PlayerSchemaReadinessOffender>,
    /// Ordered milestones needed before a readiness declaration.
    pub readiness_milestones: Vec<PlayerSchemaReadinessMilestone>,
    /// Non-fatal warnings.
    pub warnings: Vec<String>,
    /// Fatal report-building errors captured for stable JSON shape.
    pub errors: Vec<String>,
}

/// Aggregate counts for the schema-readiness report.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessSummary {
    /// Number of legacy records inspected.
    pub total_legacy_records: usize,
    /// Records that are canonical, candidate-ready, or explicitly non-schema dispositions.
    pub schema_ready_records: usize,
    /// Records blocked by schema/model decisions.
    pub schema_blocked_records: usize,
    /// Records blocked by source descriptor decisions.
    pub source_blocked_records: usize,
    /// Records blocked by descriptor-pack vocabulary decisions.
    pub descriptor_blocked_records: usize,
    /// Records blocked by player adapter availability.
    pub adapter_blocked_records: usize,
    /// Records blocked by unhandled authored fields.
    pub field_coverage_blocked_records: usize,
    /// Records still requiring owner audit.
    pub owner_audit_records: usize,
    /// Records classified as oracle-only evidence.
    pub oracle_only_records: usize,
    /// Records classified as duplicate or variant fixtures.
    pub duplicate_or_variant_records: usize,
    /// Records that remain unknown or not yet classified.
    pub unknown_records: usize,
    /// Conservative readiness percentage for planning.
    pub estimated_schema_readiness_percent: f64,
    /// Whether a 100% schema-readiness declaration is currently justified.
    pub can_declare_schema_ready: bool,
    /// Offender rows that still block schema readiness after disposition mapping.
    pub unresolved_schema_blockers: usize,
    /// Offender rows signed off as explicit holdbacks.
    pub signed_off_holdbacks: usize,
    /// Offender rows that still need exact owner decisions.
    pub explicit_owner_decision_needed: usize,
    /// Counts grouped by resolved schema-decision disposition.
    pub disposition_counts: BTreeMap<String, usize>,
    /// Remaining exact owner decisions after disposition mapping.
    pub remaining_owner_decision_count: usize,
    /// Exact remaining owner decisions, if schema readiness cannot yet be declared.
    pub remaining_owner_decisions: Vec<PlayerSchemaReadinessOwnerDecision>,
}

/// Exact remaining owner decision required before declaring schema readiness.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessOwnerDecision {
    /// Legacy path requiring the decision.
    pub path: String,
    /// Legacy family bucket for the path.
    pub family: String,
    /// Current blocker kind after evidence classification.
    pub blocker_kind: String,
    /// Specific decision required from the owner.
    pub exact_decision_required: String,
}

/// Per-family schema-readiness counts.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessFamily {
    /// Legacy family name.
    pub family: String,
    /// Records represented by the family.
    pub record_count: usize,
    /// Counts grouped by blocker kind.
    pub blocker_counts: BTreeMap<String, usize>,
}

/// One grouped schema-readiness blocker.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessBlocker {
    /// Stable blocker id.
    pub id: String,
    /// Legacy family owning the grouped records.
    pub family: String,
    /// Number of records represented by this blocker.
    pub record_count: usize,
    /// Representative legacy paths from this blocker group.
    pub representative_legacy_paths: Vec<String>,
    /// Source migration-mapping status.
    pub status_from_migration_mapping: String,
    /// Schema-readiness blocker kind.
    pub blocker_kind: String,
    /// Blocking decision to make next.
    pub blocking_decision: String,
    /// Recommended next packet or tranche.
    pub recommended_next_packet: String,
    /// Conservative confidence label.
    pub confidence: String,
    /// Whether this blocker prevents a schema-readiness declaration.
    pub is_schema_readiness_blocking: bool,
    /// Evidence notes.
    pub notes: Vec<String>,
}

/// One offender row in the opt-in schema-readiness ledger.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessOffender {
    /// Path relative to the legacy debug recipe root.
    pub legacy_path: String,
    /// Legacy family bucket.
    pub family: String,
    /// Legacy recipe file stem.
    pub legacy_recipe_name: String,
    /// Current migration-mapping status.
    pub current_status: String,
    /// Concrete schema-readiness blocker kind.
    pub blocker_kind: String,
    /// Backward-compatible alias for whether this row still blocks schema-readiness declaration.
    pub schema_readiness_blocking: bool,
    /// Resolved schema-decision disposition for this row.
    pub disposition: String,
    /// Whether this row is still a schema blocker after disposition mapping.
    pub schema_blocking: bool,
    /// Whether a holdback/backlog disposition is explicitly signed off for schema lock.
    pub holdback_signed_off: bool,
    /// Exact owner decision still required, or empty when resolved.
    pub exact_decision_required: String,
    /// Concrete next action for this row after schema decision.
    pub recommended_next_action: String,
    /// Legacy recommendation vocabulary preserved for migration-report continuity.
    pub recommended_disposition: String,
    /// Recommended follow-up packet or tranche.
    pub recommended_next_packet: String,
    /// Conservative confidence label.
    pub confidence: String,
    /// Candidate canonical path relative to the v3.1 root.
    pub candidate_canonical_path: String,
    /// Whether the candidate canonical fixture already exists.
    pub canonical_exists: bool,
    /// Descriptor ids required by a faithful fixture.
    pub required_descriptor_ids: Vec<String>,
    /// Descriptor ids missing from loaded descriptor packs.
    pub missing_descriptor_ids: Vec<String>,
    /// Source descriptor ids required by a faithful fixture.
    pub required_source_ids: Vec<String>,
    /// Source descriptor ids missing from loaded descriptor packs.
    pub missing_source_ids: Vec<String>,
    /// Authored fields that remain unsupported or undecided.
    pub unsupported_input_fields: Vec<String>,
    /// Human-readable holdback reason.
    pub holdback_reason: String,
    /// Evidence notes from migration mapping.
    pub notes: Vec<String>,
}

/// One readiness milestone needed before declaring 100% readiness.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSchemaReadinessMilestone {
    /// Milestone id.
    pub id: String,
    /// Milestone description.
    pub description: String,
    /// Whether current evidence says this milestone is complete.
    pub complete: bool,
}

// <FILE>crates/tui-vfx-player/src/cls_player_schema_readiness_report.rs</FILE> - <DESC>Schema-readiness report DTOs</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
