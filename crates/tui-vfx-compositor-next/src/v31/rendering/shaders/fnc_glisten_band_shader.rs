// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_glisten_band_shader.rs</FILE> - <DESC>Build direct v3.1 glistenBand shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glisten band shader mapping owns only its descriptor-to-style conversion.</WCTX>
// <CLOG>0.1.0: INIT — extract glistenBand shader builder.</CLOG>

use tui_vfx_contract::{NodeSpec, Value};
use tui_vfx_style::models::{ColorConfig, GlistenBandShader, GlistenDirection};

use super::col_shader_value_input::{
    color_input, number_input, optional_literal_value, optional_number_input,
};
use crate::v31::V31RenderError;

pub(crate) fn glisten_band_shader(node: &NodeSpec) -> Result<GlistenBandShader, V31RenderError> {
    Ok(GlistenBandShader {
        speed: optional_number_input(node, "speed")
            .unwrap_or(1.0)
            .clamp(0.1, 10.0) as f32,
        speed_binding: None,
        band_width: number_input(node, "bandWidth").max(1.0) as u16,
        angle_deg: optional_number_input(node, "angleDeg").unwrap_or(0.0) as f32,
        head: ColorConfig::from(color_input(node, "color")?),
        tail: ColorConfig::from(color_input(node, "color")?),
        direction: glisten_direction_input(node, "direction")?,
        direction_binding: None,
        repeat_count: 0,
        apply_to: tui_vfx_style::models::GlistenApplyTo::Foreground,
        blend_strength: optional_number_input(node, "blendStrength")
            .unwrap_or(1.0)
            .clamp(0.0, 1.0) as f32,
        blend_strength_binding: None,
    })
}

fn glisten_direction_input(node: &NodeSpec, id: &str) -> Result<GlistenDirection, V31RenderError> {
    match optional_literal_value(node, id).and_then(Value::as_enum_value) {
        Some("leftToRight") | None => Ok(GlistenDirection::Forward),
        Some("rightToLeft") => Ok(GlistenDirection::Reverse),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.glistenBand direction `{value}` is not supported by direct v3.1 rendering."
        ))),
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_glisten_band_shader.rs</FILE> - <DESC>Build direct v3.1 glistenBand shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
