// <FILE>crates/tui-vfx-compost/src/validation/shaders/mod.rs</FILE> - <DESC>Native shader input validation dispatch</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Only signed shader slices appear here; validation stays in native compost vocabulary.</WCTX>
// <CLOG>0.1.1: PATCH — keep metadata in native compost vocabulary.
// 0.1.0: INIT — add linearGradient validation dispatch.</CLOG>

mod fnc_validate_linear_gradient_inputs;

use tui_vfx_contract::{NodeId, NodeSpec};

use crate::runtime::RuntimeContext;

use crate::LoadError;

use fnc_validate_linear_gradient_inputs::validate_linear_gradient_inputs;

pub(crate) fn validate_shader_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    match node.effect.as_str() {
        "shader.linearGradient" => validate_linear_gradient_inputs(node_id, node, context),
        effect => Err(LoadError::UnsupportedEffect {
            node_id: node_id.as_str().to_string(),
            effect: effect.to_string(),
        }),
    }
}

// <FILE>crates/tui-vfx-compost/src/validation/shaders/mod.rs</FILE> - <DESC>Native shader input validation dispatch</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
