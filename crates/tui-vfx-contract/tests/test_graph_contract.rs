// <FILE>crates/tui-vfx-contract/tests/test_graph_contract.rs</FILE> - <DESC>Canonical graph source/input validation tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: validate graph value sources and node outputs.</WCTX>
// <CLOG>0.2.0: MINOR — add graph value source and node output validation cases.
// 0.1.0: INIT — lock literal, parameter, signal, effect, input, and source validation.</CLOG>

mod support;

use std::collections::BTreeMap;

use support::{
    base_graph, binding_to, graph_value_source, literal_source, output_from_effect,
    output_from_input, parameter_source, signal_source, text_value_spec,
};
use tui_vfx_contract::{
    DescriptorValidationError, EffectId, EffectInputId, NodeId, NodeSpec, ParameterId,
    ParameterSpec, Value, ValueKind, ValueSource,
};

#[test]
fn valid_graph_with_literal_input_passes() {
    let graph = base_graph(literal_source());

    assert!(graph.validate().is_ok());
}

#[test]
fn valid_graph_with_parameter_input_passes() {
    let graph = base_graph(parameter_source("opacity"));

    assert!(graph.validate().is_ok());
}

#[test]
fn valid_graph_with_signal_input_passes() {
    let graph = base_graph(signal_source("audioLevel"));

    assert!(graph.validate().is_ok());
}

#[test]
fn graph_rejects_effect_key_mismatch() {
    let mut graph = base_graph(literal_source());
    graph
        .effects
        .get_mut(&EffectId::new("terminal.opacity"))
        .unwrap()
        .id = EffectId::new("other.effect");

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::EffectIdMismatch { key, effect })
            if key.as_str() == "terminal.opacity" && effect.as_str() == "other.effect"
    ));
}

#[test]
fn graph_rejects_unknown_effect_id() {
    let mut graph = base_graph(literal_source());
    graph.nodes.get_mut(&NodeId::new("fadeIn")).unwrap().effect = EffectId::new("missing.effect");

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownEffect { id }) if id.as_str() == "missing.effect"
    ));
}

#[test]
fn graph_rejects_unknown_input_id() {
    let mut graph = base_graph(literal_source());
    graph.nodes.get_mut(&NodeId::new("fadeIn")).unwrap().inputs = BTreeMap::from([(
        EffectInputId::new("unknown"),
        ValueSource::Literal {
            value: Value::Number(0.5),
        },
    )]);

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownNodeInput { input, .. })
            if input.as_str() == "unknown"
    ));
}

#[test]
fn graph_rejects_missing_required_input() {
    let mut graph = base_graph(literal_source());
    graph
        .nodes
        .get_mut(&NodeId::new("fadeIn"))
        .unwrap()
        .inputs
        .clear();

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::MissingRequiredNodeInput { input, .. })
            if input.as_str() == "amount"
    ));
}

#[test]
fn graph_accepts_omitted_optional_input() {
    let mut graph = base_graph(literal_source());
    graph
        .effects
        .get_mut(&EffectId::new("terminal.opacity"))
        .unwrap()
        .inputs
        .get_mut(&EffectInputId::new("amount"))
        .unwrap()
        .optional = true;
    graph
        .nodes
        .get_mut(&NodeId::new("fadeIn"))
        .unwrap()
        .inputs
        .clear();

    assert!(graph.validate().is_ok());
}

#[test]
fn graph_rejects_input_kind_mismatch() {
    let graph = base_graph(ValueSource::Literal {
        value: Value::Text("wrong".to_string()),
    });

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::SourceKindMismatch {
            expected: ValueKind::Number,
            actual: ValueKind::Text
        })
    ));
}

#[test]
fn graph_rejects_parameter_source_unknown_parameter() {
    let graph = base_graph(parameter_source("missing"));

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownParameter { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn graph_rejects_signal_source_unknown_signal() {
    let graph = base_graph(signal_source("missing"));

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownSignal { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn graph_rejects_parameter_key_mismatch() {
    let mut graph = base_graph(literal_source());
    graph.parameters.insert(
        ParameterId::new("alias"),
        ParameterSpec {
            id: ParameterId::new("other"),
            display_name: None,
            description: None,
            value: text_value_spec(Some("READY")),
            bindable: true,
        },
    );

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::ParameterIdMismatch { key, parameter })
            if key.as_str() == "alias" && parameter.as_str() == "other"
    ));
}

// <FILE>crates/tui-vfx-contract/tests/test_graph_contract.rs</FILE> - <DESC>Canonical graph source/input validation tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>

#[test]
fn graph_value_source_rejects_unknown_output() {
    let graph = base_graph(graph_value_source("missing"));

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownGraphValue { id }) if id.as_str() == "missing"
    ));
}

