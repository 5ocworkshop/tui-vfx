// <FILE>crates/tui-vfx-compost/src/shaders/fnc_linear_gradient_style.rs</FILE> - <DESC>Execute native linearGradient style from canonical v3.1 fields</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Reuse the proven LinearGradientShader math while constructing it from canonical v3.1 NodeSpec fields.</WCTX>
// <CLOG>0.1.1: PATCH — read canonical channelTarget for gradient application.</CLOG>

use tui_vfx_contract::{GradientSpec, NodeSpec};
use tui_vfx_style::models::{ColorSpace, Gradient, LinearGradientApplyTo, LinearGradientShader};
use tui_vfx_style::traits::{ShaderContext, StyleShader};
use tui_vfx_types::Style;

use crate::shaders::{enum_input, gradient_input, number_input};

#[allow(clippy::too_many_arguments)]
pub(crate) fn linear_gradient_style(
    node: &NodeSpec,
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    screen_x: u16,
    screen_y: u16,
    phase_t: f64,
    base: Style,
) -> Style {
    let shader = LinearGradientShader {
        gradient: gradient_from_node(node),
        angle_deg: number_input(node, "angleDeg") as f32,
        apply_to: apply_to_from_node(node),
        intensity: number_input(node, "intensity") as f32,
    };
    let context = ShaderContext::new(
        local_x, local_y, width, height, screen_x, screen_y, phase_t, None, None,
    );
    shader.style_at(&context, base)
}

fn gradient_from_node(node: &NodeSpec) -> Gradient {
    gradient_input(node, "gradient")
        .map(gradient_from_spec)
        .unwrap_or_else(Gradient::default)
}

fn gradient_from_spec(spec: &GradientSpec) -> Gradient {
    Gradient {
        stops: spec
            .stops
            .iter()
            .map(|stop| (stop.position as f32, stop.color))
            .collect(),
        space: match spec.space.as_str() {
            "hct" => ColorSpace::Hct,
            _ => ColorSpace::Rgb,
        },
    }
}

fn apply_to_from_node(node: &NodeSpec) -> LinearGradientApplyTo {
    match enum_input(node, "channelTarget") {
        "background" => LinearGradientApplyTo::Background,
        "both" => LinearGradientApplyTo::Both,
        _ => LinearGradientApplyTo::Foreground,
    }
}

// <FILE>crates/tui-vfx-compost/src/shaders/fnc_linear_gradient_style.rs</FILE> - <DESC>Execute native linearGradient style from canonical v3.1 fields</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
