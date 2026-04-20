// <FILE>crates/tui-vfx-style/tests/models/test_style_region_legacy_parse_round_trip.rs</FILE> - <DESC>Tests that legacy bare strings ("BorderOnly"/"TextOnly"/"BackgroundOnly") parse into canonical Role(...) form and round-trip to canonical on re-serialize</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2.1 — legacy serde back-compat. The 35 recipe JSON fixtures in tui-vfx-recipes still write the legacy bare strings; this suite guarantees they parse.</WCTX>
// <CLOG>0.1.0: initial suite. Each legacy bare string parses to Role(...) canonical; serialize then re-parse yields canonical (one-way convergence); the internally-tagged Role form also parses.</CLOG>

use tui_vfx_style::models::StyleRegion;
use tui_vfx_types::RoleTag;

#[test]
fn legacy_border_only_parses_to_role_border() {
    let parsed: StyleRegion = serde_json::from_str("\"BorderOnly\"").unwrap();
    assert_eq!(parsed, StyleRegion::Role(RoleTag::Border));
}

#[test]
fn legacy_text_only_parses_to_role_text() {
    let parsed: StyleRegion = serde_json::from_str("\"TextOnly\"").unwrap();
    assert_eq!(parsed, StyleRegion::Role(RoleTag::Text));
}

#[test]
fn legacy_background_only_parses_to_role_background() {
    let parsed: StyleRegion = serde_json::from_str("\"BackgroundOnly\"").unwrap();
    assert_eq!(parsed, StyleRegion::Role(RoleTag::Background));
}

#[test]
fn legacy_variants_converge_to_canonical_on_reserialize() {
    // Step 1: parse legacy string → canonical Role(...)
    let parsed: StyleRegion = serde_json::from_str("\"BorderOnly\"").unwrap();
    // Step 2: serialize canonical → canonical JSON (never the legacy string)
    let reserialized = serde_json::to_string(&parsed).unwrap();
    assert!(
        !reserialized.contains("BorderOnly"),
        "canonical serialization must not emit the legacy bare string, got: {reserialized}"
    );
    // Step 3: canonical JSON re-parses to the same canonical Rust value
    let roundtripped: StyleRegion = serde_json::from_str(&reserialized).unwrap();
    assert_eq!(roundtripped, StyleRegion::Role(RoleTag::Border));
}

#[test]
fn canonical_role_form_parses() {
    // The canonical form: {"Role": <role-shorthand-json>}.
    // RoleTag::Border's serde shape is owned by tui-vfx-types; go through
    // a round-trip to assert the Role wrapper is parseable rather than
    // pinning inner JSON.
    let canonical = serde_json::to_string(&StyleRegion::Role(RoleTag::Border)).unwrap();
    let parsed: StyleRegion = serde_json::from_str(&canonical).unwrap();
    assert_eq!(parsed, StyleRegion::Role(RoleTag::Border));
}

#[test]
fn non_role_variants_unaffected() {
    // Assert geometry variants still round-trip identically.
    for json in [
        "\"All\"",
        r#"{"Rows":[0,2,4]}"#,
        r#"{"Column":3}"#,
        r#"{"Modulo":{"axis":"Horizontal","modulus":2,"remainder":0}}"#,
    ] {
        let parsed: StyleRegion = serde_json::from_str(json).unwrap();
        let reserialized = serde_json::to_string(&parsed).unwrap();
        let reparsed: StyleRegion = serde_json::from_str(&reserialized).unwrap();
        assert_eq!(parsed, reparsed, "round-trip failed for: {json}");
    }
}

// <FILE>crates/tui-vfx-style/tests/models/test_style_region_legacy_parse_round_trip.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
