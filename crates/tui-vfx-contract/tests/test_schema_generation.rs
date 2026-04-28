// <FILE>crates/tui-vfx-contract/tests/test_schema_generation.rs</FILE> - <DESC>Stable contract schema generation and staleness checks</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>New kernel Phase G3: include graph topology schema root.</WCTX>
// <CLOG>0.6.0: MINOR — add graph-step schema fixture.
// 0.5.0: MINOR — add graph and node schema fixtures.
// 0.4.0: MINOR — add value source, parameter, signal, and binding schema fixtures.
// 0.3.0: MINOR — add value and effect input schema fixtures and description assertions.
// 0.2.0: MINOR — add effect descriptor schema fixture and description assertions.
// 0.1.0: INIT — check strict rustdoc-backed schemas for stable surface, scope, write, diagnostic, scene, element, and outcome roots.</CLOG>

use std::{fs, path::PathBuf};

use schemars::{JsonSchema, Schema, schema_for};
use tui_vfx_contract::{
    BindingSpec, CellWrite, EffectDescriptor, EffectInputSpec, GraphSpec, GraphStep, NodeSpec,
    ParameterSpec, Scene, SceneElement, SceneOutcome, ScopeSpec, SignalSpec, Surface,
    SurfaceDiagnostic, Value, ValueSource,
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
        ("value.schema.json", canonical_schema::<Value>()),
        (
            "effect-input.schema.json",
            canonical_schema::<EffectInputSpec>(),
        ),
        (
            "value-source.schema.json",
            canonical_schema::<ValueSource>(),
        ),
        ("parameter.schema.json", canonical_schema::<ParameterSpec>()),
        ("signal.schema.json", canonical_schema::<SignalSpec>()),
        ("binding.schema.json", canonical_schema::<BindingSpec>()),
        ("graph.schema.json", canonical_schema::<GraphSpec>()),
        ("graph-step.schema.json", canonical_schema::<GraphStep>()),
        ("node.schema.json", canonical_schema::<NodeSpec>()),
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

fn assert_schema_fixture_current<T: JsonSchema>(file_name: &str) {
    let path = schema_dir().join(file_name);
    let checked_in = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("missing schema fixture {}: {error}", path.display()));

    assert_eq!(
        checked_in,
        canonical_schema::<T>(),
        "schema fixture {} is stale; rerun with UPDATE_SCHEMAS=1",
        path.display()
    );
}

#[test]
fn value_schema_is_current() {
    assert_schema_fixture_current::<Value>("value.schema.json");
}

#[test]
fn effect_input_schema_is_current() {
    assert_schema_fixture_current::<EffectInputSpec>("effect-input.schema.json");
}

#[test]
fn value_source_schema_is_current() {
    assert_schema_fixture_current::<ValueSource>("value-source.schema.json");
}

#[test]
fn parameter_schema_is_current() {
    assert_schema_fixture_current::<ParameterSpec>("parameter.schema.json");
}

#[test]
fn signal_schema_is_current() {
    assert_schema_fixture_current::<SignalSpec>("signal.schema.json");
}

#[test]
fn binding_schema_is_current() {
    assert_schema_fixture_current::<BindingSpec>("binding.schema.json");
}

#[test]
fn graph_schema_is_current() {
    assert_schema_fixture_current::<GraphSpec>("graph.schema.json");
}

#[test]
fn graph_step_schema_is_current() {
    assert_schema_fixture_current::<GraphStep>("graph-step.schema.json");
}

#[test]
fn node_schema_is_current() {
    assert_schema_fixture_current::<NodeSpec>("node.schema.json");
}

#[test]
fn effect_descriptor_schema_with_inputs_is_current() {
    let schema = canonical_schema::<EffectDescriptor>();
    assert!(schema.contains("Descriptor-local typed input specifications"));
    assert!(schema.contains("propertyNames"));
    assert!(schema.contains("^[A-Za-z][A-Za-z0-9_-]*$"));
    assert_schema_fixture_current::<EffectDescriptor>("effect-descriptor.schema.json");
}

#[test]
fn allowed_values_is_not_required_for_non_enum_value_specs() {
    let schema = canonical_schema::<EffectInputSpec>();

    assert!(schema.contains("Allowed string values when `kind` is `enum`"));
    assert!(!schema.contains(
        r#""required": [
        "kind",
        "allowedValues"
      ]"#
    ));
}

#[test]
fn runtime_mutability_schema_is_described() {
    let schema = canonical_schema::<EffectInputSpec>();

    assert!(schema.contains("When this input value may change during the effect lifecycle"));
    assert!(schema.contains("compileTime"));
    assert!(schema.contains("phaseStart"));
    assert!(schema.contains("resetOnly"));
    assert!(schema.contains("runtime"));
}

#[test]
fn bindable_flag_schema_is_described() {
    let schema = canonical_schema::<EffectInputSpec>();

    assert!(schema.contains("Whether later phases may bind this input to a source"));
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
    assert!(all_schemas.contains("Canonical tagged literal value used by effect input specs"));
    assert!(all_schemas.contains("Descriptor-local specification for one effect input"));
    assert!(all_schemas.contains("Declarative source for a typed value"));
    assert!(all_schemas.contains("Public recipe parameter contract"));
    assert!(all_schemas.contains("Host/runtime-provided signal contract"));
    assert!(all_schemas.contains("Declarative binding from a value source to a parameter target"));
    assert!(all_schemas.contains("Canonical v3.1 graph container"));
    assert!(all_schemas.contains("Canonical execution topology for a graph"));
    assert!(all_schemas.contains("One effect node in a canonical v3.1 graph"));
    assert!(all_schemas.contains("Stable graph-local node identifier"));
    assert!(all_schemas.contains("Deterministic node order by graph-local node id"));
    assert!(all_schemas.contains("Cell channels an effect may read or write"));
    assert!(
        all_schemas.contains("Scope shapes and evaluation spaces an effect descriptor supports")
    );
    assert!(all_schemas.contains("Minimal lifecycle metadata declared by an effect descriptor"));
    assert!(all_schemas.contains("Whether later phases may bind this input to a source"));
    assert!(all_schemas.contains("When this input value may change during the effect lifecycle"));
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
// <VERS>END OF VERSION: 0.6.0</VERS>
