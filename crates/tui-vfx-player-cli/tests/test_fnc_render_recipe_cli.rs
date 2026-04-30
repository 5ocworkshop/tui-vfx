// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>VERSION: 0.13.0</VERS>
// <WCTX>K2.12 schema lock: lock offender-ledger output and source.text fixture counts.</WCTX>
// <CLOG>0.13.0: MINOR — add offender-ledger regressions and update recursive fixture count.
// 0.12.0: MINOR — add schema-readiness CLI regression coverage.</CLOG>

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::{Command, Output},
};

const RECURSIVE_DEBUG_FIXTURE_COUNT: i64 = 88;

#[test]
fn test_fnc_cli_renders_single_recipe_frame_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-recipe"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "render-recipe player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.frame.1");
    assert_eq!(report["status"], "rendered");
    assert!(report["nonEmptyCells"].as_u64().expect("cell count") > 0);
}

#[test]
fn test_fnc_cli_renders_single_recipe_render_ir_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-ir"),
            str_arg("--recipe"),
            recipe_path("complex/graph_parallel_overlap_conflict_snapshot.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "render-ir player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.renderIr.1");
    assert_eq!(report["status"], "rendered");
    assert!(
        !report["styledCells"]
            .as_array()
            .expect("styled cells")
            .is_empty()
    );
    assert!(
        !report["provenance"]
            .as_array()
            .expect("provenance")
            .is_empty()
    );
    assert!(
        report["warnings"]
            .as_array()
            .expect("warnings")
            .iter()
            .any(|warning| warning["code"] == "parallelGraphValueConflict")
    );
}

#[test]
fn test_fnc_cli_renders_recursive_smoke_report_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-recipe"),
            str_arg("--json"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
        ],
        "recursive render-recipe player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.run.1");
    assert_eq!(report["summary"]["total"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["errors"], 0);
}

#[test]
fn test_fnc_cli_inventories_single_baseline_recipe_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("baseline.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.inventory.1");
    assert_eq!(report["summary"]["totalRecipes"], 1);
    assert_eq!(report["summary"]["rendered"], 1);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["recipes"][0]["status"], "rendered");
    assert_eq!(
        report["recipes"][0]["effectIds"].as_array().unwrap().len(),
        0
    );
    assert!(
        report["recipes"][0]["sourceIds"]
            .as_array()
            .expect("source ids")
            .iter()
            .any(|source| source == "source.card")
    );
}

#[test]
fn test_fnc_cli_inventories_visible_effect_adapter_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("masks/mask_wipe.json"),
    ]);

    assert_eq!(report["recipes"][0]["status"], "rendered");
    assert!(
        report["recipes"][0]["effectIds"]
            .as_array()
            .expect("effect ids")
            .iter()
            .any(|effect| effect == "mask.wipe")
    );
    let effect = find_effect(&report, "mask.wipe");
    assert_eq!(effect["descriptorCovered"], true);
    assert_eq!(effect["representedByRecipes"], true);
    assert_eq!(effect["adapterStatus"], "visible");
}

#[test]
fn test_fnc_cli_inventories_styled_effect_adapter_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["recipes"][0]["status"], "rendered");
    assert!(
        report["recipes"][0]["descriptorCoveredEffectIds"]
            .as_array()
            .expect("descriptor covered")
            .iter()
            .any(|effect| effect == "shader.linearGradient")
    );
    assert!(
        report["recipes"][0]["missingDescriptorEffectIds"]
            .as_array()
            .expect("missing descriptors")
            .is_empty()
    );
    assert!(
        report["recipes"][0]["unsupportedEffectIds"]
            .as_array()
            .expect("unsupported effects")
            .is_empty()
    );
    let effect = find_effect(&report, "shader.linearGradient");
    assert_eq!(effect["adapterStatus"], "styled");
}

#[test]
fn test_fnc_cli_inventories_recursive_debug_fixture_gate_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.inventory.1");
    assert_eq!(
        report["summary"]["totalRecipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(report["summary"]["descriptorEffectIds"], 45);
    assert_eq!(report["summary"]["representedEffectIds"], 45);
    assert_eq!(report["summary"]["unrepresentedEffectIds"], 0);
    assert_eq!(report["summary"]["unsupportedEffectIds"], 0);
}

#[test]
fn test_fnc_cli_reports_primitive_adapter_gap_json() {
    let report = primitive_adapter_gap_report();

    assert_eq!(report["schemaVersion"], "v3.1.player.primitiveAdapterGap.1");
    assert_eq!(report["summary"]["totalEffects"], 45);
    assert_eq!(report["summary"]["rendered"], 45);
    assert_eq!(report["summary"]["stillUnsupported"], 0);
    assert_eq!(report["summary"]["blockedByStyledCellSubstrate"], 0);
    assert_eq!(report["summary"]["blockedBySemanticDecision"], 0);

    assert_gap_entry(&report, "mask.dissolve", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.blinds", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.radial", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.iris", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.diamond", "rendered", "textGrid");
    assert_gap_entry(&report, "sampler.ripple", "rendered", "textGrid");
    assert_gap_entry(&report, "shader.borderSweep", "rendered", "styledCell");
    assert_gap_entry(&report, "shader.linearGradient", "rendered", "styledCell");
    assert_gap_entry(&report, "style.baseStyleOverride", "rendered", "styledCell");
    assert_gap_entry(&report, "style.colorFade", "rendered", "styledCell");
}

#[test]
fn test_fnc_cli_reports_source_text_descriptor_pilot_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    let source = find_source(&report, "source.text");
    assert_eq!(source["descriptorCovered"], true);
    assert_eq!(source["representedByRecipes"], true);
    assert_eq!(source["adapterStatus"], "visible");
}

