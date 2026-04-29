// <FILE>crates/tui-vfx-contract/tests/test_recipe_document_contract.rs</FILE> - <DESC>Canonical recipe document validation tests</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>New kernel Phase J2: keep recipe fixture builders current with descriptor pack refs.</WCTX>
// <CLOG>0.1.1: PATCH — initialize empty descriptor pack refs in recipe test fixtures.
// 0.1.0: INIT — lock recipe metadata, graph/source/asset/scene references, and element pipeline validation.</CLOG>

mod support;

use std::collections::BTreeMap;

use support::{base_graph, literal_source, text_value_spec};
use tui_vfx_contract::{
    AssetFormat, AssetId, AssetKind, AssetLocator, AssetRef, AssetRequirement, AssetSpec,
    CellWritePolicy, ClipPolicy, DescriptorValidationError, EffectId, EffectOutputId, ElementId,
    ElementPlacement, GraphId, GraphStep, GraphValueShape, NodeId, ParameterId, RecipeDocument,
    RecipeElementPipeline, RecipeId, RecipeMetadata, RecipeScene, RecipeSceneElement,
    RoleWritePolicy, SceneId, SourceDescriptor, SourceId, SourceInputId, SourceInputSpec,
    SourceInstanceId, SourceKind, SourceLifecycle, SourceOutputSize, SourceOutputSpec,
    SourceRolePolicy, SourceSpec, Value, ValueKind, ValueSource,
};
use tui_vfx_types::RoleTag;

fn text_descriptor() -> SourceDescriptor {
    SourceDescriptor {
        id: SourceId::new("source.text"),
        version: "0.1.0".to_string(),
        display_name: "Text source".to_string(),
        category: Some("source".to_string()),
        kind: SourceKind::Text,
        inputs: BTreeMap::from([(
            SourceInputId::new("text"),
            SourceInputSpec {
                display_name: Some("Text".to_string()),
                description: Some("Text rendered into a source-produced surface.".to_string()),
                value: text_value_spec(None),
                optional: false,
                bindable: true,
                runtime_mutability: tui_vfx_contract::RuntimeMutability::Runtime,
            },
        )]),
        assets: BTreeMap::new(),
        output: SourceOutputSpec {
            size: SourceOutputSize::InputDriven,
            roles: SourceRolePolicy::DefaultRole {
                role: RoleTag::Text,
            },
        },
        lifecycle: SourceLifecycle {
            deterministic_with_seed: true,
            time_aware: false,
            resize_aware: true,
        },
    }
}

fn asset_descriptor() -> SourceDescriptor {
    SourceDescriptor {
        id: SourceId::new("source.flag"),
        version: "0.1.0".to_string(),
        display_name: "Flag source".to_string(),
        category: Some("source".to_string()),
        kind: SourceKind::Procedural,
        inputs: BTreeMap::new(),
        assets: BTreeMap::from([(
            AssetId::new("flagArt"),
            AssetRequirement {
                kind: AssetKind::BrailleDotfield,
                format: AssetFormat::new("tui-vfx.braille_flag_asset.v1"),
                required: true,
                description: Some("Flag dotfield asset.".to_string()),
            },
        )]),
        output: SourceOutputSpec {
            size: SourceOutputSize::InputDriven,
            roles: SourceRolePolicy::Explicit,
        },
        lifecycle: SourceLifecycle {
            deterministic_with_seed: true,
            time_aware: true,
            resize_aware: true,
        },
    }
}

fn flag_asset() -> AssetSpec {
    AssetSpec {
        id: AssetId::new("flagArt"),
        kind: AssetKind::BrailleDotfield,
        format: AssetFormat::new("tui-vfx.braille_flag_asset.v1"),
        locator: AssetLocator::Path {
            path: "assets/flag-dots.json".to_string(),
        },
        description: Some("Flag dots used by a procedural source.".to_string()),
    }
}

fn text_source() -> SourceSpec {
    SourceSpec {
        source: SourceId::new("source.text"),
        inputs: BTreeMap::from([(
            SourceInputId::new("text"),
            ValueSource::Parameter {
                id: ParameterId::new("title"),
                fallback: Some(Value::Text("READY".to_string())),
            },
        )]),
        assets: BTreeMap::new(),
    }
}

fn scene_element() -> RecipeSceneElement {
    RecipeSceneElement {
        id: ElementId::new("heroTitle"),
        layer: None,
        z_index: 0,
        placement: ElementPlacement { x: 0, y: 0 },
        source: SourceInstanceId::new("heroText"),
        pipeline: Some(RecipeElementPipeline {
            graph: GraphId::new("heroFade"),
            topology: Some(GraphStep::Node {
                node: NodeId::new("fadeIn"),
            }),
        }),
        clip_policy: ClipPolicy::Clip,
        cell_write_policy: CellWritePolicy::WriteCell,
        role_write_policy: RoleWritePolicy::CopySampledSource,
    }
}

