// <FILE>crates/tui-vfx-next/tests/support/mod.rs</FILE> - <DESC>Shared graph execution proof fixtures</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G3: add topology and channel-specific proof fixtures.</WCTX>
// <CLOG>0.2.0: MINOR — add channel proof descriptors, color nodes, and topology-ready graphs.
// 0.1.0: INIT — shared GraphSpec, descriptor, node, surface, and value builders.</CLOG>

#![allow(dead_code)]

use std::collections::BTreeMap;

use tui_vfx_next::{
    BindingMode, BindingSpec, BindingTarget, CellAccess, CellChannel, CellWritePolicy,
    CoordinateSpace, EffectCompletion, EffectDescriptor, EffectDomain, EffectId, EffectInputId,
    EffectInputSpec, EffectLifecycle, GraphId, GraphSpec, NodeId, NodeSpec, NumericRange,
    ParameterId, ParameterSpec, RoleSpace, RoleWritePolicy, RoleWritePolicyKind, RuntimeMutability,
    ScopeKind, ScopeSpec, ScopeSupport, SignalId, SignalSpec, Surface, Value, ValueKind,
    ValueSource, ValueSpec, WriteSupport,
};
use tui_vfx_types::{Cell, Color, Modifiers, RoleTag};

pub fn cell(ch: char) -> Cell {
    Cell::styled(ch, Color::WHITE, Color::BLACK, Modifiers::NONE)
}

pub fn surface_with_cell(ch: char, role: RoleTag) -> Surface {
    let mut surface = Surface::new(1, 1, role.clone());
    surface.set_cell_and_role(0, 0, cell(ch), role);
    surface
}

pub fn value_spec(kind: ValueKind, default: Option<Value>) -> ValueSpec {
    ValueSpec {
        kind,
        default,
        range: Some(NumericRange {
            min: Some(0.0),
            max: Some(1.0),
        })
        .filter(|_| {
            matches!(
                kind,
                ValueKind::Number | ValueKind::Integer | ValueKind::Duration
            )
        }),
        allowed_values: vec![],
        unit: None,
        semantic: None,
    }
}

pub fn parameter(id: &str, default: Value) -> ParameterSpec {
    ParameterSpec {
        id: ParameterId::new(id),
        display_name: Some(id.to_string()),
        description: None,
        value: value_spec(default.kind(), Some(default)),
        bindable: true,
    }
}

pub fn signal(id: &str, kind: ValueKind, default: Option<Value>, required: bool) -> SignalSpec {
    SignalSpec {
        id: SignalId::new(id),
        display_name: Some(id.to_string()),
        description: None,
        value: value_spec(kind, default),
        required,
    }
}

pub fn descriptor(effect: &str, input: &str, kind: ValueKind) -> EffectDescriptor {
    EffectDescriptor {
        id: EffectId::new(effect),
        version: "0.1.0".to_string(),
        display_name: effect.to_string(),
        category: Some("proof".to_string()),
        domain: EffectDomain::FrameFilter,
        cell_access: CellAccess {
            reads: vec![
                CellChannel::Glyph,
                CellChannel::Foreground,
                CellChannel::Background,
            ],
            writes: vec![
                CellChannel::Glyph,
                CellChannel::Foreground,
                CellChannel::Background,
                CellChannel::Role,
            ],
        },
        scope_support: ScopeSupport {
            kinds: vec![ScopeKind::All, ScopeKind::Role],
            coordinate_spaces: vec![CoordinateSpace::DestinationLocal],
            role_spaces: vec![RoleSpace::SampledSource, RoleSpace::Destination],
        },
        write_support: WriteSupport {
            cell_policies: vec![
                CellWritePolicy::WriteCell,
                CellWritePolicy::SkipTransparentEmpty,
            ],
            role_policies: vec![
                RoleWritePolicyKind::PreserveDestination,
                RoleWritePolicyKind::CopySampledSource,
                RoleWritePolicyKind::SetExplicit,
            ],
        },
        inputs: BTreeMap::from([(
            EffectInputId::new(input),
            EffectInputSpec {
                display_name: Some(input.to_string()),
                description: None,
                value: value_spec(kind, None),
                bindable: true,
                runtime_mutability: RuntimeMutability::Runtime,
            },
        )]),
        lifecycle: EffectLifecycle {
            completion: EffectCompletion::Instant,
            resettable: true,
            seekable: true,
            deterministic_with_seed: true,
        },
    }
}

