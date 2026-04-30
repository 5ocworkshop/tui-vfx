// <FILE>crates/tui-vfx-player/src/fnc_build_implementation_readiness_report.rs</FILE> - <DESC>Build implementation-readiness reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Implementation readiness: normalize migration mapping into disposition-first player backlog.</WCTX>
// <CLOG>0.1.0: INIT — add report builder with corrected source/content vocabulary.</CLOG>

use std::{collections::BTreeMap, path::Path};

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerImplementationReadinessHoldback,
    PlayerImplementationReadinessQueue, PlayerImplementationReadinessRecord,
    PlayerImplementationReadinessReport, PlayerImplementationReadinessSummary,
    build_migration_mapping_batch_report,
};

/// Build a disposition-first implementation-readiness report from migration mapping evidence.
pub fn build_implementation_readiness_report(
    legacy_root: &Path,
    v31_root: &Path,
    descriptor_packs: Vec<DescriptorPackReport>,
    catalog: &DescriptorCatalog,
    family: Option<&str>,
    recursive: bool,
) -> Result<PlayerImplementationReadinessReport, String> {
    let mapping = build_migration_mapping_batch_report(
        legacy_root,
        v31_root,
        descriptor_packs.clone(),
        catalog,
        family,
        recursive,
    )?;
    let records = mapping
        .records
        .iter()
        .map(readiness_record)
        .collect::<Vec<_>>();
    let summary = summarize_readiness_records(&records);
    let priority_queues = build_priority_queues(&records);
    let holdbacks = build_holdbacks(&records);
    Ok(PlayerImplementationReadinessReport {
        schema_version: "v3.1.player.implementationReadiness.1",
        legacy_root: legacy_root.display().to_string(),
        v31_root: v31_root.display().to_string(),
        descriptor_packs,
        summary,
        families: mapping.families,
        records,
        priority_queues,
        holdbacks,
    })
}

fn readiness_record(
    record: &crate::PlayerMigrationMappingRecord,
) -> PlayerImplementationReadinessRecord {
    let required_content_descriptors = content_descriptors(record);
    let required_sources = record
        .required_source_ids
        .iter()
        .filter(|source| content_descriptor_for_source(source).is_none())
        .cloned()
        .collect::<Vec<_>>();
    let missing_content_descriptors = required_content_descriptors
        .iter()
        .filter(|content| {
            !record.canonical_exists && !record.required_descriptor_ids.contains(*content)
        })
        .cloned()
        .collect::<Vec<_>>();
    let disposition = disposition_for(record, &required_content_descriptors);
    let implementation_blocking = implementation_blocking(&disposition);
    let blocking_kind = blocking_kind_for(&disposition).to_string();
    let recommended_action = recommended_next_action_for(&disposition).to_string();
    PlayerImplementationReadinessRecord {
        legacy_path: record.legacy_path.clone(),
        legacy_family: record.legacy_family.clone(),
        family: record.legacy_family.clone(),
        legacy_recipe_id: record.legacy_recipe_name.clone(),
        legacy_recipe_name: record.legacy_recipe_name.clone(),
        canonical_path: record.candidate_canonical_path.clone(),
        canonical_exists: record.canonical_exists,
        raw_migration_status: record.status.clone(),
        raw_status: record.status.clone(),
        implementation_disposition: disposition.clone(),
        assigned_lane: assigned_lane_for(record, &disposition).to_string(),
        schema_blocking: schema_blocking(record),
        disposition: disposition.clone(),
        implementation_blocking,
        blocker_kind: blocking_kind.clone(),
        blocking_kind,
        recommended_action: recommended_action.clone(),
        recommended_next_action: recommended_action,
        required_descriptors: record.required_descriptor_ids.clone(),
        missing_descriptors: record.missing_descriptor_ids.clone(),
        missing_sources: record
            .missing_source_ids
            .iter()
            .filter(|source| content_descriptor_for_source(source).is_none())
            .cloned()
            .collect(),
        required_sources,
        required_content_descriptors,
        missing_content_descriptors,
        required_player_adapters: required_player_adapters(record, &disposition),
        required_runtime_features: required_runtime_features(record, &disposition),
        field_coverage_issues: record.unsupported_input_fields.clone(),
        player_adapter_status: player_adapter_status_for(&disposition).to_string(),
        backend_status: backend_status_for(record, &disposition).to_string(),
        holdback_reason: holdback_reason_for(record, &disposition),
        holdback_signed_off: holdback_signed_off(&disposition),
        owner_decision_required: disposition == "explicitOwnerDecisionNeeded",
        confidence: record.confidence.clone(),
        notes: readiness_notes(record),
    }
}