#[test]
fn test_fnc_cli_reports_migration_gap_summary_json() {
    let report = migration_gap_report();

    assert_eq!(report["schemaVersion"], "v3.1.player.migrationGap.1");
    assert!(
        !report["descriptorPacks"]
            .as_array()
            .expect("descriptor packs")
            .is_empty()
    );
    assert_eq!(report["summary"]["legacyRecipes"], 603);
    assert_eq!(
        report["summary"]["v31Recipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["representedFamilies"], 12);
    assert_eq!(report["summary"]["unrepresentedFamilies"], 8);
    assert_eq!(report["summary"]["partiallyRepresentedFamilies"], 10);
    assert_eq!(report["recommendedQueue"][0]["family"], "complex");
}

#[test]
fn test_fnc_cli_rejects_migration_gap_recipe_paths() {
    let output = run_player_cli(
        vec![
            str_arg("migration-gap"),
            str_arg("accidental-path.json"),
            str_arg("--legacy-root"),
            legacy_debug_recipe_root_path(),
            str_arg("--v31-root"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
        ],
        "migration gap player cli",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("migration-gap does not accept recipe paths"));
}

#[test]
fn test_fnc_cli_reports_migration_gap_family_status_json() {
    let report = migration_gap_report();
    let filters = find_family(&report, "filters");
    let content = find_family(&report, "content");
    let complex = find_family(&report, "complex");

    assert_eq!(filters["legacyCount"], 98);
    assert_eq!(filters["v31Count"], 13);
    assert_eq!(filters["coverage"], "partial");
    assert_eq!(filters["status"], "adapterExpansionReady");
    assert!(
        filters["knownV31EffectIds"]
            .as_array()
            .expect("known effects")
            .iter()
            .any(|effect| effect == "filter.dim")
    );
    assert_eq!(content["coverage"], "partial");
    assert_eq!(content["status"], "notYetClassified");
    assert_eq!(complex["coverage"], "partial");
    assert_eq!(complex["status"], "notYetClassified");
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_masks_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--family"),
        str_arg("masks"),
    ]);

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.migrationMappingBatch.1"
    );
    assert_eq!(report["summary"]["families"], 1);
    assert!(report["summary"]["records"].as_u64().expect("records") > 0);
    assert_eq!(report["summary"]["candidateReady"], 0);
    assert_eq!(report["summary"]["duplicateOrVariant"], 3);
    assert_eq!(report["families"][0], "masks");

    let blinds = find_mapping_record(&report, "masks/mask_blinds.json");
    assert_eq!(blinds["legacyFamily"], "masks");
    assert_eq!(blinds["status"], "canonicalExists");
    assert_eq!(blinds["recommendation"], "skipAsDuplicateVariant");
    assert_eq!(blinds["requiredDescriptorIds"][0], "mask.blinds");
    assert!(
        blinds["missingDescriptorIds"]
            .as_array()
            .expect("missing descriptor ids")
            .is_empty()
    );
    assert_eq!(blinds["requiredInputFields"][0], "count");
    assert_eq!(blinds["requiredInputFields"][1], "orientation");

    let cellular = find_mapping_record(&report, "masks/mask_cellular.json");
    assert_eq!(cellular["status"], "descriptorDecisionNeeded");
    assert_eq!(cellular["recommendation"], "deferForDescriptorDecision");

    let radial_square = find_mapping_record(&report, "masks/mask_radial_square.json");
    assert_eq!(radial_square["status"], "duplicateOrVariant");
    assert_eq!(radial_square["recommendation"], "skipAsDuplicateVariant");
}

