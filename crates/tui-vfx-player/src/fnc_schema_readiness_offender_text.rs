// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_offender_text.rs</FILE> - <DESC>Describe schema-readiness offender rows</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.12 schema lock: centralize lane-specific offender dispositions and holdback reasons.</WCTX>
// <CLOG>0.2.0: MINOR — make runtime, motion, lifecycle, and scene dispositions explicit.
// 0.1.0: INIT — add offender disposition and holdback text helpers.</CLOG>

use crate::PlayerMigrationMappingRecord;

pub(crate) fn schema_readiness_recommended_disposition(kind: &str) -> &'static str {
    match kind {
        "sourceDescriptor" => "deferForSourceDecision",
        "contentDescriptor" | "descriptorPack" | "fieldCoverage" => "deferForDescriptorDecision",
        "bindingSemantics" => "deferForBindingSemanticsDecision",
        "valueSourceSemantics" => "deferForRuntimeValueSourceDecision",
        "motionTimingSemantics" => "deferForMotionTimingDecision",
        "lifecycleSemantics" => "deferForLifecycleDecision",
        "sceneSemantics" => "deferForScenePipelineDecision",
        "schemaModel" => "deferForSchemaModelDecision",
        "playerAdapter" => "addPlayerAdapter",
        "backendRenderer" => "deferForBackend",
        "guiHumanReview" => "deferForGuiReview",
        "oracleOnly" => "markOracleOnly",
        "duplicateOrVariant" => "markDuplicateVariant",
        "ownerPolicyHoldback" => "holdBackProblematic",
        "unknown" => "requiresArchitectDecision",
        _ => "deferForSchemaDecision",
    }
}

pub(crate) fn schema_readiness_holdback_reason(
    record: &PlayerMigrationMappingRecord,
    kind: &str,
) -> String {
    match kind {
        "oracleOnly" => "offline/oracle-only artifact; not runtime schema work".to_string(),
        "contentDescriptor" if record.legacy_family == "styles" => {
            "style scope/content vocabulary needs descriptor classification".to_string()
        }
        "sourceDescriptor" if record.legacy_family == "complex" => {
            "complex source/content pipeline composition needs source descriptor decision"
                .to_string()
        }
        "descriptorPack" if record.legacy_family == "complex" => {
            "complex descriptor composition needs descriptor-pack triage".to_string()
        }
        "ownerPolicyHoldback" => "complex composition lacks a safe v3.1 owner policy".to_string(),
        "fieldCoverage" => {
            "authored fields need exact descriptor or adapter disposition".to_string()
        }
        "backendRenderer" => "requires backend/compositor rendering boundary".to_string(),
        _ => record
            .candidate_blockers
            .first()
            .cloned()
            .unwrap_or_else(|| "classified from migration mapping evidence".to_string()),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_offender_text.rs</FILE> - <DESC>Describe schema-readiness offender rows</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
