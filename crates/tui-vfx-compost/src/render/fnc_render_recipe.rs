// <FILE>crates/tui-vfx-compost/src/render/fnc_render_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through native compost modules</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 1 keeps the recipe entrypoint thin and delegates scene/element orchestration.</WCTX>
// <CLOG>0.2.0: MINOR — delegate scene rendering so all scene elements compose in paint order.
// 0.1.1: PATCH — read scene element sourceInstance after the contract naming audit.</CLOG>

use crate::LoadedRecipe;
use crate::render::{Frame, RenderError, SampleContext, render_scene};

/// Execute a load-validated canonical v3.1 recipe through compost.
pub fn render_recipe(loaded: &LoadedRecipe, sample: &SampleContext) -> Result<Frame, RenderError> {
    let recipe = loaded.recipe();
    if recipe.scenes.len() > 1 {
        return Err(RenderError::Unsupported(
            "render_recipe currently supports one scene per sample".to_string(),
        ));
    }
    let scene = recipe
        .scenes
        .first()
        .ok_or_else(|| RenderError::Unsupported("recipe has no scene to render".to_string()))?;
    let (grid, applied_effect_kinds) = render_scene(recipe, scene, sample)?;

    Ok(Frame {
        recipe_id: recipe.id.as_str().to_string(),
        width: scene.width,
        height: scene.height,
        grid,
        applied_effect_kinds,
    })
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through native compost modules</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
