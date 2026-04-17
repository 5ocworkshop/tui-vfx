// <FILE>tui-vfx-style/tests/models/test_cls_cursor_shader.rs</FILE> - <DESC>Tests for CursorShader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive T24: tests for CursorShader skeleton — defaults + mode variants</WCTX>
// <CLOG>Initial tests — default shape + mode variants</CLOG>

use tui_vfx_style::models::{CursorShader, CursorShaderMode};

#[test]
fn default_is_empty_and_mode_off() {
    let s = CursorShader::default();
    assert_eq!(s.mode, CursorShaderMode::Off);
    assert!(s.primary.is_none());
    assert!(s.trail.is_empty());
}

#[test]
fn mode_variants_exist() {
    let _ = CursorShaderMode::Off;
    let _ = CursorShaderMode::Tint;
    let _ = CursorShaderMode::Ghost;
}

// <FILE>tui-vfx-style/tests/models/test_cls_cursor_shader.rs</FILE> - <DESC>Tests for CursorShader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
