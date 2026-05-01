// <FILE>crates/tui-vfx-compost/src/validation/shaders/fnc_validate_linear_gradient_inputs.rs</FILE> - <DESC>Validate native linearGradient inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>linearGradient is the first compost shader slice and consumes canonical v3.1 fields directly.</WCTX>
// <CLOG>0.1.0: INIT — add linearGradient load validation.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec, Value};

use crate::LoadError;
use crate::validation::{
    require_declared_inputs_literal, require_enum_value, require_literal_input,
    require_number_input,
};

pub(crate) fn validate_linear_gradient_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), LoadError> {
    require_declared_inputs_literal(node_id, node)?;
    if node.inputs.contains_key(&EffectInputId::new("gradient")) {
        match require_literal_input(node_id, node, "gradient")? {
            Value::Gradient(_) => {}
            value => {
                return Err(LoadError::UnsupportedInput {
                    node_id: node_id.as_str().to_string(),
                    effect: node.effect.as_str().to_string(),
                    input: "gradient".to_string(),
                    reason: format!("expected gradient input, found {:?}", value.kind()),
                });
            }
        }
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

// <FILE>crates/tui-vfx-compost/src/validation/shaders/fnc_validate_linear_gradient_inputs.rs</FILE> - <DESC>Validate native linearGradient inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
