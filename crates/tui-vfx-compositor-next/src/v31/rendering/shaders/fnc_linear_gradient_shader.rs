// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_linear_gradient_shader.rs</FILE> - <DESC>Build direct v3.1 linearGradient shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Linear gradient shader mapping owns only its descriptor-to-style conversion.</WCTX>
// <CLOG>0.1.0: INIT — extract linearGradient shader builder.</CLOG>

use tui_vfx_contract::{NodeSpec, Value};
use tui_vfx_style::models::{ColorSpace, Gradient, LinearGradientApplyTo, LinearGradientShader};

use super::col_shader_value_input::{
    color_input, literal_value, number_input, optional_literal_value,
};
use crate::v31::V31RenderError;

pub(crate) fn linear_gradient_shader(
    node: &NodeSpec,
) -> Result<LinearGradientShader, V31RenderError> {
    Ok(LinearGradientShader {
        gradient: gradient_input(node)?,
        angle_deg: number_input(node, "angleDeg") as f32,
        apply_to: apply_to_input(node, "applyTo")?,
        intensity: number_input(node, "intensity") as f32,
    })
}

fn gradient_input(node: &NodeSpec) -> Result<Gradient, V31RenderError> {
    if let Some(Value::Gradient(gradient)) = optional_literal_value(node, "gradient") {
        return Ok(Gradient {
            stops: gradient
                .stops
                .iter()
                .map(|stop| (stop.position as f32, stop.color))
                .collect(),
            space: color_space_name(&gradient.space)?,
        });
    }

    let start = color_input(node, "startColor")?;
    let end = color_input(node, "endColor")?;
    Ok(Gradient {
        stops: vec![(0.0, start), (1.0, end)],
        space: color_space_input(node, "colorSpace")?,
    })
}

fn color_space_input(node: &NodeSpec, id: &str) -> Result<ColorSpace, V31RenderError> {
    color_space_name(literal_value(node, id)?.as_enum_value().ok_or_else(|| {
        V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.linearGradient."
        ))
    })?)
}

fn color_space_name(value: &str) -> Result<ColorSpace, V31RenderError> {
    match value {
        "rgb" => Ok(ColorSpace::Rgb),
        "hct" => Ok(ColorSpace::Hct),
        other => Err(V31RenderError::Unsupported(format!(
            "shader.linearGradient colorSpace `{other}` is not supported by direct v3.1 rendering."
        ))),
    }
}

fn apply_to_input(node: &NodeSpec, id: &str) -> Result<LinearGradientApplyTo, V31RenderError> {
    match literal_value(node, id)?.as_enum_value() {
        Some("foreground") => Ok(LinearGradientApplyTo::Foreground),
        Some("background") => Ok(LinearGradientApplyTo::Background),
        Some("both") => Ok(LinearGradientApplyTo::Both),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.linearGradient applyTo `{value}` is not supported by direct v3.1 rendering."
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.linearGradient."
        ))),
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_linear_gradient_shader.rs</FILE> - <DESC>Build direct v3.1 linearGradient shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
