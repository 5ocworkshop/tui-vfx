// <FILE>crates/tui-vfx-player/src/cls_player_schema_readiness_report.rs</FILE> - <DESC>Schema-readiness report DTOs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: expose blocker ledger report shape.</WCTX>
// <CLOG>0.1.0: INIT — add schema-readiness summary, family, blocker, and milestone DTOs.</CLOG>

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
// <VERS>END OF VERSION: 0.1.0</VERS>