#[test]
fn test_fnc_cli_reports_schema_readiness_recursive_json() {
    let report = schema_readiness_report(vec![str_arg("schema-readiness"), str_arg("--recursive")]);

    assert_eq!(report["schemaVersion"], "v3.1.player.schemaReadiness.1");
    assert_eq!(report["summary"]["totalLegacyRecords"], 603);
    assert_eq!(report["summary"]["schemaBlockedRecords"], 91);
    assert_eq!(report["summary"]["sourceBlockedRecords"], 61);
    assert_eq!(report["summary"]["fieldCoverageBlockedRecords"], 0);
    assert_eq!(report["summary"]["unknownRecords"], 0);
    assert_eq!(report["summary"]["canDeclareSchemaReady"], true);
    assert_eq!(report["summary"]["unresolvedSchemaBlockers"], 0);
    assert_eq!(report["summary"]["remainingOwnerDecisionCount"], 0);

    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .any(|blocker| blocker["blockerKind"] == "motionTimingSemantics"
                && blocker["statusFromMigrationMapping"] == "schemaDecisionNeeded")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .any(|blocker| blocker["blockerKind"] == "sourceDescriptor"
                && blocker["statusFromMigrationMapping"] == "sourceDecisionNeeded")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "fieldCoverage")
    );
}

#[test]
fn test_fnc_cli_maps_schema_readiness_blockers_json() {
    let report = schema_readiness_report(vec![str_arg("schema-readiness"), str_arg("--recursive")]);

    let value_source = find_readiness_blocker(
        &report,
        "valueSourceSemantics",
        "filters/filter_dim_sample_surface_radius.json",
    );
    assert_eq!(
        value_source["statusFromMigrationMapping"],
        "schemaDecisionNeeded"
    );
    assert_eq!(value_source["isSchemaReadinessBlocking"], false);

    let source = find_readiness_blocker(
        &report,
        "sourceDescriptor",
        "complex/complex_cellular_faultline.json",
    );
    assert_eq!(source["statusFromMigrationMapping"], "ownerAuditNeeded");
    assert_eq!(source["isSchemaReadinessBlocking"], false);
    assert!(
        source["notes"]
            .as_array()
            .expect("notes")
            .iter()
            .any(|note| note.as_str().unwrap_or("").contains("source.scramble"))
    );

    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "fieldCoverage")
    );
}

#[test]
fn test_fnc_cli_reports_schema_readiness_offenders_json() {
    let report = schema_readiness_report(vec![
        str_arg("schema-readiness"),
        str_arg("--recursive"),
        str_arg("--include-offenders"),
    ]);

    let offenders = report["offenders"].as_array().expect("offenders");
    assert_eq!(offenders.len(), 352);
    assert_eq!(report["summary"]["unresolvedSchemaBlockers"], 0);
    assert_eq!(report["summary"]["explicitOwnerDecisionNeeded"], 0);
    assert_eq!(report["summary"]["remainingOwnerDecisionCount"], 0);
    assert_eq!(report["summary"]["canDeclareSchemaReady"], true);
    assert_eq!(
        report["summary"]["dispositionCounts"]["acceptedSchema"],
        169
    );
    assert_eq!(
        report["summary"]["dispositionCounts"]["descriptorBacklog"],
        219
    );
    assert_eq!(
        offender_kind_counts(&report),
        BTreeMap::from([
            ("backendRenderer", 15),
            ("bindingSemantics", 22),
            ("descriptorPack", 151),
            ("guiHumanReview", 2),
            ("lifecycleSemantics", 1),
            ("motionTimingSemantics", 34),
            ("oracleOnly", 2),
            ("sceneSemantics", 24),
            ("sourceDescriptor", 68),
            ("valueSourceSemantics", 33),
        ])
    );
    assert!(
        offenders
            .iter()
            .all(|offender| offender["blockerKind"] != "ownerAudit")
    );
    assert!(
        offenders
            .iter()
            .all(|offender| offender["blockerKind"] != "unknown")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "ownerAudit")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "unknown")
    );
    assert!(offenders.iter().all(|offender| {
        offender
            .as_object()
            .expect("offender")
            .contains_key("disposition")
    }));
    assert!(offenders.iter().all(|offender| {
        offender
            .as_object()
            .expect("offender")
            .contains_key("schemaBlocking")
    }));

    let source = find_readiness_offender(&report, "complex/complex_cellular_faultline.json");
    assert_eq!(source["blockerKind"], "sourceDescriptor");
    assert_eq!(source["disposition"], "descriptorBacklog");
    assert_eq!(source["recommendedDisposition"], "descriptorBacklog");
    assert_eq!(source["schemaBlocking"], false);
    assert_eq!(source["schemaReadinessBlocking"], false);
    assert_json_array_contains(&source["requiredSourceIds"], "source.scramble");

    assert!(
        offenders
            .iter()
            .all(|offender| offender["blockerKind"] != "fieldCoverage")
    );

    let value_source =
        find_readiness_offender(&report, "complex/complex_field_hint_displace_shade.json");
    assert_eq!(value_source["blockerKind"], "valueSourceSemantics");
    assert_eq!(value_source["disposition"], "acceptedSchema");
    assert_eq!(value_source["schemaReadinessBlocking"], false);

    let command_capture =
        find_readiness_offender(&report, "fixtures/command_capture_chain.capture.json");
    assert_eq!(command_capture["disposition"], "oracleOnly");
    assert_eq!(command_capture["recommendedDisposition"], "oracleOnly");
    assert_eq!(command_capture["schemaReadinessBlocking"], false);
}

