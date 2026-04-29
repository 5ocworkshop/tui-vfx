// <FILE>crates/tui-vfx-next/src/orc_execute_graph_step.rs</FILE> - <DESC>Coordinate topology graph-step proof execution</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase G4: thread value-bus snapshots through topology execution.</WCTX>
// <CLOG>0.2.0: MINOR — add sequence/parallel graph value bus visibility and merge semantics.
// 0.1.0: INIT — execute node, sequence, and parallel topology steps.</CLOG>

use std::collections::BTreeMap;

use crate::{
    GraphExecutionContext, GraphExecutionError, GraphExecutor, GraphSpec, GraphStep, GraphValueBus,
    GraphValueMergePolicy, NodeId, ParallelMergePolicy, StepExecution, Surface, SurfaceDelta,
    fnc_annotate_node_diagnostics::annotate_node_diagnostics,
    fnc_merge_graph_value_delta::{
        merge_graph_value_delta, overlay_graph_value_delta, overlay_graph_values,
    },
    fnc_merge_surface_delta::merge_surface_delta,
    fnc_publish_node_outputs::publish_node_outputs,
    fnc_surface_delta_between::surface_delta_between,
    orc_apply_proof_node::apply_proof_node,
};

impl GraphExecutor {
    pub(crate) fn execute_step(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
        step: &GraphStep,
        input: &Surface,
        graph_values: &GraphValueBus,
    ) -> Result<StepExecution, GraphExecutionError> {
        match step {
            GraphStep::Node { node } => {
                self.execute_node(graph, context, node, input, graph_values)
            }
            GraphStep::Sequence { children } => {
                self.execute_sequence(graph, context, children, input, graph_values)
            }
            GraphStep::Parallel {
                children,
                merge_policy,
                value_merge_policy,
            } => self.execute_parallel(
                graph,
                context,
                children,
                *merge_policy,
                *value_merge_policy,
                input,
                graph_values,
            ),
        }
    }

    fn execute_node(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
        node_id: &NodeId,
        input: &Surface,
        graph_values: &GraphValueBus,
    ) -> Result<StepExecution, GraphExecutionError> {
        let node = graph
            .nodes
            .get(node_id)
            .expect("GraphSpec validation requires topology nodes to exist");
        let adapter = self
            .adapters
            .get(&node.effect)
            .expect("adapter preflight requires all node effects to be registered");
        let resolved_inputs = self.resolve_node_inputs(graph, context, graph_values, node)?;
        let mut next = input.clone();
        let application = apply_proof_node(
            *adapter,
            &node.effect,
            node,
            &resolved_inputs,
            input,
            &mut next,
        )?;
        let mut outcome = application.outcome;
        annotate_node_diagnostics(&mut outcome, 0, node_id);
        let delta = surface_delta_between(input, &next, node_id);
        let value_delta = publish_node_outputs(
            node_id,
            &node.outputs,
            &resolved_inputs,
            &application.effect_outputs,
        )?;
        let mut final_values = graph_values.clone();
        overlay_graph_values(&mut final_values, &value_delta);
        Ok(StepExecution {
            surface: next,
            delta,
            graph_values: final_values,
            value_delta,
            executed_nodes: vec![node_id.clone()],
            matched_cells: outcome.matched_cells,
            written_cells: outcome.written_cells,
            diagnostics: outcome.diagnostics,
        })
    }

    fn execute_sequence(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
        children: &[GraphStep],
        input: &Surface,
        graph_values: &GraphValueBus,
    ) -> Result<StepExecution, GraphExecutionError> {
        let mut combined = StepExecution::from_surface(input.clone(), graph_values.clone());
        for child in children {
            let child_execution = self.execute_step(
                graph,
                context,
                child,
                &combined.surface,
                &combined.graph_values,
            )?;
            combined.surface = child_execution.surface.clone();
            combined.graph_values = child_execution.graph_values.clone();
            combined.delta.overlay(child_execution.delta.clone());
            overlay_graph_value_delta(
                &mut combined.value_delta,
                child_execution.value_delta.clone(),
            );
            combined.extend_counts(child_execution);
        }
        Ok(combined)
    }

    fn execute_parallel(
        &self,
        graph: &GraphSpec,
        context: &GraphExecutionContext,
        children: &[GraphStep],
        policy: ParallelMergePolicy,
        value_policy: GraphValueMergePolicy,
        input: &Surface,
        graph_values: &GraphValueBus,
    ) -> Result<StepExecution, GraphExecutionError> {
        let mut joined = input.clone();
        let mut merged_delta = SurfaceDelta::new();
        let mut merged_value_delta = BTreeMap::new();
        let mut combined = StepExecution::from_surface(input.clone(), graph_values.clone());
        for child in children {
            let child_execution = self.execute_step(graph, context, child, input, graph_values)?;
            merge_surface_delta(
                &mut joined,
                &mut merged_delta,
                child_execution.delta.clone(),
                policy,
            )?;
            merge_graph_value_delta(
                &mut merged_value_delta,
                child_execution.value_delta.clone(),
                value_policy,
            )?;
            combined.extend_counts(child_execution);
        }
        let mut final_values = graph_values.clone();
        overlay_graph_values(&mut final_values, &merged_value_delta);
        combined.surface = joined;
        combined.delta = merged_delta;
        combined.graph_values = final_values;
        combined.value_delta = merged_value_delta;
        Ok(combined)
    }
}

pub(crate) fn linear_order_step(order: &[NodeId]) -> GraphStep {
    GraphStep::sequence(order.iter().cloned().map(GraphStep::node).collect())
}

// <FILE>crates/tui-vfx-next/src/orc_execute_graph_step.rs</FILE> - <DESC>Coordinate topology graph-step proof execution</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
