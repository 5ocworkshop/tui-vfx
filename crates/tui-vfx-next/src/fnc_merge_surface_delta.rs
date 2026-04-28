// <FILE>crates/tui-vfx-next/src/fnc_merge_surface_delta.rs</FILE> - <DESC>Merge proof surface deltas with conflict policy</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G3: apply channel-aware parallel branch merge semantics.</WCTX>
// <CLOG>0.1.0: INIT — compose different channels and enforce same-channel policy.</CLOG>

use crate::{
    GraphExecutionError, ParallelMergePolicy, Surface, SurfaceDelta,
    fnc_apply_surface_delta::apply_surface_delta,
};

pub(crate) fn merge_surface_delta(
    surface: &mut Surface,
    merged: &mut SurfaceDelta,
    incoming: SurfaceDelta,
    policy: ParallelMergePolicy,
) -> Result<(), GraphExecutionError> {
    for delta in incoming.into_writes() {
        if let Some(prior_node) = merged.writer(delta.x, delta.y, delta.channel)
            && policy == ParallelMergePolicy::ErrorOnSameChannelConflict
        {
            return Err(GraphExecutionError::ParallelMergeConflict {
                x: delta.x,
                y: delta.y,
                channel: delta.channel,
                prior_node: prior_node.clone(),
                conflicting_node: delta.node,
            });
        }
        apply_surface_delta(surface, &delta);
        merged.set(delta);
    }
    Ok(())
}

// <FILE>crates/tui-vfx-next/src/fnc_merge_surface_delta.rs</FILE> - <DESC>Merge proof surface deltas with conflict policy</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
