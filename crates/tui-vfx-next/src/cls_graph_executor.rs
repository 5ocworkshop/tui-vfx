// <FILE>crates/tui-vfx-next/src/cls_graph_executor.rs</FILE> - <DESC>Canonical graph proof executor</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: execute validated GraphSpec values with toy proof adapters.</WCTX>
// <CLOG>0.1.0: INIT — add validation-gated, ordered graph proof execution without runtime stores.</CLOG>

use std::collections::BTreeMap;

use crate::{
    EffectId, EffectInputId, GraphExecutionContext, GraphExecutionError, GraphExecutionOutcome,
    GraphSpec, NodeId, NodeSpec, ProofEffectAdapter, Surface, Value,
    fnc_annotate_node_diagnostics::annotate_node_diagnostics,
    fnc_apply_proof_node::apply_proof_node, fnc_resolve_value_source::resolve_value_source,
};

/// Validation-gated proof executor for canonical graph contracts.
#[derive(Clone, Debug, Default)]
pub struct GraphExecutor {
    adapters: BTreeMap<EffectId, ProofEffectAdapter>,
}

impl GraphExecutor {
    /// Create an executor with no registered proof adapters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an executor with the standard G2 proof adapter ids.
    pub fn with_standard_proof_adapters() -> Self {
        Self::new()
            .with_adapter(EffectId::new("proof.copy"), ProofEffectAdapter::Copy)
            .with_adapter(
                EffectId::new("proof.replaceGlyph"),
                ProofEffectAdapter::ReplaceGlyph,
            )
            .with_adapter(EffectId::new("proof.dim"), ProofEffectAdapter::Dim)
            .with_adapter(
                EffectId::new("proof.explicitRoleWrite"),
                ProofEffectAdapter::ExplicitRoleWrite,
            )
    }

    /// Register or replace one proof adapter.
    pub fn with_adapter(mut self, effect: EffectId, adapter: ProofEffectAdapter) -> Self {
        self.adapters.insert(effect, adapter);
        self
    }

    /// Execute a validated graph against an input surface and value snapshot.
    pub fn execute(
        &self,
        graph: &GraphSpec,
        input: &Surface,
        context: &GraphExecutionContext,
    ) -> Result<GraphExecutionOutcome, GraphExecutionError> {
        graph.validate()?;
        self.validate_adapters(graph)?;
        let resolved_inputs = self.resolve_all_inputs(graph, context)?;

        let mut current = input.clone();
        let mut executed_nodes = Vec::with_capacity(graph.order.len());
        let mut diagnostics = Vec::new();
        let mut matched_cells = 0;
        let mut written_cells = 0;

        for (index, node_id) in graph.order.iter().enumerate() {
            let node = graph
                .nodes
                .get(node_id)
                .expect("GraphSpec validation requires ordered nodes to exist");
            let adapter = self
                .adapters
                .get(&node.effect)
                .expect("adapter preflight requires all node effects to be registered");
            let read = current.clone();
            let mut next = current.clone();
            let mut outcome = apply_proof_node(
                *adapter,
                &node.effect,
                node,
                &resolved_inputs[node_id],
                &read,
                &mut next,
            )?;
            annotate_node_diagnostics(&mut outcome, index, node_id);
            matched_cells += outcome.matched_cells;
            written_cells += outcome.written_cells;
            diagnostics.extend(outcome.diagnostics);
            executed_nodes.push(node_id.clone());
            current = next;
        }

        Ok(GraphExecutionOutcome {
            surface: current,
            executed_nodes,
            matched_cells,
            written_cells,
            diagnostics,
        })
    }

    fn validate_adapters(&self, graph: &GraphSpec) -> Result<(), GraphExecutionError> {
        for node_id in &graph.order {
            let node = graph
                .nodes
                .get(node_id)
                .expect("GraphSpec validation requires ordered nodes to exist");
            if !self.adapters.contains_key(&node.effect) {
                return Err(GraphExecutionError::MissingProofAdapter {
                    effect: node.effect.clone(),
                });
            }
        }
        Ok(())
    }

    fn resolve_all_inputs(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
    ) -> Result<BTreeMap<NodeId, BTreeMap<EffectInputId, Value>>, GraphExecutionError> {
        graph
            .nodes
            .iter()
            .map(|(node_id, node)| {
                Ok((
                    node_id.clone(),
                    self.resolve_node_inputs(graph, context, node)?,
                ))
            })
            .collect()
    }

    fn resolve_node_inputs(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
        node: &NodeSpec,
    ) -> Result<BTreeMap<EffectInputId, Value>, GraphExecutionError> {
        let effect = graph
            .effects
            .get(&node.effect)
            .expect("GraphSpec validation requires node effects to exist");
        let mut resolved = BTreeMap::new();
        for (input_id, input_spec) in &effect.inputs {
            let value = if let Some(source) = node.inputs.get(input_id) {
                resolve_value_source(graph, context, source)?
            } else {
                input_spec
                    .value
                    .default
                    .clone()
                    .expect("GraphSpec validation requires missing inputs to have defaults")
            };
            input_spec.value.validate_value(&value)?;
            resolved.insert(input_id.clone(), value);
        }
        Ok(resolved)
    }
}

// <FILE>crates/tui-vfx-next/src/cls_graph_executor.rs</FILE> - <DESC>Canonical graph proof executor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
