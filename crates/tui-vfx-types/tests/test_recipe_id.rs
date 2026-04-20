// <FILE>crates/tui-vfx-types/tests/test_recipe_id.rs</FILE> - <DESC>Tests for opaque RecipeId newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests mirroring test_layer_id for RecipeId.</CLOG>

use tui_vfx_types::RecipeId;

#[test]
fn from_str_reference_works() {
    let id = RecipeId::from("splash.v2");
    assert_eq!(id.as_str(), "splash.v2");
}

#[test]
fn from_string_works() {
    let owned = String::from("toast.slide_in");
    let id = RecipeId::from(owned);
    assert_eq!(id.as_str(), "toast.slide_in");
}

#[test]
fn equality_across_same_string_constructions() {
    let a: RecipeId = "hero".into();
    let b: RecipeId = String::from("hero").into();
    assert_eq!(a, b);
}

#[test]
fn different_strings_unequal() {
    let a: RecipeId = "x".into();
    let b: RecipeId = "y".into();
    assert_ne!(a, b);
}

#[test]
fn clone_preserves_content() {
    let id: RecipeId = "ambient".into();
    let cloned = id.clone();
    assert_eq!(id, cloned);
    assert_eq!(cloned.as_str(), "ambient");
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip() {
    let id: RecipeId = "my_recipe".into();
    let json = serde_json::to_string(&id).unwrap();
    let back: RecipeId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}

// <FILE>crates/tui-vfx-types/tests/test_recipe_id.rs</FILE> - <DESC>RecipeId tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
