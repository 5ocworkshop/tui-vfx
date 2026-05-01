// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/fnc_render_v31_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through compositor-next</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Public render function coordinates source extraction, composition building, and pipeline execution only.</WCTX>
// <CLOG>0.1.0: INIT — extract render_v31_recipe from the former render hub.</CLOG>

use crate::pipeline::render_pipeline_with_spec;
use tui_vfx_types::{Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

use super::source::source_grid_from_inputs;
use super::{V31Frame, V31RenderError, V31SampleContext, composition_spec_for_element};
use crate::v31::LoadedV31Recipe;

/// Render a load-validated canonical v3.1 recipe through compositor-next directly.
pub fn render_v31_recipe(
    loaded: &LoadedV31Recipe,
    sample: &V31SampleContext,
) -> Result<V31Frame, V31RenderError> {
    let recipe = loaded.recipe();
    let scene = recipe
        .scenes
        .first()
        .ok_or_else(|| V31RenderError::Unsupported("Recipe has no scene to render.".to_string()))?;
    let element = scene.elements.first().ok_or_else(|| {
        V31RenderError::Unsupported("Recipe scene has no source element to render.".to_string())
    })?;
    let source = recipe.sources.get(&element.source).ok_or_else(|| {
        V31RenderError::Unsupported(format!(
            "Recipe scene element `{}` references missing source `{}`.",
            element.id.as_str(),
            element.source.as_str()
        ))
    })?;
    let source_grid = source_grid_from_inputs(&source.inputs, scene.width, scene.height)?;
    let source_roles = RoleMap::new_with_default(
        source_grid.width() as u16,
        source_grid.height() as u16,
        RoleTag::Text,
    );
    let mut destination = SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(scene.width, scene.height),
        RoleTag::Background,
    );
    let (spec, applied_effect_kinds) = composition_spec_for_element(recipe, element, sample)?;

    render_pipeline_with_spec(
        &source_grid,
        &source_roles,
        &mut destination,
        source_grid.width(),
        source_grid.height(),
        element.placement.x.max(0) as usize,
        element.placement.y.max(0) as usize,
        &spec,
        None,
    );

    Ok(V31Frame {
        recipe_id: recipe.id.as_str().to_string(),
        width: scene.width,
        height: scene.height,
        grid: destination,
        diagnostics: vec![],
        applied_effect_kinds,
    })
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/fnc_render_v31_recipe.rs</FILE> - <DESC>Render a loaded v3.1 recipe through compositor-next</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
