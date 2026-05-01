// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_border_sweep_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 borderSweep inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Border sweep accepts the signed literal subset and rejects position until literal override semantics exist.</WCTX>
// <CLOG>0.1.0: INIT — extract borderSweep direct-input validation.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec};

use crate::v31::V31LoadError;
use crate::v31::validation::col_direct_input::{
    direct_input_error, require_color_input, require_declared_inputs_literal,
    require_integer_input, require_number_input,
};

pub(crate) fn validate_border_sweep_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;
    require_color_input(node_id, node, "color")?;
    require_number_input(node_id, node, "speed")?;
    require_integer_input(node_id, node, "length")?;

    if node.inputs.contains_key(&EffectInputId::new("position")) {
        return Err(direct_input_error(
            node_id,
            node,
            "position",
            "shader.borderSweep position override is not supported by direct v3.1 rendering until compositor-next has a literal position path without runtime binding semantics.",
        ));
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_border_sweep_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 borderSweep inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