fn content_descriptors(record: &crate::PlayerMigrationMappingRecord) -> Vec<String> {
    let mut values = record
        .required_source_ids
        .iter()
        .filter_map(|source| content_descriptor_for_source(source))
        .collect::<Vec<_>>();
    if record.legacy_family == "content" {
        values.extend(content_descriptor_for_legacy_name(
            &record.legacy_recipe_name,
        ));
    }
    values.sort();
    values.dedup();
    values
}

fn content_descriptor_for_source(source: &str) -> Option<String> {
    let descriptor = match source {
        "source.typewriterText" => "content.typewriter",
        "source.splitFlapText" => "content.splitFlap",
        "source.odometer" => "content.odometer",
        "source.marqueeText" => "content.marquee",
        "source.scramble" => "content.scramble",
        "source.morph" => "content.morph",
        "source.redact" => "content.redact",
        "source.glyphCascade" => "content.glyphCascade",
        "source.glyphParticles" => "content.glyphParticles",
        "source.slideShift" => "content.slideShift",
        "source.mirror" => "content.mirror",
        "source.numeric" => "content.numeric",
        "source.dissolve" => "content.dissolve",
        "source.cellMotion" => "content.cellMotion",
        _ => return None,
    };
    Some(descriptor.to_string())
}

fn content_descriptor_for_legacy_name(name: &str) -> Vec<String> {
    let candidates = [
        ("typewriter", "content.typewriter"),
        ("split_flap", "content.splitFlap"),
        ("odometer", "content.odometer"),
        ("marquee", "content.marquee"),
        ("scramble", "content.scramble"),
        ("morph", "content.morph"),
        ("redact", "content.redact"),
        ("glyph_cascade", "content.glyphCascade"),
        ("glyph_particles", "content.glyphParticles"),
        ("slide_shift", "content.slideShift"),
        ("mirror", "content.mirror"),
        ("numeric", "content.numeric"),
        ("dissolve", "content.dissolve"),
        ("cell_motion", "content.cellMotion"),
        ("glitch_shift", "content.glitchShift"),
        ("scramble_glitch_shift", "content.scrambleGlitchShift"),
    ];
    candidates
        .iter()
        .filter(|(needle, _)| name.contains(needle))
        .map(|(_, descriptor)| (*descriptor).to_string())
        .collect()
}

fn disposition_for(record: &crate::PlayerMigrationMappingRecord, content: &[String]) -> String {
    if record.canonical_exists {
        return "canonicalExists".to_string();
    }
    if record.legacy_path.contains("_DEPRECATED_") {
        return "deprecatedLegacySignedOff".to_string();
    }
    if !content.is_empty() {
        return content_disposition(content);
    }
    if is_holdback(record, "backend") {
        return "backendHoldbackSignedOff".to_string();
    }
    if is_holdback(record, "gui") {
        return "guiHumanReviewHoldbackSignedOff".to_string();
    }
    match record.status.as_str() {
        "candidateReady" => "candidateReady",
        "descriptorDecisionNeeded" | "blockedByUnsupportedEffect" | "blockedByFieldCoverage" => {
            descriptor_disposition(record)
        }
        "sourceDecisionNeeded" | "blockedByUnsupportedSource" => "sourceBacklogResolved",
        "adapterDecisionNeeded" => "adapterBacklog",
        "duplicateOrVariant" => "duplicateVariantSignedOff",
        "schemaDecisionNeeded" if mentions(record, "scene") => "sceneRuntimeResolved",
        "schemaDecisionNeeded" => "graphRuntimeResolved",
        "ownerAuditNeeded" if record.recommendation == "useAsOracleOnly" => "oracleOnlySignedOff",
        "ownerAuditNeeded" => "explicitOwnerDecisionNeeded",
        _ => "explicitOwnerDecisionNeeded",
    }
    .to_string()
}

fn content_disposition(content: &[String]) -> String {
    if content.iter().any(|descriptor| {
        matches!(
            descriptor.as_str(),
            "content.glyphParticles" | "content.glyphCascade"
        )
    }) {
        "backendHoldbackSignedOff".to_string()
    } else {
        "duplicateVariantSignedOff".to_string()
    }
}

fn descriptor_disposition(record: &crate::PlayerMigrationMappingRecord) -> &'static str {
    if descriptor_needs_backend(record) {
        "backendHoldbackSignedOff"
    } else {
        "descriptorBacklogResolved"
    }
}

