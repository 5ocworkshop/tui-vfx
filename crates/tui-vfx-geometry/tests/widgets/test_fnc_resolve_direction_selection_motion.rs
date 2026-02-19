// <FILE>tui-vfx-geometry/tests/widgets/test_fnc_resolve_direction_selection_motion.rs</FILE>
// <DESC>Unit tests for resolve_direction_selection_motion</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Arc-hint semantics for edge digits</WCTX>
// <CLOG>Locked down edge digit arc mapping</CLOG>

use tui_vfx_geometry::types::{PathType, SlideDirection};
use tui_vfx_geometry::widgets::{DirectionNumpadSelection, resolve_direction_selection_motion};

#[test]
fn left_edge_up_hint_maps_to_from_left_plus_negative_arc_bulge() {
    let mut sel = DirectionNumpadSelection::new('4').unwrap();
    sel.cycle();
    sel.cycle();
    let resolved = resolve_direction_selection_motion(sel, 0.35).unwrap();

    assert_eq!(resolved.hint_direction, SlideDirection::FromTop);
    assert_eq!(resolved.base_direction, SlideDirection::FromLeft);
    assert!(matches!(resolved.path, PathType::Arc { bulge } if bulge < 0.0));
}

#[test]
fn top_edge_right_hint_maps_to_from_top_plus_positive_arc_bulge() {
    let mut sel = DirectionNumpadSelection::new('8').unwrap();
    sel.cycle();
    sel.cycle();
    let resolved = resolve_direction_selection_motion(sel, 0.35).unwrap();

    assert_eq!(resolved.hint_direction, SlideDirection::FromRight);
    assert_eq!(resolved.base_direction, SlideDirection::FromTop);
    assert!(matches!(resolved.path, PathType::Arc { bulge } if bulge > 0.0));
}

#[test]
fn corner_digit_treats_non_diagonal_selection_literally() {
    let mut sel = DirectionNumpadSelection::new('7').unwrap();
    sel.cycle();
    let resolved = resolve_direction_selection_motion(sel, 0.35).unwrap();

    assert_eq!(resolved.hint_direction, SlideDirection::FromLeft);
    assert_eq!(resolved.base_direction, SlideDirection::FromLeft);
    assert_eq!(resolved.path, PathType::Linear);
}

// <FILE>tui-vfx-geometry/tests/widgets/test_fnc_resolve_direction_selection_motion.rs</FILE>
// <DESC>Unit tests for resolve_direction_selection_motion</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
