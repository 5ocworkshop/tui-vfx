// <FILE>crates/tui-vfx-types/tests/test_scene_metadata.rs</FILE> - <DESC>Tests for SceneMetadata</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests for Default, non-exhaustive, serde round-trip.</CLOG>

use tui_vfx_types::{RecipeId, SceneMetadata};

#[test]
fn default_has_no_recipe_id() {
    let md = SceneMetadata::default();
    assert!(md.recipe_id.is_none());
    assert!(md.composer_version.is_none());
    assert!(md.produced_at.is_none());
    assert_eq!(md.layer_count, 0);
}

#[test]
fn non_exhaustive_construction_compiles_with_struct_update() {
    // #[non_exhaustive] on a struct prevents brace-init from foreign crates.
    // We construct via Default + field assignment instead.
    let mut md = SceneMetadata::default();
    md.recipe_id = Some(RecipeId::from("splash"));
    md.layer_count = 2;
    assert_eq!(md.layer_count, 2);
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip_full_metadata() {
    let mut md = SceneMetadata::default();
    md.recipe_id = Some(RecipeId::from("splash"));
    md.composer_version = Some("0.6.0".into());
    md.produced_at = Some(1234);
    md.layer_count = 4;
    let json = serde_json::to_string(&md).unwrap();
    let back: SceneMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(back.recipe_id, md.recipe_id);
    assert_eq!(back.composer_version, md.composer_version);
    assert_eq!(back.produced_at, md.produced_at);
    assert_eq!(back.layer_count, md.layer_count);
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip_default_metadata() {
    let md = SceneMetadata::default();
    let json = serde_json::to_string(&md).unwrap();
    let back: SceneMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(back.recipe_id, md.recipe_id);
    assert_eq!(back.layer_count, md.layer_count);
}

// <FILE>crates/tui-vfx-types/tests/test_scene_metadata.rs</FILE> - <DESC>SceneMetadata tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