fn descriptor_needs_backend(record: &crate::PlayerMigrationMappingRecord) -> bool {
    record
        .required_descriptor_ids
        .iter()
        .chain(record.missing_descriptor_ids.iter())
        .chain(record.legacy_effect_families.iter())
        .chain(record.candidate_blockers.iter())
        .any(|value| {
            let value = value.to_ascii_lowercase();
            value.contains("subcell")
                || value.contains("terminal")
                || value.contains("trace")
                || value.contains("orbit")
                || value.contains("chromatic")
                || value.contains("shadow")
                || value.contains("fire")
                || value.contains("water")
                || value.contains("rigidshake")
                || value.contains("stochastic")
        })
}

fn is_holdback(record: &crate::PlayerMigrationMappingRecord, needle: &str) -> bool {
    record
        .candidate_blockers
        .iter()
        .chain(record.notes.iter())
        .any(|value| value.to_ascii_lowercase().contains(needle))
}

fn mentions(record: &crate::PlayerMigrationMappingRecord, needle: &str) -> bool {
    record
        .candidate_blockers
        .iter()
        .chain(record.notes.iter())
        .chain(record.legacy_effect_families.iter())
        .any(|value| value.to_ascii_lowercase().contains(needle))
}

fn implementation_blocking(disposition: &str) -> bool {
    matches!(
        disposition,
        "descriptorBacklog"
            | "contentBacklog"
            | "sourceBacklog"
            | "adapterBacklog"
            | "sceneRuntimeBacklog"
            | "graphRuntimeBacklog"
    )
}

fn blocking_kind_for(disposition: &str) -> &'static str {
    match disposition {
        "descriptorBacklog" | "descriptorBacklogResolved" => "descriptor",
        "contentBacklog" | "contentBacklogResolved" => "content",
        "sourceBacklog" | "sourceBacklogResolved" => "source",
        "adapterBacklog" => "playerAdapter",
        "candidateReady" => "fixtureAuthoring",
        "sceneRuntimeBacklog" | "sceneRuntimeResolved" => "sceneRuntime",
        "graphRuntimeBacklog" | "graphRuntimeResolved" => "graphRuntime",
        "backendHoldback" | "backendHoldbackSignedOff" => "backend",
        "guiHumanReviewHoldback" | "guiHumanReviewHoldbackSignedOff" => "guiHumanReview",
        "explicitOwnerDecisionNeeded" => "ownerDecision",
        "oracleOnlySignedOff" => "oracle",
        "duplicateVariantSignedOff" => "duplicateVariant",
        "deprecatedLegacySignedOff" => "deprecatedLegacy",
        _ => "none",
    }
}

fn recommended_next_action_for(disposition: &str) -> &'static str {
    match disposition {
        "canonicalExists" => "none",
        "candidateReady" => "createCanonicalFixture",
        "descriptorBacklog" => "extendDescriptorPack",
        "descriptorBacklogResolved" => "none",
        "contentBacklog" => "addContentDescriptorAndAdapter",
        "contentBacklogResolved" => "none",
        "sourceBacklog" => "addSourceDescriptorOrResolver",
        "sourceBacklogResolved" => "none",
        "adapterBacklog" => "addPlayerAdapter",
        "sceneRuntimeBacklog" => "implementSceneRuntimeEvidence",
        "graphRuntimeBacklog" => "implementGraphRuntimeEvidence",
        "sceneRuntimeResolved" | "graphRuntimeResolved" => "none",
        "backendHoldback" | "backendHoldbackSignedOff" => "deferToBackendSeam",
        "guiHumanReviewHoldback" | "guiHumanReviewHoldbackSignedOff" => "deferToHumanReview",
        "oracleOnly"
        | "oracleOnlySignedOff"
        | "duplicateVariant"
        | "duplicateVariantSignedOff"
        | "deprecatedLegacy"
        | "deprecatedLegacySignedOff" => "doNotMigrate",
        _ => "requestOwnerDecision",
    }
}

fn player_adapter_status_for(disposition: &str) -> &'static str {
    match disposition {
        "canonicalExists"
        | "candidateReady"
        | "contentBacklogResolved"
        | "descriptorBacklogResolved"
        | "graphRuntimeResolved"
        | "sceneRuntimeResolved"
        | "sourceBacklogResolved" => "covered",
        "adapterBacklog" | "contentBacklog" | "descriptorBacklog" | "sourceBacklog" => "needed",
        _ => "notApplicable",
    }
}

