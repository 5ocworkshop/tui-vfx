// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene.rs</FILE> - <DESC>Render one canonical recipe scene</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Scene orchestration owns destination allocation, element paint order, and observability aggregation.</WCTX>
// <CLOG>0.3.0: MINOR — aggregate trace events from element render outcomes.
// 0.2.1: PATCH — name the scene render output tuple for clearer orchestration signatures.
// 0.2.0: MINOR — aggregate element diagnostics and trace events.
// 0.1.0: INIT — add multi-element scene render orchestration.</CLOG>

use tui_vfx_contract::{RecipeDocument, RecipeScene};
use tui_vfx_types::{OwnedGrid, RoleTag, SemanticScene};

use crate::render::{
    RenderDiagnostic, RenderError, RenderTraceEvent, SampleContext, render_scene_element,
    scene_elements_in_paint_order,
};

pub(crate) type SceneRenderOutput = (
    SemanticScene,
    Vec<String>,
    Vec<RenderDiagnostic>,
    Vec<RenderTraceEvent>,
);

pub(crate) fn render_scene(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    sample: &SampleContext,
) -> Result<SceneRenderOutput, RenderError> {
    if scene.elements.is_empty() {
        return Err(RenderError::Unsupported(
            "recipe scene has no element".to_string(),
        ));
    }

    let mut grid = SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(scene.width, scene.height),
        RoleTag::Background,
    );
    let mut applied_effect_kinds = Vec::new();
    let mut diagnostics = Vec::new();
    let mut trace_events = Vec::new();

    for element in scene_elements_in_paint_order(&scene.elements) {
        let outcome = render_scene_element(recipe, scene, element, sample, &mut grid)?;
        applied_effect_kinds.extend(outcome.applied_effect_kinds);
        diagnostics.extend(outcome.diagnostics);
        trace_events.extend(outcome.trace_events);
    }

    Ok((grid, applied_effect_kinds, diagnostics, trace_events))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene.rs</FILE> - <DESC>Render one canonical recipe scene</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
