// <FILE>crates/tui-vfx-compost/src/validation/shaders/fnc_validate_linear_gradient_inputs.rs</FILE> - <DESC>Validate native linearGradient inputs</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>linearGradient is the first compost shader slice and consumes canonical v3.1 fields directly.</WCTX>
// <CLOG>0.1.1: PATCH — validate canonical channelTarget instead of old applyTo.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec, Value};

use crate::LoadError;
use crate::runtime::RuntimeContext;
use crate::validation::{
    require_declared_inputs_resolvable, require_enum_value, require_number_input,
    require_resolved_input,
};

pub(crate) fn validate_linear_gradient_inputs(
    node_id: &NodeId,
    node: &NodeSpec,
    context: &RuntimeContext,
) -> Result<(), LoadError> {
    require_declared_inputs_resolvable(node_id, node, context)?;
    if node.inputs.contains_key(&EffectInputId::new("gradient")) {
        match require_resolved_input(node_id, node, "gradient", context)?.value() {
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
        require_resolved_input(node_id, node, "startColor", context)?;
        require_resolved_input(node_id, node, "endColor", context)?;
        require_enum_value(node_id, node, "colorSpace", &["rgb", "hct"], context)?;
    }
    require_number_input(node_id, node, "angleDeg", context)?;
    require_number_input(node_id, node, "intensity", context)?;
    require_enum_value(
        node_id,
        node,
        "channelTarget",
        &["foreground", "background", "both"],
        context,
    )?;
    Ok(())
}

// <FILE>crates/tui-vfx-compost/src/validation/shaders/fnc_validate_linear_gradient_inputs.rs</FILE> - <DESC>Validate native linearGradient inputs</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
