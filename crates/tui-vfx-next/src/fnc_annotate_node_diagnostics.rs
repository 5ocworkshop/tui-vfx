// <FILE>crates/tui-vfx-next/src/fnc_annotate_node_diagnostics.rs</FILE> - <DESC>Annotate graph node diagnostics with canonical node identity</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: diagnostics emitted by proof graph nodes must name graph/node identity.</WCTX>
// <CLOG>0.1.0: INIT — deterministic graph node path and message prefix helper.</CLOG>

use crate::{ApplyOutcome, NodeId};

/// Attach deterministic graph/node identity to every diagnostic in an outcome.
pub(crate) fn annotate_node_diagnostics(
    outcome: &mut ApplyOutcome,
    node_index: usize,
    node_id: &NodeId,
) {
    outcome.diagnostics = outcome
        .diagnostics
        .drain(..)
        .map(|mut diagnostic| {
            diagnostic.message =
                format!("graph node `{}`: {}", node_id.as_str(), diagnostic.message);
            diagnostic.path = Some(format!("graph.node[{node_index}].{}", node_id.as_str()));
            diagnostic
        })
        .collect();
}

// <FILE>crates/tui-vfx-next/src/fnc_annotate_node_diagnostics.rs</FILE> - <DESC>Annotate graph node diagnostics with canonical node identity</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
