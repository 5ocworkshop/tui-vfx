// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record.rs</FILE> - <DESC>Build one migration mapping record</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.9 migration mapping: isolate conservative per-recipe classification.</WCTX>
// <CLOG>0.2.0: REFACTOR — move legacy mask payload evidence collection into a focused helper.
// 0.1.0: INIT — add focused record builder and mask classification helpers.</CLOG>

use std::{collections::BTreeSet, path::Path};

use serde_json::Value;

use crate::{
    PlayerMigrationMappingRecord,
    fnc_collect_legacy_mask_payloads::{
        collect_legacy_mask_payloads, legacy_mask_evidence_for, required_legacy_mask_descriptors,
        required_legacy_mask_inputs,
    },
};

/// Build one migration mapping record for a legacy recipe file.
pub(crate) fn build_migration_mapping_record(
    legacy_root: &Path,
    v31_root: &Path,
    path: &Path,
    descriptor_ids: &BTreeSet<String>,
    source_ids: &BTreeSet<String>,
) -> Result<PlayerMigrationMappingRecord, String> {
    let relative = path.strip_prefix(legacy_root).unwrap_or(path);
    let legacy_path = normalize_path(relative);
    let family = legacy_path.split('/').next().unwrap_or("other").to_string();
    let recipe_name = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let candidate_canonical_path = canonical_path_for(&legacy_path, &recipe_name);
    let canonical_exists = v31_root.join(&candidate_canonical_path).is_file();
    let value = read_json(path)?;
    let mask_payloads = collect_legacy_mask_payloads(&value);
    let required_descriptor_ids = required_legacy_mask_descriptors(&mask_payloads);
    let required_input_fields = required_legacy_mask_inputs(&mask_payloads);
    let required_source_ids = vec!["source.card".to_string()];
    let missing_descriptor_ids = missing_ids(&required_descriptor_ids, descriptor_ids);
    let missing_source_ids = missing_ids(&required_source_ids, source_ids);
    let (status, recommendation, unsupported_input_fields, notes) = classify_record(
        &legacy_path,
        canonical_exists,
        &required_descriptor_ids,
        &required_input_fields,
    );
    Ok(PlayerMigrationMappingRecord {
        legacy_path,
        legacy_family: family,
        legacy_recipe_name: recipe_name,
        candidate_canonical_path,
        canonical_exists,
        status,
        recommendation,
        evidence: legacy_mask_evidence_for(&mask_payloads),
        required_descriptor_ids,
        missing_descriptor_ids,
        required_source_ids,
        missing_source_ids,
        required_input_fields,
        unsupported_input_fields,
        notes,
    })
}

fn canonical_path_for(legacy_path: &str, recipe_name: &str) -> String {
    if recipe_name.starts_with("_DEPRECATED_") {
        return legacy_path.replacen("_DEPRECATED_", "", 1);
    }
    legacy_path.to_string()
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read `{}` failed: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("parse `{}` failed: {error}", path.display()))
}

fn classify_record(
    legacy_path: &str,
    canonical_exists: bool,
    descriptor_ids: &[String],
    input_fields: &[String],
) -> (String, String, Vec<String>, Vec<String>) {
    if legacy_path.contains("_DEPRECATED_") {
        return classification(
            "ownerAuditNeeded",
            "useAsOracleOnly",
            input_fields.to_vec(),
            "deprecated legacy fixture retained as oracle evidence only",
        );
    }
    if canonical_exists {
        return classification(
            "canonicalExists",
            "skipAsDuplicateVariant",
            Vec::new(),
            "canonical fixture already exists",
        );
    }
    if legacy_path.contains("_square") {
        return classification(
            "duplicateOrVariant",
            "skipAsDuplicateVariant",
            input_fields.to_vec(),
            "geometry-clarity variant of a simple mask candidate",
        );
    }
    let Some(descriptor_id) = descriptor_ids.first() else {
        return classification(
            "notYetClassified",
            "deferForOwnerAudit",
            Vec::new(),
            "no mask payload was found",
        );
    };
    match descriptor_id.as_str() {
        "mask.blinds" | "mask.radial" | "mask.iris" | "mask.diamond" => classification(
            "candidateReady",
            "createCanonicalFixture",
            Vec::new(),
            "simple mask candidate with bounded text-grid evidence",
        ),
        _ => classification(
            "descriptorDecisionNeeded",
            "deferForDescriptorDecision",
            input_fields.to_vec(),
            "mask vocabulary is not accepted in this batch",
        ),
    }
}

fn classification(
    status: &str,
    recommendation: &str,
    unsupported_input_fields: Vec<String>,
    note: &str,
) -> (String, String, Vec<String>, Vec<String>) {
    (
        status.to_string(),
        recommendation.to_string(),
        unsupported_input_fields,
        vec![note.to_string()],
    )
}

fn missing_ids(required: &[String], available: &BTreeSet<String>) -> Vec<String> {
    required
        .iter()
        .filter(|id| !available.contains(*id))
        .cloned()
        .collect()
}

fn normalize_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

// <FILE>crates/tui-vfx-player/src/fnc_build_migration_mapping_record.rs</FILE> - <DESC>Build one migration mapping record</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
