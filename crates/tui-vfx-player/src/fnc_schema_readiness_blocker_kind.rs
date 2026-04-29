// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_kind.rs</FILE> - <DESC>Classify schema-readiness blocker kinds</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: map migration-mapping status/evidence to owner-facing blocker kinds.</WCTX>
// <CLOG>0.1.0: INIT — isolate blocker taxonomy from report grouping.</CLOG>

use crate::PlayerMigrationMappingRecord;

pub(crate) fn schema_readiness_blocker_kind(record: &PlayerMigrationMappingRecord) -> &'static str {
    match record.status.as_str() {
        "schemaDecisionNeeded" if has_blocker(record, "valueSourceOrSignalDecision") => {
            "valueSourceSemantics"
        }
        "schemaDecisionNeeded" if !record.legacy_bindings.is_empty() => "bindingSemantics",
        "schemaDecisionNeeded" if !record.legacy_signals.is_empty() => "lifecycleSemantics",
        "schemaDecisionNeeded" if record.legacy_family == "scene" => "sceneSemantics",
        "schemaDecisionNeeded" if is_motion_timing_family(record) => "motionTimingSemantics",
        "schemaDecisionNeeded" => "schemaModel",
        "sourceDecisionNeeded" | "blockedByUnsupportedSource" => "sourceDescriptor",
        "descriptorDecisionNeeded" | "blockedByUnsupportedEffect" => "descriptorPack",
        "adapterDecisionNeeded" => "playerAdapter",
        "blockedByFieldCoverage" => "fieldCoverage",
        "duplicateOrVariant" => "duplicateOrVariant",
        "ownerAuditNeeded" if has_blocker(record, "deprecated legacy recipe") => "oracleOnly",
        "ownerAuditNeeded" if record.legacy_family == "loopback" => "oracleOnly",
        "ownerAuditNeeded" if is_backend_renderer_family(record) => "backendRenderer",
        "ownerAuditNeeded" => "ownerAudit",
        "blockedByAmbiguousLegacyIntent" => "guiHumanReview",
        _ => "unknown",
    }
}

fn has_blocker(record: &PlayerMigrationMappingRecord, blocker: &str) -> bool {
    record
        .candidate_blockers
        .iter()
        .any(|candidate| candidate == blocker)
}

fn is_backend_renderer_family(record: &PlayerMigrationMappingRecord) -> bool {
    matches!(record.legacy_family.as_str(), "shadows" | "subcell_shapes")
}

fn is_motion_timing_family(record: &PlayerMigrationMappingRecord) -> bool {
    matches!(record.legacy_family.as_str(), "easings" | "motion_routes")
}

// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_kind.rs</FILE> - <DESC>Classify schema-readiness blocker kinds</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
