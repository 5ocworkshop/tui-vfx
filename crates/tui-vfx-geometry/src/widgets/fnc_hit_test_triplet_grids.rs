// <FILE>tui-vfx-geometry/src/widgets/fnc_hit_test_triplet_grids.rs</FILE>
// <DESC>Hit-testing for a triplet of 3x3 grids (start/dwell/end)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Click-to-focus UX for configurators</WCTX>
// <CLOG>Added hit testing for triplet grids</CLOG>

use tui_vfx_types::Rect;

use super::fnc_hit_test_numpad_3x3::hit_test_numpad_3x3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TripletGridPart {
    Start,
    Dwell,
    End,
}

pub fn hit_test_triplet_grids(area: Rect, x: u16, y: u16) -> Option<(TripletGridPart, char)> {
    if area.width == 0 || area.height == 0 {
        return None;
    }

    let w = area.width;
    let start_w = w / 3;
    let dwell_w = w / 3;
    let end_w = w.saturating_sub(start_w + dwell_w);

    let start_rect = Rect::new(area.x, area.y, start_w, area.height);
    let dwell_rect = Rect::new(area.x + start_w, area.y, dwell_w, area.height);
    let end_rect = Rect::new(area.x + start_w + dwell_w, area.y, end_w, area.height);

    if let Some(d) = hit_test_numpad_3x3(start_rect, x, y) {
        return Some((TripletGridPart::Start, d));
    }
    if let Some(d) = hit_test_numpad_3x3(dwell_rect, x, y) {
        return Some((TripletGridPart::Dwell, d));
    }
    if let Some(d) = hit_test_numpad_3x3(end_rect, x, y) {
        return Some((TripletGridPart::End, d));
    }
    None
}

// <FILE>tui-vfx-geometry/src/widgets/fnc_hit_test_triplet_grids.rs</FILE>
// <DESC>Hit-testing for a triplet of 3x3 grids (start/dwell/end)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
