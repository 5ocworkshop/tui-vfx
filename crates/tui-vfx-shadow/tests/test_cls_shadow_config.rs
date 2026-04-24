// <FILE>crates/tui-vfx-shadow/tests/test_cls_shadow_config.rs</FILE> - <DESC>Tests for the new source_region field on ShadowConfig (role-aware shadow extraction)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.3.3 — TDD red→green for ShadowConfig.source_region and its with_source_region builder + serde round-trip</WCTX>
// <CLOG>0.1.0: initial TDD red for ShadowConfig.source_region (default None; with_source_region sets Some; accessor returns current value; serde JSON round-trips both None and Some(RoleTag::…) shapes including Custom)</CLOG>

//! Tests for the role-aware `ShadowConfig.source_region` field introduced in
//! Sub-plan A Phase A.3.3. Verifies that:
//!
//! 1. The default value is `None` (preserving today's rect-based extrusion).
//! 2. The `with_source_region(role)` builder sets `Some(role)`.
//! 3. The `source_region()` accessor returns the current value.
//! 4. Serde round-trip preserves the field across JSON for every first-class
//!    variant plus at least one Custom role.

use tui_vfx_shadow::ShadowConfig;
use tui_vfx_types::{Color, InternedRoleName, RoleTag};

#[test]
fn default_source_region_is_none() {
    let config = ShadowConfig::default();
    assert_eq!(config.source_region, None);
    assert_eq!(config.source_region(), None);
}

#[test]
fn new_source_region_is_none() {
    let config = ShadowConfig::new(Color::BLACK.with_alpha(128));
    assert_eq!(config.source_region, None);
}

#[test]
fn with_source_region_sets_role() {
    let config =
        ShadowConfig::new(Color::BLACK.with_alpha(128)).with_source_region(RoleTag::Border);
    assert_eq!(config.source_region, Some(RoleTag::Border));
    assert_eq!(config.source_region(), Some(RoleTag::Border));
}

#[test]
fn with_source_region_overwrites() {
    let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
        .with_source_region(RoleTag::Border)
        .with_source_region(RoleTag::Text);
    assert_eq!(config.source_region(), Some(RoleTag::Text));
}

#[test]
fn serde_round_trip_preserves_none() {
    let config = ShadowConfig::new(Color::BLACK.with_alpha(180));
    let json = serde_json::to_string(&config).expect("serialize");
    let restored: ShadowConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(restored.source_region, None);
}

#[test]
fn serde_round_trip_preserves_first_class_roles() {
    let roles = [
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
    ];
    for role in roles {
        let config =
            ShadowConfig::new(Color::BLACK.with_alpha(180)).with_source_region(role.clone());
        let json = serde_json::to_string(&config).expect("serialize");
        let restored: ShadowConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(
            restored.source_region,
            Some(role.clone()),
            "serde round-trip lost source_region for {:?}",
            role
        );
    }
}

#[test]
fn serde_round_trip_preserves_custom_role() {
    let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
        .with_source_region(RoleTag::Custom(InternedRoleName::new("card-surface")));
    let json = serde_json::to_string(&config).expect("serialize");
    let restored: ShadowConfig = serde_json::from_str(&json).expect("deserialize");
    match restored.source_region {
        Some(RoleTag::Custom(name)) => assert_eq!(name.as_str(), "card-surface"),
        other => panic!("unexpected source_region: {:?}", other),
    }
}

#[test]
fn source_region_does_not_affect_existing_fields() {
    // Sanity: adding the new field must not disturb other default values.
    let config =
        ShadowConfig::new(Color::BLACK.with_alpha(200)).with_source_region(RoleTag::Border);
    assert_eq!(config.offset_x, 1);
    assert_eq!(config.offset_y, 1);
    assert_eq!(config.inset_x, None);
    assert_eq!(config.inset_y, None);
    assert!(config.soft_edges);
}

// <FILE>crates/tui-vfx-shadow/tests/test_cls_shadow_config.rs</FILE> - <DESC>Tests for source_region field on ShadowConfig</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
