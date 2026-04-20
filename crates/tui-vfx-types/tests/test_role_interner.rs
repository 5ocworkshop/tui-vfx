// <FILE>crates/tui-vfx-types/tests/test_role_interner.rs</FILE> - <DESC>Tests for RoleInterner stable IDs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests for stable IDs 0-11 for first-class variants, Custom starts at 12, resolve round-trip.</CLOG>

use tui_vfx_types::{InternedRoleName, RoleId, RoleInterner, RoleTag};

#[test]
fn first_class_variants_have_stable_ids_zero_through_eleven() {
    let mut interner = RoleInterner::new();
    let cases = [
        (RoleTag::Background, 0u16),
        (RoleTag::Text, 1),
        (RoleTag::Title, 2),
        (RoleTag::Caption, 3),
        (RoleTag::Border, 4),
        (RoleTag::Image, 5),
        (RoleTag::Icon, 6),
        (RoleTag::Indicator, 7),
        (RoleTag::Highlight, 8),
        (RoleTag::Shadow, 9),
        (RoleTag::Decoration, 10),
        (RoleTag::Procedural, 11),
    ];
    for (tag, expected) in cases {
        let id = interner.intern(&tag);
        assert_eq!(id.id(), expected, "tag {tag:?} expected id {expected}");
    }
}

#[test]
fn first_class_ids_are_preassigned_even_without_explicit_intern() {
    // A fresh interner must yield stable ids 0..=11 on the very first intern call
    // for each first-class variant, regardless of intern order.
    let mut interner = RoleInterner::new();
    assert_eq!(interner.intern(&RoleTag::Procedural).id(), 11);
    assert_eq!(interner.intern(&RoleTag::Background).id(), 0);
    assert_eq!(interner.intern(&RoleTag::Shadow).id(), 9);
}

#[test]
fn custom_ids_start_at_twelve_and_increment() {
    let mut interner = RoleInterner::new();
    let a = interner.intern(&RoleTag::Custom(InternedRoleName::new("alpha")));
    let b = interner.intern(&RoleTag::Custom(InternedRoleName::new("bravo")));
    let a_again = interner.intern(&RoleTag::Custom(InternedRoleName::new("alpha")));
    assert_eq!(a.id(), 12);
    assert_eq!(b.id(), 13);
    assert_eq!(a_again.id(), 12);
}

#[test]
fn resolve_first_class_returns_original_variant() {
    let mut interner = RoleInterner::new();
    interner.intern(&RoleTag::Border);
    let resolved = interner.resolve(RoleId::new(4));
    assert_eq!(resolved, Some(RoleTag::Border));
}

#[test]
fn resolve_custom_returns_custom_variant_with_original_name() {
    let mut interner = RoleInterner::new();
    let id = interner.intern(&RoleTag::Custom(InternedRoleName::new("logo_silhouette")));
    let resolved = interner.resolve(id);
    match resolved {
        Some(RoleTag::Custom(name)) => assert_eq!(name.as_str(), "logo_silhouette"),
        other => panic!("expected Custom(logo_silhouette), got {other:?}"),
    }
}

#[test]
fn resolve_unknown_id_returns_none() {
    let interner = RoleInterner::new();
    assert_eq!(interner.resolve(RoleId::new(999)), None);
}

// <FILE>crates/tui-vfx-types/tests/test_role_interner.rs</FILE> - <DESC>RoleInterner tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
