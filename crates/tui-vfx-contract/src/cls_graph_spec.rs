// <FILE>crates/tui-vfx-contract/src/cls_graph_spec.rs</FILE> - <DESC>Canonical graph container contract DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G3: add optional execution topology while keeping linear order fallback.</WCTX>
// <CLOG>0.2.0: MINOR — add optional topology and delegate validation to an OFPF-sized helper.
// 0.1.0: INIT — add canonical graph DTO and validation helpers without runtime execution.</CLOG>

use std::collections::BTreeMap;

use crate::{
    BindingSpec, DescriptorValidationError, EffectDescriptor, EffectId, GraphId, GraphStep, NodeId,
    NodeSpec, ParameterId, ParameterSpec, SignalId, SignalSpec,
    orc_validate_graph_spec::validate_graph_spec,
};

/// Canonical v3.1 graph container produced by future recipe compilation.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GraphSpec {
    /// Stable canonical graph identifier.
    pub id: GraphId,
    /// Contract graph version string.
    pub version: String,
    /// Public parameters available to graph value sources and bindings.
    #[schemars(transform = add_parameter_key_pattern)]
    pub parameters: BTreeMap<ParameterId, ParameterSpec>,
    /// Host/runtime signals available to graph value sources.
    #[schemars(transform = add_signal_key_pattern)]
    pub signals: BTreeMap<SignalId, SignalSpec>,
    /// Declarative parameter-target bindings owned by this graph.
    pub bindings: Vec<BindingSpec>,
    /// Effect descriptors available to nodes in this graph.
    pub effects: BTreeMap<EffectId, EffectDescriptor>,
    /// Nodes keyed by stable graph-local node id.
    #[schemars(transform = add_node_key_pattern)]
    pub nodes: BTreeMap<NodeId, NodeSpec>,
    /// Deterministic node order by graph-local node id.
    pub order: Vec<NodeId>,
    /// Optional explicit topology. When omitted, `order` is a linear sequence.
    pub topology: Option<GraphStep>,
}

impl GraphSpec {
    /// Validate graph identity, declarations, node compatibility, bindings, and order.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        validate_graph_spec(self)
    }
}

fn add_parameter_key_pattern(schema: &mut schemars::Schema) {
    add_identifier_key_pattern(schema, "Parameter ids");
}

fn add_signal_key_pattern(schema: &mut schemars::Schema) {
    add_identifier_key_pattern(schema, "Signal ids");
}

fn add_node_key_pattern(schema: &mut schemars::Schema) {
    add_identifier_key_pattern(schema, "Node ids");
}

fn add_identifier_key_pattern(schema: &mut schemars::Schema, description_prefix: &str) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": format!("{description_prefix} must start with an ASCII letter and then contain only ASCII letters, digits, underscores, or hyphens."),
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*$"
        })
        .to_value(),
    );
}

// <FILE>crates/tui-vfx-contract/src/cls_graph_spec.rs</FILE> - <DESC>Canonical graph container contract DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
