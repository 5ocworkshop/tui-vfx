// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_linear_gradient_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 linearGradient inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Linear gradient accepts canonical gradient input or legacy start/end pair already signed into the direct path.</WCTX>
// <CLOG>0.1.0: INIT — extract linearGradient direct-input validation.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec};

use crate::v31::V31LoadError;
use crate::v31::validation::col_direct_input::{
    require_declared_inputs_literal, require_enum_value, require_literal_input,
    require_number_input,
};

pub(crate) fn validate_linear_gradient_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;
    let has_gradient = node.inputs.contains_key(&EffectInputId::new("gradient"));
    if has_gradient {
        require_literal_input(node_id, node, "gradient")?;
    } else {
        require_literal_input(node_id, node, "startColor")?;
        require_literal_input(node_id, node, "endColor")?;
        require_enum_value(node_id, node, "colorSpace", &["rgb", "hct"])?;
    }
    require_number_input(node_id, node, "angleDeg")?;
    require_number_input(node_id, node, "intensity")?;
    require_enum_value(
        node_id,
        node,
        "applyTo",
        &["foreground", "background", "both"],
    )?;
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_linear_gradient_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 linearGradient inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
