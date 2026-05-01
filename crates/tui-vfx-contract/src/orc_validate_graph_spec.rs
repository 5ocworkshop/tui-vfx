// <FILE>crates/tui-vfx-contract/src/orc_validate_graph_spec.rs</FILE> - <DESC>Validate canonical graph contracts</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>K2.13 schema decision burn-down: honor optional effect inputs during node validation.</WCTX>
// <CLOG>0.3.0: MINOR — allow descriptor inputs marked optional to be omitted without defaults.
// 0.2.0: MINOR — validate graph-local value sources and node output declarations.</CLOG>

use std::collections::BTreeSet;

use crate::{
    DescriptorValidationError, GraphSpec, GraphStep, GraphValueKinds, NodeId, NodeOutputSource,
    NodeSpec, fnc_collect_graph_value_kinds::collect_graph_value_kinds,
};

pub(crate) fn validate_graph_spec(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    if !graph.id.is_valid() {
        return Err(DescriptorValidationError::InvalidGraphId {
            id: graph.id.clone(),
        });
    }

    validate_parameters(graph)?;
    validate_signals(graph)?;
    validate_bindings(graph)?;
    validate_effects(graph)?;
    let graph_values = collect_graph_value_kinds(graph)?;
    validate_nodes(graph, &graph_values)?;
    validate_order(graph)?;
    validate_topology(graph)?;
    Ok(())
}

fn validate_parameters(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    for (id, parameter) in &graph.parameters {
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

fn validate_signals(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    for (id, signal) in &graph.signals {
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

fn validate_bindings(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    for binding in &graph.bindings {
        binding.validate(&graph.parameters, &graph.signals)?;
    }
    Ok(())
}

fn validate_effects(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    for (id, effect) in &graph.effects {
        if &effect.id != id {
            return Err(DescriptorValidationError::EffectIdMismatch {
                key: id.clone(),
                effect: effect.id.clone(),
            });
        }
        effect.validate_io()?;
    }
    Ok(())
}

fn validate_nodes(
    graph: &GraphSpec,
    graph_values: &GraphValueKinds,
) -> Result<(), DescriptorValidationError> {
    for (id, node) in &graph.nodes {
        if !id.is_valid() {
            return Err(DescriptorValidationError::InvalidNodeId { id: id.clone() });
        }
        if &node.id != id {
            return Err(DescriptorValidationError::NodeIdMismatch {
                key: id.clone(),
                node: node.id.clone(),
            });
        }
        validate_node(graph, node, graph_values)?;
    }
    Ok(())
}

fn validate_node(
    graph: &GraphSpec,
    node: &NodeSpec,
    graph_values: &GraphValueKinds,
) -> Result<(), DescriptorValidationError> {
    let effect = graph.effects.get(&node.effect).ok_or_else(|| {
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
        source.validate_kind_with_graph_values(
            input.value.kind,
            &graph.parameters,
            &graph.signals,
            Some(graph_values),
        )?;
    }

    for (input_id, input) in &effect.inputs {
        if !node.inputs.contains_key(input_id) && input.value.default.is_none() && !input.optional {
            return Err(DescriptorValidationError::MissingRequiredNodeInput {
                effect: node.effect.clone(),
                input: input_id.clone(),
            });
        }
    }

    for (output_id, output) in &node.outputs {
        if !output_id.is_valid() {
            return Err(DescriptorValidationError::InvalidGraphValueId {
                id: output_id.clone(),
            });
        }
        match &output.output_source {
            NodeOutputSource::EffectOutput { id } => {
                if !id.is_valid() {
                    return Err(DescriptorValidationError::InvalidEffectOutputId {
                        id: id.clone(),
                    });
                }
                if !effect.outputs.contains_key(id) {
                    return Err(DescriptorValidationError::UnknownEffectOutput {
                        effect: node.effect.clone(),
                        output: id.clone(),
                    });
                }
            }
            NodeOutputSource::Input { id } => {
                if !id.is_valid() {
                    return Err(DescriptorValidationError::InvalidInputId { id: id.clone() });
                }
                if !effect.inputs.contains_key(id) {
                    return Err(DescriptorValidationError::UnknownNodeOutputInput {
                        effect: node.effect.clone(),
                        input: id.clone(),
                    });
                }
            }
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

fn validate_order(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    let mut seen = BTreeSet::new();
    for id in &graph.order {
        if !graph.nodes.contains_key(id) {
            return Err(DescriptorValidationError::UnknownOrderNode { id: id.clone() });
        }
        if !seen.insert(id.clone()) {
            return Err(DescriptorValidationError::DuplicateOrderNode { id: id.clone() });
        }
    }
    validate_node_coverage(graph.nodes.keys(), &seen)
}

fn validate_topology(graph: &GraphSpec) -> Result<(), DescriptorValidationError> {
    let Some(topology) = &graph.topology else {
        return Ok(());
    };
    let mut seen = BTreeSet::new();
    collect_topology_nodes(topology, graph, &mut seen)?;
    validate_node_coverage(graph.nodes.keys(), &seen)
}

fn collect_topology_nodes(
    step: &GraphStep,
    graph: &GraphSpec,
    seen: &mut BTreeSet<NodeId>,
) -> Result<(), DescriptorValidationError> {
    match step {
        GraphStep::Node { node } => {
            if !graph.nodes.contains_key(node) {
                return Err(DescriptorValidationError::UnknownOrderNode { id: node.clone() });
            }
            if !seen.insert(node.clone()) {
                return Err(DescriptorValidationError::DuplicateOrderNode { id: node.clone() });
            }
        }
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            for child in children {
                collect_topology_nodes(child, graph, seen)?;
            }
        }
    }
    Ok(())
}

fn validate_node_coverage<'a>(
    nodes: impl Iterator<Item = &'a NodeId>,
    seen: &BTreeSet<NodeId>,
) -> Result<(), DescriptorValidationError> {
    for id in nodes {
        if !seen.contains(id) {
            return Err(DescriptorValidationError::NodeMissingFromOrder { id: id.clone() });
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract/src/orc_validate_graph_spec.rs</FILE> - <DESC>Validate canonical graph contracts</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