pub fn copy_descriptor() -> EffectDescriptor {
    let mut descriptor = descriptor("proof.copy", "unused", ValueKind::Text);
    descriptor.inputs.clear();
    descriptor
}

pub fn standard_effects() -> BTreeMap<EffectId, EffectDescriptor> {
    BTreeMap::from([
        (EffectId::new("proof.copy"), copy_descriptor()),
        (
            EffectId::new("proof.replaceGlyph"),
            descriptor("proof.replaceGlyph", "glyph", ValueKind::Text),
        ),
        (
            EffectId::new("proof.dim"),
            descriptor("proof.dim", "factor", ValueKind::Number),
        ),
        (
            EffectId::new("proof.explicitRoleWrite"),
            descriptor("proof.explicitRoleWrite", "role", ValueKind::Role),
        ),
        (
            EffectId::new("proof.setForeground"),
            descriptor("proof.setForeground", "color", ValueKind::Color),
        ),
        (
            EffectId::new("proof.setBackground"),
            descriptor("proof.setBackground", "color", ValueKind::Color),
        ),
    ])
}

pub fn node(id: &str, effect: &str, inputs: BTreeMap<EffectInputId, ValueSource>) -> NodeSpec {
    NodeSpec {
        id: NodeId::new(id),
        effect: EffectId::new(effect),
        inputs,
        scope: Some(ScopeSpec::All),
        cell_write_policy: Some(CellWritePolicy::WriteCell),
        role_write_policy: None,
    }
}

pub fn graph(nodes: Vec<NodeSpec>) -> GraphSpec {
    GraphSpec {
        id: GraphId::new("proofGraph"),
        version: "3.1".to_string(),
        parameters: BTreeMap::from([(
            ParameterId::new("glyphParam"),
            parameter("glyphParam", Value::Text("P".to_string())),
        )]),
        signals: BTreeMap::from([
            (
                SignalId::new("glyphSignal"),
                signal("glyphSignal", ValueKind::Text, None, false),
            ),
            (
                SignalId::new("level"),
                signal("level", ValueKind::Number, None, true),
            ),
        ]),
        bindings: vec![],
        effects: standard_effects(),
        order: nodes.iter().map(|node| node.id.clone()).collect(),
        topology: None,
        nodes: nodes
            .into_iter()
            .map(|node| (node.id.clone(), node))
            .collect(),
    }
}

pub fn glyph_node(id: &str, source: ValueSource) -> NodeSpec {
    node(
        id,
        "proof.replaceGlyph",
        BTreeMap::from([(EffectInputId::new("glyph"), source)]),
    )
}

pub fn dim_node(id: &str, source: ValueSource) -> NodeSpec {
    node(
        id,
        "proof.dim",
        BTreeMap::from([(EffectInputId::new("factor"), source)]),
    )
}

pub fn role_node(id: &str, role: RoleTag) -> NodeSpec {
    let mut node = node(
        id,
        "proof.explicitRoleWrite",
        BTreeMap::from([(
            EffectInputId::new("role"),
            ValueSource::Literal {
                value: Value::Role(role.clone()),
            },
        )]),
    );
    node.role_write_policy = Some(RoleWritePolicy::SetExplicit { role });
    node
}

pub fn foreground_node(id: &str, color: Color) -> NodeSpec {
    node(
        id,
        "proof.setForeground",
        BTreeMap::from([(
            EffectInputId::new("color"),
            ValueSource::Literal {
                value: Value::Color(color),
            },
        )]),
    )
}

pub fn background_node(id: &str, color: Color) -> NodeSpec {
    node(
        id,
        "proof.setBackground",
        BTreeMap::from([(
            EffectInputId::new("color"),
            ValueSource::Literal {
                value: Value::Color(color),
            },
        )]),
    )
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

// <FILE>crates/tui-vfx-next/tests/support/mod.rs</FILE> - <DESC>Shared graph execution proof fixtures</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
