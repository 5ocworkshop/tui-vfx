// <FILE>crates/tui-vfx-contract/tests/support/mod.rs</FILE> - <DESC>Shared graph contract test fixtures</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: add graph value validation fixtures.</WCTX>
// <CLOG>0.2.0: MINOR — add graph value output and source fixture helpers.
// 0.1.0: INIT — shared descriptor, value, parameter, signal, and graph builders.</CLOG>

#![allow(dead_code)]

use std::collections::BTreeMap;

use tui_vfx_contract::{
    BindingMode, BindingSpec, BindingTarget, CellAccess, CellChannel, CellWritePolicy,
    CoordinateSpace, EffectCompletion, EffectDescriptor, EffectDomain, EffectId, EffectInputId,
    EffectInputSpec, EffectLifecycle, EffectOutputId, EffectOutputSpec, GraphId, GraphSpec,
    GraphValueId, GraphValueKind, GraphValueShape, NodeId, NodeOutputSource, NodeOutputSpec,
    NodeSpec, NumericRange, ParameterId, ParameterSpec, RoleSpace, RoleWritePolicy,
    RoleWritePolicyKind, RuntimeMutability, ScopeKind, ScopeSpec, ScopeSupport, SignalId,
    SignalSpec, Value, ValueKind, ValueSource, ValueSpec, WriteSupport,
};
pub fn ratio_value_spec(default: Option<Value>) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Number,
        default,
        range: Some(NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        }),
        allowed_values: vec![],
        unit: Some("ratio".to_string()),
        semantic: Some("opacity".to_string()),
    }
}

pub fn text_value_spec(default: Option<&str>) -> ValueSpec {
    ValueSpec {
        kind: ValueKind::Text,
        default: default.map(|value| Value::Text(value.to_string())),
        range: None,
        allowed_values: vec![],
        unit: None,
        semantic: Some("label".to_string()),
    }
}

pub fn parameter(id: &str, value: ValueSpec, bindable: bool) -> ParameterSpec {
    ParameterSpec {
        id: ParameterId::new(id),
        display_name: Some(id.to_string()),
        description: None,
        value,
        bindable,
    }
}

pub fn signal(id: &str, value: ValueSpec) -> SignalSpec {
    SignalSpec {
        id: SignalId::new(id),
        display_name: Some(id.to_string()),
        description: None,
        value,
        required: false,
    }
}

pub fn number_output(shape: GraphValueShape) -> EffectOutputSpec {
    EffectOutputSpec {
        kind: GraphValueKind::Number,
        shape,
        description: Some("Numeric proof output.".to_string()),
    }
}

pub fn opacity_descriptor() -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new("terminal.opacity"),
        version: "0.1.0".to_string(),
        display_name: "Opacity".to_string(),
        category: Some("visual".to_string()),
        domain: EffectDomain::FrameFilter,
        cell_access: CellAccess {
            reads: vec![CellChannel::Foreground],
            writes: vec![CellChannel::Foreground],
        },
        scope_support: ScopeSupport {
            kinds: vec![ScopeKind::All, ScopeKind::Role],
            coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
            role_spaces: vec![RoleSpace::Destination],
        },
        write_support: WriteSupport {
            cell_policies: vec![CellWritePolicy::WriteCell],
            role_policies: vec![RoleWritePolicyKind::PreserveDestination],
        },
        inputs: BTreeMap::from([(
            EffectInputId::new("amount"),
            EffectInputSpec {
                display_name: Some("Amount".to_string()),
                description: Some("Opacity amount.".to_string()),
                value: ratio_value_spec(None),
                optional: false,
                bindable: true,
                runtime_mutability: RuntimeMutability::Runtime,
            },
        )]),
        outputs: BTreeMap::new(),
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: true,
            seekable: true,
            deterministic_with_seed: true,
        },
    }
}

pub fn base_node(source: ValueSource) -> NodeSpec {
    NodeSpec {
        id: NodeId::new("fadeIn"),
        effect: EffectId::new("terminal.opacity"),
        inputs: BTreeMap::from([(EffectInputId::new("amount"), source)]),
        outputs: BTreeMap::new(),
        scope: Some(ScopeSpec::All),
        cell_write_policy: Some(CellWritePolicy::WriteCell),
        role_write_policy: Some(RoleWritePolicy::PreserveDestination),
    }
}

pub fn base_graph(source: ValueSource) -> GraphSpec {
    GraphSpec {
        id: GraphId::new("heroFade"),
        version: "3.1".to_string(),
        parameters: BTreeMap::from([
            (
                ParameterId::new("opacity"),
                parameter("opacity", ratio_value_spec(Some(Value::Number(0.5))), true),
            ),
            (
                ParameterId::new("locked"),
                parameter("locked", ratio_value_spec(Some(Value::Number(0.5))), false),
            ),
            (
                ParameterId::new("title"),
                parameter("title", text_value_spec(Some("READY")), true),
            ),
        ]),
        signals: BTreeMap::from([(
            SignalId::new("audioLevel"),
            signal("audioLevel", ratio_value_spec(Some(Value::Number(0.0)))),
        )]),
        bindings: vec![],
        effects: BTreeMap::from([(EffectId::new("terminal.opacity"), opacity_descriptor())]),
        nodes: BTreeMap::from([(NodeId::new("fadeIn"), base_node(source))]),
        order: vec![NodeId::new("fadeIn")],
        topology: None,
    }
}

pub fn literal_source() -> ValueSource {
    ValueSource::Literal {
        value: Value::Number(0.75),
    }
}

pub fn parameter_source(id: &str) -> ValueSource {
    ValueSource::Parameter {
        id: ParameterId::new(id),
        fallback: None,
    }
}

pub fn signal_source(id: &str) -> ValueSource {
    ValueSource::Signal {
        id: SignalId::new(id),
        fallback: None,
    }
}

pub fn graph_value_source(id: &str) -> ValueSource {
    ValueSource::GraphValue {
        id: GraphValueId::new(id),
        fallback: None,
    }
}

pub fn output_from_input(mut node: NodeSpec, output: &str, input: &str) -> NodeSpec {
    node.outputs = BTreeMap::from([(
        GraphValueId::new(output),
        NodeOutputSpec {
            source: NodeOutputSource::Input {
                id: EffectInputId::new(input),
            },
        },
    )]);
    node
}

pub fn output_from_effect(mut node: NodeSpec, output: &str, effect_output: &str) -> NodeSpec {
    node.outputs = BTreeMap::from([(
        GraphValueId::new(output),
        NodeOutputSpec {
            source: NodeOutputSource::EffectOutput {
                id: EffectOutputId::new(effect_output),
            },
        },
    )]);
    node
}

pub fn binding_to(parameter_id: &str, source: ValueSource) -> BindingSpec {
    BindingSpec {
        target: BindingTarget::Parameter {
            id: ParameterId::new(parameter_id),
        },
        source,
        mode: BindingMode::Replace,
    }
}

// <FILE>crates/tui-vfx-contract/tests/support/mod.rs</FILE> - <DESC>Shared graph contract test fixtures</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
