// <FILE>crates/tui-vfx-next/tests/test_schema_generation.rs</FILE> - <DESC>Schema generation and staleness checks for v3.1 clean-room contracts</DESC>
// <VERS>VERSION: 0.3.2</VERS>
// <WCTX>New kernel Phase D1 verifier fix: include public scene outcome schema root.</WCTX>
// <CLOG>0.3.2: TEST — include public SceneOutcome in checked schema roots.
// 0.3.1: TEST — add named D1 scene schema freshness test.
// 0.3.0: TEST — generate and validate Phase D1 scene and element schema roots.
// 0.2.1: TEST — require descriptions on `$ref` payload properties as well as inline properties.
// 0.2.0: TEST — assert schema object strictness and property descriptions for generated roots.
// 0.1.0: INIT — generate/check the six Phase D0 schema roots and assert rustdoc descriptions are present.</CLOG>

use std::{fs, path::PathBuf};

use schemars::{JsonSchema, Schema, schema_for};
use tui_vfx_next::{
    CellWrite, PipelineSampler, Scene, SceneElement, SceneOutcome, ScopeSpec, Surface,
    SurfaceDiagnostic, SurfacePipeline,
};

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
        ("surface.schema.json", canonical_schema::<Surface>()),
        ("scope.schema.json", canonical_schema::<ScopeSpec>()),
        ("write.schema.json", canonical_schema::<CellWrite>()),
        ("sampler.schema.json", canonical_schema::<PipelineSampler>()),
        (
            "pipeline.schema.json",
            canonical_schema::<SurfacePipeline>(),
        ),
        (
            "diagnostic.schema.json",
            canonical_schema::<SurfaceDiagnostic>(),
        ),
        ("scene.schema.json", canonical_schema::<Scene>()),
        ("element.schema.json", canonical_schema::<SceneElement>()),
        ("outcome.schema.json", canonical_schema::<SceneOutcome>()),
    ]
}

fn assert_object_shapes_are_strict(schema: &serde_json::Value, path: &str) {
    match schema {
        serde_json::Value::Object(map) => {
            if map.get("type") == Some(&serde_json::Value::String("object".to_string())) {
                assert_eq!(
                    map.get("additionalProperties"),
                    Some(&serde_json::Value::Bool(false)),
                    "object schema at {path} must deny additional properties"
                );
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
fn generated_schema_contains_rustdoc_descriptions() {
    let all_schemas = schema_roots()
        .into_iter()
        .map(|(_, schema)| schema)
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        all_schemas.contains("Dense rectangular semantic render surface"),
        "surface rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Minimal Phase A/B scope algebra"),
        "scope rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Policy for how a cell write updates cell channels"),
        "write policy rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Samplers supported by the Phase C toy pipeline stages"),
        "sampler rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Ordered multi-stage surface pipeline"),
        "pipeline rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Structured diagnostic emitted by surface contract operations"),
        "diagnostic rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Scene composed from one or more placed semantic elements"),
        "scene rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("One placed semantic surface inside a scene"),
        "element rustdoc description should be present"
    );
    assert!(
        all_schemas.contains("Result of composing scene elements into one final semantic surface"),
        "scene outcome rustdoc description should be present"
    );
}

#[test]
fn generated_schema_objects_are_strict_and_described() {
    for (file_name, schema) in schema_roots() {
        let value: serde_json::Value = serde_json::from_str(&schema).expect("schema is JSON");
        assert_object_shapes_are_strict(&value, file_name);
        assert_properties_are_described(&value, file_name);
    }
}

#[test]
fn scene_schema_generation_is_current() {
    let dir = schema_dir();
    let scene_schema =
        fs::read_to_string(dir.join("scene.schema.json")).expect("scene schema fixture exists");
    let element_schema =
        fs::read_to_string(dir.join("element.schema.json")).expect("element schema fixture exists");
    let outcome_schema =
        fs::read_to_string(dir.join("outcome.schema.json")).expect("outcome schema fixture exists");

    assert_eq!(scene_schema, canonical_schema::<Scene>());
    assert_eq!(element_schema, canonical_schema::<SceneElement>());
    assert_eq!(outcome_schema, canonical_schema::<SceneOutcome>());
}

#[test]
fn checked_in_schemas_are_current() {
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

// <FILE>crates/tui-vfx-next/tests/test_schema_generation.rs</FILE> - <DESC>Schema generation and staleness checks for v3.1 clean-room contracts</DESC>
// <VERS>END OF VERSION: 0.3.2</VERS>