#[test]
fn test_fnc_cli_classifies_complex_and_style_offenders_json() {
    let report = schema_readiness_report(vec![
        str_arg("schema-readiness"),
        str_arg("--recursive"),
        str_arg("--include-offenders"),
    ]);

    let complex = find_readiness_offender(&report, "complex/complex_full_pipeline.json");
    assert_eq!(complex["blockerKind"], "sourceDescriptor");
    assert_ne!(
        complex["recommendedDisposition"],
        "requiresArchitectDecision"
    );
    assert!(
        complex["holdbackReason"]
            .as_str()
            .unwrap_or("")
            .contains("composition")
    );

    let sequence =
        find_readiness_offender(&report, "complex/complex_nested_parallel_sequences.json");
    assert_eq!(sequence["blockerKind"], "sceneSemantics");
    assert_eq!(sequence["disposition"], "acceptedSchema");
    assert_eq!(sequence["schemaReadinessBlocking"], false);

    let visual_conflict = find_readiness_offender(
        &report,
        "complex/v3_scheduler_overlap_conflict_mixed_family.json",
    );
    assert_eq!(visual_conflict["blockerKind"], "guiHumanReview");
    assert_eq!(visual_conflict["disposition"], "guiHumanReviewHoldback");
    assert_eq!(visual_conflict["holdbackSignedOff"], true);

    let backend = find_readiness_offender(
        &report,
        "complex/complex_shadow_mask_sampler_shader_filter_native_mix.json",
    );
    assert_eq!(backend["blockerKind"], "backendRenderer");
    assert_eq!(backend["disposition"], "backendHoldback");
    assert_eq!(backend["holdbackSignedOff"], true);

    for style_path in [
        "styles/style_modulo_horizontal_every_third_row.json",
        "styles/style_modulo_vertical_every_fourth_column_offset.json",
        "styles/style_non_empty_scope.json",
        "styles/style_outer_scope_band.json",
        "styles/style_predicate_interior.json",
    ] {
        assert!(
            report["offenders"]
                .as_array()
                .expect("offenders")
                .iter()
                .all(|entry| entry["legacyPath"] != style_path),
            "style scope fixture should now be canonical rather than an offender: {style_path}"
        );
    }
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_recursive_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--recursive"),
    ]);

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.migrationMappingBatch.1"
    );
    assert!(report["summary"]["families"].as_u64().expect("families") > 1);
    assert!(
        report["recommendationQueue"]
            .as_array()
            .expect("recommendation queue")
            .iter()
            .any(|item| item["legacyFamily"] == "masks")
    );

    let families = report["families"].as_array().expect("families");
    for family in ["complex", "content", "filters", "masks", "samplers"] {
        assert!(
            families.iter().any(|entry| entry == family),
            "missing family {family}"
        );
    }
    assert_eq!(report["summary"]["records"], 603);
    assert_eq!(report["summary"]["candidateReady"], 0);
    assert_eq!(report["summary"]["schemaDecisionNeeded"], 91);
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_filter_records_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--family"),
        str_arg("filters"),
    ]);

    assert_eq!(report["families"][0], "filters");
    let dim = find_mapping_record(&report, "filters/filter_dim.json");
    assert_eq!(dim["legacyFamily"], "filters");
    assert_eq!(dim["requiredDescriptorIds"][0], "filter.dim");
    assert_eq!(dim["status"], "canonicalExists");
    assert_eq!(dim["recommendation"], "skipAsDuplicateVariant");

    let crt = find_mapping_record(&report, "filters/filter_crt.json");
    assert_eq!(crt["status"], "canonicalExists");
    assert_eq!(crt["recommendation"], "skipAsDuplicateVariant");

    let value_source_record =
        find_mapping_record(&report, "filters/filter_dim_sample_surface_radius.json");
    assert_ne!(value_source_record["status"], "candidateReady");
    assert_eq!(value_source_record["status"], "schemaDecisionNeeded");
    assert_eq!(
        value_source_record["recommendation"],
        "deferForSchemaDecision"
    );
    assert!(
        value_source_record["unsupportedInputFields"]
            .as_array()
            .expect("unsupported input fields")
            .iter()
            .any(|field| field == "factor")
    );
    assert!(
        value_source_record["candidateBlockers"]
            .as_array()
            .expect("candidate blockers")
            .iter()
            .any(|blocker| blocker == "valueSourceOrSignalDecision")
    );
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_content_source_decisions_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--family"),
        str_arg("content"),
    ]);

    let marquee = find_mapping_record(&report, "content/content_marquee.json");
    assert_eq!(marquee["legacyFamily"], "content");
    assert_eq!(marquee["status"], "canonicalExists");
    assert_eq!(marquee["recommendation"], "skipAsDuplicateVariant");
    assert!(
        marquee["requiredSourceIds"]
            .as_array()
            .expect("required sources")
            .iter()
            .any(|source| source == "source.marqueeText")
    );

    let deprecated = find_mapping_record(&report, "content/_DEPRECATED_content_marquee.json");
    assert_ne!(deprecated["status"], "candidateReady");
    assert_eq!(deprecated["status"], "ownerAuditNeeded");
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_keeps_legacy_root_read_only() {
    let before = legacy_recipe_file_snapshot();

    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--recursive"),
    ]);

    assert_eq!(report["summary"]["records"], 603);
    assert_eq!(before, legacy_recipe_file_snapshot());
}

