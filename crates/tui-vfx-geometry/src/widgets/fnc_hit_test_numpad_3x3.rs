// <FILE>tui-vfx-geometry/src/widgets/fnc_hit_test_numpad_3x3.rs</FILE>
// <DESC>Hit-testing for a single numpad 3x3 grid</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Click-to-focus UX for configurators</WCTX>
// <CLOG>Added hit testing for one grid</CLOG>

use tui_vfx_types::Rect;

fn digit_for_cell(row: u16, col: u16) -> char {
    match (row, col) {
        (0, 0) => '7',
        (0, 1) => '8',
        (0, 2) => '9',
        (1, 0) => '4',
        (1, 1) => '5',
        (1, 2) => '6',
        (2, 0) => '1',
        (2, 1) => '2',
        _ => '3',
    }
}

pub fn hit_test_numpad_3x3(area: Rect, x: u16, y: u16) -> Option<char> {
    if area.width == 0 || area.height == 0 {
        return None;
    }
    if x < area.x || y < area.y || x >= area.x + area.width || y >= area.y + area.height {
        return None;
    }

    let rel_x = x - area.x;
    let rel_y = y - area.y;

    let col = ((rel_x as u32) * 3 / (area.width as u32)).min(2) as u16;
    let row = ((rel_y as u32) * 3 / (area.height as u32)).min(2) as u16;
    Some(digit_for_cell(row, col))
}

// <FILE>tui-vfx-geometry/src/widgets/fnc_hit_test_numpad_3x3.rs</FILE>
// <DESC>Hit-testing for a single numpad 3x3 grid</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
