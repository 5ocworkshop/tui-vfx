// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/col_direct_input.rs</FILE> - <DESC>Leaf helpers for direct v3.1 graph input validation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep repeated literal/type validation in one pure helper file used by per-shader validators.</WCTX>
// <CLOG>0.1.0: INIT — extract direct input validation helpers.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeId, NodeSpec, Value, ValueSource};

use crate::v31::V31LoadError;

pub(crate) fn require_declared_inputs_literal(
    node_id: &NodeId,
    node: &NodeSpec,
) -> Result<(), V31LoadError> {
    for input in node.inputs.keys() {
        literal_direct_value(node_id, node, input.as_str())?;
    }
    Ok(())
}

pub(crate) fn require_literal_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    literal_direct_value(node_id, node, input)?;
    Ok(())
}

pub(crate) fn require_color_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)? {
        Value::Color(_) => Ok(()),
        value => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected color input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )),
    }
}

pub(crate) fn require_number_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<f64, V31LoadError> {
    let value = literal_direct_value(node_id, node, input)?;
    value.as_range_number().ok_or_else(|| {
        direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected numeric input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )
    })
}

pub(crate) fn require_integer_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)? {
        Value::Integer(_) => Ok(()),
        value => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected integer input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )),
    }
}

pub(crate) fn require_integer_valued_number_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    let value = literal_direct_value(node_id, node, input)?;
    let Some(number) = value.as_range_number() else {
        return Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected numeric integer input `{input}` but found `{:?}`.",
                value.kind()
            ),
        ));
    };
    if number.fract() == 0.0 {
        Ok(())
    } else {
        Err(direct_input_error(
            node_id,
            node,
            input,
            &format!("Direct v3.1 rendering requires `{input}` to be integer-valued."),
        ))
    }
}

pub(crate) fn require_bool_input(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)? {
        Value::Boolean(_) => Ok(()),
        value => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering expected boolean input `{input}` but found `{:?}`.",
                value.kind()
            ),
        )),
    }
}

pub(crate) fn require_enum_value(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    allowed: &[&str],
) -> Result<(), V31LoadError> {
    match literal_direct_value(node_id, node, input)?.as_enum_value() {
        Some(value) if allowed.contains(&value) => Ok(()),
        Some(value) => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering does not support `{input}` enum value `{value}` for `{}`.",
                node.effect.as_str()
            ),
        )),
        None => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!("Direct v3.1 rendering expected enum input `{input}`."),
        )),
    }
}

pub(crate) fn direct_input_error(
    node_id: &NodeId,
    node: &NodeSpec,
    input: &str,
    reason: &str,
) -> V31LoadError {
    V31LoadError::UnsupportedDirectInput {
        node_id: node_id.as_str().to_string(),
        effect: node.effect.as_str().to_string(),
        input: input.to_string(),
        reason: reason.to_string(),
    }
}

fn literal_direct_value<'a>(
    node_id: &NodeId,
    node: &'a NodeSpec,
    input: &str,
) -> Result<&'a Value, V31LoadError> {
    match node.inputs.get(&EffectInputId::new(input)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!("Direct v3.1 rendering requires literal graph input `{input}`."),
        )),
        None => Err(direct_input_error(
            node_id,
            node,
            input,
            &format!(
                "Direct v3.1 rendering requires input `{input}` for `{}`.",
                node.effect.as_str()
            ),
        )),
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/validation/col_direct_input.rs</FILE> - <DESC>Leaf helpers for direct v3.1 graph input validation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