#[test]
fn graph_value_source_rejects_kind_mismatch() {
    let mut graph = base_graph(graph_value_source("numberOut"));
    let mut producer_descriptor = graph
        .effects
        .get(&EffectId::new("terminal.opacity"))
        .unwrap()
        .clone();
    producer_descriptor.id = EffectId::new("terminal.producer");
    producer_descriptor.outputs = BTreeMap::from([(
        tui_vfx_contract::EffectOutputId::new("value"),
        support::number_output(tui_vfx_contract::GraphValueShape::FrameValue),
    )]);
    graph
        .effects
        .insert(EffectId::new("terminal.producer"), producer_descriptor);
    let mut text_consumer = graph
        .effects
        .get_mut(&EffectId::new("terminal.opacity"))
        .unwrap()
        .clone();
    let amount_spec = &mut text_consumer
        .inputs
        .get_mut(&EffectInputId::new("amount"))
        .unwrap()
        .value;
    amount_spec.kind = ValueKind::Text;
    amount_spec.range = None;
    graph
        .effects
        .insert(EffectId::new("terminal.opacity"), text_consumer);
    graph.nodes.insert(
        NodeId::new("producer"),
        output_from_effect(
            NodeSpec {
                id: NodeId::new("producer"),
                effect: EffectId::new("terminal.producer"),
                inputs: BTreeMap::from([(
                    EffectInputId::new("amount"),
                    ValueSource::Literal {
                        value: Value::Number(0.5),
                    },
                )]),
                outputs: BTreeMap::new(),
                scope: None,
                cell_write_policy: None,
                role_write_policy: None,
            },
            "numberOut",
            "value",
        ),
    );
    graph.order.push(NodeId::new("producer"));

    let error = graph
        .validate()
        .expect_err("graph value kind mismatch fails");
    assert!(matches!(
        error,
        DescriptorValidationError::SourceKindMismatch {
            expected: ValueKind::Text,
            actual: ValueKind::Number
        }
    ));
}

#[test]
fn graph_rejects_duplicate_graph_value_shape_mismatch() {
    let mut graph = base_graph(literal_source());
    let mut producer_descriptor = graph
        .effects
        .get(&EffectId::new("terminal.opacity"))
        .unwrap()
        .clone();
    producer_descriptor.id = EffectId::new("terminal.producer");
    producer_descriptor.outputs = BTreeMap::from([
        (
            tui_vfx_contract::EffectOutputId::new("frame"),
            support::number_output(tui_vfx_contract::GraphValueShape::FrameValue),
        ),
        (
            tui_vfx_contract::EffectOutputId::new("field"),
            support::number_output(tui_vfx_contract::GraphValueShape::CellField),
        ),
    ]);
    graph
        .effects
        .insert(EffectId::new("terminal.producer"), producer_descriptor);
    graph.nodes.insert(
        NodeId::new("frameProducer"),
        output_from_effect(
            NodeSpec {
                id: NodeId::new("frameProducer"),
                effect: EffectId::new("terminal.producer"),
                inputs: BTreeMap::from([(EffectInputId::new("amount"), literal_source())]),
                outputs: BTreeMap::new(),
                scope: None,
                cell_write_policy: None,
                role_write_policy: None,
            },
            "sharedValue",
            "frame",
        ),
    );
    graph.nodes.insert(
        NodeId::new("fieldProducer"),
        output_from_effect(
            NodeSpec {
                id: NodeId::new("fieldProducer"),
                effect: EffectId::new("terminal.producer"),
                inputs: BTreeMap::from([(EffectInputId::new("amount"), literal_source())]),
                outputs: BTreeMap::new(),
                scope: None,
                cell_write_policy: None,
                role_write_policy: None,
            },
            "sharedValue",
            "field",
        ),
    );
    graph
        .order
        .extend([NodeId::new("frameProducer"), NodeId::new("fieldProducer")]);

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::GraphValueShapeMismatch { id, .. })
            if id.as_str() == "sharedValue"
    ));
}

#[test]
fn descriptor_rejects_node_output_not_declared_by_effect() {
    let mut graph = base_graph(literal_source());
    let node = graph.nodes.get(&NodeId::new("fadeIn")).unwrap().clone();
    graph.nodes.insert(
        NodeId::new("fadeIn"),
        output_from_effect(node, "dimFactor", "undeclared"),
    );

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownEffectOutput { output, .. })
            if output.as_str() == "undeclared"
    ));
}

#[test]
fn node_output_from_input_rejects_unknown_input() {
    let mut graph = base_graph(literal_source());
    let node = graph.nodes.get(&NodeId::new("fadeIn")).unwrap().clone();
    graph.nodes.insert(
        NodeId::new("fadeIn"),
        output_from_input(node, "dimFactor", "unknown"),
    );

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::UnknownNodeOutputInput { input, .. })
            if input.as_str() == "unknown"
    ));
}

#[test]
fn graph_value_source_is_rejected_in_binding_context() {
    let mut graph = base_graph(literal_source());
    graph.bindings = vec![binding_to("opacity", graph_value_source("dimFactor"))];

    assert!(matches!(
        graph.validate(),
        Err(DescriptorValidationError::GraphValueSourceNotAllowed { id })
            if id.as_str() == "dimFactor"
    ));
}
