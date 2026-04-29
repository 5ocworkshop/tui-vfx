// <FILE>crates/tui-vfx-next/src/cls_graph_executor.rs</FILE> - <DESC>Canonical graph proof executor</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase G4: execute graph-local value bus during topology traversal.</WCTX>
// <CLOG>0.3.0: MINOR — resolve node inputs at execution time against the value bus.
// 0.2.0: MINOR — add sequence/parallel topology execution and channel-aware deltas.
// 0.1.0: INIT — add validation-gated, ordered graph proof execution without runtime stores.</CLOG>

use std::collections::BTreeMap;

use crate::{
    EffectId, EffectInputId, GraphExecutionContext, GraphExecutionError, GraphExecutionOutcome,
    GraphSpec, GraphValueId, NodeSpec, ProofEffectAdapter, ProofValue, Surface,
    fnc_resolve_value_source::resolve_value_source, orc_execute_graph_step::linear_order_step,
};

/// Validation-gated proof executor for canonical graph contracts.
#[derive(Clone, Debug, Default)]
pub struct GraphExecutor {
    pub(crate) adapters: BTreeMap<EffectId, ProofEffectAdapter>,
}

impl GraphExecutor {
    /// Create an executor with no registered proof adapters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an executor with the standard proof adapter ids.
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
            .with_adapter(
                EffectId::new("proof.setForeground"),
                ProofEffectAdapter::SetForeground,
            )
            .with_adapter(
                EffectId::new("proof.setBackground"),
                ProofEffectAdapter::SetBackground,
            )
            .with_adapter(
                EffectId::new("proof.consumeNumber"),
                ProofEffectAdapter::ConsumeNumber,
            )
            .with_adapter(
                EffectId::new("proof.spatialScalarField"),
                ProofEffectAdapter::SpatialScalarField,
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

        let step = graph
            .topology
            .clone()
            .unwrap_or_else(|| linear_order_step(&graph.order));
        let graph_values = BTreeMap::new();
        let execution = self.execute_step(graph, context, &step, input, &graph_values)?;

        Ok(GraphExecutionOutcome {
            surface: execution.surface,
            executed_nodes: execution.executed_nodes,
            matched_cells: execution.matched_cells,
            written_cells: execution.written_cells,
            diagnostics: execution.diagnostics,
        })
    }

    fn validate_adapters(&self, graph: &GraphSpec) -> Result<(), GraphExecutionError> {
        for node in graph.nodes.values() {
            if !self.adapters.contains_key(&node.effect) {
                return Err(GraphExecutionError::MissingProofAdapter {
                    effect: node.effect.clone(),
                });
            }
        }
        Ok(())
    }

    pub(crate) fn resolve_node_inputs(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
        graph_values: &BTreeMap<GraphValueId, ProofValue>,
        node: &NodeSpec,
    ) -> Result<BTreeMap<EffectInputId, ProofValue>, GraphExecutionError> {
        let effect = graph
            .effects
            .get(&node.effect)
            .expect("GraphSpec validation requires node effects to exist");
        let mut resolved = BTreeMap::new();
        for (input_id, input_spec) in &effect.inputs {
            let value =
                if let Some(source) = node.inputs.get(input_id) {
                    resolve_value_source(graph, context, graph_values, source)?
                } else {
                    ProofValue::Frame(
                        input_spec.value.default.clone().expect(
                            "GraphSpec validation requires missing inputs to have defaults",
                        ),
                    )
                };
            value.validate_against(&input_spec.value)?;
            resolved.insert(input_id.clone(), value);
        }
        Ok(resolved)
    }
}

// <FILE>crates/tui-vfx-next/src/cls_graph_executor.rs</FILE> - <DESC>Canonical graph proof executor</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
