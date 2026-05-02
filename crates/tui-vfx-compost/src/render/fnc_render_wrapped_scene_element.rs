// <FILE>crates/tui-vfx-compost/src/render/fnc_render_wrapped_scene_element.rs</FILE> - <DESC>Render wrap-overflow scene element behavior</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Wrap overflow maps every source-local cell into scene bounds using signed modulo placement.</WCTX>
// <CLOG>0.4.0: MINOR — evaluate wrapped-cell scopes in element-local source coordinates.
// 0.3.0: MINOR — aggregate trace evidence from actual wrapped cell execution.
// 0.2.0: MINOR — compute effect-stack trace evidence for wrapped rendering.
// 0.1.0: INIT — split wrapped scene element rendering from element orchestration.</CLOG>

use tui_vfx_contract::{RecipeScene, RecipeSceneElement, ShadowCompositeMode};
use tui_vfx_types::{Grid, Rect, SemanticScene};

use crate::render::{
    EffectStack, ElementRenderOutcome, RenderError, RenderStageAccumulator, SampleContext,
    ScopeCoordinateMode, apply_effect_stack, render_element_shadow, wrap_element_cell_bounds,
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
) -> Result<ElementRenderOutcome, RenderError> {
    let mut stage_accumulator = RenderStageAccumulator::default();
    if let Some(cells) = render_wrapped_shadow_for_mode(
        scene,
        element,
        destination,
        source_grid,
        ShadowCompositeMode::Under,
    ) {
        stage_accumulator.record_shadow_cells(effect_stack.stage_count(), cells);
    }
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
            let cell_trace = apply_effect_stack(
                source_grid,
                destination,
                bounds,
                sample,
                effect_stack,
                runtime_context,
                ScopeCoordinateMode::SourceElement,
            )?;
            stage_accumulator.extend(cell_trace);
        }
    }
    if let Some(cells) = render_wrapped_shadow_for_mode(
        scene,
        element,
        destination,
        source_grid,
        ShadowCompositeMode::Over,
    ) {
        stage_accumulator.record_shadow_cells(effect_stack.stage_count() + 1, cells);
    }
    let stage_trace = stage_accumulator.finish(scene, element);
    Ok(ElementRenderOutcome::applied_with_diagnostics_and_trace(
        stage_trace.applied_effect_kinds,
        Vec::new(),
        stage_trace.trace_events,
    ))
}

fn render_wrapped_shadow_for_mode(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    destination: &mut SemanticScene,
    source_grid: &SemanticScene,
    mode: ShadowCompositeMode,
) -> Option<u32> {
    let shadow = element
        .surface
        .as_ref()
        .and_then(|surface| surface.shadow.as_ref())?;
    if shadow.composite_mode != mode {
        return None;
    }
    let mut written_cells = 0;
    for local_y in 0..source_grid.grid().height() {
        for local_x in 0..source_grid.grid().width() {
            let bounds = wrap_element_cell_bounds(
                element.placement,
                local_x,
                local_y,
                scene.width,
                scene.height,
            )?;
            written_cells += render_element_shadow(
                destination,
                shadow,
                Rect::new(bounds.dest_x_start as u16, bounds.dest_y_start as u16, 1, 1),
                1.0,
            );
        }
    }
    Some(written_cells)
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_wrapped_scene_element.rs</FILE> - <DESC>Render wrap-overflow scene element behavior</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
