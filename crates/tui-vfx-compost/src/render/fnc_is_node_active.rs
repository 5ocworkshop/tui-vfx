// <FILE>crates/tui-vfx-compost/src/render/fnc_is_node_active.rs</FILE> - <DESC>Check whether a node is active under current compost lifecycle support</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Timing validation rejects activePhases at load time until lifecycle phase resolution can honor them.</WCTX>
// <CLOG>0.1.0: INIT — add native active-node predicate for the current always-active substrate.</CLOG>

use tui_vfx_contract::NodeSpec;

pub(crate) fn is_node_active(node: &NodeSpec) -> bool {
    node.active_phases.is_empty()
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_is_node_active.rs</FILE> - <DESC>Check whether a node is active under current compost lifecycle support</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
