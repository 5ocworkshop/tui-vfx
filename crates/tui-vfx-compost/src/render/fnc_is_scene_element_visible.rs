// <FILE>crates/tui-vfx-compost/src/render/fnc_is_scene_element_visible.rs</FILE> - <DESC>Evaluate scene element visibility policies</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Visibility is render-time substrate because it depends on sample lifecycle and runtime values.</WCTX>
// <CLOG>0.1.0: INIT — add always, phase, and predicate visibility evaluation.</CLOG>

use tui_vfx_contract::{RecipeSceneElement, SceneElementVisibility};

use crate::render::evaluate_value_predicate;
use crate::runtime::{RuntimeContext, resolve_value_source};

pub(crate) fn is_scene_element_visible(
    element: &RecipeSceneElement,
    context: &RuntimeContext,
) -> bool {
    match &element.visibility {
        None | Some(SceneElementVisibility::Always) => true,
        Some(SceneElementVisibility::Phase { phases }) => context
            .lifecycle_phase
            .is_some_and(|phase| phases.contains(&phase)),
        Some(SceneElementVisibility::Predicate {
            predicate_source,
            predicate,
        }) => resolve_value_source(predicate_source, context)
            .map(|value| evaluate_value_predicate(value.value(), predicate))
            .unwrap_or(false),
    }
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_is_scene_element_visible.rs</FILE> - <DESC>Evaluate scene element visibility policies</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
