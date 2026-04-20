// <FILE>crates/tui-vfx-types/tests/test_layer_id.rs</FILE> - <DESC>Tests for opaque LayerId newtype</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive tests</WCTX>
// <CLOG>0.1.0: TDD red tests for From<&str>/From<String>, equality across same-string constructions, as_str accessor.</CLOG>

use tui_vfx_types::LayerId;

#[test]
fn from_str_reference_works() {
    let id = LayerId::from("logo_layer");
    assert_eq!(id.as_str(), "logo_layer");
}

#[test]
fn from_string_works() {
    let owned = String::from("card_layer");
    let id = LayerId::from(owned);
    assert_eq!(id.as_str(), "card_layer");
}

#[test]
fn equality_across_same_string_constructions() {
    let a: LayerId = "hero_text".into();
    let b: LayerId = String::from("hero_text").into();
    assert_eq!(a, b);
}

#[test]
fn different_strings_unequal() {
    let a: LayerId = "a".into();
    let b: LayerId = "b".into();
    assert_ne!(a, b);
}

#[test]
fn clone_preserves_content() {
    let id: LayerId = "my_layer".into();
    let cloned = id.clone();
    assert_eq!(id, cloned);
    assert_eq!(cloned.as_str(), "my_layer");
}

#[cfg(feature = "serde")]
#[test]
fn serde_round_trip() {
    let id: LayerId = "splash_logo".into();
    let json = serde_json::to_string(&id).unwrap();
    let back: LayerId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}

// <FILE>crates/tui-vfx-types/tests/test_layer_id.rs</FILE> - <DESC>LayerId tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
