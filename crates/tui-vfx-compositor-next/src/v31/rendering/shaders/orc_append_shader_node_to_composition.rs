// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/orc_append_shader_node_to_composition.rs</FILE> - <DESC>Append supported v3.1 shader nodes to a composition spec</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Shader dispatch is additive; each primitive builder lives in its own OFPF file.</WCTX>
// <CLOG>0.1.0: INIT — extract shader append dispatch.</CLOG>

use crate::pipeline::{CompositionSpec, ShaderLayerSpec};
use tui_vfx_contract::NodeSpec;
use tui_vfx_style::models::{SpatialShaderType, StyleRegion};

use super::fnc_border_sweep_shader::border_sweep_shader;
use super::fnc_focus_field_shader::focus_field_shader;
use super::fnc_glisten_band_shader::glisten_band_shader;
use super::fnc_highlighter_shader::highlighter_shader;
use super::fnc_linear_gradient_shader::linear_gradient_shader;
use crate::v31::V31RenderError;

pub(crate) fn append_shader_node_to_composition(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    applied_effect_kinds: &mut Vec<String>,
) -> Result<(), V31RenderError> {
    let shader = match node.effect.as_str() {
        "shader.linearGradient" => SpatialShaderType::LinearGradient(linear_gradient_shader(node)?),
        "shader.highlighter" => SpatialShaderType::Highlighter(highlighter_shader(node)?),
        "shader.glistenBand" => SpatialShaderType::GlistenBand(glisten_band_shader(node)?),
        "shader.focusField" => SpatialShaderType::FocusField(focus_field_shader(node)?),
        "shader.borderSweep" => SpatialShaderType::BorderSweep(border_sweep_shader(node)?),
        other => {
            return Err(V31RenderError::Unsupported(format!(
                "Direct v3.1 rendering does not support effect `{other}`."
            )));
        }
    };
    spec.shader_layers.push(ShaderLayerSpec {
        shader,
        region: StyleRegion::All,
    });
    applied_effect_kinds.push(node.effect.as_str().to_string());
    Ok(())
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/orc_append_shader_node_to_composition.rs</FILE> - <DESC>Append supported v3.1 shader nodes to a composition spec</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
