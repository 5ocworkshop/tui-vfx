// <FILE>crates/tui-vfx-contract/src/fnc_collect_graph_value_kinds.rs</FILE> - <DESC>Collect declared graph value kinds from node outputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: keep graph validation coordinator below OFPF hard limits.</WCTX>
// <CLOG>0.1.0: INIT — extract graph value kind collection from graph validation.</CLOG>

use std::collections::BTreeMap;

use crate::{DescriptorValidationError, GraphSpec, GraphValueKinds, NodeOutputSource};

pub(crate) fn collect_graph_value_kinds(
    graph: &GraphSpec,
) -> Result<GraphValueKinds, DescriptorValidationError> {
    let mut values = BTreeMap::new();
    for node in graph.nodes.values() {
        let effect = graph.effects.get(&node.effect).ok_or_else(|| {
            DescriptorValidationError::UnknownEffect {
                id: node.effect.clone(),
            }
        })?;
        for (graph_value_id, output) in &node.outputs {
            let kind = match &output.source {
                NodeOutputSource::EffectOutput { id } => effect
                    .outputs
                    .get(id)
                    .ok_or_else(|| DescriptorValidationError::UnknownEffectOutput {
                        effect: node.effect.clone(),
                        output: id.clone(),
                    })?
                    .kind
                    .value_kind(),
                NodeOutputSource::Input { id } => {
                    effect
                        .inputs
                        .get(id)
                        .ok_or_else(|| DescriptorValidationError::UnknownNodeOutputInput {
                            effect: node.effect.clone(),
                            input: id.clone(),
                        })?
                        .value
                        .kind
                }
            };
            values.insert(graph_value_id.clone(), kind);
        }
    }
    Ok(values)
}

// <FILE>crates/tui-vfx-contract/src/fnc_collect_graph_value_kinds.rs</FILE> - <DESC>Collect declared graph value kinds from node outputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