#[test]
fn test_fnc_cli_has_corpus_mapping_backlog_docs_checked_in() {
    for relative in [
        "docs/new_kernel/K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md",
        "docs/new_kernel/K2_10_MIGRATION_BACKLOG_BOARD.md",
        "docs/new_kernel/K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md",
        "docs/new_kernel/PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md",
    ] {
        assert!(
            workspace_root().join(relative).is_file(),
            "missing checked-in doc {relative}"
        );
    }
}

#[test]
fn test_fnc_cli_renders_single_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("baseline.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.visualFrameReport.1");
    assert_eq!(report["summary"]["total"], 1);
    assert_eq!(report["summary"]["rendered"], 1);
    assert_eq!(report["frames"][0]["status"], "rendered");
    assert_eq!(report["frames"][0]["sampleT"], 1.0);
    assert!(report["frames"][0]["loopT"].is_null());
    assert_eq!(report["frames"][0]["absoluteTimeMs"], 0);
    assert_eq!(report["frames"][0]["substrate"], "textGrid");
    assert_eq!(report["frames"][0]["cellSource"], "rows");
    assert_eq!(report["frames"][0]["styleKnown"], false);
    assert!(
        !report["frames"][0]["rows"]
            .as_array()
            .expect("rows")
            .is_empty()
    );
    assert!(
        !report["frames"][0]["cells"]
            .as_array()
            .expect("cells")
            .is_empty()
    );
    let first_cell = &report["frames"][0]["cells"][0];
    assert_eq!(first_cell["foreground"], "defaultForeground");
    assert_eq!(first_cell["background"], "transparent");
    assert!(
        first_cell["modifiers"]
            .as_array()
            .expect("modifiers")
            .is_empty()
    );
    assert!(first_cell["role"].is_null());
}

#[test]
fn test_fnc_cli_renders_recursive_visual_frame_report_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.visualFrameReport.1");
    assert_eq!(report["summary"]["total"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(
        report["frames"].as_array().expect("frames").len() as i64,
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
}

#[test]
fn test_fnc_cli_renders_styled_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["frames"][0]["status"], "rendered");
    assert!(
        report["frames"][0]["unsupportedEffectIds"]
            .as_array()
            .expect("unsupported effect ids")
            .is_empty()
    );
    assert_eq!(report["frames"][0]["substrate"], "styledCell");
    assert_eq!(report["frames"][0]["cellSource"], "styledCells");
    assert_eq!(report["frames"][0]["styleKnown"], true);
    assert!(
        !report["frames"][0]["rows"]
            .as_array()
            .expect("rows")
            .is_empty()
    );
    assert!(
        report["frames"][0]["cells"]
            .as_array()
            .expect("cells")
            .iter()
            .any(|cell| cell["foreground"] != "defaultForeground"
                || cell["background"] != "transparent"
                || !cell["modifiers"].as_array().expect("modifiers").is_empty()
                || !cell["role"].is_null())
    );
    assert!(
        report["frames"][0]["errors"]
            .as_array()
            .expect("errors")
            .is_empty()
    );
}

#[test]
fn test_fnc_cli_renders_filter_field_handling_with_styled_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("filters/filter_tint.json"),
    ]);

    let frame = &report["frames"][0];
    assert_eq!(frame["status"], "rendered");
    assert_eq!(frame["substrate"], "styledCell");
    assert_eq!(frame["cellSource"], "styledCells");
    assert_eq!(frame["styleKnown"], true);
    assert!(
        frame["cells"]
            .as_array()
            .expect("cells")
            .iter()
            .any(|cell| cell["role"] == "FilterTint")
    );
}

