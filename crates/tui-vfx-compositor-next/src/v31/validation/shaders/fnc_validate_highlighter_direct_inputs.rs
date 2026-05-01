// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_highlighter_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 highlighter inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Highlighter direct rendering supports the signed band-mode literal subset only.</WCTX>
// <CLOG>0.1.0: INIT — extract highlighter direct-input validation.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec};

use crate::v31::V31LoadError;
use crate::v31::validation::col_direct_input::{
    direct_input_error, require_bool_input, require_color_input, require_declared_inputs_literal,
    require_enum_value, require_integer_input, require_number_input,
};

pub(crate) fn validate_highlighter_direct_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    require_declared_inputs_literal(node_id, node)?;
    require_color_input(node_id, node, "color")?;
    require_number_input(node_id, node, "bandWidth")?;
    require_number_input(node_id, node, "blendStrength")?;
    let text_contrast = require_number_input(node_id, node, "textContrast")?;
    if text_contrast > 0.0 {
        return Err(direct_input_error(
            node_id,
            node,
            "textContrast",
            "shader.highlighter textContrast values above 0.0 are not supported by direct v3.1 rendering.",
        ));
    }
    require_bool_input(node_id, node, "softEdge")?;
    require_integer_input(node_id, node, "rowMask")?;
    require_enum_value(node_id, node, "mode", &["band"])?;
    require_enum_value(
        node_id,
        node,
        "direction",
        &["leftToRight", "rightToLeft", "topToBottom", "bottomToTop"],
    )?;
    require_enum_value(
        node_id,
        node,
        "applyTo",
        &["foreground", "background", "both"],
    )?;

    for input in ["pulse", "sparkleDensity", "modeStrength"] {
        if node.inputs.contains_key(&EffectInputId::new(input)) {
            return Err(direct_input_error(
                node_id,
                node,
                input,
                &format!(
                    "shader.highlighter input `{input}` is not supported by direct v3.1 rendering."
                ),
            ));
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/shaders/fnc_validate_highlighter_direct_inputs.rs</FILE> - <DESC>Validate direct v3.1 highlighter inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
