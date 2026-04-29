// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_blockers.rs</FILE> - <DESC>Build grouped schema-readiness blockers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: group migration mapping records into owner-facing blockers.</WCTX>
// <CLOG>0.1.0: INIT — move blocker grouping out of report orchestration.</CLOG>

use std::collections::BTreeMap;

use crate::{
    PlayerMigrationMappingRecord, PlayerSchemaReadinessBlocker, is_schema_readiness_blocking,
    schema_readiness_blocker_kind, schema_readiness_blocker_notes,
    schema_readiness_blocking_decision, schema_readiness_next_packet,
};

pub(crate) fn build_schema_readiness_blockers(
    records: &[PlayerMigrationMappingRecord],
) -> Vec<PlayerSchemaReadinessBlocker> {
    grouped_blocker_records(records)
        .into_iter()
        .map(|(key, group)| build_blocker(key, group))
        .collect()
}

fn grouped_blocker_records(
    records: &[PlayerMigrationMappingRecord],
) -> BTreeMap<String, Vec<&PlayerMigrationMappingRecord>> {
    let mut groups: BTreeMap<String, Vec<&PlayerMigrationMappingRecord>> = BTreeMap::new();
    for record in records {
        if record.status == "canonicalExists" || record.status == "candidateReady" {
            continue;
        }
        let kind = schema_readiness_blocker_kind(record);
        groups
            .entry(format!(
                "{}|{}|{}",
                record.legacy_family, record.status, kind
            ))
            .or_default()
            .push(record);
    }
    groups
}

fn build_blocker(
    key: String,
    group: Vec<&PlayerMigrationMappingRecord>,
) -> PlayerSchemaReadinessBlocker {
    let mut parts = key.split('|');
    let family = parts.next().unwrap_or_default().to_string();
    let status = parts.next().unwrap_or_default().to_string();
    let kind = parts.next().unwrap_or("unknown").to_string();
    PlayerSchemaReadinessBlocker {
        id: blocker_id(&family, &status, &kind),
        family,
        record_count: group.len(),
        representative_legacy_paths: representative_paths(&group),
        status_from_migration_mapping: status,
        blocker_kind: kind.clone(),
        blocking_decision: schema_readiness_blocking_decision(&kind).to_string(),
        recommended_next_packet: schema_readiness_next_packet(&kind).to_string(),
        confidence: lowest_confidence(&group),
        is_schema_readiness_blocking: is_schema_readiness_blocking(&kind),
        notes: schema_readiness_blocker_notes(&group),
    }
}

fn representative_paths(records: &[&PlayerMigrationMappingRecord]) -> Vec<String> {
    records
        .iter()
        .take(10)
        .map(|record| record.legacy_path.clone())
        .collect()
}

fn lowest_confidence(records: &[&PlayerMigrationMappingRecord]) -> String {
    if records.iter().any(|record| record.confidence == "low") {
        "low".to_string()
    } else if records.iter().any(|record| record.confidence == "medium") {
        "medium".to_string()
    } else {
        "high".to_string()
    }
}

fn blocker_id(family: &str, status: &str, kind: &str) -> String {
    format!(
        "{}-{}-{}",
        slug_part(family),
        slug_part(status),
        slug_part(kind)
    )
}

fn slug_part(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_schema_readiness_blockers.rs</FILE> - <DESC>Build grouped schema-readiness blockers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
