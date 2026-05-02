// <FILE>crates/tui-vfx-compost/src/render/fnc_wrap_element_cell_bounds.rs</FILE> - <DESC>Build wrapped one-cell element bounds</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Wrap overflow maps every source-local cell into scene coordinates with signed modulo placement.</WCTX>
// <CLOG>0.1.0: INIT — add deterministic per-cell wrap bounds for overflow wrapping.</CLOG>

use tui_vfx_contract::ElementPlacement;

use crate::render::ElementClipBounds;

pub(crate) fn wrap_element_cell_bounds(
    placement: ElementPlacement,
    local_x: usize,
    local_y: usize,
    scene_width: usize,
    scene_height: usize,
) -> Option<ElementClipBounds> {
    if scene_width == 0 || scene_height == 0 {
        return None;
    }
    let dest_x = (i64::from(placement.x) + local_x as i64).rem_euclid(scene_width as i64);
    let dest_y = (i64::from(placement.y) + local_y as i64).rem_euclid(scene_height as i64);
    Some(ElementClipBounds {
        local_x_start: local_x,
        local_y_start: local_y,
        dest_x_start: dest_x as usize,
        dest_y_start: dest_y as usize,
        width: 1,
        height: 1,
    })
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_wrap_element_cell_bounds.rs</FILE> - <DESC>Build wrapped one-cell element bounds</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
