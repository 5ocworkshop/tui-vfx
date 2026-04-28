// <FILE>crates/tui-vfx-contract/src/cls_node_spec.rs</FILE> - <DESC>Canonical graph node contract DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G1: describe one effect node with declarative ValueSource inputs.</WCTX>
// <CLOG>0.1.0: INIT — add node DTO for graph-level descriptor/input validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    CellWritePolicy, EffectId, EffectInputId, NodeId, RoleWritePolicy, ScopeSpec, ValueSource,
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
    /// Optional scope limiting where this node applies.
    pub scope: Option<ScopeSpec>,
    /// Optional cell write policy requested by this node.
    pub cell_write_policy: Option<CellWritePolicy>,
    /// Optional role write policy requested by this node.
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

// <FILE>crates/tui-vfx-contract/src/cls_node_spec.rs</FILE> - <DESC>Canonical graph node contract DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
