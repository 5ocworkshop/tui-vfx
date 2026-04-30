// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_descriptor_packs.rs</FILE> - <DESC>Validate descriptor-pack recipe CLI behavior</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>K2.12 source fixture: keep descriptor-pack validation counts aligned with v3.1 corpus.</WCTX>
// <CLOG>0.3.0: MINOR — update canonical fixture count after source.text fixture addition.
// 0.2.0: MINOR — update canonical fixture count after simple mask additions.
// 0.1.0: INIT — cover pack file/dir loading, missing pack, unknown effect, and duplicate descriptor diagnostics.</CLOG>

mod support;

use serde_json::Value;
use support::{
    descriptor_pack_dir, descriptor_pack_path, mutated_recipe_path, read_recipe, recipe_path,
    recipe_root, remove_temp, run_failure_args, run_success, write_json,
};

const CANONICAL_RECIPE_COUNT: i64 = 88;

#[test]
fn validates_canonical_recipe_directory_with_descriptor_pack() {
    let pack_path = descriptor_pack_path();
    let recipe_root = recipe_root();
    let report = run_success(&[
        "validate-recipe",
        "--descriptor-pack",
        pack_path.to_str().expect("utf8 pack path"),
        "--json",
        "--recursive",
        recipe_root.to_str().expect("utf8 recipe root"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.validator.report.1");
    assert_eq!(report["descriptorPacks"][0]["id"], "v3.1.primitive");
    assert_eq!(report["summary"]["total"], CANONICAL_RECIPE_COUNT);
    assert_eq!(report["summary"]["valid"], CANONICAL_RECIPE_COUNT);
    assert_eq!(report["summary"]["invalid"], 0);
}

#[test]
fn validates_canonical_recipe_directory_with_descriptor_pack_dir() {
    let pack_dir = descriptor_pack_dir();
    let recipe_root = recipe_root();
    let report = run_success(&[
        "validate-recipe",
        "--descriptor-pack-dir",
        pack_dir.to_str().expect("utf8 pack dir"),
        "--json",
        "--recursive",
        recipe_root.to_str().expect("utf8 recipe root"),
    ]);

    assert_eq!(report["descriptorPacks"][0]["id"], "v3.1.primitive");
    assert_eq!(report["summary"]["total"], CANONICAL_RECIPE_COUNT);
    assert_eq!(report["summary"]["valid"], CANONICAL_RECIPE_COUNT);
}

#[test]
fn rejects_missing_descriptor_pack_with_stable_code() {
    let recipe_path = recipe_path("masks/mask_dissolve.json");
    let report = run_failure_args(&["validate-recipe", recipe_path.to_str().expect("utf8 path")]);

    assert_eq!(
        report["recipes"][0]["errors"][0]["code"],
        "unknownDescriptorPack"
    );
}

#[test]
fn rejects_pack_provided_unknown_descriptor_with_stable_code() {
    let path = mutated_recipe_path("unknown-pack-effect");
    let pack_path = descriptor_pack_path();
    let mut recipe = read_recipe("masks/mask_dissolve.json");
    recipe["graph"]["nodes"]["maskDissolveEnter"]["effect"] = Value::from("mask.missing");
    write_json(&path, &recipe);

    let report = run_failure_args(&[
        "validate-recipe",
        "--descriptor-pack",
        pack_path.to_str().expect("utf8 pack path"),
        path.to_str().expect("utf8 path"),
    ]);
    remove_temp(&path);

    assert_eq!(report["recipes"][0]["errors"][0]["code"], "unknownEffect");
}

#[test]
fn rejects_duplicate_pack_effect_descriptor_with_stable_code() {
    let recipe_path = mutated_recipe_path("duplicate-pack-recipe");
    let pack_path = mutated_recipe_path("duplicate-pack");
    let primitive_pack_path = descriptor_pack_path();
    let mut recipe = read_recipe("masks/mask_dissolve.json");
    recipe["descriptorPacks"] = serde_json::json!([
        { "id": "v3.1.primitive" },
        { "id": "v3.1.duplicate" }
    ]);
    write_json(&recipe_path, &recipe);
    write_pack_with_duplicate_effect(&pack_path);

    let report = run_failure_args(&[
        "validate-recipe",
        "--descriptor-pack",
        primitive_pack_path.to_str().expect("utf8 pack path"),
        "--descriptor-pack",
        pack_path.to_str().expect("utf8 path"),
        recipe_path.to_str().expect("utf8 path"),
    ]);
    remove_temp(&recipe_path);
    remove_temp(&pack_path);

    assert_eq!(
        report["recipes"][0]["errors"][0]["code"],
        "duplicatePackEffectDescriptor"
    );
}

fn write_pack_with_duplicate_effect(path: &std::path::PathBuf) {
    let pack_path = descriptor_pack_path();
    let mut pack: Value = serde_json::from_str(
        &std::fs::read_to_string(pack_path).expect("read primitive descriptor pack"),
    )
    .expect("parse primitive descriptor pack");
    pack["id"] = Value::from("v3.1.duplicate");
    pack["sourceDescriptors"] = serde_json::json!({});
    let duplicate_effect = pack["effects"]["mask.dissolve"].clone();
    pack["effects"] = serde_json::json!({ "mask.dissolve": duplicate_effect });
    write_json(path, &pack);
}

// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_descriptor_packs.rs</FILE> - <DESC>Validate descriptor-pack recipe CLI behavior</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
