// <FILE>crates/tui-vfx-next/src/cls_graph_execution_context.rs</FILE> - <DESC>Proof graph execution value snapshot</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: resolve graph ValueSource inputs without runtime stores.</WCTX>
// <CLOG>0.1.0: INIT — add parameter and signal value snapshot for proof graph execution.</CLOG>

use std::collections::BTreeMap;

use crate::{ParameterId, SignalId, Value};

/// One-shot value snapshot used by the proof graph executor.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GraphExecutionContext {
    /// Explicit parameter values for this proof execution.
    pub parameter_values: BTreeMap<ParameterId, Value>,
    /// Explicit signal values for this proof execution.
    pub signal_values: BTreeMap<SignalId, Value>,
}

impl GraphExecutionContext {
    /// Create an empty execution snapshot.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or replace one parameter value.
    pub fn with_parameter(mut self, id: ParameterId, value: Value) -> Self {
        self.parameter_values.insert(id, value);
        self
    }

    /// Add or replace one signal value.
    pub fn with_signal(mut self, id: SignalId, value: Value) -> Self {
        self.signal_values.insert(id, value);
        self
    }
}

// <FILE>crates/tui-vfx-next/src/cls_graph_execution_context.rs</FILE> - <DESC>Proof graph execution value snapshot</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
