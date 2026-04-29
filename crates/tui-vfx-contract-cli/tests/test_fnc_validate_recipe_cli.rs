// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs</FILE> - <DESC>Validate canonical recipe CLI behavior</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J1: prove recursive reports and stable negative diagnostics.</WCTX>
// <CLOG>0.2.0: MINOR — cover recursive corpus validation and three contract-negative diagnostics.
// 0.1.0: INIT — assert success for J0 fixture corpus and nonzero failure for invalid JSON.</CLOG>

use std::{fs, path::PathBuf, process::Command};

use serde_json::Value;

const RECIPE_ROOT: &str = "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes";

fn contract_cli() -> &'static str {
    env!("CARGO_BIN_EXE_tui-vfx-contract-cli")
}

#[test]
fn validates_j0_canonical_recipe_directory_recursively() {
    let report = run_success(&["validate-recipe", "--json", "--recursive", RECIPE_ROOT]);

    assert_eq!(report["schemaVersion"], "v3.1.validator.report.1");
    assert_eq!(report["root"], RECIPE_ROOT);
    assert_eq!(report["summary"]["total"], 10);
    assert_eq!(report["summary"]["valid"], 10);
    assert_eq!(report["summary"]["invalid"], 0);
}

#[test]
fn validates_multiple_recipe_files() {
    let report = run_success(&[
        "validate-recipe",
        &format!("{RECIPE_ROOT}/baseline.json"),
        &format!("{RECIPE_ROOT}/filters/filter_dim.json"),
    ]);

    assert_eq!(report["root"], "<multiple>");
    assert_eq!(report["summary"]["total"], 2);
    assert_eq!(report["summary"]["valid"], 2);
}

#[test]
fn rejects_unknown_effect_with_stable_code() {
    let path = mutated_recipe_path("unknown-effect");
    let mut recipe = read_recipe("filters/filter_dim.json");
    recipe["graph"]["nodes"]["filterDimEnter"]["effect"] = Value::from("filter.missing");
    write_recipe(&path, recipe);

    let report = run_failure(&path);
    let _ = fs::remove_file(&path);

    assert_eq!(report["summary"]["invalid"], 1);
    assert_eq!(report["recipes"][0]["errors"][0]["code"], "unknownEffect");
    assert!(report["recipes"][0]["errors"][0]["hint"].is_string());
}

#[test]
fn rejects_unknown_scene_element_source_with_stable_code() {
    let path = mutated_recipe_path("unknown-source-instance");
    let mut recipe = read_recipe("baseline.json");
    recipe["scenes"][0]["elements"][0]["source"] = Value::from("missingSource");
    write_recipe(&path, recipe);

    let report = run_failure(&path);
    let _ = fs::remove_file(&path);

    assert_eq!(
        report["recipes"][0]["errors"][0]["code"],
        "unknownSceneElementSource"
    );
}

#[test]
fn rejects_lifecycle_trigger_missing_signal_with_stable_code() {
    let path = mutated_recipe_path("missing-trigger-signal");
    let mut recipe = read_recipe("event_driven_dwell/bool_binding_demo.json");
    recipe["graph"]["signals"] = serde_json::json!({});
    write_recipe(&path, recipe);

    let report = run_failure(&path);
    let _ = fs::remove_file(&path);

    assert_eq!(report["recipes"][0]["errors"][0]["code"], "unknownSignal");
}

#[test]
fn rejects_invalid_recipe_json_with_deserialize_code() {
    let path = mutated_recipe_path("invalid-json");
    fs::write(&path, "{ this is not strict JSON }").expect("write invalid recipe fixture");

    let report = run_failure(&path);
    let _ = fs::remove_file(&path);

    assert_eq!(
        report["recipes"][0]["errors"][0]["code"],
        "deserializeFailed"
    );
    assert_eq!(report["recipes"][0]["valid"], false);
}

fn run_success(args: &[&str]) -> Value {
    let output = Command::new(contract_cli())
        .args(args)
        .output()
        .expect("run contract validation CLI");
    assert!(
        output.status.success(),
        "expected validation success; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("parse validation report")
}

fn run_failure(path: &PathBuf) -> Value {
    let output = Command::new(contract_cli())
        .arg("validate-recipe")
        .arg(path)
        .output()
        .expect("run contract validation CLI");
    assert_eq!(output.status.code(), Some(1));
    serde_json::from_slice(&output.stdout).expect("parse validation report")
}

fn read_recipe(relative_path: &str) -> Value {
    let path = PathBuf::from(RECIPE_ROOT).join(relative_path);
    serde_json::from_str(&fs::read_to_string(path).expect("read canonical recipe"))
        .expect("parse canonical recipe")
}

fn write_recipe(path: &PathBuf, recipe: Value) {
    fs::write(
        path,
        serde_json::to_string_pretty(&recipe).expect("serialize mutated recipe"),
    )
    .expect("write mutated recipe");
}

fn mutated_recipe_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "tui-vfx-contract-cli-{label}-{}.json",
        std::process::id()
    ))
}

// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs</FILE> - <DESC>Validate canonical recipe CLI behavior</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
