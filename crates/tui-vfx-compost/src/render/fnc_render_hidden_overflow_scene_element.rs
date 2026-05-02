// <FILE>crates/tui-vfx-compost/src/render/fnc_render_hidden_overflow_scene_element.rs</FILE> - <DESC>Render hide-overflow scene element behavior</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Hide overflow skips the whole element when any source-local cell leaves scene bounds.</WCTX>
// <CLOG>0.2.0: MINOR — emit trace evidence when hide overflow skips an element.
// 0.1.0: INIT — split hide-overflow rendering from element orchestration.</CLOG>

use tui_vfx_contract::{RecipeScene, RecipeSceneElement};
use tui_vfx_types::SemanticScene;

use crate::render::{
    EffectStack, ElementRenderOutcome, RenderError, RenderSkipReason, SampleContext,
    render_clipped_scene_element, source_fits_scene, trace_element_skipped,
};
use crate::runtime::RuntimeContext;

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_hidden_overflow_scene_element(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    sample: &SampleContext,
    destination: &mut SemanticScene,
    source_grid: &SemanticScene,
    effect_stack: &EffectStack<'_>,
    runtime_context: &RuntimeContext,
) -> Result<ElementRenderOutcome, RenderError> {
    if !source_fits_scene(scene, element, source_grid) {
        let message = format!(
            "scene element `{}` hidden because overflow policy is hide",
            element.id.as_str()
        );
        return Ok(ElementRenderOutcome::skipped_with_trace(
            message,
            trace_element_skipped(scene, element, RenderSkipReason::OverflowHide),
        ));
    }
    render_clipped_scene_element(
        scene,
        element,
        sample,
        destination,
        source_grid,
        effect_stack,
        runtime_context,
    )
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_hidden_overflow_scene_element.rs</FILE> - <DESC>Render hide-overflow scene element behavior</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
