// <FILE>crates/tui-vfx-player/src/fnc_summarize_migration_mapping_batch.rs</FILE> - <DESC>Summarize migration mapping batch records</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.9 migration mapping: aggregate status counts and conservative action queue.</WCTX>
// <CLOG>0.1.0: INIT — add focused summary and recommendation queue helpers.</CLOG>

use std::collections::BTreeSet;

use crate::{
    PlayerMigrationMappingBatchSummary, PlayerMigrationMappingQueueItem,
    PlayerMigrationMappingRecord,
};

/// Collect stable family names represented by the batch records.
pub(crate) fn migration_mapping_record_families(
    records: &[PlayerMigrationMappingRecord],
) -> Vec<String> {
    sorted_unique(records.iter().map(|record| record.legacy_family.clone()))
}

/// Build aggregate status counts for a migration mapping batch.
pub(crate) fn summarize_migration_mapping_records(
    families: &[String],
    records: &[PlayerMigrationMappingRecord],
) -> PlayerMigrationMappingBatchSummary {
    PlayerMigrationMappingBatchSummary {
        families: families.len(),
        records: records.len(),
        canonical_exists: count_status(records, "canonicalExists"),
        candidate_ready: count_status(records, "candidateReady"),
        descriptor_decision_needed: count_status(records, "descriptorDecisionNeeded"),
        schema_decision_needed: count_status(records, "schemaDecisionNeeded"),
        owner_audit_needed: count_status(records, "ownerAuditNeeded"),
        adapter_decision_needed: count_status(records, "adapterDecisionNeeded"),
        source_decision_needed: count_status(records, "sourceDecisionNeeded"),
        blocked_by_unsupported_source: count_status(records, "blockedByUnsupportedSource"),
        blocked_by_unsupported_effect: count_status(records, "blockedByUnsupportedEffect"),
        blocked_by_field_coverage: count_status(records, "blockedByFieldCoverage"),
        blocked_by_ambiguous_legacy_intent: count_status(records, "blockedByAmbiguousLegacyIntent"),
        duplicate_or_variant: count_status(records, "duplicateOrVariant"),
        not_yet_classified: count_status(records, "notYetClassified"),
    }
}

/// Build a stable queue of non-terminal recommendations by family.
pub(crate) fn build_migration_mapping_recommendation_queue(
    records: &[PlayerMigrationMappingRecord],
) -> Vec<PlayerMigrationMappingQueueItem> {
    sorted_unique(records.iter().filter_map(actionable_recommendation_key))
        .into_iter()
        .filter_map(|key| {
            key.split_once('|')
                .map(|(family, recommendation)| PlayerMigrationMappingQueueItem {
                    legacy_family: family.to_string(),
                    recommendation: recommendation.to_string(),
                    rationale: recommendation_rationale(recommendation).to_string(),
                })
        })
        .collect()
}

fn count_status(records: &[PlayerMigrationMappingRecord], status: &str) -> usize {
    records
        .iter()
        .filter(|record| record.status == status)
        .count()
}

fn actionable_recommendation_key(record: &PlayerMigrationMappingRecord) -> Option<String> {
    let is_terminal = matches!(
        record.recommendation.as_str(),
        "skipAsDuplicateVariant" | "useAsOracleOnly"
    );
    (!is_terminal).then(|| format!("{}|{}", record.legacy_family, record.recommendation))
}

fn recommendation_rationale(recommendation: &str) -> &'static str {
    match recommendation {
        "createCanonicalFixture" => {
            "candidateReady records can move after descriptor decision review"
        }
        "extendDescriptorPack" | "deferForDescriptorDecision" => {
            "descriptorDecisionNeeded records require descriptor vocabulary review before migration"
        }
        "addPlayerAdapter" => {
            "adapterDecisionNeeded records require player support before migration"
        }
        "addSourceDescriptor" => "sourceDecisionNeeded records require source descriptor review",
        "addFieldHandling" => {
            "blockedByFieldCoverage records require field handling before migration"
        }
        "deferForSchemaDecision" => "schemaDecisionNeeded records require contract schema review",
        "deferForOwnerAudit" => "ownerAuditNeeded records require owner review before migration",
        _ => "records require owner review before migration",
    }
}

fn sorted_unique(values: impl IntoIterator<Item = String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_migration_mapping_batch.rs</FILE> - <DESC>Summarize migration mapping batch records</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
