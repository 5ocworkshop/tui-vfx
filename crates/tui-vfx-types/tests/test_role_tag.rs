// <FILE>crates/tui-vfx-types/tests/test_role_tag.rs</FILE> - <DESC>Tests for RoleTag enum + shorthand parsing + serde round-trip</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: initial TDD red tests for RoleTag first-class variants, Custom, non-exhaustive match, serde round-trip, and string-shorthand parsing.</CLOG>

use tui_vfx_types::{InternedRoleName, RoleTag};

fn first_class_variants() -> Vec<RoleTag> {
    vec![
        RoleTag::Background,
        RoleTag::Text,
        RoleTag::Title,
        RoleTag::Caption,
        RoleTag::Border,
        RoleTag::Image,
        RoleTag::Icon,
        RoleTag::Indicator,
        RoleTag::Highlight,
        RoleTag::Shadow,
        RoleTag::Decoration,
        RoleTag::Procedural,
    ]
}

#[test]
fn all_first_class_variants_parse_round_trip_by_shorthand() {
    // Shorthand schema: lowercase variant name (e.g. "background", "text", "procedural")
    let cases = [
        ("background", RoleTag::Background),
        ("text", RoleTag::Text),
        ("title", RoleTag::Title),
        ("caption", RoleTag::Caption),
        ("border", RoleTag::Border),
        ("image", RoleTag::Image),
        ("icon", RoleTag::Icon),
        ("indicator", RoleTag::Indicator),
        ("highlight", RoleTag::Highlight),
        ("shadow", RoleTag::Shadow),
        ("decoration", RoleTag::Decoration),
        ("procedural", RoleTag::Procedural),
    ];
    for (s, tag) in cases {
        let parsed = RoleTag::from_shorthand(s);
        assert_eq!(parsed, tag, "parsing {s:?}");
        assert_eq!(parsed.shorthand_name(), s, "round-trip name for {s:?}");
    }
}

#[test]
fn custom_shorthand_parses_to_custom_variant() {
    // Schema: any name that does not match a first-class variant becomes Custom.
    // Explicit `custom:foo` prefix is also accepted.
    let a = RoleTag::from_shorthand("custom:logo_silhouette");
    let b = RoleTag::from_shorthand("logo_silhouette");
    assert_eq!(a, b);
    match a {
        RoleTag::Custom(name) => assert_eq!(name.as_str(), "logo_silhouette"),
        other => panic!("expected Custom, got {other:?}"),
    }
}

#[test]
fn custom_shorthand_round_trips_without_prefix() {
    let tag = RoleTag::Custom(InternedRoleName::new("card_inner_glow"));
    let shorthand = tag.shorthand_name();
    let reparsed = RoleTag::from_shorthand(&shorthand);
    assert_eq!(reparsed, tag);
}

#[test]
fn non_exhaustive_match_compiles_via_wildcard() {
    // This test intentionally uses a wildcard arm to confirm #[non_exhaustive]
    // forces foreign crates to keep a default arm.
    let tag = RoleTag::Background;
    let label = match tag {
        RoleTag::Background => "bg",
        RoleTag::Text => "text",
        _ => "other",
    };
    assert_eq!(label, "bg");
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip_every_first_class_variant() {
    for tag in first_class_variants() {
        let json = serde_json::to_string(&tag).unwrap();
        let back: RoleTag = serde_json::from_str(&json).unwrap();
        assert_eq!(back, tag, "round-trip for {tag:?}");
    }
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip_custom_variant() {
    let tag = RoleTag::Custom(InternedRoleName::new("my_custom_role"));
    let json = serde_json::to_string(&tag).unwrap();
    let back: RoleTag = serde_json::from_str(&json).unwrap();
    assert_eq!(back, tag);
}

// <FILE>crates/tui-vfx-types/tests/test_role_tag.rs</FILE> - <DESC>Tests for RoleTag</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
