// <FILE>crates/tui-vfx-next/src/cls_graph_execution_outcome.rs</FILE> - <DESC>Proof graph execution outcome DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: return final surface, node list, and diagnostics after graph proof execution.</WCTX>
// <CLOG>0.1.0: INIT — add graph execution outcome for proof executor tests.</CLOG>

use crate::{NodeId, Surface, SurfaceDiagnostic};

/// Result of executing a canonical graph with proof adapters.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GraphExecutionOutcome {
    /// Final surface after ordered proof node execution.
    pub surface: Surface,
    /// Node ids executed in graph order.
    pub executed_nodes: Vec<NodeId>,
    /// Total matched cells across all executed nodes.
    pub matched_cells: usize,
    /// Total written cells across all executed nodes.
    pub written_cells: usize,
    /// Graph/node annotated diagnostics emitted by proof execution.
    pub diagnostics: Vec<SurfaceDiagnostic>,
}

// <FILE>crates/tui-vfx-next/src/cls_graph_execution_outcome.rs</FILE> - <DESC>Proof graph execution outcome DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
