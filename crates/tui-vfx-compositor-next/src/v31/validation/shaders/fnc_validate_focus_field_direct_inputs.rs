// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_focus_field_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 focusField inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Focus field direct rendering supports descriptor literals that map cleanly to the copied compositor shader.</WCTX>
// <CLOG>0.1.0: INIT — extract focusField direct-input validation.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec};

use crate::v31::V31LoadError;
use crate::v31::validation::col_direct_input::{
    direct_input_error, require_color_input, require_declared_inputs_literal, require_enum_value,
    require_integer_valued_number_input, require_number_input,
};

pub(crate) fn validate_focus_field_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;
    require_color_input(node_id, node, "color")?;
    require_integer_valued_number_input(node_id, node, "radius")?;
    require_integer_valued_number_input(node_id, node, "centerX")?;
    require_integer_valued_number_input(node_id, node, "centerY")?;
    if node.inputs.contains_key(&EffectInputId::new("intensity")) {
        require_number_input(node_id, node, "intensity")?;
    }
    if node.inputs.contains_key(&EffectInputId::new("shape")) {
        require_enum_value(node_id, node, "shape", &["circle", "ellipse"])?;
    }
    if node.inputs.contains_key(&EffectInputId::new("applyTo")) {
        require_enum_value(
            node_id,
            node,
            "applyTo",
            &["foreground", "background", "both"],
        )?;
    }

    for input in ["rectHeight", "rectWidth", "rectX", "rectY"] {
        if node.inputs.contains_key(&EffectInputId::new(input)) {
            return Err(direct_input_error(
                node_id,
                node,
                input,
                &format!(
                    "shader.focusField input `{input}` is not supported by direct v3.1 rendering."
                ),
            ));
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_focus_field_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 focusField inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
