// <FILE>crates/tui-vfx-compost/src/render/fnc_is_node_active.rs</FILE> - <DESC>Check whether a node is active under current compost lifecycle support</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Lifecycle-aware graph execution uses the sample's named phase to gate active nodes.</WCTX>
// <CLOG>0.2.0: MINOR — evaluate activePhases against SampleContext lifecycle phase.
// 0.1.0: INIT — add native active-node predicate for the current always-active substrate.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::render::SampleContext;

pub(crate) fn is_node_active(node: &NodeSpec, sample: &SampleContext) -> bool {
    node.active_phases.is_empty()
        || sample
            .lifecycle_phase
            .is_some_and(|phase| node.active_phases.contains(&phase))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_is_node_active.rs</FILE> - <DESC>Check whether a node is active under current compost lifecycle support</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
