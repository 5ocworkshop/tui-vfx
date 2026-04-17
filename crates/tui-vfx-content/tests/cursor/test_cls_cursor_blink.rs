// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_blink.rs</FILE> - <DESC>Tests for CursorBlink</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: CursorBlink tests</WCTX>
// <CLOG>Initial tests</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::cursor::CursorBlink;

#[test]
fn default_is_no_blink() {
    let b = CursorBlink::default();
    match b.interval_ms {
        SignalOrFloat::Static(v) => assert_eq!(v, 0.0),
        _ => panic!("expected Static(0.0)"),
    }
}

#[test]
fn roundtrips_through_serde_json() {
    let b = CursorBlink {
        interval_ms: SignalOrFloat::Static(500.0),
    };
    let json = serde_json::to_string(&b).unwrap();
    let back: CursorBlink = serde_json::from_str(&json).unwrap();
    assert_eq!(b, back);
}

#[test]
fn deserializes_minimal_object() {
    let b: CursorBlink = serde_json::from_str("{\"interval_ms\": 0}").unwrap();
    assert_eq!(b, CursorBlink::default());
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_blink.rs</FILE> - <DESC>Tests for CursorBlink</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
