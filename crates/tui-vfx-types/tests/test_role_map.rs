// <FILE>crates/tui-vfx-types/tests/test_role_map.rs</FILE> - <DESC>Tests for RoleMap cell storage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests for empty/all_background/new_with_default, get/set bounds handling, row-major iter, serde round-trip.</CLOG>

use tui_vfx_types::{RoleMap, RoleTag};

#[test]
fn empty_produces_background_at_every_cell() {
    let map = RoleMap::empty(4, 3);
    assert_eq!(map.width(), 4);
    assert_eq!(map.height(), 3);
    for y in 0..3u16 {
        for x in 0..4u16 {
            assert_eq!(map.get((x, y)), Some(RoleTag::Background));
        }
    }
}

#[test]
fn all_background_matches_empty() {
    let a = RoleMap::empty(5, 2);
    let b = RoleMap::all_background(5, 2);
    for y in 0..2u16 {
        for x in 0..5u16 {
            assert_eq!(a.get((x, y)), b.get((x, y)));
        }
    }
    assert_eq!(a.width(), b.width());
    assert_eq!(a.height(), b.height());
}

#[test]
fn new_with_default_respects_default() {
    let map = RoleMap::new_with_default(3, 2, RoleTag::Border);
    for y in 0..2u16 {
        for x in 0..3u16 {
            assert_eq!(map.get((x, y)), Some(RoleTag::Border));
        }
    }
}

#[test]
fn set_then_get_round_trip_in_bounds() {
    let mut map = RoleMap::empty(4, 4);
    map.set((2, 1), RoleTag::Text);
    map.set((0, 0), RoleTag::Title);
    map.set((3, 3), RoleTag::Shadow);
    assert_eq!(map.get((2, 1)), Some(RoleTag::Text));
    assert_eq!(map.get((0, 0)), Some(RoleTag::Title));
    assert_eq!(map.get((3, 3)), Some(RoleTag::Shadow));
    assert_eq!(map.get((1, 1)), Some(RoleTag::Background));
}

#[test]
fn out_of_bounds_get_returns_none() {
    let map = RoleMap::empty(3, 3);
    assert_eq!(map.get((3, 0)), None);
    assert_eq!(map.get((0, 3)), None);
    assert_eq!(map.get((100, 100)), None);
}

#[test]
fn out_of_bounds_set_is_no_op_and_does_not_panic() {
    let mut map = RoleMap::empty(2, 2);
    map.set((5, 5), RoleTag::Highlight);
    // All cells still Background
    for y in 0..2u16 {
        for x in 0..2u16 {
            assert_eq!(map.get((x, y)), Some(RoleTag::Background));
        }
    }
}

#[test]
fn iter_is_row_major() {
    let map = RoleMap::empty(3, 2);
    let collected: Vec<(u16, u16, RoleTag)> = map.iter().collect();
    assert_eq!(collected.len(), 6);
    assert_eq!(collected.first().map(|c| (c.0, c.1)), Some((0, 0)));
    assert_eq!(collected.last().map(|c| (c.0, c.1)), Some((2, 1)));
    // Row-major means (0,0), (1,0), (2,0), (0,1), (1,1), (2,1)
    let coords: Vec<(u16, u16)> = collected.iter().map(|c| (c.0, c.1)).collect();
    assert_eq!(
        coords,
        vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)]
    );
}

#[test]
fn iter_returns_custom_tags_after_set() {
    let mut map = RoleMap::empty(2, 2);
    map.set((0, 0), RoleTag::Border);
    map.set((1, 1), RoleTag::Text);
    let tags: Vec<RoleTag> = map.iter().map(|c| c.2).collect();
    assert_eq!(
        tags,
        vec![
            RoleTag::Border,
            RoleTag::Background,
            RoleTag::Background,
            RoleTag::Text,
        ]
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip_preserves_cells() {
    let mut map = RoleMap::empty(3, 2);
    map.set((1, 0), RoleTag::Border);
    map.set((2, 1), RoleTag::Shadow);
    let json = serde_json::to_string(&map).unwrap();
    let back: RoleMap = serde_json::from_str(&json).unwrap();
    assert_eq!(back.width(), 3);
    assert_eq!(back.height(), 2);
    for y in 0..2u16 {
        for x in 0..3u16 {
            assert_eq!(back.get((x, y)), map.get((x, y)));
        }
    }
}

// <FILE>crates/tui-vfx-types/tests/test_role_map.rs</FILE> - <DESC>RoleMap tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
