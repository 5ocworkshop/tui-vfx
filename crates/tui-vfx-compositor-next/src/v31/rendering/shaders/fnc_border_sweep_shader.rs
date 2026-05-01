// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_border_sweep_shader.rs</FILE> - <DESC>Build direct v3.1 borderSweep shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Border sweep shader mapping owns only its descriptor-to-style conversion.</WCTX>
// <CLOG>0.1.0: INIT — extract borderSweep shader builder.</CLOG>

use tui_vfx_contract::NodeSpec;
use tui_vfx_style::models::{BorderSweepShader, ColorConfig};

use super::col_shader_value_input::{color_input, integer_input, number_input};
use crate::v31::V31RenderError;

pub(crate) fn border_sweep_shader(node: &NodeSpec) -> Result<BorderSweepShader, V31RenderError> {
    Ok(BorderSweepShader {
        speed: number_input(node, "speed").max(0.0) as f32,
        length: integer_input(node, "length")?.max(1) as u16,
        color: ColorConfig::from(color_input(node, "color")?),
        head: None,
        tail: None,
        position_binding: None,
    })
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/shaders/fnc_border_sweep_shader.rs</FILE> - <DESC>Build direct v3.1 borderSweep shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