fn valid_recipe() -> RecipeDocument {
    RecipeDocument {
        id: RecipeId::new("heroRecipe"),
        version: "3.1".to_string(),
        metadata: RecipeMetadata {
            title: Some("Hero Recipe".to_string()),
            description: Some("Canonical H1 proof recipe.".to_string()),
            authors: vec!["new-kernel".to_string()],
            expected_visual: None,
            tags: vec!["proof".to_string()],
        },
        lifecycle: None,
        assets: BTreeMap::new(),
        descriptor_packs: vec![],
        source_descriptors: BTreeMap::from([(SourceId::new("source.text"), text_descriptor())]),
        sources: BTreeMap::from([(SourceInstanceId::new("heroText"), text_source())]),
        graph: base_graph(literal_source()),
        scenes: vec![RecipeScene {
            id: SceneId::new("main"),
            width: 40,
            height: 5,
            elements: vec![scene_element()],
        }],
    }
}

#[test]
fn valid_recipe_document_passes() {
    let recipe = valid_recipe();

    assert!(recipe.validate().is_ok());
}

#[test]
fn recipe_rejects_invalid_recipe_id() {
    let mut recipe = valid_recipe();
    recipe.id = RecipeId::new("not valid");

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::InvalidRecipeId { id }) if id.as_str() == "not valid"
    ));
}

#[test]
fn recipe_rejects_unknown_parameter_ref_through_graph() {
    let mut recipe = valid_recipe();
    recipe.graph = base_graph(ValueSource::Parameter {
        id: ParameterId::new("missing"),
        fallback: None,
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownParameter { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn recipe_rejects_unknown_signal_ref_through_graph() {
    let mut recipe = valid_recipe();
    recipe.graph = base_graph(ValueSource::Signal {
        id: tui_vfx_contract::SignalId::new("missingSignal"),
        fallback: None,
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownSignal { id }) if id.as_str() == "missingSignal"
    ));
}

#[test]
fn recipe_rejects_unknown_source_descriptor_ref() {
    let mut recipe = valid_recipe();
    recipe
        .sources
        .get_mut(&SourceInstanceId::new("heroText"))
        .unwrap()
        .source = SourceId::new("source.missing");

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownSource { id }) if id.as_str() == "source.missing"
    ));
}

#[test]
fn recipe_rejects_graph_order_error_through_graph() {
    let mut recipe = valid_recipe();
    recipe.graph.order.push(NodeId::new("fadeIn"));

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::DuplicateOrderNode { id }) if id.as_str() == "fadeIn"
    ));
}

#[test]
fn recipe_rejects_graph_topology_error_through_graph() {
    let mut recipe = valid_recipe();
    recipe.graph.topology = Some(GraphStep::Node {
        node: NodeId::new("missingTopologyNode"),
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownOrderNode { id })
            if id.as_str() == "missingTopologyNode"
    ));
}

#[test]
fn recipe_rejects_graph_value_shape_mismatch_through_graph() {
    let mut recipe = valid_recipe();
    let mut producer_descriptor = recipe
        .graph
        .effects
        .get(&EffectId::new("terminal.opacity"))
        .unwrap()
        .clone();
    producer_descriptor.id = EffectId::new("terminal.producer");
    producer_descriptor.outputs = BTreeMap::from([
        (
            EffectOutputId::new("frame"),
            support::number_output(GraphValueShape::FrameValue),
        ),
        (
            EffectOutputId::new("field"),
            support::number_output(GraphValueShape::CellField),
        ),
    ]);
    recipe
        .graph
        .effects
        .insert(EffectId::new("terminal.producer"), producer_descriptor);
    recipe.graph.nodes.insert(
        NodeId::new("frameProducer"),
        support::output_from_effect(support::base_node(literal_source()), "sharedValue", "frame"),
    );
    recipe
        .graph
        .nodes
        .get_mut(&NodeId::new("frameProducer"))
        .unwrap()
        .id = NodeId::new("frameProducer");
    recipe
        .graph
        .nodes
        .get_mut(&NodeId::new("frameProducer"))
        .unwrap()
        .effect = EffectId::new("terminal.producer");
    recipe.graph.nodes.insert(
        NodeId::new("fieldProducer"),
        support::output_from_effect(support::base_node(literal_source()), "sharedValue", "field"),
    );
    recipe
        .graph
        .nodes
        .get_mut(&NodeId::new("fieldProducer"))
        .unwrap()
        .id = NodeId::new("fieldProducer");
    recipe
        .graph
        .nodes
        .get_mut(&NodeId::new("fieldProducer"))
        .unwrap()
        .effect = EffectId::new("terminal.producer");
    recipe
        .graph
        .order
        .extend([NodeId::new("frameProducer"), NodeId::new("fieldProducer")]);

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::GraphValueShapeMismatch { id, .. })
            if id.as_str() == "sharedValue"
    ));
}

