// <FILE>crates/tui-vfx-next/tests/test_schema_generation.rs</FILE> - <DESC>Proof-pipeline schema generation checks for v3.1 clean-room incubator</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>New kernel Phase E0: keep only proof-pipeline schema roots in tui-vfx-next after contract split.</WCTX>
// <CLOG>0.4.0: MINOR — move stable contract schema checks to tui-vfx-contract and retain sampler/pipeline proof roots here.
// 0.3.2: TEST — include public SceneOutcome in checked schema roots.</CLOG>

use std::{fs, path::PathBuf};

use schemars::{JsonSchema, Schema, schema_for};
use tui_vfx_next::{PipelineSampler, SurfacePipeline};

fn schema_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas/v3.1/next")
}

fn canonical_schema<T: JsonSchema>() -> String {
    let schema: Schema = schema_for!(T);
    let mut json = serde_json::to_string_pretty(&schema).expect("schema serializes to JSON");
    json.push('\n');
    json
}

fn schema_roots() -> Vec<(&'static str, String)> {
    vec![
        ("sampler.schema.json", canonical_schema::<PipelineSampler>()),
        (
            "pipeline.schema.json",
            canonical_schema::<SurfacePipeline>(),
        ),
    ]
}

fn assert_object_shapes_are_strict(schema: &serde_json::Value, path: &str) {
    match schema {
        serde_json::Value::Object(map) => {
            if map.get("type") == Some(&serde_json::Value::String("object".to_string())) {
                match map.get("additionalProperties") {
                    Some(serde_json::Value::Bool(false)) => {}
                    Some(serde_json::Value::Object(_)) if map.get("properties").is_none() => {}
                    actual => panic!(
                        "object schema at {path} must deny additional properties or constrain map values, got {actual:?}"
                    ),
                }
            }
            for (key, value) in map {
                assert_object_shapes_are_strict(value, &format!("{path}/{key}"));
            }
        }
        serde_json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                assert_object_shapes_are_strict(value, &format!("{path}/{index}"));
            }
        }
        _ => {}
    }
}

fn assert_properties_are_described(schema: &serde_json::Value, path: &str) {
    match schema {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(properties)) = map.get("properties") {
                for (name, property) in properties {
                    if name != "kind" {
                        assert!(
                            property.get("description").is_some(),
                            "schema property {path}/{name} must have a description"
                        );
                    }
                }
            }
            for (key, value) in map {
                assert_properties_are_described(value, &format!("{path}/{key}"));
            }
        }
        serde_json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                assert_properties_are_described(value, &format!("{path}/{index}"));
            }
        }
        _ => {}
    }
}

#[test]
fn generated_proof_schema_contains_rustdoc_descriptions() {
    let all_schemas = schema_roots()
        .into_iter()
        .map(|(_, schema)| schema)
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        all_schemas.contains("Samplers supported by the Phase C toy pipeline stages"),
        "sampler rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Ordered multi-stage surface pipeline"),
        "pipeline rustdoc description should be present"
    );
}

#[test]
fn generated_proof_schema_objects_are_strict_and_described() {
    for (file_name, schema) in schema_roots() {
        let value: serde_json::Value = serde_json::from_str(&schema).expect("schema is JSON");
        assert_object_shapes_are_strict(&value, file_name);
        assert_properties_are_described(&value, file_name);
    }
}

#[test]
fn checked_in_proof_schemas_are_current() {
    let dir = schema_dir();
    if std::env::var_os("UPDATE_SCHEMAS").is_some() {
        fs::create_dir_all(&dir).expect("schema directory can be created");
        for (file_name, schema) in schema_roots() {
            fs::write(dir.join(file_name), schema).expect("schema fixture can be written");
        }
        return;
    }

    for (file_name, generated) in schema_roots() {
        let path = dir.join(file_name);
        let checked_in = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("missing schema fixture {}: {error}", path.display()));
        assert_eq!(
            checked_in,
            generated,
            "schema fixture {} is stale; rerun with UPDATE_SCHEMAS=1",
            path.display()
        );
    }
}

// <FILE>crates/tui-vfx-next/tests/test_schema_generation.rs</FILE> - <DESC>Proof-pipeline schema generation checks for v3.1 clean-room incubator</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
