// <FILE>crates/tui-vfx-compost/src/render/fnc_render_wrapped_scene_element.rs</FILE> - <DESC>Render wrap-overflow scene element behavior</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Wrap overflow maps every source-local cell into scene bounds using signed modulo placement.</WCTX>
// <CLOG>0.1.0: INIT — split wrapped scene element rendering from element orchestration.</CLOG>

use tui_vfx_contract::{RecipeScene, RecipeSceneElement};
use tui_vfx_types::{Grid, SemanticScene};

use crate::render::{
    EffectStack, ElementRenderOutcome, RenderError, SampleContext, apply_effect_stack,
    wrap_element_cell_bounds,
};
use crate::runtime::RuntimeContext;

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_wrapped_scene_element(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    sample: &SampleContext,
    destination: &mut SemanticScene,
    source_grid: &SemanticScene,
    effect_stack: &EffectStack<'_>,
    runtime_context: &RuntimeContext,
    applied_effect_kinds: Vec<String>,
) -> Result<ElementRenderOutcome, RenderError> {
    for local_y in 0..source_grid.grid().height() {
        for local_x in 0..source_grid.grid().width() {
            let Some(bounds) = wrap_element_cell_bounds(
                element.placement,
                local_x,
                local_y,
                scene.width,
                scene.height,
            ) else {
                continue;
            };
            apply_effect_stack(
                source_grid,
                destination,
                bounds,
                sample,
                effect_stack,
                runtime_context,
            )?;
        }
    }
    Ok(ElementRenderOutcome::applied(applied_effect_kinds))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_wrapped_scene_element.rs</FILE> - <DESC>Render wrap-overflow scene element behavior</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
