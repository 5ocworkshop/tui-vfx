// <FILE>crates/tui-vfx-player/tests/test_compositor_next_primitive_tree.rs</FILE> - <DESC>Regression tests for compositor-next co-located primitive tree scaffolding</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-next Phase 5/6 foundation — prove the first vertical primitive tree is descriptor-backed before runtime code generation expands.</WCTX>
// <CLOG>0.1.0: add descriptor-backed checks for the first compositor-next primitive tree.</CLOG>

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives under <repo>/crates/tui-vfx-player")
        .to_path_buf()
}

#[test]
fn linear_gradient_primitive_tree_copies_descriptor_source_of_truth() {
    let root = repo_root();
    let pack_path = root.join("descriptors/v3.1/packs/primitive.json");
    let primitive_descriptor_path =
        root.join("primitives/shader/linear_gradient/descriptor.v31.json");

    let pack: Value = serde_json::from_str(&fs::read_to_string(pack_path).unwrap()).unwrap();
    let descriptor: Value =
        serde_json::from_str(&fs::read_to_string(primitive_descriptor_path).unwrap()).unwrap();

    let source = &pack["effects"]["shader.linearGradient"];
    assert_eq!(descriptor, *source);
    assert_eq!(descriptor["id"], "shader.linearGradient");
}

#[test]
fn linear_gradient_primitive_tree_records_generation_and_validation_scope() {
    let root = repo_root();
    let primitive_root = root.join("primitives/shader/linear_gradient");
    let primitive_toml = fs::read_to_string(primitive_root.join("primitive.toml")).unwrap();
    let field_coverage =
        fs::read_to_string(primitive_root.join("tests/field_coverage.toml")).unwrap();
    let fixture: Value = serde_json::from_str(
        &fs::read_to_string(primitive_root.join("fixtures/minimal.v31.json")).unwrap(),
    )
    .unwrap();

    for required in [
        "id = \"shader.linearGradient\"",
        "family = \"shader\"",
        "descriptor = \"descriptor.v31.json\"",
        "runtime_status = \"copied-compositor-parity-only\"",
    ] {
        assert!(primitive_toml.contains(required), "missing {required}");
    }

    for field in [
        "startColor",
        "endColor",
        "colorSpace",
        "angleDeg",
        "intensity",
        "gradient",
        "applyTo",
    ] {
        assert!(
            field_coverage.contains(field),
            "missing field coverage for {field}"
        );
    }

    assert_eq!(fixture["schemaVersion"], "3.1");
    assert_eq!(
        fixture["metadata"]["id"],
        "compositor-next-linear-gradient-minimal"
    );
}

#[test]
fn linear_gradient_generated_scaffolding_is_descriptor_derived() {
    let root = repo_root();
    let primitive_root = root.join("primitives/shader/linear_gradient");
    let descriptor: Value = serde_json::from_str(
        &fs::read_to_string(primitive_root.join("descriptor.v31.json")).unwrap(),
    )
    .unwrap();
    let input_manifest: Value = serde_json::from_str(
        &fs::read_to_string(primitive_root.join("generated/linear_gradient_input_manifest.json"))
            .unwrap(),
    )
    .unwrap();
    let validation_manifest: Value = serde_json::from_str(
        &fs::read_to_string(
            primitive_root.join("generated/linear_gradient_validation_manifest.json"),
        )
        .unwrap(),
    )
    .unwrap();
    let control_catalog: Value = serde_json::from_str(
        &fs::read_to_string(primitive_root.join("generated/linear_gradient_control_catalog.json"))
            .unwrap(),
    )
    .unwrap();
    let generated_inputs =
        fs::read_to_string(primitive_root.join("generated/linear_gradient_inputs.rs")).unwrap();
    let generated_accessors =
        fs::read_to_string(primitive_root.join("generated/linear_gradient_accessors.rs")).unwrap();

    let mut descriptor_fields: Vec<String> = descriptor["inputs"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    let mut manifest_fields: Vec<String> = input_manifest["inputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|input| input["name"].as_str().unwrap().to_string())
        .collect();
    descriptor_fields.sort();
    manifest_fields.sort();

    assert_eq!(input_manifest["descriptorId"], "shader.linearGradient");
    assert_eq!(manifest_fields, descriptor_fields);
    assert_eq!(validation_manifest["descriptorId"], "shader.linearGradient");
    assert_eq!(
        validation_manifest["fixturePath"],
        "fixtures/minimal.v31.json"
    );
    assert_eq!(control_catalog["descriptorId"], "shader.linearGradient");
    assert_eq!(
        control_catalog["controls"].as_array().unwrap().len(),
        descriptor_fields.len()
    );

    for field in descriptor_fields {
        assert!(
            generated_inputs.contains(&field),
            "generated Rust input skeleton omits {field}"
        );
        assert!(
            generated_accessors.contains(&field),
            "generated accessor skeleton omits {field}"
        );
    }
}

// <FILE>crates/tui-vfx-player/tests/test_compositor_next_primitive_tree.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
