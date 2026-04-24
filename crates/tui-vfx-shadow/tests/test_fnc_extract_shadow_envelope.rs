// <FILE>crates/tui-vfx-shadow/tests/test_fnc_extract_shadow_envelope.rs</FILE> - <DESC>Tests for the pure extract_shadow_envelope function — role-filtered cell mask extraction backing the shadow stage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.3.4 — TDD red→green for fnc_extract_shadow_envelope: all-non-empty baseline, role-filtered, empty map, tiny-rect (1x1, 2x1)</WCTX>
// <CLOG>0.1.0: initial TDD red covering (a) source_region=None returns every non-empty source cell; (b) source_region=Some(role) returns only role-matched cells; (c) tiny rectangles (1x1 and 2x1) handled without panic; (d) empty/zero-dim source returns an empty mask; (e) CellMask::bounding_rect matches hand-computed bbox.</CLOG>

//! Tests for the pure `extract_shadow_envelope` free function.
//!
//! The envelope is the set of source cells that SHOULD extrude shadow. A
//! `None` source_region means "every non-empty source cell is a shadow
//! source" (back-compat with today's rect-based extrusion, collapsed to a
//! per-cell mask). A `Some(role)` source_region restricts the mask to cells
//! whose role matches. Tiny rectangles (1x1, 2x1) must not panic.

use tui_vfx_shadow::extract_shadow_envelope;
use tui_vfx_types::{Cell, Grid, OwnedGrid, Rect, RoleMap, RoleTag};

fn filled(width: usize, height: usize, fill_rect: Rect) -> OwnedGrid {
    let mut g = OwnedGrid::new(width, height);
    for y in fill_rect.y..fill_rect.y + fill_rect.height {
        for x in fill_rect.x..fill_rect.x + fill_rect.width {
            g.set(x as usize, y as usize, Cell::new('X'));
        }
    }
    g
}

fn roles_with_rect(width: u16, height: u16, rect: Rect, role: RoleTag) -> RoleMap {
    let mut r = RoleMap::empty(width, height);
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            r.set((x, y), role.clone());
        }
    }
    r
}

#[test]
fn none_filter_masks_all_non_empty_cells() {
    let grid = filled(6, 4, Rect::new(1, 1, 3, 2));
    let roles = RoleMap::all_background(6, 4);
    let mask = extract_shadow_envelope(&grid, &roles, None);
    assert_eq!(mask.width(), 6);
    assert_eq!(mask.height(), 4);
    // Every filled cell should be in the mask.
    for y in 0..4u16 {
        for x in 0..6u16 {
            let filled_here = (1..4).contains(&x) && (1..3).contains(&y);
            assert_eq!(
                mask.get((x, y)),
                filled_here,
                "mask mismatch at ({x}, {y}); expected {filled_here}"
            );
        }
    }
}

#[test]
fn role_filter_masks_only_matching_cells() {
    let grid = filled(6, 4, Rect::new(0, 0, 6, 4));
    // Role out a border row (y=0) vs interior as Text
    let mut roles = RoleMap::new_with_default(6, 4, RoleTag::Text);
    for x in 0..6u16 {
        roles.set((x, 0), RoleTag::Border);
    }
    let mask = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
    for y in 0..4u16 {
        for x in 0..6u16 {
            assert_eq!(
                mask.get((x, y)),
                y == 0,
                "Border filter should mask only y=0 ({x},{y})"
            );
        }
    }
}

#[test]
fn role_filter_with_empty_matches_yields_empty_mask() {
    let grid = filled(4, 3, Rect::new(0, 0, 4, 3));
    let roles = RoleMap::new_with_default(4, 3, RoleTag::Text);
    let mask = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
    for y in 0..3u16 {
        for x in 0..4u16 {
            assert!(!mask.get((x, y)));
        }
    }
    assert!(mask.bounding_rect().is_none());
}

#[test]
fn tiny_rect_1x1_handled_without_panic() {
    let grid = filled(1, 1, Rect::new(0, 0, 1, 1));
    let roles = RoleMap::new_with_default(1, 1, RoleTag::Border);
    let mask = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
    assert_eq!(mask.width(), 1);
    assert_eq!(mask.height(), 1);
    assert!(mask.get((0, 0)));
    assert_eq!(mask.bounding_rect(), Some(Rect::new(0, 0, 1, 1)));
}

#[test]
fn tiny_rect_2x1_handled_without_panic() {
    let grid = filled(2, 1, Rect::new(0, 0, 2, 1));
    let roles = roles_with_rect(2, 1, Rect::new(0, 0, 1, 1), RoleTag::Border);
    let mask = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
    assert_eq!(mask.width(), 2);
    assert_eq!(mask.height(), 1);
    assert!(mask.get((0, 0)));
    assert!(!mask.get((1, 0)));
    assert_eq!(mask.bounding_rect(), Some(Rect::new(0, 0, 1, 1)));
}

#[test]
fn zero_dim_source_returns_empty_mask() {
    let grid = OwnedGrid::new(0, 0);
    let roles = RoleMap::empty(0, 0);
    let mask = extract_shadow_envelope(&grid, &roles, None);
    assert_eq!(mask.width(), 0);
    assert_eq!(mask.height(), 0);
    assert!(mask.bounding_rect().is_none());
}

#[test]
fn bounding_rect_tight() {
    let grid = filled(10, 6, Rect::new(3, 2, 4, 2));
    let roles = roles_with_rect(10, 6, Rect::new(4, 2, 2, 2), RoleTag::Border);
    let mask = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
    assert_eq!(mask.bounding_rect(), Some(Rect::new(4, 2, 2, 2)));
}

#[test]
fn dimension_mismatch_uses_smaller_bounds() {
    // Role map smaller than grid — extract should consult the intersection.
    let grid = filled(6, 4, Rect::new(0, 0, 6, 4));
    let roles = RoleMap::new_with_default(4, 4, RoleTag::Border);
    let mask = extract_shadow_envelope(&grid, &roles, Some(RoleTag::Border));
    // Mask takes the role map's dimensions.
    assert_eq!(mask.width(), 4);
    assert_eq!(mask.height(), 4);
    // Every cell in the 4x4 role map should match (all Border).
    for y in 0..4u16 {
        for x in 0..4u16 {
            assert!(mask.get((x, y)));
        }
    }
}

// <FILE>crates/tui-vfx-shadow/tests/test_fnc_extract_shadow_envelope.rs</FILE> - <DESC>Tests for extract_shadow_envelope</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
