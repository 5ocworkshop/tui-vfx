// <FILE>crates/tui-vfx-compost/src/render/col_scene_elements_in_paint_order.rs</FILE> - <DESC>Order scene elements for deterministic painting</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 canonical scene rendering uses z-index order without layer DTOs.</WCTX>
// <CLOG>0.1.0: INIT — add stable z-index paint ordering helper.</CLOG>

use tui_vfx_contract::RecipeSceneElement;

pub(crate) fn scene_elements_in_paint_order(
    elements: &[RecipeSceneElement],
) -> Vec<&RecipeSceneElement> {
    let mut indexed = elements.iter().enumerate().collect::<Vec<_>>();
    // Layer is lightweight grouping metadata in Phase 1; z-index plus declaration
    // order owns paint order until a later layer-policy substrate says otherwise.
    indexed.sort_by(|(left_index, left), (right_index, right)| {
        left.z_index
            .cmp(&right.z_index)
            .then_with(|| left_index.cmp(right_index))
    });
    indexed.into_iter().map(|(_, element)| element).collect()
}

// <FILE>crates/tui-vfx-compost/src/render/col_scene_elements_in_paint_order.rs</FILE> - <DESC>Order scene elements for deterministic painting</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