#[test]
fn test_fnc_cli_reports_primitive_field_coverage_for_fixture_corpus_json() {
    let report = primitive_field_coverage_report();

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.primitiveFieldCoverage.1"
    );
    assert_eq!(
        report["summary"]["totalRecipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["usedButUnhandledInputFields"], 0);
    assert_eq!(report["summary"]["missingDescriptorInputFields"], 0);
    assert_eq!(report["summary"]["schemaDecisionNeededFields"], 0);
    assert!(
        report["summary"]["totalPrimitiveInstances"]
            .as_u64()
            .expect("instances")
            > RECURSIVE_DEBUG_FIXTURE_COUNT as u64
    );
}

#[test]
fn test_fnc_cli_reports_fixture_qc_for_fixture_corpus_json() {
    let report = player_cli_json(
        vec![
            str_arg("fixture-qc"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "fixture qc player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.fixtureQcReport.1");
    assert_eq!(
        report["summary"]["totalRecipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(
        report["summary"]["validated"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["validationErrors"], 0);
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["playerErrors"], 0);
    assert_eq!(report["summary"]["fieldCoverageUnhandled"], 0);
    assert_eq!(report["summary"]["adapterGapUnresolved"], 0);
    assert_eq!(report["summary"]["timelineSmokePassed"], true);
    assert_eq!(report["summary"]["diffSmokePassed"], true);
    assert_eq!(report["summary"]["overallStatus"], "pass");
    assert_eq!(
        report["reports"]["render"]["schemaVersion"],
        "v3.1.player.run.1"
    );
    assert_eq!(
        report["reports"]["visualFrame"]["schemaVersion"],
        "v3.1.player.visualFrameReport.1"
    );
    assert_eq!(
        report["reports"]["fieldCoverage"]["schemaVersion"],
        "v3.1.player.primitiveFieldCoverage.1"
    );
    assert_eq!(
        report["reports"]["adapterGap"]["schemaVersion"],
        "v3.1.player.primitiveAdapterGap.1"
    );
}

#[test]
fn test_fnc_cli_fixture_qc_smoke_fields_fail_for_unrendered_recipe_json() {
    let temp_root = std::env::temp_dir().join("tui-vfx-fixture-qc-negative");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create temp fixture root");
    let recipe = unsupported_effect_recipe();
    let recipe_path = temp_root.join("unsupported.json");
    fs::write(
        &recipe_path,
        serde_json::to_string_pretty(&recipe).expect("serialize negative recipe"),
    )
    .expect("write negative recipe");

    let report = player_cli_json(
        vec![
            str_arg("fixture-qc"),
            str_arg("--recursive"),
            temp_root.display().to_string(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "negative fixture qc player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.fixtureQcReport.1");
    assert_eq!(report["summary"]["totalRecipes"], 1);
    assert_eq!(report["summary"]["rendered"], 0);
    assert_eq!(report["summary"]["playerErrors"], 1);
    assert_eq!(report["summary"]["timelineSmokePassed"], false);
    assert_eq!(report["summary"]["diffSmokePassed"], false);
    assert_eq!(report["summary"]["overallStatus"], "fail");
}

#[test]
fn test_fnc_cli_reports_honest_primitive_field_coverage_shape_json() {
    let report = primitive_field_coverage_report();

    assert_eq!(
        report["summary"]["usedInputFields"],
        report["summary"]["handledInputFields"]
    );
    assert_eq!(report["summary"]["declaredButUnusedInputFields"], 57);

    let first_recipe = &report["recipes"].as_array().expect("recipes")[0];
    assert!(
        first_recipe["recipePath"]
            .as_str()
            .expect("recipe path")
            .ends_with(".json")
    );
    assert_eq!(first_recipe["status"], "scanned");
    assert!(
        first_recipe["errors"]
            .as_array()
            .expect("errors")
            .is_empty()
    );

    let instance = first_recipe["primitiveInstances"]
        .as_array()
        .expect("instances")
        .first()
        .expect("primitive instance");
    assert!(instance["kind"] == "source" || instance["kind"] == "effect");
    assert!(
        instance["descriptorId"]
            .as_str()
            .expect("descriptor id")
            .contains('.')
    );
    assert!(
        instance["descriptorInputs"]
            .as_array()
            .expect("descriptor inputs")
            .len()
            >= instance["usedInputs"]
                .as_array()
                .expect("used inputs")
                .len()
    );
    assert_eq!(instance["classification"], "usedAndHandled");
    assert_eq!(instance["recommendation"], "none");
}

#[test]
fn test_fnc_cli_keeps_render_frame_schema_unchanged_after_report_commands() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("baseline.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.visualFrameReport.1");
    assert_eq!(report["frames"][0]["sampleT"], 1.0);
    assert_eq!(report["frames"][0]["absoluteTimeMs"], 0);
    assert!(report["frames"][0]["from"].is_null());
}

#[test]
fn test_fnc_cli_timeline_emits_deterministic_multiple_frames_json() {
    let first = timeline_report();
    let second = timeline_report();

    assert_eq!(first["schemaVersion"], "v3.1.player.frameTimeline.1");
    assert_eq!(first["frames"].as_array().expect("frames").len(), 3);
    assert_eq!(first["frames"][0]["sampleT"], 0.0);
    assert_eq!(first["frames"][2]["sampleT"], 1.0);
    assert_eq!(first["frames"][1]["absoluteTimeMs"], 500);
    assert_eq!(
        first["frames"][0]["renderHash"],
        second["frames"][0]["renderHash"]
    );
}

#[test]
fn test_fnc_cli_frame_diff_reports_changed_cells_when_sample_t_differs_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-frame-diff"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--from-sample-t"),
            str_arg("0.0"),
            str_arg("--to-sample-t"),
            str_arg("1.0"),
            str_arg("--json"),
        ],
        "render-frame-diff player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.frameDiff.1");
    let report_object = report.as_object().expect("diff report object");
    assert!(report_object.contains_key("from"));
    assert!(report_object.contains_key("to"));
    assert!(!report_object.contains_key("fromFrame"));
    assert!(!report_object.contains_key("toFrame"));
    assert_eq!(report["hashChanged"], true);
    assert!(report["changedCellCount"].as_u64().expect("changed count") > 0);
    assert!(
        !report["changedCells"]
            .as_array()
            .expect("changed cells")
            .is_empty()
    );
    assert_ne!(report["nonEmptyDelta"], 0);
}

#[test]
fn test_fnc_cli_frame_diff_reports_styled_cell_changes_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-frame-diff"),
            str_arg("--recipe"),
            recipe_path("shaders/compositions/shader_border_sweep.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--from-sample-t"),
            str_arg("0.0"),
            str_arg("--to-sample-t"),
            str_arg("0.5"),
            str_arg("--json"),
        ],
        "styled render-frame-diff player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.frameDiff.1");
    assert_eq!(report["from"]["substrate"], "styledCell");
    assert_eq!(report["to"]["substrate"], "styledCell");
    assert_eq!(report["hashChanged"], true);
    assert!(report["changedCellCount"].as_u64().expect("changed count") > 0);
    assert!(
        report["changedCells"]
            .as_array()
            .expect("changed cells")
            .iter()
            .any(
                |cell| cell["from"].as_str().expect("from cell").contains("fg=")
                    || cell["to"].as_str().expect("to cell").contains("fg=")
            )
    );
}

