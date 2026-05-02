// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene_element.rs</FILE> - <DESC>Render one source-backed scene element</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Element orchestration materializes semantic source surfaces, applies visibility and overflow, clips placement, applies effects, and returns evidence.</WCTX>
// <CLOG>0.6.0: MINOR — execute phase/predicate visibility, warn clipping, hide overflow, and wrap overflow.
// 0.5.0: MINOR — pass semantic source surfaces and lifecycle-aware applied-effect evidence.
// 0.4.0: MINOR — return diagnostics for skipped fully clipped elements.
// 0.3.0: MINOR — route element effects through the effect stack substrate.
// 0.2.1: PATCH — keep render imports rustfmt-aligned after source dispatch wiring.
// 0.2.0: MINOR — route source materialization through descriptor dispatch seam.
// 0.1.0: INIT — add element render orchestration for scene composition.</CLOG>

use tui_vfx_contract::{
    RecipeDocument, RecipeScene, RecipeSceneElement, SceneElementOverflowPolicy,
};
use tui_vfx_types::SemanticScene;

use crate::render::{
    ElementRenderOutcome, RenderError, SampleContext, build_effect_stack, is_scene_element_visible,
    render_clipped_scene_element, render_hidden_overflow_scene_element,
    render_wrapped_scene_element,
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
    let runtime_context = RuntimeContext::from_sample(sample).with_graph_defaults(&recipe.graph);
    if !is_scene_element_visible(element, &runtime_context) {
        return Ok(ElementRenderOutcome::default());
    }

    let source = recipe
        .sources
        .get(&element.source_instance)
        .ok_or_else(|| missing_source_error(element))?;
    let source_grid = materialize_source(source, &runtime_context)?;
    let effect_stack = build_effect_stack(recipe, element)?;
    let applied_effect_kinds = effect_stack.applied_effect_kinds(sample);

    match element.overflow.unwrap_or(SceneElementOverflowPolicy::Clip) {
        SceneElementOverflowPolicy::Clip => render_clipped_scene_element(
            scene,
            element,
            sample,
            destination,
            &source_grid,
            &effect_stack,
            &runtime_context,
            applied_effect_kinds,
        ),
        SceneElementOverflowPolicy::Hide => render_hidden_overflow_scene_element(
            scene,
            element,
            sample,
            destination,
            &source_grid,
            &effect_stack,
            &runtime_context,
            applied_effect_kinds,
        ),
        SceneElementOverflowPolicy::Wrap => render_wrapped_scene_element(
            scene,
            element,
            sample,
            destination,
            &source_grid,
            &effect_stack,
            &runtime_context,
            applied_effect_kinds,
        ),
    }
}

fn missing_source_error(element: &RecipeSceneElement) -> RenderError {
    RenderError::Unsupported(format!(
        "scene element `{}` references missing source `{}`",
        element.id.as_str(),
        element.source_instance.as_str()
    ))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene_element.rs</FILE> - <DESC>Render one source-backed scene element</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
