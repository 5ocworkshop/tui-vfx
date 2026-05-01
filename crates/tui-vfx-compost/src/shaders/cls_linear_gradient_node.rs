// <FILE>crates/tui-vfx-compost/src/shaders/cls_linear_gradient_node.rs</FILE> - <DESC>Native linearGradient node wrapper</DESC>
// <VERS>VERSION: 0.1.2</VERS>
// <WCTX>LinearGradientNode executes canonical v3.1 shader.linearGradient fields directly and exposes its node for timing resolution.</WCTX>
// <CLOG>0.1.2: PATCH — remove redundant effect-id accessor after constructor validation.
// 0.1.1: PATCH — expose borrowed node for the timing seam.
// 0.1.0: INIT — add linearGradient node wrapper.</CLOG>

use tui_vfx_contract::NodeSpec;
use tui_vfx_types::Style;

use crate::RenderError;
use crate::shaders::linear_gradient_style;

/// Borrowed v3.1 graph node for `shader.linearGradient`.
#[derive(Debug)]
pub(crate) struct LinearGradientNode<'a> {
    node: &'a NodeSpec,
}

impl<'a> LinearGradientNode<'a> {
    pub(crate) fn new(node: &'a NodeSpec) -> Result<Self, RenderError> {
        match node.effect.as_str() {
            "shader.linearGradient" => Ok(Self { node }),
            other => Err(RenderError::Unsupported(format!(
                "native render does not support effect `{other}`"
            ))),
        }
    }

    pub(crate) fn node(&self) -> &'a NodeSpec {
        self.node
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn style_at(
        &self,
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        screen_x: u16,
        screen_y: u16,
        phase_t: f64,
        base: Style,
    ) -> Style {
        linear_gradient_style(
            self.node, local_x, local_y, width, height, screen_x, screen_y, phase_t, base,
        )
    }
}

// <FILE>crates/tui-vfx-compost/src/shaders/cls_linear_gradient_node.rs</FILE> - <DESC>Native linearGradient node wrapper</DESC>
// <VERS>END OF VERSION: 0.1.2</VERS>