fn inventory_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "inventory player cli")
}

fn render_frame_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "render-frame player cli")
}

fn primitive_adapter_gap_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("primitive-adapter-gap"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "primitive adapter gap player cli",
    )
}

fn primitive_field_coverage_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("primitive-field-coverage"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "primitive field coverage player cli",
    )
}

fn timeline_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("render-timeline"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--frames"),
            str_arg("3"),
            str_arg("--json"),
        ],
        "render-timeline player cli",
    )
}

fn migration_gap_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("migration-gap"),
            str_arg("--legacy-root"),
            legacy_debug_recipe_root_path(),
            str_arg("--v31-root"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "migration gap player cli",
    )
}

fn migration_mapping_batch_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--legacy-root"),
        legacy_debug_recipe_root_path(),
        str_arg("--v31-root"),
        debug_recipe_root_path(),
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "migration mapping batch player cli")
}

fn schema_readiness_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--legacy-root"),
        legacy_debug_recipe_root_path(),
        str_arg("--v31-root"),
        debug_recipe_root_path(),
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "schema readiness player cli")
}

fn player_cli_json(args: Vec<String>, context: &str) -> serde_json::Value {
    let output = run_player_cli(args, context);

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    serde_json::from_slice(&output.stdout).expect("json report")
}

fn run_player_cli(args: Vec<String>, context: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {context}: {error}"))
}

fn assert_gap_entry(
    report: &serde_json::Value,
    effect_id: &str,
    expected_outcome: &str,
    expected_adapter_class: &str,
) {
    let entry = find_gap_entry(report, effect_id);

    assert_eq!(entry["outcome"], expected_outcome);
    assert_eq!(entry["adapterClass"], expected_adapter_class);
}

fn find_source<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .find(|source| source["id"] == id)
        .expect("source entry")
}

fn find_effect<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .find(|effect| effect["id"] == id)
        .expect("effect entry")
}

fn find_gap_entry<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .find(|effect| effect["effectId"] == id)
        .expect("adapter gap entry")
}

