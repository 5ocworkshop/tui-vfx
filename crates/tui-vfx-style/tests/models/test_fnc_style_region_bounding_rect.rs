// <FILE>crates/tui-vfx-style/tests/models/test_fnc_style_region_bounding_rect.rs</FILE> - <DESC>Tests for bounding_rect extraction; covers bounded (Cell, Cells) and unbounded (All, Role, geometry families) cases</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2.0 — TDD red→green for bounding_rect extraction. `Role(RoleTag)` is unbounded (role membership isn't expressible as a rectangle).</WCTX>
// <CLOG>0.1.0: initial suite. Returns `Rect` (origin + size) instead of the legacy tuple, which composes with other geometry primitives.</CLOG>

use tui_vfx_style::models::fnc_style_region_bounding_rect::bounding_rect;
use tui_vfx_style::models::{BindableU16, CellCoord, ModuloAxis, StyleRegion};
use tui_vfx_types::{Rect, RoleTag};

fn a(w: u16, h: u16) -> Rect {
    Rect::new(0, 0, w, h)
}

#[test]
fn all_is_unbounded() {
    assert!(bounding_rect(&StyleRegion::All, a(10, 5)).is_none());
}

#[test]
fn role_is_unbounded() {
    for tag in RoleTag::FIRST_CLASS.iter() {
        assert!(
            bounding_rect(&StyleRegion::Role(tag.clone()), a(10, 5)).is_none(),
            "Role({:?}) must be unbounded — membership isn't rectangular",
            tag
        );
    }
}

#[test]
fn rows_is_unbounded_in_x() {
    assert!(bounding_rect(&StyleRegion::Rows(vec![0, 2]), a(10, 5)).is_none());
}

#[test]
fn column_is_unbounded_in_y() {
    assert!(bounding_rect(&StyleRegion::Column(3), a(10, 5)).is_none());
}

#[test]
fn modulo_is_unbounded() {
    let r = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: BindableU16::Literal(2),
        remainder: BindableU16::Literal(0),
    };
    assert!(bounding_rect(&r, a(10, 5)).is_none());
}

#[test]
fn cell_with_literal_coords_has_1x1_bounding() {
    let r = StyleRegion::Cell {
        x: BindableU16::Literal(7),
        y: BindableU16::Literal(2),
    };
    assert_eq!(bounding_rect(&r, a(10, 5)), Some(Rect::new(7, 2, 1, 1)));
}

#[test]
fn cell_with_unresolved_binding_has_no_bounding_rect() {
    let r = StyleRegion::Cell {
        x: BindableU16::Binding("hx".to_string()),
        y: BindableU16::Literal(2),
    };
    assert!(bounding_rect(&r, a(10, 5)).is_none());
}

#[test]
fn cells_returns_tight_bounding_box() {
    let r = StyleRegion::Cells(vec![
        CellCoord::new(1, 2),
        CellCoord::new(4, 2),
        CellCoord::new(1, 5),
    ]);
    // x ∈ [1, 4], y ∈ [2, 5] → width 4, height 4
    assert_eq!(bounding_rect(&r, a(10, 10)), Some(Rect::new(1, 2, 4, 4)));
}

#[test]
fn cells_empty_has_no_bounding_rect() {
    let r = StyleRegion::Cells(vec![]);
    assert!(bounding_rect(&r, a(10, 10)).is_none());
}

// <FILE>crates/tui-vfx-style/tests/models/test_fnc_style_region_bounding_rect.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
