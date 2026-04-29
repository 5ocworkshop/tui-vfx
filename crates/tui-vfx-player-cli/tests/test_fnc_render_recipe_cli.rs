// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Primitive adapter work: keep CLI fixtures portable and assert adapter-gap outcomes.</WCTX>
// <CLOG>0.7.0: MINOR — add primitive adapter gap regression coverage and project-derived recipe paths.
// 0.6.0: PATCH — assert loop/provenance/style-placeholder fields.
// 0.5.0: MINOR — add render-frame single, recursive, and unsupported tests.</CLOG>

use std::{
    path::PathBuf,
    process::{Command, Output},
};

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
    assert_eq!(report["summary"]["total"], 16);
    assert_eq!(report["summary"]["rendered"], 12);
    assert_eq!(report["summary"]["unsupported"], 4);
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
fn test_fnc_cli_inventories_unsupported_effect_adapter_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["recipes"][0]["status"], "unsupported");
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
            .iter()
            .any(|effect| effect == "shader.linearGradient")
    );
    let effect = find_effect(&report, "shader.linearGradient");
    assert_eq!(effect["adapterStatus"], "unsupported");
}

#[test]
fn test_fnc_cli_inventories_recursive_debug_fixture_gate_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.inventory.1");
    assert_eq!(report["summary"]["totalRecipes"], 16);
    assert_eq!(report["summary"]["rendered"], 12);
    assert_eq!(report["summary"]["unsupported"], 4);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(report["summary"]["descriptorEffectIds"], 14);
    assert_eq!(report["summary"]["representedEffectIds"], 14);
    assert_eq!(report["summary"]["unrepresentedEffectIds"], 0);
    assert_eq!(report["summary"]["unsupportedEffectIds"], 4);
}

#[test]
fn test_fnc_cli_reports_primitive_adapter_gap_json() {
    let report = primitive_adapter_gap_report();

    assert_eq!(report["schemaVersion"], "v3.1.player.primitiveAdapterGap.1");
    assert_eq!(report["summary"]["totalEffects"], 14);
    assert_eq!(report["summary"]["rendered"], 10);
    assert_eq!(report["summary"]["stillUnsupported"], 0);
    assert_eq!(report["summary"]["blockedByStyledCellSubstrate"], 4);
    assert_eq!(report["summary"]["blockedBySemanticDecision"], 0);

    assert_gap_entry(&report, "mask.dissolve", "rendered", "textGrid");
    assert_gap_entry(&report, "sampler.ripple", "rendered", "textGrid");
    assert_gap_entry(
        &report,
        "shader.borderSweep",
        "blockedByStyledCellSubstrate",
        "styledCell",
    );
    assert_gap_entry(
        &report,
        "shader.linearGradient",
        "blockedByStyledCellSubstrate",
        "styledCell",
    );
    assert_gap_entry(
        &report,
        "style.baseStyleOverride",
        "blockedByStyledCellSubstrate",
        "styledCell",
    );
    assert_gap_entry(
        &report,
        "style.colorFade",
        "blockedByStyledCellSubstrate",
        "styledCell",
    );
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
    assert_eq!(report["summary"]["v31Recipes"], 16);
    assert_eq!(report["summary"]["representedFamilies"], 8);
    assert_eq!(report["summary"]["unrepresentedFamilies"], 11);
    assert_eq!(report["summary"]["partiallyRepresentedFamilies"], 7);
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
    assert_eq!(filters["v31Count"], 4);
    assert_eq!(filters["coverage"], "partial");
    assert_eq!(filters["status"], "adapterExpansionReady");
    assert!(
        filters["knownV31EffectIds"]
            .as_array()
            .expect("known effects")
            .iter()
            .any(|effect| effect == "filter.dim")
    );
    assert_eq!(content["coverage"], "none");
    assert_eq!(content["status"], "migrationCandidateReady");
    assert_eq!(complex["coverage"], "none");
    assert_eq!(complex["status"], "adapterExpansionReady");
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
    assert_eq!(first_cell["foreground"], "transparent");
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
    assert_eq!(report["summary"]["total"], 16);
    assert_eq!(report["summary"]["rendered"], 12);
    assert_eq!(report["summary"]["unsupported"], 4);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(report["frames"].as_array().expect("frames").len(), 16);
}

#[test]
fn test_fnc_cli_renders_unsupported_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["frames"][0]["status"], "unsupported");
    assert!(
        report["frames"][0]["unsupportedEffectIds"]
            .as_array()
            .expect("unsupported effect ids")
            .iter()
            .any(|effect| effect == "shader.linearGradient")
    );
    assert!(
        !report["frames"][0]["errors"]
            .as_array()
            .expect("errors")
            .is_empty()
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

// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>END OF VERSION: 0.7.0</VERS>
