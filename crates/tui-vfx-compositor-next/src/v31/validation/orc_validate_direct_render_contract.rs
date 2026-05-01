// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/orc_validate_direct_render_contract.rs</FILE> - <DESC>Orchestrate direct v3.1 render-contract validation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Validate sources and graph nodes once at load time before direct compositor-next rendering.</WCTX>
// <CLOG>0.1.0: INIT — extract direct render-contract validation orchestration.</CLOG>

use tui_vfx_contract::RecipeDocument;

use super::fnc_validate_direct_source_inputs::validate_direct_source_inputs;
use super::shaders::validate_shader_direct_inputs;
use crate::v31::V31LoadError;

pub(crate) fn validate_direct_render_contract(recipe: &RecipeDocument) -> Result<(), V31LoadError> {
    for (source_id, source) in &recipe.sources {
        validate_direct_source_inputs(source_id, source)?;
    }
    for (node_id, node) in &recipe.graph.nodes {
        validate_shader_direct_inputs(node_id, node)?;
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/orc_validate_direct_render_contract.rs</FILE> - <DESC>Orchestrate direct v3.1 render-contract validation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
