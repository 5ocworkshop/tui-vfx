// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_glisten_band_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 glistenBand inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glisten band direct rendering supports the signed literal subset only.</WCTX>
// <CLOG>0.1.0: INIT — extract glistenBand direct-input validation.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec};

use crate::v31::V31LoadError;
use crate::v31::validation::col_direct_input::{
    direct_input_error, require_color_input, require_declared_inputs_literal, require_enum_value,
    require_integer_valued_number_input, require_number_input,
};

pub(crate) fn validate_glisten_band_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;
    require_color_input(node_id, node, "color")?;
    require_integer_valued_number_input(node_id, node, "bandWidth")?;
    if node.inputs.contains_key(&EffectInputId::new("speed")) {
        require_number_input(node_id, node, "speed")?;
    }
    if node.inputs.contains_key(&EffectInputId::new("angleDeg")) {
        require_number_input(node_id, node, "angleDeg")?;
    }
    if node
        .inputs
        .contains_key(&EffectInputId::new("blendStrength"))
    {
        require_number_input(node_id, node, "blendStrength")?;
    }
    for input in ["head", "tail"] {
        if !node.inputs.contains_key(&EffectInputId::new(input)) {
            continue;
        }
        return Err(direct_input_error(
            node_id,
            node,
            input,
            "shader.glistenBand numeric head/tail band-position fields are not supported by direct v3.1 rendering.",
        ));
    }
    if node.inputs.contains_key(&EffectInputId::new("direction")) {
        require_enum_value(node_id, node, "direction", &["leftToRight", "rightToLeft"])?;
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_glisten_band_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 glistenBand inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
