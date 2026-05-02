// <FILE>crates/tui-vfx-compost/src/render/fnc_render_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through native compost modules</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>The recipe entrypoint stays thin and delegates scene rendering plus observability aggregation.</WCTX>
// <CLOG>0.4.0: MINOR — add explicit scene rendering for multi-scene recipes.
// 0.3.1: PATCH — use stable one-scene unsupported diagnostic wording.
// 0.3.0: MINOR — carry diagnostics and trace events into Frame.
// 0.2.0: MINOR — delegate scene rendering so all scene elements compose in paint order.
// 0.1.1: PATCH — read scene element sourceInstance after the contract naming audit.</CLOG>

use crate::LoadedRecipe;
use crate::render::{Frame, RenderError, SampleContext, render_scene};

/// Execute a load-validated canonical v3.1 recipe through compost.
pub fn render_recipe(loaded: &LoadedRecipe, sample: &SampleContext) -> Result<Frame, RenderError> {
    let recipe = loaded.recipe();
    if recipe.scenes.len() > 1 {
        return Err(RenderError::Unsupported(
            "render_recipe requires exactly one scene per sample".to_string(),
        ));
    }
    let scene = recipe
        .scenes
        .first()
        .ok_or_else(|| RenderError::Unsupported("recipe has no scene to render".to_string()))?;
    frame_from_scene(loaded, scene.id.as_str(), sample)
}

/// Execute one named scene from a load-validated canonical v3.1 recipe.
pub fn render_recipe_scene(
    loaded: &LoadedRecipe,
    scene_id: &str,
    sample: &SampleContext,
) -> Result<Frame, RenderError> {
    frame_from_scene(loaded, scene_id, sample)
}

fn frame_from_scene(
    loaded: &LoadedRecipe,
    scene_id: &str,
    sample: &SampleContext,
) -> Result<Frame, RenderError> {
    let recipe = loaded.recipe();
    let scene = recipe
        .scenes
        .iter()
        .find(|scene| scene.id.as_str() == scene_id)
        .ok_or_else(|| RenderError::Unsupported(format!("recipe has no scene `{scene_id}`")))?;
    let (grid, applied_effect_kinds, diagnostics, trace_events) =
        render_scene(recipe, scene, sample)?;

    Ok(Frame {
        recipe_id: recipe.id.as_str().to_string(),
        width: scene.width,
        height: scene.height,
        grid,
        applied_effect_kinds,
        diagnostics,
        trace_events,
    })
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through native compost modules</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
