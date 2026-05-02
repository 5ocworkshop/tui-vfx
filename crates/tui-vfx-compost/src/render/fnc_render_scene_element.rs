// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene_element.rs</FILE> - <DESC>Render one source-backed scene element</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Element orchestration materializes semantic source surfaces, clips placement, applies effects, and returns observability evidence.</WCTX>
// <CLOG>0.5.0: MINOR — pass semantic source surfaces and lifecycle-aware applied-effect evidence.
// 0.4.0: MINOR — return diagnostics for skipped fully clipped elements.
// 0.3.0: MINOR — route element effects through the effect stack substrate.
// 0.2.1: PATCH — keep render imports rustfmt-aligned after source dispatch wiring.
// 0.2.0: MINOR — route source materialization through descriptor dispatch seam.
// 0.1.0: INIT — add element render orchestration for scene composition.</CLOG>

use tui_vfx_contract::{RecipeDocument, RecipeScene, RecipeSceneElement};
use tui_vfx_types::{Grid, SemanticScene};

use crate::render::{
    ElementRenderOutcome, RenderError, SampleContext, apply_effect_stack, build_effect_stack,
    clip_element_bounds,
};
use crate::runtime::RuntimeContext;
use crate::source::materialize_source;

pub(crate) fn render_scene_element(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    sample: &SampleContext,
    destination: &mut SemanticScene,
) -> Result<ElementRenderOutcome, RenderError> {
    let source = recipe
        .sources
        .get(&element.source_instance)
        .ok_or_else(|| {
            RenderError::Unsupported(format!(
                "scene element `{}` references missing source `{}`",
                element.id.as_str(),
                element.source_instance.as_str()
            ))
        })?;
    let runtime_context = RuntimeContext::from_sample(sample).with_graph_defaults(&recipe.graph);
    let source_grid = materialize_source(source, &runtime_context)?;
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
    let effect_stack = build_effect_stack(recipe, element)?;
    let applied_effect_kinds = effect_stack.applied_effect_kinds(sample);

    apply_effect_stack(
        &source_grid,
        destination,
        bounds,
        sample,
        &effect_stack,
        &runtime_context,
    )?;

    Ok(ElementRenderOutcome::applied(applied_effect_kinds))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene_element.rs</FILE> - <DESC>Render one source-backed scene element</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
