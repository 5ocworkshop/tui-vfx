// <FILE>crates/tui-vfx-compost/src/render/fnc_clip_element_bounds.rs</FILE> - <DESC>Clip element-local bounds into scene coordinates</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Element clipping preserves signed placement origin while clipping negative and overflow cells.</WCTX>
// <CLOG>0.1.0: INIT — add signed placement clipping helper.</CLOG>

use tui_vfx_contract::ElementPlacement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ElementClipBounds {
    pub(crate) local_x_start: usize,
    pub(crate) local_y_start: usize,
    pub(crate) dest_x_start: usize,
    pub(crate) dest_y_start: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

pub(crate) fn clip_element_bounds(
    placement: ElementPlacement,
    source_width: usize,
    source_height: usize,
    scene_width: usize,
    scene_height: usize,
) -> Option<ElementClipBounds> {
    // Preserve the mature compositor render-area idea — a source-sized surface
    // written at an explicit destination offset — while adapting it to canonical
    // v3.1 signed scene placement.
    let source_left = i64::from(placement.x);
    let source_top = i64::from(placement.y);
    let source_right = source_left + source_width as i64;
    let source_bottom = source_top + source_height as i64;
    let visible_left = source_left.max(0);
    let visible_top = source_top.max(0);
    let visible_right = source_right.min(scene_width as i64);
    let visible_bottom = source_bottom.min(scene_height as i64);

    if visible_left >= visible_right || visible_top >= visible_bottom {
        return None;
    }

    Some(ElementClipBounds {
        local_x_start: (visible_left - source_left) as usize,
        local_y_start: (visible_top - source_top) as usize,
        dest_x_start: visible_left as usize,
        dest_y_start: visible_top as usize,
        width: (visible_right - visible_left) as usize,
        height: (visible_bottom - visible_top) as usize,
    })
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_clip_element_bounds.rs</FILE> - <DESC>Clip element-local bounds into scene coordinates</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
