// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/col_collect_graph_step_nodes.rs</FILE> - <DESC>Collect ordered graph nodes for direct v3.1 rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pure graph-step traversal stays separate from composition orchestration.</WCTX>
// <CLOG>0.1.0: INIT — extract graph-step node collection.</CLOG>

use std::collections::BTreeSet;

use tui_vfx_contract::{GraphStep, NodeId};

pub(crate) fn collect_graph_step_nodes(step: Option<&GraphStep>, node_ids: &mut Vec<NodeId>) {
    let Some(step) = step else { return };
    match step {
        GraphStep::Node { node } => node_ids.push(node.clone()),
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            let mut seen = BTreeSet::new();
            for child in children {
                collect_graph_step_nodes(Some(child), node_ids);
            }
            node_ids.retain(|node| seen.insert(node.clone()));
        }
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/col_collect_graph_step_nodes.rs</FILE> - <DESC>Collect ordered graph nodes for direct v3.1 rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
