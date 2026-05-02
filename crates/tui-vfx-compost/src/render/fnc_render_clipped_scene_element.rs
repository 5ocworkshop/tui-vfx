// <FILE>crates/tui-vfx-compost/src/render/fnc_render_clipped_scene_element.rs</FILE> - <DESC>Render clipped scene element overflow behavior</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Clip and warn clipping share mature render-area style bounds with element diagnostics and stage trace evidence.</WCTX>
// <CLOG>0.4.0: MINOR — allocate shadow trace identities after authored and graph-stage identities.
// 0.3.0: MINOR — aggregate trace evidence after actual shadow and effect execution.
// 0.2.0: MINOR — emit shadow and effect-stack trace evidence from clipped rendering.
// 0.1.0: INIT — split clipped scene element rendering from element orchestration.</CLOG>

use tui_vfx_contract::{ClipPolicy, RecipeScene, RecipeSceneElement, ShadowCompositeMode};
use tui_vfx_types::{Grid, SemanticScene};

use crate::render::{
    EffectStack, ElementClipBounds, ElementRenderOutcome, RenderDiagnostic, RenderError,
    RenderSkipReason, SampleContext, ScopeCoordinateMode, apply_effect_stack, clip_element_bounds,
    element_bounds_fully_visible, render_element_shadow, shadow_cast_rect, shadow_edge_progress,
    trace_element_skipped,
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
) -> Result<ElementRenderOutcome, RenderError> {
    let Some(bounds) = clip_element_bounds(
        element.placement,
        source_grid.grid().width(),
        source_grid.grid().height(),
        scene.width,
        scene.height,
    ) else {
        let message = format!(
            "scene element `{}` is fully clipped by scene bounds",
            element.id.as_str()
        );
        return Ok(ElementRenderOutcome::skipped_with_trace(
            message,
            trace_element_skipped(scene, element, RenderSkipReason::FullyClipped),
        ));
    };

    let diagnostics = clip_warning(scene, element, source_grid);
    let under_shadow_cells = render_shadow_for_mode(
        element,
        source_grid,
        destination,
        ShadowCompositeMode::Under,
        bounds,
    );
    let mut stage_accumulator = apply_effect_stack(
        source_grid,
        destination,
        bounds,
        sample,
        effect_stack,
        runtime_context,
        ScopeCoordinateMode::VisibleBounds,
    )?;
    if let Some(cells) = under_shadow_cells {
        stage_accumulator.record_shadow_cells(under_shadow_stage_index(effect_stack), cells);
    }
    if let Some(cells) = render_shadow_for_mode(
        element,
        source_grid,
        destination,
        ShadowCompositeMode::Over,
        bounds,
    ) {
        stage_accumulator.record_shadow_cells(over_shadow_stage_index(effect_stack), cells);
    }
    let stage_trace = stage_accumulator.finish(scene, element);
    Ok(ElementRenderOutcome::applied_with_diagnostics_and_trace(
        stage_trace.applied_effect_kinds,
        diagnostics,
        stage_trace.trace_events,
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

fn under_shadow_stage_index(effect_stack: &EffectStack<'_>) -> usize {
    effect_stack.stage_count() + effect_stack.synthetic_graph_stage_count()
}

fn over_shadow_stage_index(effect_stack: &EffectStack<'_>) -> usize {
    under_shadow_stage_index(effect_stack) + 1
}

fn render_shadow_for_mode(
    element: &RecipeSceneElement,
    source_grid: &SemanticScene,
    destination: &mut SemanticScene,
    mode: ShadowCompositeMode,
    bounds: ElementClipBounds,
) -> Option<u32> {
    let shadow = element
        .surface
        .as_ref()
        .and_then(|surface| surface.shadow.as_ref())?;
    if shadow.composite_mode != mode {
        return None;
    }
    let rect = shadow_cast_rect(element, source_grid, bounds, shadow);
    Some(render_element_shadow(
        destination,
        shadow,
        rect,
        shadow_edge_progress(source_grid, bounds, shadow),
    ))
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
// <VERS>END OF VERSION: 0.4.0</VERS>
