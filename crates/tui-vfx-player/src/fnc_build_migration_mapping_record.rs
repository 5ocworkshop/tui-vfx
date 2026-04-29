// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record.rs</FILE> - <DESC>Build one migration mapping record</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>K2.10 corpus mapping: keep record construction focused after classifier split.</WCTX>
// <CLOG>0.6.0: MINOR — pass value-source decision evidence into classification.</CLOG>

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use serde_json::Value;

use crate::{
    PlayerMigrationMappingRecord,
    fnc_build_migration_mapping_record_paths::{
        migration_mapping_canonical_path_for, migration_mapping_family_for,
        normalize_migration_mapping_path,
    },
    fnc_classify_migration_mapping_record::classify_migration_mapping_record,
    fnc_collect_legacy_migration_mapping_evidence::collect_legacy_migration_mapping_evidence,
    fnc_collect_unsupported_migration_mapping_fields::collect_unsupported_migration_mapping_fields,
};

/// Build one migration mapping record for a legacy recipe file.
pub(crate) fn build_migration_mapping_record(
    legacy_root: &Path,
    v31_root: &Path,
    path: &Path,
    descriptor_ids: &BTreeSet<String>,
    source_ids: &BTreeSet<String>,
    descriptor_input_fields: &BTreeMap<String, BTreeSet<String>>,
) -> Result<PlayerMigrationMappingRecord, String> {
    let relative = path.strip_prefix(legacy_root).unwrap_or(path);
    let legacy_path = normalize_migration_mapping_path(relative);
    let family = migration_mapping_family_for(&legacy_path);
    let recipe_name = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let candidate_canonical_path = migration_mapping_canonical_path_for(&legacy_path, &recipe_name);
    let canonical_exists = v31_root.join(&candidate_canonical_path).is_file();
    let evidence = collect_legacy_migration_mapping_evidence(&family, &read_json(path)?);
    let missing_descriptor_ids = missing_ids(&evidence.required_descriptor_ids, descriptor_ids);
    let missing_source_ids = missing_ids(&evidence.required_source_ids, source_ids);
    let unsupported_fields = collect_unsupported_migration_mapping_fields(
        &evidence.required_descriptor_ids,
        &evidence.required_input_fields,
        descriptor_input_fields,
    );
    let classification = classify_migration_mapping_record(
        &family,
        &legacy_path,
        canonical_exists,
        &evidence.required_descriptor_ids,
        &evidence.required_input_fields,
        &missing_descriptor_ids,
        &missing_source_ids,
        &unsupported_fields,
        &evidence.value_source_decision_fields,
    );
    Ok(PlayerMigrationMappingRecord {
        legacy_path,
        legacy_family: family,
        legacy_recipe_name: recipe_name,
        candidate_canonical_path,
        canonical_exists,
        status: classification.status,
        recommendation: classification.recommendation,
        evidence: evidence.summary,
        required_descriptor_ids: evidence.required_descriptor_ids,
        missing_descriptor_ids,
        required_source_ids: evidence.required_source_ids,
        missing_source_ids,
        required_input_fields: evidence.required_input_fields,
        unsupported_input_fields: classification.unsupported_input_fields,
        notes: classification.notes,
        legacy_signals: evidence.legacy_signals,
        legacy_bindings: evidence.legacy_bindings,
        legacy_source_kinds: evidence.legacy_source_kinds,
        legacy_effect_families: evidence.legacy_effect_families,
        candidate_blockers: classification.candidate_blockers,
        confidence: classification.confidence,
    })
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read `{}` failed: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("parse `{}` failed: {error}", path.display()))
}

fn missing_ids(required: &[String], available: &BTreeSet<String>) -> Vec<String> {
    required
        .iter()
        .filter(|id| !available.contains(*id))
        .cloned()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record.rs</FILE> - <DESC>Build one migration mapping record</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
