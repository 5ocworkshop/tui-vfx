// <FILE>crates/tui-vfx-next/src/fnc_publish_node_outputs.rs</FILE> - <DESC>Publish declared node outputs into graph value deltas</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: keep topology execution coordinator below OFPF hard limits.</WCTX>
// <CLOG>0.1.0: INIT — extract node output publication from graph-step execution.</CLOG>

use std::collections::BTreeMap;

use crate::{
    EffectInputId, EffectOutputId, GraphExecutionError, GraphValueDelta, GraphValueId, NodeId,
    NodeOutputSource, NodeOutputSpec, ProofValue,
};

pub(crate) fn publish_node_outputs(
    node_id: &NodeId,
    outputs: &BTreeMap<GraphValueId, NodeOutputSpec>,
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    effect_outputs: &BTreeMap<EffectOutputId, ProofValue>,
) -> Result<GraphValueDelta, GraphExecutionError> {
    let mut delta = BTreeMap::new();
    for (id, output) in outputs {
        let value = match &output.source {
            NodeOutputSource::EffectOutput { id } => effect_outputs
                .get(id)
                .cloned()
                .expect("GraphSpec validation and adapter contract require declared output"),
            NodeOutputSource::Input { id } => inputs
                .get(id)
                .cloned()
                .expect("GraphSpec validation requires re-emitted input to resolve"),
        };
        delta.insert(id.clone(), (node_id.clone(), value));
    }
    Ok(delta)
}

// <FILE>crates/tui-vfx-next/src/fnc_publish_node_outputs.rs</FILE> - <DESC>Publish declared node outputs into graph value deltas</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
