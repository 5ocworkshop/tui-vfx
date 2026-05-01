// <FILE>crates/tui-vfx-contract/src/cls_node_spec.rs</FILE> - <DESC>Canonical graph node contract DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: add node graph-value output declarations.</WCTX>
// <CLOG>0.2.0: MINOR — add graph-local output map to node DTO.
// 0.1.0: INIT — add node DTO for graph-level descriptor/input validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    CellWritePolicy, EffectId, EffectInputId, GraphValueId, LifecyclePhase, NodeId, NodeOutputSpec,
    RoleWritePolicy, ScopeSpec, ValueSource,
};

/// One effect node in a canonical v3.1 graph.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeSpec {
    /// Stable graph-local node identifier.
    pub id: NodeId,
    /// Effect descriptor used by this node.
    pub effect: EffectId,
    /// Declarative values supplied to descriptor-local effect inputs.
    #[schemars(transform = add_effect_input_key_pattern)]
    pub inputs: BTreeMap<EffectInputId, ValueSource>,
    /// Graph-local values this node publishes after input resolution/effect execution.
    #[serde(default)]
    #[schemars(transform = add_graph_value_key_pattern)]
    pub outputs: BTreeMap<GraphValueId, NodeOutputSpec>,
    /// Optional lifecycle phases in which this node is active; empty means all phases.
    #[serde(default)]
    pub active_phases: Vec<LifecyclePhase>,
    /// Optional scope limiting where this node applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeSpec>,
    /// Optional cell write policy requested by this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_write_policy: Option<CellWritePolicy>,
    /// Optional role write policy requested by this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_write_policy: Option<RoleWritePolicy>,
}

fn add_effect_input_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Effect input ids must start with an ASCII letter and then contain only ASCII letters, digits, underscores, or hyphens.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*$"
        })
        .to_value(),
    );
}

fn add_graph_value_key_pattern(schema: &mut schemars::Schema) {
    schema.insert(
        "propertyNames".to_string(),
        schemars::json_schema!({
            "description": "Graph value ids must start with an ASCII letter and then contain only ASCII letters, digits, underscores, or hyphens.",
            "type": "string",
            "pattern": "^[A-Za-z][A-Za-z0-9_-]*$"
        })
        .to_value(),
    );
}

// <FILE>crates/tui-vfx-contract/src/cls_node_spec.rs</FILE> - <DESC>Canonical graph node contract DTO</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
