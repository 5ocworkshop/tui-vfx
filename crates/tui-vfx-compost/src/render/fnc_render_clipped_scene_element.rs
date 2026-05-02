// <FILE>crates/tui-vfx-compost/src/render/fnc_render_clipped_scene_element.rs</FILE> - <DESC>Render clipped scene element overflow behavior</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Clip and warn clipping share mature render-area style bounds with element diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — split clipped scene element rendering from element orchestration.</CLOG>

use tui_vfx_contract::{ClipPolicy, RecipeScene, RecipeSceneElement};
use tui_vfx_types::{Grid, SemanticScene};

use crate::render::{
    EffectStack, ElementRenderOutcome, RenderDiagnostic, RenderError, SampleContext,
    apply_effect_stack, clip_element_bounds, element_bounds_fully_visible,
};
use crate::runtime::RuntimeContext;

#[allow(clippy::too_many_arguments)]
pub(crate) fn render_clipped_scene_element(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    sample: &SampleContext,
    destination: &mut SemanticScene,
    source_grid: &SemanticScene,
    effect_stack: &EffectStack<'_>,
    runtime_context: &RuntimeContext,
    applied_effect_kinds: Vec<String>,
) -> Result<ElementRenderOutcome, RenderError> {
    let Some(bounds) = clip_element_bounds(
        element.placement,
        source_grid.grid().width(),
        source_grid.grid().height(),
        scene.width,
        scene.height,
    ) else {
        return Ok(ElementRenderOutcome::skipped(format!(
            "scene element `{}` is fully clipped by scene bounds",
            element.id.as_str()
        )));
    };

    let diagnostics = clip_warning(scene, element, source_grid);
    apply_effect_stack(
        source_grid,
        destination,
        bounds,
        sample,
        effect_stack,
        runtime_context,
    )?;
    Ok(ElementRenderOutcome::applied_with_diagnostics(
        applied_effect_kinds,
        diagnostics,
    ))
}

pub(crate) fn source_fits_scene(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    source_grid: &SemanticScene,
) -> bool {
    element_bounds_fully_visible(
        element.placement,
        source_grid.grid().width(),
        source_grid.grid().height(),
        scene.width,
        scene.height,
    )
}

fn clip_warning(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    source_grid: &SemanticScene,
) -> Vec<RenderDiagnostic> {
    if element.clip_policy != ClipPolicy::Warn || source_fits_scene(scene, element, source_grid) {
        return Vec::new();
    }
    vec![RenderDiagnostic::new(format!(
        "scene element `{}` clipped by scene bounds with warn clip policy",
        element.id.as_str()
    ))]
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_clipped_scene_element.rs</FILE> - <DESC>Render clipped scene element overflow behavior</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