fn backend_status_for(
    record: &crate::PlayerMigrationMappingRecord,
    disposition: &str,
) -> &'static str {
    if disposition == "backendHoldback"
        || disposition == "backendHoldbackSignedOff"
        || is_holdback(record, "backend")
    {
        "heldBack"
    } else {
        "notRequired"
    }
}

fn holdback_signed_off(disposition: &str) -> bool {
    matches!(
        disposition,
        "backendHoldback"
            | "backendHoldbackSignedOff"
            | "guiHumanReviewHoldback"
            | "guiHumanReviewHoldbackSignedOff"
            | "oracleOnly"
            | "oracleOnlySignedOff"
            | "duplicateVariantSignedOff"
            | "deprecatedLegacySignedOff"
            | "graphRuntimeResolved"
            | "sceneRuntimeResolved"
            | "sourceBacklogResolved"
            | "descriptorBacklogResolved"
    )
}

fn schema_blocking(record: &crate::PlayerMigrationMappingRecord) -> bool {
    matches!(
        record.status.as_str(),
        "schemaDecisionNeeded" | "blockedByAmbiguousLegacyIntent"
    )
}

fn assigned_lane_for(
    record: &crate::PlayerMigrationMappingRecord,
    disposition: &str,
) -> &'static str {
    match disposition {
        "contentBacklog" | "contentBacklogResolved" | "duplicateVariantSignedOff"
            if record.legacy_family == "content" =>
        {
            "content"
        }
        "sourceBacklog" | "sourceBacklogResolved" => "source",
        "descriptorBacklog" | "descriptorBacklogResolved" | "backendHoldbackSignedOff"
            if matches!(
                record.legacy_family.as_str(),
                "filters" | "masks" | "samplers" | "shaders" | "styles"
            ) =>
        {
            if matches!(record.legacy_family.as_str(), "shaders" | "styles") {
                "shaderStyleDescriptor"
            } else {
                "filterMaskSamplerDescriptor"
            }
        }
        "graphRuntimeBacklog" | "graphRuntimeResolved" => "graphRuntime",
        "sceneRuntimeBacklog" | "sceneRuntimeResolved" => "sceneRuntime",
        "backendHoldback" | "backendHoldbackSignedOff" => "holdback",
        "guiHumanReviewHoldback" | "guiHumanReviewHoldbackSignedOff" => "holdback",
        "oracleOnly" | "oracleOnlySignedOff" | "deprecatedLegacySignedOff" => "holdback",
        "candidateReady" => "fixtureAuthoring",
        _ if !record.unsupported_input_fields.is_empty() => "fieldCoverage",
        _ => "blockerLedger",
    }
}

fn required_player_adapters(
    record: &crate::PlayerMigrationMappingRecord,
    disposition: &str,
) -> Vec<String> {
    if matches!(
        disposition,
        "backendHoldbackSignedOff" | "oracleOnlySignedOff" | "deprecatedLegacySignedOff"
    ) {
        return Vec::new();
    }
    record
        .required_descriptor_ids
        .iter()
        .map(|descriptor| format!("playerAdapter:{descriptor}"))
        .collect()
}

fn required_runtime_features(
    record: &crate::PlayerMigrationMappingRecord,
    disposition: &str,
) -> Vec<String> {
    let mut features = Vec::new();
    if disposition == "graphRuntimeResolved" || disposition == "graphRuntimeBacklog" {
        if record
            .candidate_blockers
            .iter()
            .any(|blocker| blocker == "valueSourceOrSignalDecision")
        {
            features.push("valueSourceOrSignalDisposition".to_string());
        } else {
            features.push("runtimeDataModelDisposition".to_string());
        }
    }
    if disposition == "sceneRuntimeResolved" || disposition == "sceneRuntimeBacklog" {
        features.push("sceneLocalRuntimeDisposition".to_string());
    }
    if disposition == "backendHoldbackSignedOff" {
        features.push("futurePlayerRenderBackendEvidence".to_string());
    }
    features
}

