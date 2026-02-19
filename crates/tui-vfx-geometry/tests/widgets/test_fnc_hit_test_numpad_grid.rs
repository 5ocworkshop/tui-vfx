// <FILE>tui-vfx-geometry/tests/widgets/test_fnc_hit_test_numpad_grid.rs</FILE>
// <DESC>Tests for numpad grid hit-testing</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Ensure click mapping is stable</WCTX>
// <CLOG>Added hit-test tests</CLOG>

use tui_vfx_geometry::widgets::{TripletGridPart, hit_test_numpad_3x3, hit_test_triplet_grids};
use tui_vfx_types::Rect;

#[test]
fn hit_test_numpad_cells_map_to_expected_digits() {
    let area = Rect::new(0, 0, 30, 9);

    assert_eq!(hit_test_numpad_3x3(area, 1, 1), Some('7'));
    assert_eq!(hit_test_numpad_3x3(area, 15, 1), Some('8'));
    assert_eq!(hit_test_numpad_3x3(area, 28, 1), Some('9'));

    assert_eq!(hit_test_numpad_3x3(area, 1, 4), Some('4'));
    assert_eq!(hit_test_numpad_3x3(area, 15, 4), Some('5'));
    assert_eq!(hit_test_numpad_3x3(area, 28, 4), Some('6'));

    assert_eq!(hit_test_numpad_3x3(area, 1, 8), Some('1'));
    assert_eq!(hit_test_numpad_3x3(area, 15, 8), Some('2'));
    assert_eq!(hit_test_numpad_3x3(area, 28, 8), Some('3'));
}

#[test]
fn hit_test_triplet_grids_returns_part_and_digit() {
    let triplet = Rect::new(0, 0, 30, 9);

    // Left third
    let (part, digit) = hit_test_triplet_grids(triplet, 1, 1).expect("hit");
    assert_eq!(part, TripletGridPart::Start);
    assert_eq!(digit, '7');

    // Middle third
    let (part, digit) = hit_test_triplet_grids(triplet, 15, 4).expect("hit");
    assert_eq!(part, TripletGridPart::Dwell);
    assert_eq!(digit, '5');

    // Right third
    let (part, digit) = hit_test_triplet_grids(triplet, 29, 8).expect("hit");
    assert_eq!(part, TripletGridPart::End);
    assert_eq!(digit, '3');
}

// <FILE>tui-vfx-geometry/tests/widgets/test_fnc_hit_test_numpad_grid.rs</FILE>
// <DESC>Tests for numpad grid hit-testing</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
