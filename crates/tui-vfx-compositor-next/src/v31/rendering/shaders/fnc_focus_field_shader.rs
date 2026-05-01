// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_focus_field_shader.rs</FILE> - <DESC>Build direct v3.1 focusField shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Focus field shader mapping owns only its descriptor-to-style conversion.</WCTX>
// <CLOG>0.1.0: INIT — extract focusField shader builder.</CLOG>

use tui_vfx_contract::{NodeSpec, Value};
use tui_vfx_style::models::{ColorConfig, FocusFieldApplyTo, FocusFieldShader, FocusFieldShape};

use super::col_shader_value_input::{
    color_input, number_input, optional_literal_value, optional_number_input_or,
};
use crate::v31::V31RenderError;

pub(crate) fn focus_field_shader(node: &NodeSpec) -> Result<FocusFieldShader, V31RenderError> {
    let radius = number_input(node, "radius").max(1.0) as u16;
    Ok(FocusFieldShader {
        color: ColorConfig::from(color_input(node, "color")?),
        shape: focus_field_shape_input(node, "shape")?,
        center_x: number_input(node, "centerX").max(0.0) as u16,
        center_y: number_input(node, "centerY").max(0.0) as u16,
        radius_x: radius,
        radius_y: radius,
        intensity: optional_number_input_or(node, "intensity", 1.0).clamp(0.0, 1.0) as f32,
        apply_to: focus_field_apply_to_input(node, "applyTo")?,
        ..FocusFieldShader::default()
    })
}

fn focus_field_shape_input(node: &NodeSpec, id: &str) -> Result<FocusFieldShape, V31RenderError> {
    match optional_literal_value(node, id).and_then(Value::as_enum_value) {
        Some("circle") | Some("ellipse") | None => Ok(FocusFieldShape::Ellipse),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.focusField shape `{value}` is not supported by direct v3.1 rendering."
        ))),
    }
}

fn focus_field_apply_to_input(
    node: &NodeSpec,
    id: &str,
) -> Result<FocusFieldApplyTo, V31RenderError> {
    match optional_literal_value(node, id).and_then(Value::as_enum_value) {
        Some("foreground") | None => Ok(FocusFieldApplyTo::Foreground),
        Some("background") => Ok(FocusFieldApplyTo::Background),
        Some("both") => Ok(FocusFieldApplyTo::Both),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.focusField applyTo `{value}` is not supported by direct v3.1 rendering."
        ))),
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_focus_field_shader.rs</FILE> - <DESC>Build direct v3.1 focusField shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
