// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_scan.rs</FILE> - <DESC>Tests for CursorScan + ScanMode</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-scan: peer tests for the scan config struct — defaults, noop alias, enum default, serde snake_case roundtrip.</WCTX>
// <CLOG>Initial tests mirroring the Wake/GrowIn shape.</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::cursor::{CursorScan, ScanMode};

#[test]
fn scan_mode_default_is_off() {
    assert_eq!(ScanMode::default(), ScanMode::Off);
}

#[test]
fn scan_mode_serde_snake_case() {
    assert_eq!(serde_json::to_string(&ScanMode::Pulse).unwrap(), "\"pulse\"");
    assert_eq!(
        serde_json::to_string(&ScanMode::HalfBlockBounce).unwrap(),
        "\"half_block_bounce\""
    );
    let back: ScanMode = serde_json::from_str("\"pulse\"").unwrap();
    assert_eq!(back, ScanMode::Pulse);
    let back: ScanMode = serde_json::from_str("\"half_block_bounce\"").unwrap();
    assert_eq!(back, ScanMode::HalfBlockBounce);
}

#[test]
fn cursor_scan_default_is_noop() {
    let s = CursorScan::default();
    assert_eq!(s.mode, ScanMode::Off);
    assert!(matches!(s.period_ms, SignalOrFloat::Static(0.0)));
    assert!(matches!(s.curve, SignalOrFloat::Static(1.0)));
}

#[test]
fn cursor_scan_noop_equals_default() {
    assert_eq!(CursorScan::noop(), CursorScan::default());
}

#[test]
fn cursor_scan_serde_roundtrip() {
    let s = CursorScan {
        mode: ScanMode::HalfBlockBounce,
        period_ms: SignalOrFloat::Static(900.0),
        curve: SignalOrFloat::Static(1.0),
    };
    let json = serde_json::to_string(&s).unwrap();
    let back: CursorScan = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}

#[test]
fn cursor_scan_parses_minimal_json() {
    let parsed: CursorScan =
        serde_json::from_str(r#"{"mode":"pulse","period_ms":1500}"#).unwrap();
    assert_eq!(parsed.mode, ScanMode::Pulse);
    assert!(matches!(parsed.period_ms, SignalOrFloat::Static(1500.0)));
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_scan.rs</FILE> - <DESC>Tests for CursorScan + ScanMode</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
