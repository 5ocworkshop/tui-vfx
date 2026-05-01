// <FILE>crates/tui-vfx-compost/src/render/fnc_merge_parallel_surfaces.rs</FILE> - <DESC>Parallel graph surface merge support gate</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Parallel merge validation rejects authored parallel graph topology until native surface and graph-value merge semantics are implemented.</WCTX>
// <CLOG>0.1.0: INIT — detect parallel graph steps for load-time rejection.</CLOG>

use tui_vfx_contract::GraphStep;

pub(crate) fn has_parallel_surface_merge(step: Option<&GraphStep>) -> bool {
    match step {
        Some(GraphStep::Parallel { .. }) => true,
        Some(GraphStep::Sequence { children }) => children
            .iter()
            .any(|child| has_parallel_surface_merge(Some(child))),
        Some(GraphStep::Node { .. }) | None => false,
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_merge_parallel_surfaces.rs</FILE> - <DESC>Parallel graph surface merge support gate</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
