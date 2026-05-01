// <FILE>crates/tui-vfx-contract/tests/test_schema_generation.rs</FILE> - <DESC>Stable contract schema generation and staleness checks</DESC>
// <VERS>VERSION: 0.14.0</VERS>
// <WCTX>v3.1 pre-release scene vocabulary: include scroll factor schema root.</WCTX>
// <CLOG>0.14.0: MINOR — guard v3.1 schema and descriptor input names against ambiguous field vocabulary.
// 0.13.0: MINOR — add signal expression, shadow, style color, and visibility geometry schema fixtures.
// 0.12.0: MINOR — add ScrollFactor schema fixture for scene-element scroll metadata.
// 0.11.0: MINOR — add descriptor pack, pack ref, and catalog schema fixtures.
// 0.10.0: MINOR — add lifecycle/time/trigger schema fixtures.
// 0.9.0: MINOR — add canonical recipe document schema fixtures.
// 0.8.0: MINOR — add source and asset schema fixtures.
// 0.7.0: MINOR — add graph value and node output schema fixtures.
// 0.6.0: MINOR — add graph-step schema fixture.
// 0.5.0: MINOR — add graph and node schema fixtures.
// 0.4.0: MINOR — add value source, parameter, signal, and binding schema fixtures.
// 0.3.0: MINOR — add value and effect input schema fixtures and description assertions.
// 0.2.0: MINOR — add effect descriptor schema fixture and description assertions.
// 0.1.0: INIT — check strict rustdoc-backed schemas for stable surface, scope, write, diagnostic, scene, element, and outcome roots.</CLOG>

use std::{fs, path::PathBuf};

use schemars::{JsonSchema, Schema, schema_for};
use tui_vfx_contract::{
    AssetRef, AssetRequirement, AssetSpec, BindingSpec, CellWrite, ClockSpec, ClockValueSource,
    DescriptorCatalog, DescriptorPack, DescriptorPackId, DescriptorPackRef, DurationSpec,
    DwellPolicy, EasingSpec, EffectDescriptor, EffectInputSpec, EffectOutputSpec, GraphSpec,
    GraphStep, GraphValueId, GraphValueKind, GraphValueMergePolicy, GraphValueShape, LifecycleSpec,
    NodeOutputSpec, NodeSpec, ParameterSpec, PhaseSpec, RecipeDocument, RecipeElementGraphBinding,
    RecipeMetadata, RecipeScene, RecipeSceneElement, Scene, SceneElement, SceneOutcome, ScopeSpec,
    ScrollFactor, ShadowSpec, SignalExpressionSpec, SignalSpec, SourceDescriptor, SourceInputSpec,
    SourceInstanceId, SourceOutputSpec, SourceSpec, StyleColorSource, Surface, SurfaceDiagnostic,
    TransitionSpec, TransitionVisibilityGeometry, TriggerSpec, Value, ValuePredicate, ValueSource,
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
        (
            "signal-expression.schema.json",
            canonical_schema::<SignalExpressionSpec>(),
        ),
        ("binding.schema.json", canonical_schema::<BindingSpec>()),
        ("duration.schema.json", canonical_schema::<DurationSpec>()),
        ("easing.schema.json", canonical_schema::<EasingSpec>()),
        ("clock.schema.json", canonical_schema::<ClockSpec>()),
        (
            "clock-value-source.schema.json",
            canonical_schema::<ClockValueSource>(),
        ),
        (
            "dwell-policy.schema.json",
            canonical_schema::<DwellPolicy>(),
        ),
        ("trigger.schema.json", canonical_schema::<TriggerSpec>()),
        (
            "value-predicate.schema.json",
            canonical_schema::<ValuePredicate>(),
        ),
        ("phase.schema.json", canonical_schema::<PhaseSpec>()),
        ("lifecycle.schema.json", canonical_schema::<LifecycleSpec>()),
        ("asset.schema.json", canonical_schema::<AssetSpec>()),
        (
            "descriptor-pack-id.schema.json",
            canonical_schema::<DescriptorPackId>(),
        ),
        (
            "descriptor-pack-ref.schema.json",
            canonical_schema::<DescriptorPackRef>(),
        ),
        (
            "descriptor-pack.schema.json",
            canonical_schema::<DescriptorPack>(),
        ),
        (
            "descriptor-catalog.schema.json",
            canonical_schema::<DescriptorCatalog>(),
        ),
        (
            "asset-requirement.schema.json",
            canonical_schema::<AssetRequirement>(),
        ),
        ("asset-ref.schema.json", canonical_schema::<AssetRef>()),
        (
            "source-input.schema.json",
            canonical_schema::<SourceInputSpec>(),
        ),
        (
            "source-output.schema.json",
            canonical_schema::<SourceOutputSpec>(),
        ),
        (
            "source-descriptor.schema.json",
            canonical_schema::<SourceDescriptor>(),
        ),
        ("source.schema.json", canonical_schema::<SourceSpec>()),
        (
            "source-instance-id.schema.json",
            canonical_schema::<SourceInstanceId>(),
        ),
        (
            "recipe-metadata.schema.json",
            canonical_schema::<RecipeMetadata>(),
        ),
        (
            "recipe-element-graph-binding.schema.json",
            canonical_schema::<RecipeElementGraphBinding>(),
        ),
        (
            "recipe-scene.schema.json",
            canonical_schema::<RecipeScene>(),
        ),
        (
            "recipe-scene-element.schema.json",
            canonical_schema::<RecipeSceneElement>(),
        ),
        (
            "scroll-factor.schema.json",
            canonical_schema::<ScrollFactor>(),
        ),
        ("shadow.schema.json", canonical_schema::<ShadowSpec>()),
        (
            "style-color-source.schema.json",
            canonical_schema::<StyleColorSource>(),
        ),
        (
            "transition-visibility-geometry.schema.json",
            canonical_schema::<TransitionVisibilityGeometry>(),
        ),
        (
            "transition.schema.json",
            canonical_schema::<TransitionSpec>(),
        ),
        ("recipe.schema.json", canonical_schema::<RecipeDocument>()),
        ("graph.schema.json", canonical_schema::<GraphSpec>()),
        ("graph-step.schema.json", canonical_schema::<GraphStep>()),
        (
            "graph-value-id.schema.json",
            canonical_schema::<GraphValueId>(),
        ),
        (
            "graph-value-kind.schema.json",
            canonical_schema::<GraphValueKind>(),
        ),
        (
            "graph-value-shape.schema.json",
            canonical_schema::<GraphValueShape>(),
        ),
        (
            "graph-value-merge-policy.schema.json",
            canonical_schema::<GraphValueMergePolicy>(),
        ),
        (
            "effect-output.schema.json",
            canonical_schema::<EffectOutputSpec>(),
        ),
        (
            "node-output.schema.json",
            canonical_schema::<NodeOutputSpec>(),
        ),
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

fn assert_v31_schema_names_are_precise(schema: &serde_json::Value, path: &str) {
    const AMBIGUOUS_PROPERTY_NAMES: &[&str] = &[
        "affect",
        "amount",
        "applyTo",
        "color",
        "direction",
        "mode",
        "motion",
        "progress",
        "source",
        "speed",
        "target",
        "type",
    ];

    match schema {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(properties)) = map.get("properties") {
                for name in properties.keys() {
                    if AMBIGUOUS_PROPERTY_NAMES.contains(&name.as_str())
                        && !is_allowed_precise_schema_property(path, name)
                    {
                        panic!(
                            "ambiguous v3.1 schema property {path}/{name}; use a domain-specific field name"
                        );
                    }
                }
            }
            for (key, value) in map {
                assert_v31_schema_names_are_precise(value, &format!("{path}/{key}"));
            }
        }
        serde_json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                assert_v31_schema_names_are_precise(value, &format!("{path}/{index}"));
            }
        }
        _ => {}
    }
}

