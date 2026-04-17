// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_wake.rs</FILE> - <DESC>Tests for Wake + WakeMode</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: Wake tests</WCTX>
// <CLOG>Initial tests, enum + struct portions</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::cursor::{Wake, WakeMode};

#[test]
fn wake_mode_default_is_off() {
    assert_eq!(WakeMode::default(), WakeMode::Off);
}

#[test]
fn wake_mode_serde_snake_case() {
    assert_eq!(serde_json::to_string(&WakeMode::Tint).unwrap(), "\"tint\"");
    let back: WakeMode = serde_json::from_str("\"ghost\"").unwrap();
    assert_eq!(back, WakeMode::Ghost);
}

#[test]
fn wake_default_is_noop() {
    let w = Wake::default();
    assert_eq!(w.mode, WakeMode::Off);
    assert!(matches!(w.decay_seconds, SignalOrFloat::Static(0.0)));
    assert_eq!(w.max_cells, 0);
    assert!(matches!(w.curve, SignalOrFloat::Static(1.0)));
}

#[test]
fn wake_noop_equals_default() {
    assert_eq!(Wake::noop(), Wake::default());
}

#[test]
fn wake_serde_roundtrip() {
    let w = Wake {
        mode: WakeMode::Tint,
        decay_seconds: SignalOrFloat::Static(1.5),
        max_cells: 8,
        curve: SignalOrFloat::Static(1.0),
        tint: tui_vfx_style::models::ColorConfig::default(),
    };
    let json = serde_json::to_string(&w).unwrap();
    let back: Wake = serde_json::from_str(&json).unwrap();
    assert_eq!(w, back);
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_wake.rs</FILE> - <DESC>Tests for Wake + WakeMode</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
