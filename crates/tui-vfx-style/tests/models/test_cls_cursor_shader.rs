// <FILE>tui-vfx-style/tests/models/test_cls_cursor_shader.rs</FILE> - <DESC>Tests for CursorShader</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>feat/cursor-primitive T26: add Tint-mode trail blend tests — wake trail cells alpha-blend the tint color onto the base fg, zero-alpha leaves base, non-trail cells still untouched</WCTX>
// <CLOG>Add T26 tests: tint_trail_blends_tint_color_onto_cell, tint_trail_cell_with_zero_alpha_leaves_base, trail_ignored_in_off_mode</CLOG>

use tui_vfx_style::models::{
    ColorConfig, CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail,
};
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

// ---------------- T26: Tint-mode trail blending ----------------

#[test]
fn tint_trail_blends_tint_color_onto_cell() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Tint;
    shader.tint = ColorConfig::Rgb {
        r: 255,
        g: 180,
        b: 100,
    };
    shader.trail = vec![CursorShaderTrail {
        position: (0, 3),
        alpha: 0.5,
        glyph: None,
    }];
    let base = Style::default().with_fg(Color::rgb(50, 50, 50));
    let out = shader.style_at(&ctx(3, 0), base);
    // Blended fg should sit roughly halfway between base (50) and tint (255, 180, 100).
    // (50 + (255-50)*0.5) ≈ 152, (50 + (180-50)*0.5) ≈ 115, (50 + (100-50)*0.5) ≈ 75.
    assert!(
        out.fg.r > 100 && out.fg.r < 200,
        "r channel should blend toward tint: got {}",
        out.fg.r
    );
    assert!(
        out.fg.g > 80 && out.fg.g < 160,
        "g channel should blend toward tint: got {}",
        out.fg.g
    );
    assert_ne!(out.fg, base.fg, "trail cell should be tinted");
}

#[test]
fn tint_trail_cell_with_zero_alpha_leaves_base() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Tint;
    shader.tint = ColorConfig::Rgb {
        r: 255,
        g: 180,
        b: 100,
    };
    shader.trail = vec![CursorShaderTrail {
        position: (0, 3),
        alpha: 0.0,
        glyph: None,
    }];
    let base = Style::default().with_fg(Color::rgb(50, 50, 50));
    let out = shader.style_at(&ctx(3, 0), base);
    assert_eq!(out, base);
}

#[test]
fn trail_cell_outside_trail_list_untouched() {
    let mut shader = CursorShader::default();
    shader.mode = CursorShaderMode::Tint;
    shader.tint = ColorConfig::Rgb {
        r: 255,
        g: 180,
        b: 100,
    };
    shader.trail = vec![CursorShaderTrail {
        position: (0, 3),
        alpha: 0.8,
        glyph: None,
    }];
    let base = Style::default().with_fg(Color::rgb(50, 50, 50));
    let out = shader.style_at(&ctx(7, 0), base); // not in trail
    assert_eq!(out, base);
}

// <FILE>tui-vfx-style/tests/models/test_cls_cursor_shader.rs</FILE> - <DESC>Tests for CursorShader</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
