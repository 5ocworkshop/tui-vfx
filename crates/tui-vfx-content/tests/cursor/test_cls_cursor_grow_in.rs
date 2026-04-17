// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_grow_in.rs</FILE> - <DESC>Tests for GrowIn + enums</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: GrowIn tests</WCTX>
// <CLOG>Initial tests, enum + struct portions</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::cursor::{GrowDirection, GrowIn, GrowInMode};

#[test]
fn grow_in_mode_default_is_never() {
    assert_eq!(GrowInMode::default(), GrowInMode::Never);
}

#[test]
fn grow_direction_default_is_up() {
    assert_eq!(GrowDirection::default(), GrowDirection::Up);
}

#[test]
fn grow_in_mode_serde_snake_case() {
    let json = serde_json::to_string(&GrowInMode::EveryShow).unwrap();
    assert_eq!(json, "\"every_show\"");
    let back: GrowInMode = serde_json::from_str("\"once\"").unwrap();
    assert_eq!(back, GrowInMode::Once);
}

#[test]
fn grow_direction_serde_snake_case() {
    let json = serde_json::to_string(&GrowDirection::Center).unwrap();
    assert_eq!(json, "\"center\"");
    let back: GrowDirection = serde_json::from_str("\"down\"").unwrap();
    assert_eq!(back, GrowDirection::Down);
}

#[test]
fn grow_in_default_is_noop() {
    let g = GrowIn::default();
    assert_eq!(g.mode, GrowInMode::Never);
    assert_eq!(g.direction, GrowDirection::Up);
    assert!(matches!(g.duration_ms, SignalOrFloat::Static(0.0)));
    assert!(matches!(g.grow_out_ms, SignalOrFloat::Static(0.0)));
    // curve defaults to linear — Static(1.0), sampled as identity.
    assert!(matches!(g.curve, SignalOrFloat::Static(1.0)));
}

#[test]
fn grow_in_noop_equals_default() {
    assert_eq!(GrowIn::noop(), GrowIn::default());
}

#[test]
fn grow_in_serde_omits_defaults_gracefully() {
    // Minimal object should parse to defaults.
    let g: GrowIn = serde_json::from_str("{}").unwrap();
    assert_eq!(g, GrowIn::default());
}

#[test]
fn grow_in_serde_accepts_fields() {
    let json = r#"{
        "mode": "once",
        "direction": "center",
        "duration_ms": 200,
        "grow_out_ms": 100,
        "curve": 1.0
    }"#;
    let g: GrowIn = serde_json::from_str(json).unwrap();
    assert_eq!(g.mode, GrowInMode::Once);
    assert_eq!(g.direction, GrowDirection::Center);
    assert!(matches!(g.duration_ms, SignalOrFloat::Static(200.0)));
    assert!(matches!(g.grow_out_ms, SignalOrFloat::Static(100.0)));
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_grow_in.rs</FILE> - <DESC>Tests for GrowIn + enums</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
