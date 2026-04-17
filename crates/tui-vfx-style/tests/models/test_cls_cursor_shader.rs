// <FILE>tui-vfx-style/tests/models/test_cls_cursor_shader.rs</FILE> - <DESC>Tests for CursorShader</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>feat/cursor-primitive T25: add StyleShader impl tests — primary-cell alpha modulation covered by three cases (mid alpha, full alpha, non-primary cell untouched)</WCTX>
// <CLOG>Add T25 tests: primary_cell_applies_alpha_to_foreground, primary_cell_at_alpha_one_preserves_fg, non_primary_cell_untouched_when_no_trail</CLOG>

use tui_vfx_style::models::{CursorShader, CursorShaderMode, CursorShaderPrimary};
use tui_vfx_style::traits::{ShaderContext, StyleShader};
use tui_vfx_types::{Color, Style};

fn ctx(x: u16, y: u16) -> ShaderContext {
    let mut c = ShaderContext::default();
    c.local_x = x;
    c.local_y = y;
    c.width = 80;
    c.height = 1;
    c
}

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

// ---------------- T25: primary-cell alpha modulation ----------------

#[test]
fn primary_cell_applies_alpha_to_foreground() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Tint; // non-Off to exercise painting
    shader.primary = Some(CursorShaderPrimary {
        position: (0, 5),
        alpha: 0.5,
    });
    let base = Style::default().with_fg(Color::rgb(200, 200, 200));
    let out = shader.style_at(&ctx(5, 0), base);
    // At alpha=0.5 the primary cell gets fg.a ~ 128 (0.5 * 255 rounded).
    assert!(out.fg.a < 255);
    assert!(out.fg.a >= 100 && out.fg.a <= 140);
}

#[test]
fn primary_cell_at_alpha_one_preserves_fg() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Tint;
    shader.primary = Some(CursorShaderPrimary {
        position: (0, 5),
        alpha: 1.0,
    });
    let base = Style::default().with_fg(Color::rgb(200, 200, 200));
    let out = shader.style_at(&ctx(5, 0), base);
    assert_eq!(out.fg.a, 255);
}

#[test]
fn non_primary_cell_untouched_when_no_trail() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Tint;
    shader.primary = Some(CursorShaderPrimary {
        position: (0, 5),
        alpha: 0.5,
    });
    let base = Style::default().with_fg(Color::rgb(200, 200, 200));
    let out = shader.style_at(&ctx(0, 0), base); // different cell
    assert_eq!(out, base);
}

#[test]
fn off_mode_short_circuits_to_base() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Off;
    shader.primary = Some(CursorShaderPrimary {
        position: (0, 5),
        alpha: 0.5,
    });
    let base = Style::default().with_fg(Color::rgb(200, 200, 200));
    let out = shader.style_at(&ctx(5, 0), base);
    assert_eq!(out, base);
}

// <FILE>tui-vfx-style/tests/models/test_cls_cursor_shader.rs</FILE> - <DESC>Tests for CursorShader</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
