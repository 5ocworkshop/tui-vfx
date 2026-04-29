// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_descriptor_packs.rs</FILE> - <DESC>Validate descriptor-pack recipe CLI behavior</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: prove external descriptor pack resolution and diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — cover pack file/dir loading, missing pack, unknown effect, and duplicate descriptor diagnostics.</CLOG>

mod support;

use serde_json::Value;
use support::{
    RECIPE_ROOT, mutated_recipe_path, read_recipe, remove_temp, run_failure_args, run_success,
    write_json,
};

const PACK_PATH: &str = "/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json";
const PACK_DIR: &str = "/usr/projects/tui-vfx/descriptors/v3.1/packs";

#[test]
fn validates_canonical_recipe_directory_with_descriptor_pack() {
    let report = run_success(&[
        "validate-recipe",
        "--descriptor-pack",
        PACK_PATH,
        "--json",
        "--recursive",
        RECIPE_ROOT,
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.validator.report.1");
    assert_eq!(report["descriptorPacks"][0]["id"], "v3.1.primitive");
    assert_eq!(report["summary"]["total"], 16);
    assert_eq!(report["summary"]["valid"], 16);
    assert_eq!(report["summary"]["invalid"], 0);
}

#[test]
fn validates_canonical_recipe_directory_with_descriptor_pack_dir() {
    let report = run_success(&[
        "validate-recipe",
        "--descriptor-pack-dir",
        PACK_DIR,
        "--json",
        "--recursive",
        RECIPE_ROOT,
    ]);

    assert_eq!(report["descriptorPacks"][0]["id"], "v3.1.primitive");
    assert_eq!(report["summary"]["total"], 16);
    assert_eq!(report["summary"]["valid"], 16);
}

#[test]
fn rejects_missing_descriptor_pack_with_stable_code() {
    let report = run_failure_args(&[
        "validate-recipe",
        &format!("{RECIPE_ROOT}/masks/mask_dissolve.json"),
    ]);

    assert_eq!(
        report["recipes"][0]["errors"][0]["code"],
        "unknownDescriptorPack"
    );
}

#[test]
fn rejects_pack_provided_unknown_descriptor_with_stable_code() {
    let path = mutated_recipe_path("unknown-pack-effect");
    let mut recipe = read_recipe("masks/mask_dissolve.json");
    recipe["graph"]["nodes"]["maskDissolveEnter"]["effect"] = Value::from("mask.missing");
    write_json(&path, &recipe);

    let report = run_failure_args(&[
        "validate-recipe",
        "--descriptor-pack",
        PACK_PATH,
        path.to_str().expect("utf8 path"),
    ]);
    remove_temp(&path);

    assert_eq!(report["recipes"][0]["errors"][0]["code"], "unknownEffect");
}

#[test]
fn rejects_duplicate_pack_effect_descriptor_with_stable_code() {
    let recipe_path = mutated_recipe_path("duplicate-pack-recipe");
    let pack_path = mutated_recipe_path("duplicate-pack");
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
        PACK_PATH,
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
    let mut pack: Value = serde_json::from_str(
        &std::fs::read_to_string(PACK_PATH).expect("read primitive descriptor pack"),
    )
    .expect("parse primitive descriptor pack");
    pack["id"] = Value::from("v3.1.duplicate");
    pack["sourceDescriptors"] = serde_json::json!({});
    let duplicate_effect = pack["effects"]["mask.dissolve"].clone();
    pack["effects"] = serde_json::json!({ "mask.dissolve": duplicate_effect });
    write_json(path, &pack);
}

// <FILE>crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_descriptor_packs.rs</FILE> - <DESC>Validate descriptor-pack recipe CLI behavior</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
