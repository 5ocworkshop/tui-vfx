// <FILE>crates/tui-vfx-compost/src/render/col_collect_graph_step_nodes.rs</FILE> - <DESC>Collect ordered graph nodes for native rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Graph topology traversal remains a small helper separate from render orchestration.</WCTX>
// <CLOG>0.1.0: INIT — add graph step node collector.</CLOG>

use tui_vfx_contract::{GraphStep, NodeId};

pub(crate) fn collect_graph_step_nodes(step: Option<&GraphStep>, out: &mut Vec<NodeId>) {
    match step {
        Some(GraphStep::Node { node }) => out.push(node.clone()),
        Some(GraphStep::Sequence { children }) | Some(GraphStep::Parallel { children, .. }) => {
            for child in children {
                collect_graph_step_nodes(Some(child), out);
            }
        }
        None => {}
    }
}

// <FILE>crates/tui-vfx-compost/src/render/col_collect_graph_step_nodes.rs</FILE> - <DESC>Collect ordered graph nodes for native rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
