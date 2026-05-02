// <FILE>crates/tui-vfx-compost/src/render/fnc_element_bounds_fully_visible.rs</FILE> - <DESC>Check whether an element surface is fully inside scene bounds</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Warn clipping and hide overflow need the same source-sized placement test.</WCTX>
// <CLOG>0.1.0: INIT — add full-visibility bounds check for scene elements.</CLOG>

use tui_vfx_contract::ElementPlacement;

pub(crate) fn element_bounds_fully_visible(
    placement: ElementPlacement,
    source_width: usize,
    source_height: usize,
    scene_width: usize,
    scene_height: usize,
) -> bool {
    let left = i64::from(placement.x);
    let top = i64::from(placement.y);
    let right = left + source_width as i64;
    let bottom = top + source_height as i64;
    left >= 0 && top >= 0 && right <= scene_width as i64 && bottom <= scene_height as i64
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_element_bounds_fully_visible.rs</FILE> - <DESC>Check whether an element surface is fully inside scene bounds</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
