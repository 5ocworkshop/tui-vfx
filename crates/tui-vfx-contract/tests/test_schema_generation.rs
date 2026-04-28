// <FILE>crates/tui-vfx-contract/tests/test_schema_generation.rs</FILE> - <DESC>Stable contract schema generation and staleness checks</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase E1: include effect descriptor schema root.</WCTX>
// <CLOG>0.2.0: MINOR — add effect descriptor schema fixture and description assertions.
// 0.1.0: INIT — check strict rustdoc-backed schemas for stable surface, scope, write, diagnostic, scene, element, and outcome roots.</CLOG>

use std::{fs, path::PathBuf};

use schemars::{JsonSchema, Schema, schema_for};
use tui_vfx_contract::{
    CellWrite, EffectDescriptor, Scene, SceneElement, SceneOutcome, ScopeSpec, Surface,
    SurfaceDiagnostic,
};

fn schema_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas/v3.1/contract")
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
        (
            "diagnostic.schema.json",
            canonical_schema::<SurfaceDiagnostic>(),
        ),
        ("scene.schema.json", canonical_schema::<Scene>()),
        ("element.schema.json", canonical_schema::<SceneElement>()),
        ("outcome.schema.json", canonical_schema::<SceneOutcome>()),
        (
            "effect-descriptor.schema.json",
            canonical_schema::<EffectDescriptor>(),
        ),
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
fn generated_contract_schema_contains_rustdoc_descriptions() {
    let all_schemas = schema_roots()
        .into_iter()
        .map(|(_, schema)| schema)
        .collect::<Vec<_>>()
        .join("\n");

    assert!(all_schemas.contains("Dense rectangular semantic render surface"));
    assert!(all_schemas.contains("Minimal Phase A/B scope algebra"));
    assert!(all_schemas.contains("Policy for how a cell write updates cell channels"));
    assert!(all_schemas.contains("Structured diagnostic emitted by surface contract operations"));
    assert!(all_schemas.contains("Scene composed from one or more placed semantic elements"));
    assert!(all_schemas.contains("One placed semantic surface inside a scene"));
    assert!(
        all_schemas.contains("Result of composing scene elements into one final semantic surface")
    );
    assert!(all_schemas.contains("Minimal durable v3.1 effect descriptor contract"));
    assert!(all_schemas.contains("Cell channels an effect may read or write"));
    assert!(
        all_schemas.contains("Scope shapes and evaluation spaces an effect descriptor supports")
    );
    assert!(all_schemas.contains("Minimal lifecycle metadata declared by an effect descriptor"));
}

#[test]
fn generated_contract_schema_objects_are_strict_and_described() {
    for (file_name, schema) in schema_roots() {
        let value: serde_json::Value = serde_json::from_str(&schema).expect("schema is JSON");
        assert_object_shapes_are_strict(&value, file_name);
        assert_properties_are_described(&value, file_name);
    }
}

#[test]
fn checked_in_contract_schemas_are_current() {
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

// <FILE>crates/tui-vfx-contract/tests/test_schema_generation.rs</FILE> - <DESC>Stable contract schema generation and staleness checks</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
