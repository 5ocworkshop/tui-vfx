// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_context.rs</FILE> - <DESC>Native value resolver context</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Runtime resolver follows the mature prepare-context pattern with one shared per-sample bundle.</WCTX>
// <CLOG>0.3.0: MINOR — preserve distinct normalized coordinates and elapsed clocks.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{GraphSpec, GraphValueId, LifecyclePhase, ParameterId, SignalId, Value};

use crate::render::SampleContext;

#[derive(Clone, Debug, Default)]
pub(crate) struct RuntimeContext {
    pub(crate) phase_t: f64,
    pub(crate) loop_t: Option<f64>,
    pub(crate) absolute_time_ms: Option<u64>,
    pub(crate) phase_time_ms: Option<u64>,
    pub(crate) loop_time_ms: Option<u64>,
    pub(crate) lifecycle_phase: Option<LifecyclePhase>,
    pub(crate) width: Option<u16>,
    pub(crate) height: Option<u16>,
    pub(crate) cell_x: Option<u16>,
    pub(crate) cell_y: Option<u16>,
    parameters: BTreeMap<ParameterId, Value>,
    signals: BTreeMap<SignalId, Value>,
    graph_values: BTreeMap<GraphValueId, Value>,
}

impl RuntimeContext {
    pub(crate) fn load_time() -> Self {
        Self::default()
    }

    pub(crate) fn from_sample(sample: &SampleContext) -> Self {
        Self {
            phase_t: sample.phase_t.clamp(0.0, 1.0),
            loop_t: sample.loop_t.map(|loop_t| loop_t.clamp(0.0, 1.0)),
            absolute_time_ms: sample.absolute_time_ms,
            phase_time_ms: sample.phase_time_ms,
            loop_time_ms: sample.loop_time_ms,
            lifecycle_phase: sample.lifecycle_phase,
            ..Self::default()
        }
    }

    pub(crate) fn with_graph_defaults(mut self, graph: &GraphSpec) -> Self {
        for (id, spec) in &graph.parameters {
            if let Some(default) = &spec.value.default {
                self.parameters.insert(id.clone(), default.clone());
            }
        }
        for (id, spec) in &graph.signals {
            if let Some(default) = &spec.value.default {
                self.signals.insert(id.clone(), default.clone());
            }
        }
        self
    }

    pub(crate) fn with_dimensions(mut self, width: u16, height: u16) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub(crate) fn with_cell(mut self, x: u16, y: u16) -> Self {
        self.cell_x = Some(x);
        self.cell_y = Some(y);
        self
    }

    pub(crate) fn parameter(&self, id: &ParameterId) -> Option<&Value> {
        self.parameters.get(id)
    }

    pub(crate) fn signal(&self, id: &SignalId) -> Option<&Value> {
        self.signals.get(id)
    }

    pub(crate) fn graph_value(&self, id: &GraphValueId) -> Option<&Value> {
        self.graph_values.get(id)
    }

    pub(crate) fn effective_loop_t(&self) -> f64 {
        self.loop_t.unwrap_or(self.phase_t).clamp(0.0, 1.0)
    }

    pub(crate) fn phase_progress(&self, phase: LifecyclePhase) -> f64 {
        if self.lifecycle_phase == Some(phase) {
            self.phase_t
        } else {
            0.0
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/cls_runtime_context.rs</FILE> - <DESC>Native value resolver context</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
