// <FILE>crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs</FILE> - <DESC>Validate native direct-render contract at recipe load</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validation dispatches supported native primitives without creating legacy lowerers.</WCTX>
// <CLOG>0.1.0: INIT — add native render contract validation.</CLOG>

use tui_vfx_contract::RecipeDocument;

use crate::LoadError;
use crate::validation::shaders::validate_shader_inputs;

use super::validate_source_inputs;

pub(crate) fn validate_render_contract(recipe: &RecipeDocument) -> Result<(), LoadError> {
    for (source_id, source) in &recipe.sources {
        validate_source_inputs(source_id, source)?;
    }

    for node_id in &recipe.graph.order {
        let Some(node) = recipe.graph.nodes.get(node_id) else {
            continue;
        };
        match node.effect.as_str() {
            "shader.linearGradient" => validate_shader_inputs(node_id, node)?,
            effect => {
                return Err(LoadError::UnsupportedEffect {
                    node_id: node_id.as_str().to_string(),
                    effect: effect.to_string(),
                });
            }
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs</FILE> - <DESC>Validate native direct-render contract at recipe load</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
