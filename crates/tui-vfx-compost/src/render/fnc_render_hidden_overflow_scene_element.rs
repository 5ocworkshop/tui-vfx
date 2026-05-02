// <FILE>crates/tui-vfx-compost/src/render/fnc_render_hidden_overflow_scene_element.rs</FILE> - <DESC>Render hide-overflow scene element behavior</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Hide overflow skips the whole element when any source-local cell leaves scene bounds.</WCTX>
// <CLOG>0.1.0: INIT — split hide-overflow rendering from element orchestration.</CLOG>

use tui_vfx_contract::{RecipeScene, RecipeSceneElement};
use tui_vfx_types::SemanticScene;

use crate::render::{
    EffectStack, ElementRenderOutcome, RenderError, SampleContext, render_clipped_scene_element,
    source_fits_scene,
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
    applied_effect_kinds: Vec<String>,
) -> Result<ElementRenderOutcome, RenderError> {
    if !source_fits_scene(scene, element, source_grid) {
        return Ok(ElementRenderOutcome::skipped(format!(
            "scene element `{}` hidden because overflow policy is hide",
            element.id.as_str()
        )));
    }
    render_clipped_scene_element(
        scene,
        element,
        sample,
        destination,
        source_grid,
        effect_stack,
        runtime_context,
        applied_effect_kinds,
    )
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_hidden_overflow_scene_element.rs</FILE> - <DESC>Render hide-overflow scene element behavior</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