fn is_allowed_precise_schema_property(path: &str, name: &str) -> bool {
    matches!(
        (path, name),
        (path, "color") if path.contains("GradientStop")
    )
}

fn assert_descriptor_input_names_are_precise(descriptor_pack: &serde_json::Value) {
    const AMBIGUOUS_INPUT_IDS: &[&str] = &[
        "affect",
        "amount",
        "applyTo",
        "color",
        "direction",
        "mode",
        "motion",
        "progress",
        "source",
        "speed",
        "target",
        "type",
    ];

    let effects = descriptor_pack
        .get("effects")
        .and_then(serde_json::Value::as_object)
        .expect("descriptor pack has effects object");
    for (effect_id, effect) in effects {
        let Some(inputs) = effect.get("inputs").and_then(serde_json::Value::as_object) else {
            continue;
        };
        for input_id in inputs.keys() {
            if AMBIGUOUS_INPUT_IDS.contains(&input_id.as_str()) {
                panic!(
                    "ambiguous v3.1 descriptor input {effect_id}.{input_id}; use a domain-specific input id"
                );
            }
        }
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
fn lifecycle_schema_roots_are_current() {
    assert_schema_fixture_current::<DurationSpec>("duration.schema.json");
    assert_schema_fixture_current::<ClockSpec>("clock.schema.json");
    assert_schema_fixture_current::<DwellPolicy>("dwell-policy.schema.json");
    assert_schema_fixture_current::<TriggerSpec>("trigger.schema.json");
    assert_schema_fixture_current::<ValuePredicate>("value-predicate.schema.json");
    assert_schema_fixture_current::<PhaseSpec>("phase.schema.json");
    assert_schema_fixture_current::<LifecycleSpec>("lifecycle.schema.json");
}

#[test]
fn recipe_schema_generation_is_current() {
    assert_schema_fixture_current::<RecipeDocument>("recipe.schema.json");
}

#[test]
fn source_schema_roots_are_current() {
    assert_schema_fixture_current::<AssetSpec>("asset.schema.json");
    assert_schema_fixture_current::<DescriptorPackId>("descriptor-pack-id.schema.json");
    assert_schema_fixture_current::<DescriptorPackRef>("descriptor-pack-ref.schema.json");
    assert_schema_fixture_current::<DescriptorPack>("descriptor-pack.schema.json");
    assert_schema_fixture_current::<DescriptorCatalog>("descriptor-catalog.schema.json");
    assert_schema_fixture_current::<AssetRequirement>("asset-requirement.schema.json");
    assert_schema_fixture_current::<AssetRef>("asset-ref.schema.json");
    assert_schema_fixture_current::<SourceInputSpec>("source-input.schema.json");
    assert_schema_fixture_current::<SourceOutputSpec>("source-output.schema.json");
    assert_schema_fixture_current::<SourceDescriptor>("source-descriptor.schema.json");
    assert_schema_fixture_current::<SourceSpec>("source.schema.json");
}

#[test]
fn recipe_schema_roots_are_current() {
    assert_schema_fixture_current::<SourceInstanceId>("source-instance-id.schema.json");
    assert_schema_fixture_current::<RecipeMetadata>("recipe-metadata.schema.json");
    assert_schema_fixture_current::<RecipeElementGraphBinding>(
        "recipe-element-graph-binding.schema.json",
    );
    assert_schema_fixture_current::<RecipeScene>("recipe-scene.schema.json");
    assert_schema_fixture_current::<RecipeSceneElement>("recipe-scene-element.schema.json");
    assert_schema_fixture_current::<ScrollFactor>("scroll-factor.schema.json");
    assert_schema_fixture_current::<RecipeDocument>("recipe.schema.json");
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
fn graph_value_schema_roots_are_current() {
    assert_schema_fixture_current::<GraphValueId>("graph-value-id.schema.json");
    assert_schema_fixture_current::<GraphValueKind>("graph-value-kind.schema.json");
    assert_schema_fixture_current::<GraphValueShape>("graph-value-shape.schema.json");
    assert_schema_fixture_current::<GraphValueMergePolicy>("graph-value-merge-policy.schema.json");
    assert_schema_fixture_current::<EffectOutputSpec>("effect-output.schema.json");
    assert_schema_fixture_current::<NodeOutputSpec>("node-output.schema.json");
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
    assert!(
        all_schemas
            .contains("Canonical asset declaration available for structural source asset refs")
    );
    assert!(
        all_schemas.contains("Structural reference from a source asset slot to a declared asset")
    );
    assert!(all_schemas.contains("Stable v3.1 descriptor for a source"));
    assert!(all_schemas.contains("Canonical source instance referencing a source descriptor"));
    assert!(
        all_schemas.contains("Contract for the semantic surface produced by a source descriptor")
    );
    assert!(all_schemas.contains("Strict canonical v3.1 recipe document"));
    assert!(
        all_schemas
            .contains("Shared descriptor pack that can provide source and effect descriptors")
    );
    assert!(all_schemas.contains("Recipe-local declaration that an external descriptor pack"));
    assert!(
        all_schemas
            .contains("Scene element whose surface is produced by a declared source instance")
    );
    assert!(all_schemas.contains("Optional element-local graph binding into the canonical graph"));
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
    assert!(
        all_schemas.contains(
            "Recipe-level lifecycle contract from enter through dwell and exit to finished"
        )
    );
    assert!(all_schemas.contains("Native v3.1 state-change composition interval"));
    assert!(all_schemas.contains("Executable canonical V3.1 transition track"));
    assert!(all_schemas.contains(
        "Canonical lifecycle trigger with explicit condition, latch, reset, and action semantics"
    ));
    assert!(
        all_schemas
            .contains("Policy controlling how long the dwell lifecycle phase remains active")
    );
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
fn v31_schema_and_descriptor_field_names_are_domain_specific() {
    for (file_name, schema) in schema_roots() {
        let value: serde_json::Value = serde_json::from_str(&schema).expect("schema is JSON");
        assert_v31_schema_names_are_precise(&value, file_name);
    }

    let descriptor_pack_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../descriptors/v3.1/packs/primitive.json");
    let descriptor_pack: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&descriptor_pack_path)
            .unwrap_or_else(|error| panic!("descriptor pack can be read: {error}")),
    )
    .expect("descriptor pack is JSON");
    assert_descriptor_input_names_are_precise(&descriptor_pack);
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
// <VERS>END OF VERSION: 0.14.0</VERS>
