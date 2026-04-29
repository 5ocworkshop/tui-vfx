// <FILE>crates/tui-vfx-next/src/cls_graph_value_bus.rs</FILE> - <DESC>Proof graph value bus type aliases</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: share proof value-bus maps across execution helpers.</WCTX>
// <CLOG>0.1.0: INIT — add proof-only graph value bus and delta aliases.</CLOG>

use std::collections::BTreeMap;

use crate::{GraphValueId, NodeId, ProofValue};

/// Proof-only graph-local value snapshot visible to a graph step.
pub(crate) type GraphValueBus = BTreeMap<GraphValueId, ProofValue>;

/// Proof-only graph-local value writes produced by a step, retaining writer identity.
pub(crate) type GraphValueDelta = BTreeMap<GraphValueId, (NodeId, ProofValue)>;

// <FILE>crates/tui-vfx-next/src/cls_graph_value_bus.rs</FILE> - <DESC>Proof graph value bus type aliases</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