#[test]
fn recipe_rejects_unknown_graph_value_ref_through_graph() {
    let mut recipe = valid_recipe();
    recipe.graph = base_graph(ValueSource::GraphValue {
        id: tui_vfx_contract::GraphValueId::new("missingGraphValue"),
        fallback: None,
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownGraphValue { id })
            if id.as_str() == "missingGraphValue"
    ));
}

#[test]
fn recipe_rejects_unknown_effect_ref_through_graph() {
    let mut recipe = valid_recipe();
    recipe
        .graph
        .nodes
        .get_mut(&NodeId::new("fadeIn"))
        .unwrap()
        .effect = EffectId::new("missing.effect");

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownEffect { id }) if id.as_str() == "missing.effect"
    ));
}

#[test]
fn recipe_rejects_node_input_kind_mismatch_through_graph() {
    let mut recipe = valid_recipe();
    recipe.graph = base_graph(ValueSource::Literal {
        value: Value::Text("wrong".to_string()),
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::SourceKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Text
        })
    ));
}

#[test]
fn recipe_rejects_missing_required_source_input() {
    let mut recipe = valid_recipe();
    recipe
        .sources
        .get_mut(&SourceInstanceId::new("heroText"))
        .unwrap()
        .inputs
        .clear();

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::MissingRequiredSourceInput { input, .. })
            if input.as_str() == "text"
    ));
}

#[test]
fn recipe_rejects_unknown_asset_ref() {
    let mut recipe = valid_recipe();
    recipe
        .source_descriptors
        .insert(SourceId::new("source.flag"), asset_descriptor());
    recipe.sources.insert(
        SourceInstanceId::new("heroText"),
        SourceSpec {
            source: SourceId::new("source.flag"),
            inputs: BTreeMap::new(),
            assets: BTreeMap::from([(
                AssetId::new("flagArt"),
                AssetRef {
                    id: AssetId::new("missingAsset"),
                },
            )]),
        },
    );

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownAssetRef { id })
            if id.as_str() == "missingAsset"
    ));
}

#[test]
fn recipe_accepts_structural_asset_ref() {
    let mut recipe = valid_recipe();
    recipe.assets.insert(AssetId::new("flagArt"), flag_asset());
    recipe
        .source_descriptors
        .insert(SourceId::new("source.flag"), asset_descriptor());
    recipe.sources.insert(
        SourceInstanceId::new("heroText"),
        SourceSpec {
            source: SourceId::new("source.flag"),
            inputs: BTreeMap::new(),
            assets: BTreeMap::from([(
                AssetId::new("flagArt"),
                AssetRef {
                    id: AssetId::new("flagArt"),
                },
            )]),
        },
    );

    assert!(recipe.validate().is_ok());
}

#[test]
fn recipe_rejects_unknown_scene_source_instance() {
    let mut recipe = valid_recipe();
    recipe.scenes[0].elements[0].source = SourceInstanceId::new("missingSource");

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownSceneElementSource { source, .. })
            if source.as_str() == "missingSource"
    ));
}

#[test]
fn recipe_rejects_unknown_element_pipeline_graph() {
    let mut recipe = valid_recipe();
    recipe.scenes[0].elements[0]
        .pipeline
        .as_mut()
        .unwrap()
        .graph = GraphId::new("otherGraph");

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownElementPipelineGraph { graph, .. })
            if graph.as_str() == "otherGraph"
    ));
}

#[test]
fn recipe_rejects_unknown_element_pipeline_node() {
    let mut recipe = valid_recipe();
    recipe.scenes[0].elements[0]
        .pipeline
        .as_mut()
        .unwrap()
        .topology = Some(GraphStep::Node {
        node: NodeId::new("missingNode"),
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::UnknownElementPipelineNode { node, .. })
            if node.as_str() == "missingNode"
    ));
}

#[test]
fn recipe_rejects_duplicate_element_pipeline_node() {
    let mut recipe = valid_recipe();
    recipe.scenes[0].elements[0]
        .pipeline
        .as_mut()
        .unwrap()
        .topology = Some(GraphStep::Sequence {
        children: vec![
            GraphStep::Node {
                node: NodeId::new("fadeIn"),
            },
            GraphStep::Node {
                node: NodeId::new("fadeIn"),
            },
        ],
    });

    assert!(matches!(
        recipe.validate(),
        Err(DescriptorValidationError::DuplicateElementPipelineNode { node, .. })
            if node.as_str() == "fadeIn"
    ));
}

// <FILE>crates/tui-vfx-contract/tests/test_recipe_document_contract.rs</FILE> - <DESC>Canonical recipe document validation tests</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
