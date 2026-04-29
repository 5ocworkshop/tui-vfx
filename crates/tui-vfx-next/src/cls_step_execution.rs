// <FILE>crates/tui-vfx-next/src/cls_step_execution.rs</FILE> - <DESC>Proof graph-step execution accumulator</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: keep topology recursion state below OFPF hard limits.</WCTX>
// <CLOG>0.1.0: INIT — extract step execution accumulator from graph-step coordinator.</CLOG>

use crate::{GraphValueBus, GraphValueDelta, NodeId, Surface, SurfaceDelta};

#[derive(Clone, Debug)]
pub(crate) struct StepExecution {
    pub(crate) surface: Surface,
    pub(crate) delta: SurfaceDelta,
    pub(crate) graph_values: GraphValueBus,
    pub(crate) value_delta: GraphValueDelta,
    pub(crate) executed_nodes: Vec<NodeId>,
    pub(crate) matched_cells: usize,
    pub(crate) written_cells: usize,
    pub(crate) diagnostics: Vec<crate::SurfaceDiagnostic>,
}

impl StepExecution {
    pub(crate) fn from_surface(surface: Surface, graph_values: GraphValueBus) -> Self {
        Self {
            surface,
            delta: SurfaceDelta::new(),
            graph_values,
            value_delta: GraphValueDelta::new(),
            executed_nodes: vec![],
            matched_cells: 0,
            written_cells: 0,
            diagnostics: vec![],
        }
    }

    pub(crate) fn extend_counts(&mut self, other: Self) {
        self.executed_nodes.extend(other.executed_nodes);
        self.matched_cells += other.matched_cells;
        self.written_cells += other.written_cells;
        self.diagnostics.extend(other.diagnostics);
    }
}

// <FILE>crates/tui-vfx-next/src/cls_step_execution.rs</FILE> - <DESC>Proof graph-step execution accumulator</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
