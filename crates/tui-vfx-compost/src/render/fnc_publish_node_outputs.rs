// <FILE>crates/tui-vfx-compost/src/render/fnc_publish_node_outputs.rs</FILE> - <DESC>Publish node graph values after native node execution</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Graph value publication follows the native runtime resolver so later nodes can consume graphValue sources.</WCTX>
// <CLOG>0.1.0: INIT — support input re-emission graph value outputs for native nodes.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{EffectInputId, GraphValueId, NodeOutputSource, NodeSpec, Value};

use crate::render::RenderError;
use crate::runtime::{RuntimeContext, resolve_value_source};

pub(crate) fn publish_node_outputs(
    node: &NodeSpec,
    context: &mut RuntimeContext,
) -> Result<BTreeMap<GraphValueId, Value>, RenderError> {
    let mut published = BTreeMap::new();
    for (graph_value_id, output) in &node.outputs {
        let value = match &output.output_source {
            NodeOutputSource::Input { id } => resolved_input_value(node, id, context)?,
            NodeOutputSource::EffectOutput { id } => {
                return Err(RenderError::Unsupported(format!(
                    "node `{}` publishes unsupported effect output `{}`",
                    node.id.as_str(),
                    id.as_str()
                )));
            }
        };
        context.set_graph_value(graph_value_id.clone(), value.clone());
        published.insert(graph_value_id.clone(), value);
    }
    Ok(published)
}

fn resolved_input_value(
    node: &NodeSpec,
    input_id: &EffectInputId,
    context: &RuntimeContext,
) -> Result<tui_vfx_contract::Value, RenderError> {
    let Some(source) = node.inputs.get(input_id) else {
        return Err(RenderError::Unsupported(format!(
            "node `{}` output references missing input `{}`",
            node.id.as_str(),
            input_id.as_str()
        )));
    };
    resolve_value_source(source, context)
        .map(|resolved| resolved.value().clone())
        .map_err(|error| RenderError::Unsupported(error.reason()))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_publish_node_outputs.rs</FILE> - <DESC>Publish node graph values after native node execution</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
