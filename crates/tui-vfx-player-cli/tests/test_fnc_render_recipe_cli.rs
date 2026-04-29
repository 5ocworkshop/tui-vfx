// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: lock report behavior and scoped CLI validation.</WCTX>
// <CLOG>0.4.0: PATCH — add migration-gap option validation regression.</CLOG>

use std::process::Command;

#[test]
fn test_fnc_cli_renders_single_recipe_frame_json() {
    let descriptor_pack = descriptor_pack();
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args([
            "render-recipe",
            "--recipe",
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json",
            "--descriptor-pack",
            descriptor_pack.to_str().expect("utf8 path"),
            "--json",
        ])
        .output()
        .expect("run player cli");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json report");
    assert_eq!(report["schemaVersion"], "v3.1.player.frame.1");
    assert_eq!(report["status"], "rendered");
    assert!(report["nonEmptyCells"].as_u64().expect("cell count") > 0);
}

#[test]
fn test_fnc_cli_renders_recursive_smoke_report_json() {
    let descriptor_pack = descriptor_pack();
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args([
            "render-recipe",
            "--json",
            "--recursive",
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
            "--descriptor-pack",
            descriptor_pack.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run recursive player cli");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("json report");
    assert_eq!(report["schemaVersion"], "v3.1.player.run.1");
    assert_eq!(report["summary"]["total"], 16);
    assert!(report["summary"]["rendered"].as_u64().expect("rendered") > 0);
    assert!(
        report["summary"]["unsupported"]
            .as_u64()
            .expect("unsupported")
            > 0
    );
}

#[test]
fn test_fnc_cli_inventories_single_baseline_recipe_json() {
    let report = inventory_report(&[
        "inventory-recipes",
        "--recipe",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json",
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
    let report = inventory_report(&[
        "inventory-recipes",
        "--recipe",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json",
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
    let report = inventory_report(&[
        "inventory-recipes",
        "--recipe",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient.json",
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
    let report = inventory_report(&[
        "inventory-recipes",
        "--recursive",
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.inventory.1");
    assert_eq!(report["summary"]["totalRecipes"], 16);
    assert_eq!(report["summary"]["rendered"], 10);
    assert_eq!(report["summary"]["unsupported"], 6);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(report["summary"]["descriptorEffectIds"], 14);
    assert_eq!(report["summary"]["representedEffectIds"], 14);
    assert_eq!(report["summary"]["unrepresentedEffectIds"], 0);
    assert_eq!(report["summary"]["unsupportedEffectIds"], 6);
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
    let descriptor_pack = descriptor_pack();
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args([
            "migration-gap",
            "accidental-path.json",
            "--legacy-root",
            "/usr/projects/tui-vfx-recipes/recipes/debug_recipes",
            "--v31-root",
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
            "--descriptor-pack",
            descriptor_pack.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("run migration gap player cli");

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

fn inventory_report(args: &[&str]) -> serde_json::Value {
    let descriptor_pack = descriptor_pack();
    let mut command_args = args.to_vec();
    command_args.extend([
        "--descriptor-pack",
        descriptor_pack.to_str().expect("utf8 path"),
        "--json",
    ]);
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args(command_args)
        .output()
        .expect("run inventory player cli");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    serde_json::from_slice(&output.stdout).expect("json report")
}

fn migration_gap_report() -> serde_json::Value {
    let descriptor_pack = descriptor_pack();
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args([
            "migration-gap",
            "--legacy-root",
            "/usr/projects/tui-vfx-recipes/recipes/debug_recipes",
            "--v31-root",
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
            "--descriptor-pack",
            descriptor_pack.to_str().expect("utf8 path"),
            "--json",
        ])
        .output()
        .expect("run migration gap player cli");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    serde_json::from_slice(&output.stdout).expect("json report")
}

fn find_effect<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .find(|effect| effect["id"] == id)
        .expect("effect entry")
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

fn descriptor_pack() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .join("descriptors/v3.1/packs/primitive.json")
}

// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
