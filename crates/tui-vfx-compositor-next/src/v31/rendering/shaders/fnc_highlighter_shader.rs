// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_highlighter_shader.rs</FILE> - <DESC>Build direct v3.1 highlighter shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Highlighter shader mapping owns only its descriptor-to-style conversion.</WCTX>
// <CLOG>0.1.0: INIT — extract highlighter shader builder.</CLOG>

use tui_vfx_contract::NodeSpec;
use tui_vfx_style::models::{
    ColorConfig, HighlighterApplyTo, HighlighterDirection, HighlighterMode, HighlighterRowMask,
    HighlighterShader, TextContrast,
};

use super::col_shader_value_input::{
    bool_input, color_input, integer_input, literal_value, number_input,
};
use crate::v31::V31RenderError;

pub(crate) fn highlighter_shader(node: &NodeSpec) -> Result<HighlighterShader, V31RenderError> {
    let text_contrast = number_input(node, "textContrast");
    if text_contrast > 0.0 {
        return Err(V31RenderError::Unsupported(
            "shader.highlighter textContrast values above 0.0 are not supported by direct v3.1 rendering."
                .to_string(),
        ));
    }

    Ok(HighlighterShader {
        color: ColorConfig::from(color_input(node, "color")?),
        apply_to: highlighter_apply_to_input(node, "applyTo")?,
        text_contrast: TextContrast::Preserve,
        mode: highlighter_mode_input(node, "mode")?,
        band_width: number_input(node, "bandWidth").max(1.0) as u16,
        soft_edge: if bool_input(node, "softEdge")? {
            1.0
        } else {
            0.0
        },
        blend_strength: number_input(node, "blendStrength").clamp(0.0, 1.0) as f32,
        blend_strength_binding: None,
        speed: 1.0,
        speed_binding: None,
        direction: highlighter_direction_input(node, "direction")?,
        direction_binding: None,
        row_mask: highlighter_row_mask_input(node, "rowMask")?,
    })
}

fn highlighter_apply_to_input(
    node: &NodeSpec,
    id: &str,
) -> Result<HighlighterApplyTo, V31RenderError> {
    match literal_value(node, id)?.as_enum_value() {
        Some("foreground") => Ok(HighlighterApplyTo::Foreground),
        Some("background") => Ok(HighlighterApplyTo::Background),
        Some("both") => Ok(HighlighterApplyTo::Both),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.highlighter applyTo `{value}` is not supported by direct v3.1 rendering."
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.highlighter."
        ))),
    }
}

fn highlighter_mode_input(node: &NodeSpec, id: &str) -> Result<HighlighterMode, V31RenderError> {
    match literal_value(node, id)?.as_enum_value() {
        Some("band") => Ok(HighlighterMode::Band),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.highlighter mode `{value}` is not supported by direct v3.1 rendering."
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.highlighter."
        ))),
    }
}

fn highlighter_direction_input(
    node: &NodeSpec,
    id: &str,
) -> Result<HighlighterDirection, V31RenderError> {
    match literal_value(node, id)?.as_enum_value() {
        Some("leftToRight") => Ok(HighlighterDirection::Forward),
        Some("rightToLeft") => Ok(HighlighterDirection::Reverse),
        Some("topToBottom") => Ok(HighlighterDirection::TopDown),
        Some("bottomToTop") => Ok(HighlighterDirection::BottomUp),
        Some(value) => Err(V31RenderError::Unsupported(format!(
            "shader.highlighter direction `{value}` is not supported by direct v3.1 rendering."
        ))),
        None => Err(V31RenderError::Unsupported(format!(
            "Direct v3.1 rendering expected enum input `{id}` for shader.highlighter."
        ))),
    }
}

fn highlighter_row_mask_input(
    node: &NodeSpec,
    id: &str,
) -> Result<HighlighterRowMask, V31RenderError> {
    let row = integer_input(node, id)?;
    if row >= 0 {
        let row = row as u16;
        Ok(HighlighterRowMask::Range {
            start: row,
            end: row,
        })
    } else {
        Ok(HighlighterRowMask::AllRows)
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_highlighter_shader.rs</FILE> - <DESC>Build direct v3.1 highlighter shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
