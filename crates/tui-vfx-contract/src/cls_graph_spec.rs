// <FILE>crates/tui-vfx-contract/src/cls_graph_spec.rs</FILE> - <DESC>Canonical graph container contract DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G1: validate descriptors, parameters, signals, bindings, nodes, and order together.</WCTX>
// <CLOG>0.1.0: INIT — add canonical graph DTO and validation helpers without runtime execution.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    BindingSpec, DescriptorValidationError, EffectDescriptor, EffectId, GraphId, NodeId, NodeSpec,
    ParameterId, ParameterSpec, SignalId, SignalSpec,
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
}

impl GraphSpec {
    /// Validate graph identity, declarations, node compatibility, bindings, and order.
    pub fn validate(&self) -> Result<(), DescriptorValidationError> {
        if !self.id.is_valid() {
            return Err(DescriptorValidationError::InvalidGraphId {
                id: self.id.clone(),
            });
        }

        self.validate_parameters()?;
        self.validate_signals()?;
        self.validate_bindings()?;
        self.validate_effects()?;
        self.validate_nodes()?;
        self.validate_order()?;
        Ok(())
    }

    fn validate_parameters(&self) -> Result<(), DescriptorValidationError> {
        for (id, parameter) in &self.parameters {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidParameterId { id: id.clone() });
            }
            if &parameter.id != id {
                return Err(DescriptorValidationError::ParameterIdMismatch {
                    key: id.clone(),
                    parameter: parameter.id.clone(),
                });
            }
            parameter.validate()?;
        }
        Ok(())
    }

    fn validate_signals(&self) -> Result<(), DescriptorValidationError> {
        for (id, signal) in &self.signals {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidSignalId { id: id.clone() });
            }
            if &signal.id != id {
                return Err(DescriptorValidationError::SignalIdMismatch {
                    key: id.clone(),
                    signal: signal.id.clone(),
                });
            }
            signal.validate()?;
        }
        Ok(())
    }

    fn validate_bindings(&self) -> Result<(), DescriptorValidationError> {
        for binding in &self.bindings {
            binding.validate(&self.parameters, &self.signals)?;
        }
        Ok(())
    }

    fn validate_effects(&self) -> Result<(), DescriptorValidationError> {
        for (id, effect) in &self.effects {
            if &effect.id != id {
                return Err(DescriptorValidationError::EffectIdMismatch {
                    key: id.clone(),
                    effect: effect.id.clone(),
                });
            }
            effect.validate_inputs()?;
        }
        Ok(())
    }

    fn validate_nodes(&self) -> Result<(), DescriptorValidationError> {
        for (id, node) in &self.nodes {
            if !id.is_valid() {
                return Err(DescriptorValidationError::InvalidNodeId { id: id.clone() });
            }
            if &node.id != id {
                return Err(DescriptorValidationError::NodeIdMismatch {
                    key: id.clone(),
                    node: node.id.clone(),
                });
            }
            self.validate_node(node)?;
        }
        Ok(())
    }

    fn validate_node(&self, node: &NodeSpec) -> Result<(), DescriptorValidationError> {
        let effect = self.effects.get(&node.effect).ok_or_else(|| {
            DescriptorValidationError::UnknownEffect {
                id: node.effect.clone(),
            }
        })?;
        for (input_id, source) in &node.inputs {
            if !input_id.is_valid() {
                return Err(DescriptorValidationError::InvalidInputId {
                    id: input_id.clone(),
                });
            }
            let input = effect.inputs.get(input_id).ok_or_else(|| {
                DescriptorValidationError::UnknownNodeInput {
                    effect: node.effect.clone(),
                    input: input_id.clone(),
                }
            })?;
            source.validate_kind(input.value.kind, &self.parameters, &self.signals)?;
        }

        for (input_id, input) in &effect.inputs {
            if !node.inputs.contains_key(input_id) && input.value.default.is_none() {
                return Err(DescriptorValidationError::MissingRequiredNodeInput {
                    effect: node.effect.clone(),
                    input: input_id.clone(),
                });
            }
        }

        if let Some(scope) = &node.scope {
            effect.validate_scope(scope)?;
        }
        if let Some(policy) = node.cell_write_policy {
            effect.validate_cell_write_policy(policy)?;
        }
        if let Some(policy) = &node.role_write_policy {
            effect.validate_role_write_policy(policy)?;
        }

        Ok(())
    }

    fn validate_order(&self) -> Result<(), DescriptorValidationError> {
        let mut seen = BTreeSet::new();
        for id in &self.order {
            if !self.nodes.contains_key(id) {
                return Err(DescriptorValidationError::UnknownOrderNode { id: id.clone() });
            }
            if !seen.insert(id) {
                return Err(DescriptorValidationError::DuplicateOrderNode { id: id.clone() });
            }
        }
        for id in self.nodes.keys() {
            if !seen.contains(id) {
                return Err(DescriptorValidationError::NodeMissingFromOrder { id: id.clone() });
            }
        }
        Ok(())
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
// <VERS>END OF VERSION: 0.1.0</VERS>
