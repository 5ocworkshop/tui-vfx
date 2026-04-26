// <FILE>crates/tui-vfx-style/tests/models/test_fnc_style_region_should_style.rs</FILE> - <DESC>Tests for the free should_style function covering every StyleRegion variant including role-aware matching</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2.0 — TDD red→green for should_style extraction. Cover Role(RoleTag::…) for every first-class variant + 1 Custom; cover geometry variants preserving legacy semantics.</WCTX>
// <CLOG>0.1.0: initial suite. Role variants iterate RoleTag::FIRST_CLASS + a Custom exemplar. Geometry variants exercise one canonical case each plus safety guards (Modulo zero, inverted RowRange).</CLOG>

use tui_vfx_style::models::fnc_style_region_should_style::should_style;
use tui_vfx_style::models::{BindableU16, CellCoord, ModuloAxis, StyleRegion};
use tui_vfx_types::{InternedRoleName, Rect, RoleTag};

fn area(w: u16, h: u16) -> Rect {
    Rect::new(0, 0, w, h)
}

#[test]
fn all_matches_every_cell_regardless_of_role() {
    let region = StyleRegion::All;
    let a = area(10, 5);
    assert!(should_style(&region, 0, 0, None, a));
    assert!(should_style(&region, 9, 4, Some(RoleTag::Border), a));
    assert!(should_style(&region, 5, 2, Some(RoleTag::Background), a));
}

#[test]
fn role_matches_each_first_class_variant() {
    let a = area(4, 4);
    for tag in RoleTag::FIRST_CLASS.iter() {
        let region = StyleRegion::Role(tag.clone());
        assert!(
            should_style(&region, 0, 0, Some(tag.clone()), a),
            "Role({:?}) should match role={:?}",
            tag,
            tag
        );
    }
}

#[test]
fn role_no_match_when_roles_differ() {
    let region = StyleRegion::Role(RoleTag::Border);
    let a = area(4, 4);
    assert!(!should_style(&region, 0, 0, Some(RoleTag::Text), a));
    assert!(!should_style(&region, 0, 0, Some(RoleTag::Background), a));
    assert!(!should_style(&region, 0, 0, None, a));
}

#[test]
fn role_custom_variant_matches_same_interned_name() {
    let custom_name = InternedRoleName::new("logo_silhouette");
    let region = StyleRegion::Role(RoleTag::Custom(custom_name.clone()));
    let cell_role = Some(RoleTag::Custom(custom_name));
    let a = area(4, 4);
    assert!(should_style(&region, 2, 1, cell_role, a));
}

#[test]
fn role_custom_variant_no_match_for_different_name() {
    let region = StyleRegion::Role(RoleTag::Custom(InternedRoleName::new("foo")));
    let other = Some(RoleTag::Custom(InternedRoleName::new("bar")));
    let a = area(4, 4);
    assert!(!should_style(&region, 0, 0, other, a));
}

#[test]
fn rows_matches_listed_rows() {
    let region = StyleRegion::Rows(vec![0, 2, 4]);
    let a = area(10, 5);
    assert!(should_style(&region, 3, 0, None, a));
    assert!(should_style(&region, 9, 2, None, a));
    assert!(!should_style(&region, 3, 1, None, a));
    assert!(!should_style(&region, 3, 3, None, a));
}

#[test]
fn row_range_matches_half_open_interval() {
    let region = StyleRegion::RowRange { start: BindableU16::Literal(1), end: BindableU16::Literal(4) };
    let a = area(10, 5);
    assert!(!should_style(&region, 0, 0, None, a));
    assert!(should_style(&region, 0, 1, None, a));
    assert!(should_style(&region, 0, 3, None, a));
    assert!(!should_style(&region, 0, 4, None, a));
}

#[test]
fn column_matches_single_x() {
    let region = StyleRegion::Column(3);
    let a = area(10, 5);
    assert!(should_style(&region, 3, 0, None, a));
    assert!(!should_style(&region, 4, 0, None, a));
}

#[test]
fn column_range_matches_half_open_interval() {
    let region = StyleRegion::ColumnRange { start: BindableU16::Literal(2), end: BindableU16::Literal(5) };
    let a = area(10, 5);
    assert!(!should_style(&region, 1, 0, None, a));
    assert!(should_style(&region, 2, 0, None, a));
    assert!(should_style(&region, 4, 0, None, a));
    assert!(!should_style(&region, 5, 0, None, a));
}

#[test]
fn cells_matches_any_listed_coord() {
    let region = StyleRegion::Cells(vec![CellCoord::new(1, 1), CellCoord::new(5, 2)]);
    let a = area(10, 5);
    assert!(should_style(&region, 1, 1, None, a));
    assert!(should_style(&region, 5, 2, None, a));
    assert!(!should_style(&region, 2, 2, None, a));
}

#[test]
fn cell_with_literal_coords_matches_exact() {
    let region = StyleRegion::Cell {
        x: BindableU16::Literal(2),
        y: BindableU16::Literal(3),
    };
    let a = area(10, 5);
    assert!(should_style(&region, 2, 3, None, a));
    assert!(!should_style(&region, 2, 2, None, a));
}

#[test]
fn cell_with_unresolved_binding_never_matches() {
    // Hot-loop contract: callers must `resolved()` before evaluation.
    // Unresolved bindings silently match nothing.
    let region = StyleRegion::Cell {
        x: BindableU16::Binding("x".to_string()),
        y: BindableU16::Literal(3),
    };
    let a = area(10, 5);
    assert!(!should_style(&region, 0, 3, None, a));
}

#[test]
fn modulo_horizontal_matches_chosen_rows() {
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: BindableU16::Literal(2),
        remainder: BindableU16::Literal(0),
    };
    let a = area(6, 6);
    assert!(should_style(&region, 3, 0, None, a));
    assert!(should_style(&region, 3, 2, None, a));
    assert!(!should_style(&region, 3, 1, None, a));
}

#[test]
fn modulo_zero_modulus_matches_nothing() {
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: BindableU16::Literal(0),
        remainder: BindableU16::Literal(0),
    };
    assert!(!should_style(&region, 0, 0, None, area(4, 4)));
}

#[test]
fn modulo_remainder_exceeds_modulus_matches_nothing() {
    let region = StyleRegion::Modulo {
        axis: ModuloAxis::Horizontal,
        modulus: BindableU16::Literal(2),
        remainder: BindableU16::Literal(5),
    };
    assert!(!should_style(&region, 0, 0, None, area(4, 4)));
}

// <FILE>crates/tui-vfx-style/tests/models/test_fnc_style_region_should_style.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
