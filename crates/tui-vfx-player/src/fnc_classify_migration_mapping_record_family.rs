// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_record_family.rs</FILE> - <DESC>Classify migration mapping records by legacy family</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.10 corpus mapping: isolate family-first classifications for schema-sensitive groups.</WCTX>
// <CLOG>0.1.0: INIT — add family-first migration mapping classifications.</CLOG>

use crate::{
    MigrationMappingRecordClassification,
    fnc_build_migration_mapping_record_classification::{
        build_migration_mapping_record_classification, migration_mapping_blocker,
    },
};

/// Return family-first classification when a family has an explicit migration disposition.
pub(crate) fn classify_migration_mapping_record_family(
    family: &str,
    input_fields: &[String],
) -> Option<MigrationMappingRecordClassification> {
    let result = match family {
        "scene" => build(
            "candidateReady",
            "createCanonicalFixture",
            &[],
            "scene source/layer schema accepted",
            "high",
            "scene/layer authoring maps to source.scene plus scene elements and layer visibility",
        ),
        "fixtures" => build(
            "candidateReady",
            "createCanonicalFixture",
            &[],
            "source-only fixture schema accepted",
            "high",
            "command-capture artifacts map to source.commandCaptureArtifact without executing commands during render",
        ),
        "signals" | "easings" | "motion_routes" | "bindable_rates" | "event_driven_dwell" => build(
            "candidateReady",
            "createCanonicalFixture",
            &[],
            "runtime schema accepted",
            "high",
            "runtime, timing, signal, and binding semantics map to lifecycle, bindings, active phases, and element-local motion metadata",
        ),
        "loopback" => build(
            "ownerAuditNeeded",
            "useAsOracleOnly",
            input_fields,
            "loopback demo-layer decision",
            "high",
            "legacy loopback fixtures are demo/oracle evidence, not canonical runtime data",
        ),
        "complex" | "shadows" | "subcell_shapes" => build(
            "ownerAuditNeeded",
            "useAsOracleOnly",
            input_fields,
            "composition/backend decision",
            "medium",
            "legacy complex, shadow, or subcell recipes need owner/backend review",
        ),
        _ => return None,
    };
    Some(result)
}

fn build(
    status: &str,
    recommendation: &str,
    input_fields: &[String],
    blocker: &str,
    confidence: &str,
    note: &str,
) -> MigrationMappingRecordClassification {
    build_migration_mapping_record_classification(
        status,
        recommendation,
        input_fields,
        &migration_mapping_blocker(blocker),
        confidence,
        note,
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_migration_mapping_record_family.rs</FILE> - <DESC>Classify migration mapping records by legacy family</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
