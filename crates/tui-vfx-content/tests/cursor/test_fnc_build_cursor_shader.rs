// <FILE>tui-vfx-content/tests/cursor/test_fnc_build_cursor_shader.rs</FILE> - <DESC>Tests for CursorPaintOps + Wake → CursorShader bridge</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive T28: tests for fnc_build_cursor_shader — verifies the directional bridge that converts a CursorPaintOps snapshot + Wake config into a flat tui-vfx-style CursorShader, preserving mode/tint/primary/trail</WCTX>
// <CLOG>Initial tests — mode mapping (Off/Tint/Ghost), primary forwarding, trail forwarding with glyph distinction</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_content::cursor::{
    CursorPaintOps, PrimaryOp, TrailOp, Wake, WakeMode, fnc_build_cursor_shader,
};
use tui_vfx_style::models::{ColorConfig, CursorShaderMode};

fn wake_mode_with(mode: WakeMode, tint: ColorConfig) -> Wake {
    Wake {
        mode,
        decay_seconds: SignalOrFloat::Static(1.0),
        max_cells: 8,
        gap_cells: 0,
        curve: SignalOrFloat::Static(1.0),
        tint,
    }
}

#[test]
fn off_wake_maps_to_off_mode() {
    let ops = CursorPaintOps::default();
    let wake = wake_mode_with(WakeMode::Off, ColorConfig::Yellow);
    let shader = fnc_build_cursor_shader(&ops, &wake);
    assert_eq!(shader.mode, CursorShaderMode::Off);
    assert_eq!(shader.tint, ColorConfig::Yellow);
    assert!(shader.primary.is_none());
    assert!(shader.trail.is_empty());
}

#[test]
fn tint_wake_maps_to_tint_mode() {
    let wake = wake_mode_with(
        WakeMode::Tint,
        ColorConfig::Rgb {
            r: 255,
            g: 180,
            b: 100,
        },
    );
    let shader = fnc_build_cursor_shader(&CursorPaintOps::default(), &wake);
    assert_eq!(shader.mode, CursorShaderMode::Tint);
}

#[test]
fn ghost_wake_maps_to_ghost_mode() {
    let wake = wake_mode_with(WakeMode::Ghost, ColorConfig::Cyan);
    let shader = fnc_build_cursor_shader(&CursorPaintOps::default(), &wake);
    assert_eq!(shader.mode, CursorShaderMode::Ghost);
}

#[test]
fn forwards_primary_op_fields() {
    let ops = CursorPaintOps {
        primary: Some(PrimaryOp {
            position: (0, 5),
            glyph: "█".into(),
            alpha: 0.5,
        }),
        trail: vec![],
    };
    let wake = wake_mode_with(WakeMode::Tint, ColorConfig::Yellow);
    let shader = fnc_build_cursor_shader(&ops, &wake);
    let primary = shader.primary.expect("primary should be present");
    assert_eq!(primary.position, (0, 5));
    assert!((primary.alpha - 0.5).abs() < 1e-6);
}

#[test]
fn forwards_trail_entries_preserving_glyph_none() {
    let ops = CursorPaintOps {
        primary: None,
        trail: vec![
            TrailOp {
                position: (0, 4),
                glyph: None,
                alpha: 0.3,
            },
            TrailOp {
                position: (0, 3),
                glyph: Some("▓".into()),
                alpha: 0.6,
            },
        ],
    };
    let wake = wake_mode_with(WakeMode::Ghost, ColorConfig::Yellow);
    let shader = fnc_build_cursor_shader(&ops, &wake);
    assert_eq!(shader.trail.len(), 2);
    assert_eq!(shader.trail[0].position, (0, 4));
    assert!(shader.trail[0].glyph.is_none(), "Tint entries carry no glyph");
    assert_eq!(shader.trail[1].position, (0, 3));
    assert_eq!(shader.trail[1].glyph.as_deref(), Some("▓"));
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_build_cursor_shader.rs</FILE> - <DESC>Tests for CursorPaintOps + Wake → CursorShader bridge</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