fn find_family<'a>(report: &'a serde_json::Value, family: &str) -> &'a serde_json::Value {
    report["families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|entry| entry["family"] == family)
        .expect("family entry")
}

fn find_readiness_blocker<'a>(
    report: &'a serde_json::Value,
    blocker_kind: &str,
    legacy_path: &str,
) -> &'a serde_json::Value {
    report["blockers"]
        .as_array()
        .expect("readiness blockers")
        .iter()
        .find(|entry| {
            entry["blockerKind"] == blocker_kind
                && entry["representativeLegacyPaths"]
                    .as_array()
                    .expect("representative paths")
                    .iter()
                    .any(|path| path == legacy_path)
        })
        .expect("schema readiness blocker")
}

fn find_readiness_offender<'a>(
    report: &'a serde_json::Value,
    legacy_path: &str,
) -> &'a serde_json::Value {
    report["offenders"]
        .as_array()
        .expect("readiness offenders")
        .iter()
        .find(|entry| entry["legacyPath"] == legacy_path)
        .expect("schema readiness offender")
}

fn offender_kind_counts(report: &serde_json::Value) -> BTreeMap<&str, usize> {
    let mut counts = BTreeMap::new();
    for offender in report["offenders"].as_array().expect("readiness offenders") {
        let kind = offender["blockerKind"].as_str().expect("blocker kind");
        *counts.entry(kind).or_insert(0) += 1;
    }
    counts
}

fn assert_json_array_contains(values: &serde_json::Value, expected: &str) {
    assert!(
        values
            .as_array()
            .expect("json array")
            .iter()
            .any(|value| value == expected),
        "missing {expected} in {values:?}"
    );
}

fn find_mapping_record<'a>(
    report: &'a serde_json::Value,
    legacy_path: &str,
) -> &'a serde_json::Value {
    report["records"]
        .as_array()
        .expect("mapping records")
        .iter()
        .find(|entry| entry["legacyPath"] == legacy_path)
        .expect("mapping record")
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn recipe_path(relative: &str) -> String {
    debug_recipe_root().join(relative).display().to_string()
}

fn debug_recipe_root_path() -> String {
    debug_recipe_root().display().to_string()
}

fn legacy_debug_recipe_root_path() -> String {
    recipe_repo_root()
        .join("recipes/debug_recipes")
        .display()
        .to_string()
}

fn descriptor_pack_path() -> String {
    workspace_root()
        .join("descriptors/v3.1/packs/primitive.json")
        .display()
        .to_string()
}

fn debug_recipe_root() -> PathBuf {
    recipe_repo_root().join("recipes/v3.1/debug_recipes")
}

fn recipe_repo_root() -> PathBuf {
    if let Ok(path) = std::env::var("RECIPE_REPO") {
        return PathBuf::from(path);
    }

    workspace_root()
        .parent()
        .expect("workspace parent")
        .join("tui-vfx-recipes")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn str_arg(value: &str) -> String {
    value.to_owned()
}

fn legacy_recipe_file_snapshot() -> Vec<(String, u64, std::time::SystemTime)> {
    let root = recipe_repo_root().join("recipes/debug_recipes");
    let mut snapshot = Vec::new();
    collect_legacy_recipe_file_snapshot(&root, &root, &mut snapshot);
    snapshot.sort_by(|left, right| left.0.cmp(&right.0));
    snapshot
}

fn collect_legacy_recipe_file_snapshot(
    root: &std::path::Path,
    current: &std::path::Path,
    snapshot: &mut Vec<(String, u64, std::time::SystemTime)>,
) {
    for entry in fs::read_dir(current).expect("read legacy recipe dir") {
        let path = entry.expect("read legacy recipe entry").path();
        if path.is_dir() {
            collect_legacy_recipe_file_snapshot(root, &path, snapshot);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            let metadata = fs::metadata(&path).expect("legacy recipe metadata");
            snapshot.push((
                path.strip_prefix(root)
                    .expect("legacy relative path")
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/"),
                metadata.len(),
                metadata.modified().expect("legacy recipe mtime"),
            ));
        }
    }
}

fn unsupported_effect_recipe() -> serde_json::Value {
    let text = fs::read_to_string(debug_recipe_root().join("baseline.json"))
        .expect("read baseline fixture");
    let mut recipe: serde_json::Value =
        serde_json::from_str(&text).expect("baseline fixture parses");
    recipe["graph"]["nodes"]["missingAdapter"] = serde_json::json!({
        "id": "missingAdapter",
        "effect": "effect.notInPack",
        "inputs": {},
        "outputs": {},
        "scope": { "kind": "all" },
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    });
    recipe["graph"]["order"]
        .as_array_mut()
        .expect("order array")
        .push(serde_json::Value::String("missingAdapter".to_string()));
    recipe
}

// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>END OF VERSION: 0.12.0</VERS>
