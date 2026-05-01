// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/col_shader_value_input.rs</FILE> - <DESC>Leaf helpers for direct v3.1 shader literal inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Per-shader builders share literal input access without growing orchestration files.</WCTX>
// <CLOG>0.1.0: INIT — extract shader input helpers.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeSpec, Value, ValueSource};
use tui_vfx_types::Color;

use crate::v31::V31RenderError;

pub(crate) fn optional_literal_value<'a>(node: &'a NodeSpec, id: &str) -> Option<&'a Value> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Some(value),
        _ => None,
    }
}

pub(crate) fn literal_value<'a>(node: &'a NodeSpec, id: &str) -> Result<&'a Value, V31RenderError> {
    match node.inputs.get(&EffectInputId::new(id)) {
        Some(ValueSource::Literal { value }) => Ok(value),
        Some(_) => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering requires literal input `{id}` for `{}`.",
            node.effect.as_str()
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering requires input `{id}` for `{}`.",
            node.effect.as_str()
        ))),
    }
}

pub(crate) fn color_input(node: &NodeSpec, id: &str) -> Result<Color, V31RenderError> {
    match literal_value(node, id)? {
        Value::Color(value) => Ok(*value),
        value => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected color input `{id}` but found `{:?}`.",
            value.kind()
        ))),
    }
}

pub(crate) fn bool_input(node: &NodeSpec, id: &str) -> Result<bool, V31RenderError> {
    match literal_value(node, id)? {
        Value::Boolean(value) => Ok(*value),
        value => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected boolean input `{id}` but found `{:?}`.",
            value.kind()
        ))),
    }
}

pub(crate) fn number_input(node: &NodeSpec, id: &str) -> f64 {
    literal_value(node, id)
        .ok()
        .and_then(Value::as_range_number)
        .expect("direct v3.1 load validates required numeric literals")
}

pub(crate) fn integer_input(node: &NodeSpec, id: &str) -> Result<i64, V31RenderError> {
    match literal_value(node, id)? {
        Value::Integer(value) => Ok(*value),
        value => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected integer input `{id}` but found `{:?}`.",
            value.kind()
        ))),
    }
}

pub(crate) fn optional_number_input(node: &NodeSpec, id: &str) -> Option<f64> {
    optional_literal_value(node, id).and_then(Value::as_range_number)
}

pub(crate) fn optional_number_input_or(node: &NodeSpec, id: &str, fallback: f64) -> f64 {
    optional_number_input(node, id).unwrap_or(fallback)
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/col_shader_value_input.rs</FILE> - <DESC>Leaf helpers for direct v3.1 shader literal inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
