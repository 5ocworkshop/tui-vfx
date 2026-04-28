// <FILE>crates/tui-vfx-contract/tests/test_graph_contract.rs</FILE> - <DESC>Canonical graph source/input validation tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G1: prove canonical graph nodes validate descriptors and ValueSource inputs together.</WCTX>
// <CLOG>0.1.0: INIT — lock literal, parameter, signal, effect, input, and source validation.</CLOG>

mod support;

use std::collections::BTreeMap;

use support::{base_graph, literal_source, parameter_source, signal_source, text_value_spec};
use tui_vfx_contract::{
    DescriptorValidationError, EffectId, EffectInputId, NodeId, ParameterId, ParameterSpec, Value,
    ValueKind, ValueSource,
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
// <VERS>END OF VERSION: 0.1.0</VERS>