fn holdback_reason_for(record: &crate::PlayerMigrationMappingRecord, disposition: &str) -> String {
    match disposition {
        "backendHoldbackSignedOff" => {
            if !record.missing_descriptor_ids.is_empty() {
                format!(
                    "backend/compositor fidelity or descriptor semantics required for {}",
                    record.missing_descriptor_ids.join(", ")
                )
            } else {
                "backend/compositor fidelity required before visual parity claims".to_string()
            }
        }
        "guiHumanReviewHoldbackSignedOff" => {
            "requires deterministic GUI human-review evidence, not schema changes".to_string()
        }
        "oracleOnlySignedOff" => {
            "legacy path is retained as offline oracle evidence only".to_string()
        }
        "duplicateVariantSignedOff" => {
            "covered by representative canonical v3.1 fixture for the same descriptor family"
                .to_string()
        }
        "deprecatedLegacySignedOff" => {
            "deprecated legacy recipe is excluded from canonical source corpus".to_string()
        }
        "graphRuntimeResolved" => {
            "generic graph backlog resolved into exact runtime feature disposition".to_string()
        }
        "sceneRuntimeResolved" => {
            "generic scene backlog resolved into exact scene-runtime disposition".to_string()
        }
        "sourceBacklogResolved" => {
            "true source backlog resolved to offline/source-material disposition".to_string()
        }
        "descriptorBacklogResolved" => {
            "descriptor backlog resolved to exact descriptor path disposition".to_string()
        }
        _ => String::new(),
    }
}

fn readiness_notes(record: &crate::PlayerMigrationMappingRecord) -> Vec<String> {
    record
        .notes
        .iter()
        .map(|note| normalize_note(note))
        .collect()
}

fn normalize_note(note: &str) -> String {
    let mut text = note.to_string();
    for (source, content) in [
        ("source.typewriterText", "content.typewriter"),
        ("source.splitFlapText", "content.splitFlap"),
        ("source.odometer", "content.odometer"),
        ("source.marqueeText", "content.marquee"),
    ] {
        text = text.replace(source, content);
    }
    text
}

fn summarize_readiness_records(
    records: &[PlayerImplementationReadinessRecord],
) -> PlayerImplementationReadinessSummary {
    let mut disposition_counts = BTreeMap::new();
    let mut implementation_blocking_counts = BTreeMap::new();
    for record in records {
        *disposition_counts
            .entry(record.disposition.clone())
            .or_insert(0) += 1;
        if record.implementation_blocking {
            *implementation_blocking_counts
                .entry(record.blocking_kind.clone())
                .or_insert(0) += 1;
        }
    }
    PlayerImplementationReadinessSummary {
        records: records.len(),
        canonical_exists: count_disposition(records, "canonicalExists"),
        candidate_ready: count_disposition(records, "candidateReady"),
        explicit_owner_decision_needed: count_disposition(records, "explicitOwnerDecisionNeeded"),
        implementation_blocking: records
            .iter()
            .filter(|record| record.implementation_blocking)
            .count(),
        disposition_counts,
        implementation_blocking_counts,
    }
}

fn count_disposition(records: &[PlayerImplementationReadinessRecord], disposition: &str) -> usize {
    records
        .iter()
        .filter(|record| record.disposition == disposition)
        .count()
}

fn build_priority_queues(
    records: &[PlayerImplementationReadinessRecord],
) -> Vec<PlayerImplementationReadinessQueue> {
    let mut grouped: BTreeMap<String, Vec<&PlayerImplementationReadinessRecord>> = BTreeMap::new();
    for record in records
        .iter()
        .filter(|record| record.implementation_blocking)
    {
        grouped
            .entry(record.disposition.clone())
            .or_default()
            .push(record);
    }
    grouped
        .into_iter()
        .map(
            |(disposition, records)| PlayerImplementationReadinessQueue {
                recommended_next_action: recommended_next_action_for(&disposition).to_string(),
                count: records.len(),
                representative_legacy_paths: records
                    .into_iter()
                    .take(5)
                    .map(|record| record.legacy_path.clone())
                    .collect(),
                disposition,
            },
        )
        .collect()
}

fn build_holdbacks(
    records: &[PlayerImplementationReadinessRecord],
) -> Vec<PlayerImplementationReadinessHoldback> {
    let mut grouped: BTreeMap<String, Vec<&PlayerImplementationReadinessRecord>> = BTreeMap::new();
    for record in records.iter().filter(|record| record.holdback_signed_off) {
        grouped
            .entry(record.disposition.clone())
            .or_default()
            .push(record);
    }
    grouped
        .into_iter()
        .map(
            |(disposition, records)| PlayerImplementationReadinessHoldback {
                count: records.len(),
                signed_off: true,
                representative_legacy_paths: records
                    .into_iter()
                    .take(5)
                    .map(|record| record.legacy_path.clone())
                    .collect(),
                disposition,
            },
        )
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_implementation_readiness_report.rs</FILE> - <DESC>Build implementation-readiness reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
