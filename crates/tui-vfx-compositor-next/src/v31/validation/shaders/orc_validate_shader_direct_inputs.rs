// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/orc_validate_shader_direct_inputs.rs</FILE> - <DESC>Dispatch direct v3.1 shader validation by descriptor id</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep shader validation dispatch small and additive as primitives are signed.</WCTX>
// <CLOG>0.1.0: INIT — extract shader validation dispatcher.</CLOG>

use tui_vfx_contract::{NodeId, NodeSpec};

use super::fnc_validate_border_sweep_direct_inputs::validate_border_sweep_direct_inputs;
use super::fnc_validate_focus_field_direct_inputs::validate_focus_field_direct_inputs;
use super::fnc_validate_glisten_band_direct_inputs::validate_glisten_band_direct_inputs;
use super::fnc_validate_highlighter_direct_inputs::validate_highlighter_direct_inputs;
use super::fnc_validate_linear_gradient_direct_inputs::validate_linear_gradient_direct_inputs;
use crate::v31::V31LoadError;

pub(crate) fn validate_shader_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    match node.effect.as_str() {
        "shader.linearGradient" => validate_linear_gradient_direct_inputs(node_id, node),
        "shader.highlighter" => validate_highlighter_direct_inputs(node_id, node),
        "shader.glistenBand" => validate_glisten_band_direct_inputs(node_id, node),
        "shader.focusField" => validate_focus_field_direct_inputs(node_id, node),
        "shader.borderSweep" => validate_border_sweep_direct_inputs(node_id, node),
        _ => Ok(()),
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/orc_validate_shader_direct_inputs.rs</FILE> - <DESC>Dispatch direct v3.1 shader validation by descriptor id</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
