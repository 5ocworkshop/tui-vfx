// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_offender_text.rs</FILE> - <DESC>Describe schema-readiness offender rows</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>K2.13 schema decision burn-down: centralize final disposition and next-action text.</WCTX>
// <CLOG>0.3.0: MINOR — add accepted/backlog/holdback disposition text for schema decision readiness.
// 0.2.0: MINOR — make runtime, motion, lifecycle, and scene dispositions explicit.</CLOG>

use crate::PlayerMigrationMappingRecord;

pub(crate) fn schema_readiness_disposition(kind: &str) -> &'static str {
    match kind {
        "descriptorPack" | "sourceDescriptor" => "descriptorBacklog",
        "playerAdapter" => "adapterBacklog",
        "backendRenderer" => "backendHoldback",
        "guiHumanReview" => "guiHumanReviewHoldback",
        "oracleOnly" => "oracleOnly",
        "duplicateOrVariant" => "duplicateVariant",
        "unknown" | "ownerPolicyHoldback" | "ownerAudit" => "explicitOwnerDecisionNeeded",
        "bindingSemantics"
        | "contentDescriptor"
        | "fieldCoverage"
        | "lifecycleSemantics"
        | "motionTimingSemantics"
        | "sceneSemantics"
        | "schemaModel"
        | "valueSourceSemantics" => "acceptedSchema",
        _ => "explicitOwnerDecisionNeeded",
    }
}

pub(crate) fn schema_readiness_exact_decision_required(
    record: &PlayerMigrationMappingRecord,
    kind: &str,
) -> String {
    if schema_readiness_disposition(kind) == "explicitOwnerDecisionNeeded" {
        format!(
            "Decide canonical v3.1 schema disposition for {} ({kind})",
            record.legacy_path
        )
    } else {
        String::new()
    }
}

pub(crate) fn schema_readiness_recommended_next_action(kind: &str) -> &'static str {
    match schema_readiness_disposition(kind) {
        "acceptedSchema" => {
            "migrate canonical fixture or add player evidence using accepted schema semantics"
        }
        "descriptorBacklog" => {
            "add descriptor vocabulary and adapter evidence without changing schema"
        }
        "adapterBacklog" => "add player adapter support for already accepted schema",
        "backendHoldback" => "defer to backend renderer packet with visual evidence policy",
        "guiHumanReviewHoldback" => "defer to human visual review and conflict policy signoff",
        "oracleOnly" => "retain as offline oracle evidence only",
        "duplicateVariant" => "retain duplicate or variant disposition",
        _ => "obtain exact owner schema decision",
    }
}

pub(crate) fn schema_readiness_recommended_disposition(kind: &str) -> &'static str {
    schema_readiness_disposition(kind)
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
// <VERS>END OF VERSION: 0.3.0</VERS>
