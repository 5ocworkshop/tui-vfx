// <FILE>crates/tui-vfx-contract/src/fnc_collect_graph_value_kinds.rs</FILE> - <DESC>Collect declared graph value kinds from node outputs</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase H1: validate duplicate graph value kind and shape declarations.</WCTX>
// <CLOG>0.2.0: MINOR — reject duplicate graph value declarations with incompatible shape.
// 0.1.0: INIT — extract graph value kind collection from graph validation.</CLOG>

use std::collections::BTreeMap;

use crate::{
    DescriptorValidationError, GraphSpec, GraphValueKinds, GraphValueShape, NodeOutputSource,
    ValueKind,
};

pub(crate) fn collect_graph_value_kinds(
    graph: &GraphSpec,
) -> Result<GraphValueKinds, DescriptorValidationError> {
    let mut values = BTreeMap::new();
    let mut shapes = BTreeMap::new();
    for node in graph.nodes.values() {
        let effect = graph.effects.get(&node.effect).ok_or_else(|| {
            DescriptorValidationError::UnknownEffect {
                id: node.effect.clone(),
            }
        })?;
        for (graph_value_id, output) in &node.outputs {
            let (kind, shape) = match &output.output_source {
                NodeOutputSource::EffectOutput { id } => {
                    let output = effect.outputs.get(id).ok_or_else(|| {
                        DescriptorValidationError::UnknownEffectOutput {
                            effect: node.effect.clone(),
                            output: id.clone(),
                        }
                    })?;
                    (output.kind.value_kind(), output.shape)
                }
                NodeOutputSource::Input { id } => {
                    let input = effect.inputs.get(id).ok_or_else(|| {
                        DescriptorValidationError::UnknownNodeOutputInput {
                            effect: node.effect.clone(),
                            input: id.clone(),
                        }
                    })?;
                    (input.value.kind, GraphValueShape::FrameValue)
                }
            };
            validate_duplicate_value(graph_value_id, kind, shape, &values, &shapes)?;
            values.insert(graph_value_id.clone(), kind);
            shapes.insert(graph_value_id.clone(), shape);
        }
    }
    Ok(values)
}

fn validate_duplicate_value(
    id: &crate::GraphValueId,
    kind: ValueKind,
    shape: GraphValueShape,
    values: &GraphValueKinds,
    shapes: &BTreeMap<crate::GraphValueId, GraphValueShape>,
) -> Result<(), DescriptorValidationError> {
    if let Some(expected) = values.get(id)
        && *expected != kind
    {
        return Err(DescriptorValidationError::SourceKindMismatch {
            expected: *expected,
            actual: kind,
        });
    }
    if let Some(expected) = shapes.get(id)
        && *expected != shape
    {
        return Err(DescriptorValidationError::GraphValueShapeMismatch {
            id: id.clone(),
            expected: *expected,
            actual: shape,
        });
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract/src/fnc_collect_graph_value_kinds.rs</FILE> - <DESC>Collect declared graph value kinds from node outputs</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
