// <FILE>crates/tui-vfx-next/src/fnc_merge_graph_value_delta.rs</FILE> - <DESC>Merge proof graph value deltas with conflict policy</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: keep value-bus merge policy separate from topology recursion.</WCTX>
// <CLOG>0.1.0: INIT — add value-delta merge, bus overlay, and delta overlay helpers.</CLOG>

use crate::{GraphExecutionError, GraphValueBus, GraphValueDelta, GraphValueMergePolicy};

pub(crate) fn merge_graph_value_delta(
    merged: &mut GraphValueDelta,
    incoming: GraphValueDelta,
    policy: GraphValueMergePolicy,
) -> Result<(), GraphExecutionError> {
    for (id, (node, value)) in incoming {
        if let Some((prior_node, _)) = merged.get(&id)
            && policy == GraphValueMergePolicy::ErrorOnSameValueConflict
        {
            return Err(GraphExecutionError::ParallelValueMergeConflict {
                id,
                prior_node: prior_node.clone(),
                conflicting_node: node,
            });
        }
        merged.insert(id, (node, value));
    }
    Ok(())
}

pub(crate) fn overlay_graph_values(target: &mut GraphValueBus, delta: &GraphValueDelta) {
    for (id, (_node, value)) in delta {
        target.insert(id.clone(), value.clone());
    }
}

pub(crate) fn overlay_graph_value_delta(target: &mut GraphValueDelta, delta: GraphValueDelta) {
    for (id, write) in delta {
        target.insert(id, write);
    }
}

// <FILE>crates/tui-vfx-next/src/fnc_merge_graph_value_delta.rs</FILE> - <DESC>Merge proof graph value deltas with conflict policy</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
