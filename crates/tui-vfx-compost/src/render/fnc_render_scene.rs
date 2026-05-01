// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene.rs</FILE> - <DESC>Render one canonical recipe scene</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 scene orchestration owns destination allocation and element paint order.</WCTX>
// <CLOG>0.1.0: INIT — add multi-element scene render orchestration.</CLOG>

use tui_vfx_contract::{RecipeDocument, RecipeScene};
use tui_vfx_types::{OwnedGrid, RoleTag, SemanticScene};

use crate::render::{
    RenderError, SampleContext, render_scene_element, scene_elements_in_paint_order,
};

pub(crate) fn render_scene(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    sample: &SampleContext,
) -> Result<(SemanticScene, Vec<String>), RenderError> {
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

    for element in scene_elements_in_paint_order(&scene.elements) {
        let applied = render_scene_element(recipe, scene, element, sample, &mut grid)?;
        applied_effect_kinds.extend(applied);
    }

    Ok((grid, applied_effect_kinds))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_render_scene.rs</FILE> - <DESC>Render one canonical recipe scene</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
