// <FILE>crates/tui-vfx-compost/src/render/fnc_build_effect_stack.rs</FILE> - <DESC>Build native effect stack from canonical graph binding</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Effect stack construction keeps element write policy with the collected native effect stages.</WCTX>
// <CLOG>0.2.0: MINOR — carry cell and role write policies into the stack.
// 0.1.0: INIT — collect deterministic effect stages for one scene element.</CLOG>

use tui_vfx_contract::{RecipeDocument, RecipeSceneElement};

use crate::render::{EffectStack, EffectStage, RenderError, collect_graph_step_nodes};

pub(crate) fn build_effect_stack<'a>(
    recipe: &'a RecipeDocument,
    element: &RecipeSceneElement,
) -> Result<EffectStack<'a>, RenderError> {
    let mut node_ids = Vec::new();
    let topology = element
        .graph_binding
        .as_ref()
        .and_then(|graph_binding| graph_binding.topology.as_ref())
        .or(recipe.graph.topology.as_ref());
    collect_graph_step_nodes(topology, &mut node_ids);
    if node_ids.is_empty() {
        node_ids.extend(recipe.graph.order.iter().cloned());
    }

    let mut stages = Vec::with_capacity(node_ids.len());
    for requested_node_id in node_ids {
        let Some((node_id, node)) = recipe.graph.nodes.get_key_value(&requested_node_id) else {
            return Err(RenderError::Unsupported(format!(
                "native render references missing node `{}`",
                requested_node_id.as_str()
            )));
        };
        stages.push(EffectStage::new(node_id, node));
    }

    Ok(EffectStack::new(
        stages,
        element.cell_write_policy,
        element.role_write_policy.clone(),
    ))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_build_effect_stack.rs</FILE> - <DESC>Build native effect stack from canonical graph binding</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
