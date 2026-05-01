// <FILE>crates/tui-vfx-compost/src/render/orc_render_observability.rs</FILE> - <DESC>Native render observability helpers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render observability builds frame trace evidence from canonical scene and element ids.</WCTX>
// <CLOG>0.1.0: INIT — add trace event builders for applied element stages.</CLOG>

use tui_vfx_contract::{RecipeScene, RecipeSceneElement};

use crate::render::RenderTraceEvent;

pub(crate) fn trace_applied_effects(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    applied_effect_kinds: &[String],
) -> Vec<RenderTraceEvent> {
    applied_effect_kinds
        .iter()
        .enumerate()
        .map(|(stage_index, effect)| {
            RenderTraceEvent::effect_stage(
                scene.id.as_str(),
                element.id.as_str(),
                stage_index,
                effect.as_str(),
            )
        })
        .collect()
}

// <FILE>crates/tui-vfx-compost/src/render/orc_render_observability.rs</FILE> - <DESC>Native render observability helpers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
