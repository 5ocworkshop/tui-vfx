// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/orc_composition_spec_for_element.rs</FILE> - <DESC>Build compositor-next composition spec for one v3.1 scene element</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Composition construction is separate from public render entrypoint and per-shader builders.</WCTX>
// <CLOG>0.1.0: INIT — extract direct composition-spec orchestration.</CLOG>

use crate::pipeline::CompositionSpec;
use tui_vfx_contract::{RecipeDocument, RecipeSceneElement};

use super::shaders::append_shader_node_to_composition;
use super::{V31RenderError, V31SampleContext, collect_graph_step_nodes};

pub(crate) fn composition_spec_for_element(
    recipe: &RecipeDocument,
    element: &RecipeSceneElement,
    sample: &V31SampleContext,
) -> Result<(CompositionSpec, Vec<String>), V31RenderError> {
    let mut node_ids = Vec::new();
    let topology = element
        .pipeline
        .as_ref()
        .and_then(|pipeline| pipeline.topology.as_ref())
        .or(recipe.graph.topology.as_ref());
    collect_graph_step_nodes(topology, &mut node_ids);
    if node_ids.is_empty() {
        node_ids.extend(recipe.graph.order.iter().cloned());
    }

    let mut spec = CompositionSpec {
        t: sample.phase_t,
        ..CompositionSpec::default()
    };
    let mut applied_effect_kinds = Vec::new();
    for node_id in node_ids {
        let node = recipe.graph.nodes.get(&node_id).ok_or_else(|| {
            V31RenderError::Unsupported(format!(
                "Direct v3.1 rendering references missing node `{}`.",
                node_id.as_str()
            ))
        })?;
        append_shader_node_to_composition(node, &mut spec, &mut applied_effect_kinds)?;
    }
    Ok((spec, applied_effect_kinds))
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/orc_composition_spec_for_element.rs</FILE> - <DESC>Build compositor-next composition spec for one v3.1 scene element</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
