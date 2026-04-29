// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs</FILE> - <DESC>Validate canonical recipe CLI behavior</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase J2: keep embedded descriptor CLI regression tests focused.</WCTX>
// <CLOG>0.3.0: MINOR — move descriptor-pack tests to a dedicated file and shared support.
// 0.2.0: MINOR — cover recursive corpus validation and three contract-negative diagnostics.
// 0.1.0: INIT — assert success for J0 fixture corpus and nonzero failure for invalid JSON.</CLOG>

mod support;

use std::fs;

use serde_json::Value;
use support::{
    mutated_recipe_path, read_recipe, recipe_path, remove_temp, run_failure_args, run_success,
    write_json,
};

#[test]
fn validates_embedded_j0_recipe_without_descriptor_pack() {
    let baseline_path = recipe_path("baseline.json");
    let report = run_success(&[
        "validate-recipe",
        "--json",
        baseline_path.to_str().expect("utf8 recipe path"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.validator.report.1");
    assert_eq!(report["summary"]["total"], 1);
    assert_eq!(report["summary"]["valid"], 1);
}

#[test]
fn validates_multiple_recipe_files() {
    let baseline_path = recipe_path("baseline.json");
    let filter_path = recipe_path("filters/filter_dim.json");
    let report = run_success(&[
        "validate-recipe",
        baseline_path.to_str().expect("utf8 recipe path"),
        filter_path.to_str().expect("utf8 recipe path"),
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
    write_json(&path, &recipe);

    let report = run_failure_args(&["validate-recipe", path.to_str().expect("utf8 path")]);
    remove_temp(&path);

    assert_eq!(report["summary"]["invalid"], 1);
    assert_eq!(report["recipes"][0]["errors"][0]["code"], "unknownEffect");
    assert!(report["recipes"][0]["errors"][0]["hint"].is_string());
}

#[test]
fn rejects_unknown_scene_element_source_with_stable_code() {
    let path = mutated_recipe_path("unknown-source-instance");
    let mut recipe = read_recipe("baseline.json");
    recipe["scenes"][0]["elements"][0]["source"] = Value::from("missingSource");
    write_json(&path, &recipe);

    let report = run_failure_args(&["validate-recipe", path.to_str().expect("utf8 path")]);
    remove_temp(&path);

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
    write_json(&path, &recipe);

    let report = run_failure_args(&["validate-recipe", path.to_str().expect("utf8 path")]);
    remove_temp(&path);

    assert_eq!(report["recipes"][0]["errors"][0]["code"], "unknownSignal");
}

#[test]
fn rejects_invalid_recipe_json_with_deserialize_code() {
    let path = mutated_recipe_path("invalid-json");
    fs::write(&path, "{ this is not strict JSON }").expect("write invalid recipe fixture");

    let report = run_failure_args(&["validate-recipe", path.to_str().expect("utf8 path")]);
    remove_temp(&path);

    assert_eq!(
        report["recipes"][0]["errors"][0]["code"],
        "deserializeFailed"
    );
    assert_eq!(report["recipes"][0]["valid"], false);
}

// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs</FILE> - <DESC>Validate canonical recipe CLI behavior</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
