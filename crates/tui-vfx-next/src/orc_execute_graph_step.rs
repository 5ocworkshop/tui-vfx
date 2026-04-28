// <FILE>crates/tui-vfx-next/src/orc_execute_graph_step.rs</FILE> - <DESC>Coordinate topology graph-step proof execution</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: keep GraphExecutor topology recursion below OFPF limits.</WCTX>
// <CLOG>0.1.0: INIT — execute node, sequence, and parallel topology steps.</CLOG>

use std::collections::BTreeMap;

use crate::{
    EffectInputId, GraphExecutionError, GraphExecutor, GraphSpec, GraphStep, NodeId,
    ParallelMergePolicy, Surface, SurfaceDelta, Value,
    fnc_annotate_node_diagnostics::annotate_node_diagnostics,
    fnc_merge_surface_delta::merge_surface_delta, fnc_surface_delta_between::surface_delta_between,
    orc_apply_proof_node::apply_proof_node,
};

impl GraphExecutor {
    pub(crate) fn execute_step(
        &self,
        graph: &GraphSpec,
        resolved_inputs: &BTreeMap<NodeId, BTreeMap<EffectInputId, Value>>,
        step: &GraphStep,
        input: &Surface,
    ) -> Result<StepExecution, GraphExecutionError> {
        match step {
            GraphStep::Node { node } => self.execute_node(graph, resolved_inputs, node, input),
            GraphStep::Sequence { children } => {
                self.execute_sequence(graph, resolved_inputs, children, input)
            }
            GraphStep::Parallel {
                children,
                merge_policy,
            } => self.execute_parallel(graph, resolved_inputs, children, *merge_policy, input),
        }
    }

    fn execute_node(
        &self,
        graph: &GraphSpec,
        resolved_inputs: &BTreeMap<NodeId, BTreeMap<EffectInputId, Value>>,
        node_id: &NodeId,
        input: &Surface,
    ) -> Result<StepExecution, GraphExecutionError> {
        let node = graph
            .nodes
            .get(node_id)
            .expect("GraphSpec validation requires topology nodes to exist");
        let adapter = self
            .adapters
            .get(&node.effect)
            .expect("adapter preflight requires all node effects to be registered");
        let mut next = input.clone();
        let mut outcome = apply_proof_node(
            *adapter,
            &node.effect,
            node,
            &resolved_inputs[node_id],
            input,
            &mut next,
        )?;
        annotate_node_diagnostics(&mut outcome, 0, node_id);
        let delta = surface_delta_between(input, &next, node_id);
        Ok(StepExecution {
            surface: next,
            delta,
            executed_nodes: vec![node_id.clone()],
            matched_cells: outcome.matched_cells,
            written_cells: outcome.written_cells,
            diagnostics: outcome.diagnostics,
        })
    }

    fn execute_sequence(
        &self,
        graph: &GraphSpec,
        resolved_inputs: &BTreeMap<NodeId, BTreeMap<EffectInputId, Value>>,
        children: &[GraphStep],
        input: &Surface,
    ) -> Result<StepExecution, GraphExecutionError> {
        let mut combined = StepExecution::from_surface(input.clone());
        for child in children {
            let child_execution =
                self.execute_step(graph, resolved_inputs, child, &combined.surface)?;
            combined.surface = child_execution.surface.clone();
            combined.delta.overlay(child_execution.delta.clone());
            combined.extend_counts(child_execution);
        }
        Ok(combined)
    }

    fn execute_parallel(
        &self,
        graph: &GraphSpec,
        resolved_inputs: &BTreeMap<NodeId, BTreeMap<EffectInputId, Value>>,
        children: &[GraphStep],
        policy: ParallelMergePolicy,
        input: &Surface,
    ) -> Result<StepExecution, GraphExecutionError> {
        let mut joined = input.clone();
        let mut merged_delta = SurfaceDelta::new();
        let mut combined = StepExecution::from_surface(input.clone());
        for child in children {
            let child_execution = self.execute_step(graph, resolved_inputs, child, input)?;
            merge_surface_delta(
                &mut joined,
                &mut merged_delta,
                child_execution.delta.clone(),
                policy,
            )?;
            combined.extend_counts(child_execution);
        }
        combined.surface = joined;
        combined.delta = merged_delta;
        Ok(combined)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StepExecution {
    pub(crate) surface: Surface,
    pub(crate) delta: SurfaceDelta,
    pub(crate) executed_nodes: Vec<NodeId>,
    pub(crate) matched_cells: usize,
    pub(crate) written_cells: usize,
    pub(crate) diagnostics: Vec<crate::SurfaceDiagnostic>,
}

impl StepExecution {
    fn from_surface(surface: Surface) -> Self {
        Self {
            surface,
            delta: SurfaceDelta::new(),
            executed_nodes: vec![],
            matched_cells: 0,
            written_cells: 0,
            diagnostics: vec![],
        }
    }

    fn extend_counts(&mut self, other: Self) {
        self.executed_nodes.extend(other.executed_nodes);
        self.matched_cells += other.matched_cells;
        self.written_cells += other.written_cells;
        self.diagnostics.extend(other.diagnostics);
    }
}

pub(crate) fn linear_order_step(order: &[NodeId]) -> GraphStep {
    GraphStep::sequence(order.iter().cloned().map(GraphStep::node).collect())
}

// <FILE>crates/tui-vfx-next/src/orc_execute_graph_step.rs</FILE> - <DESC>Coordinate topology graph-step proof execution</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
