// <FILE>crates/tui-vfx-compositor/tests/cursor_integration.rs</FILE> - <DESC>End-to-end cursor rendering integration</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: Typewriter::transform_with_cursor signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>0.2.0: migrate ctx() helper to return TransformContext<'static> so the cursor-integration test compiles against the new transform_with_cursor signature.</CLOG>

//! End-to-end cursor integration test.
//!
//! Exercises the full flow the consumer bridging pattern documents:
//!
//! 1. `fnc_advance_cursor` (called inside `Typewriter::transform_with_cursor`)
//!    mutates `CursorState` given a new grapheme position.
//! 2. `Typewriter::transform_with_cursor` returns the revealed text with the
//!    primary cursor glyph spliced in, plus a `CursorPaintOps` snapshot.
//! 3. `fnc_build_cursor_shader` converts the ops + wake config into a
//!    `CursorShader` ready to install on a `ShaderLayerSpec`.
//! 4. `render_pipeline_with_spec` dispatches through
//!    `SpatialShaderType::Cursor` and paints the trail tint onto the
//!    destination grid.

#[path = "pipeline/test_helpers.rs"]
mod test_helpers;
use test_helpers::render_pipeline_with_spec_legacy;

use mixed_signals::prelude::SignalContext;
use std::sync::OnceLock;
use tui_vfx_compositor::pipeline::{CompositionSpec, ShaderLayerSpec};
use tui_vfx_content::cursor::{Cursor, CursorState, fnc_build_cursor_shader};
use tui_vfx_content::traits::TransformContext;
use tui_vfx_content::transformers::Typewriter;
use tui_vfx_style::models::SpatialShaderType;
use tui_vfx_style::traits::ShaderRuntimeParams;
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid};

static CTX_PARTS: OnceLock<(SignalContext, ShaderRuntimeParams)> = OnceLock::new();

fn ctx() -> TransformContext<'static> {
    let p = CTX_PARTS.get_or_init(|| (SignalContext::new(0, 0), ShaderRuntimeParams::new()));
    TransformContext::new(&p.0, &p.1)
}

#[test]
fn cursor_primary_is_spliced_into_typewriter_output() {
    let mut state = CursorState::new();
    let cursor = Cursor::block().with_wake_tint(1.0, 8);
    let tw = Typewriter::default();
    let (text, _ops) =
        tw.transform_with_cursor("hello", 0.6, &ctx(), &cursor, &mut state, 0.0, 0.016);
    // 5 graphemes × 0.6 = 3 revealed → cursor at idx 3 with block glyph '█'.
    assert!(
        text.contains('█'),
        "expected block cursor glyph in {text:?}"
    );
    assert!(
        text.starts_with("hel"),
        "expected revealed prefix 'hel' in {text:?}"
    );
}

#[test]
fn cursor_shader_paints_wake_tint_on_dest_grid() {
    let mut state = CursorState::new();
    let cursor = Cursor::block().with_wake_tint(2.0, 8);
    let tw = Typewriter::default();

    // Two advances at different progress values so the previous position
    // becomes a wake-trail entry after the second frame.
    let _ = tw.transform_with_cursor("abcdef", 0.2, &ctx(), &cursor, &mut state, 0.0, 0.016);
    let (text, ops) =
        tw.transform_with_cursor("abcdef", 0.4, &ctx(), &cursor, &mut state, 0.05, 0.05);

    assert!(
        !ops.trail.is_empty(),
        "second advance should have produced at least one trail entry"
    );

    // Paint the revealed text into a source grid. Width is large enough to
    // hold the full reveal + cursor glyph.
    let mut source = OwnedGrid::new(20, 1);
    for (i, ch) in text.chars().enumerate() {
        source.set(
            i,
            0,
            Cell {
                ch,
                fg: Color::rgb(200, 200, 200),
                ..Cell::default()
            },
        );
    }

    // Install the cursor shader on a spec with an otherwise-default
    // composition (identity render).
    let shader = fnc_build_cursor_shader(&ops, &cursor.wake);
    let spec = CompositionSpec {
        shader_layers: vec![ShaderLayerSpec {
            shader: SpatialShaderType::Cursor(shader),
            region: Default::default(),
        }],
        ..Default::default()
    };

    let mut dest = OwnedGrid::new(20, 1);
    render_pipeline_with_spec_legacy(&source, &mut dest, 20, 1, 0, 0, &spec, None);

    // The trail position (row, col) should have had its fg shifted away from
    // the flat grey the source grid holds. That proves the shader dispatched
    // and the Tint blend touched the cell.
    let (row, col) = ops.trail[0].position;
    let tinted = dest
        .get(col as usize, row as usize)
        .expect("trail cell should be inside dest grid");
    assert_ne!(
        tinted.fg,
        Color::rgb(200, 200, 200),
        "wake trail cell fg should be tinted away from source grey (got {:?})",
        tinted.fg
    );
}

// <FILE>crates/tui-vfx-compositor/tests/cursor_integration.rs</FILE> - <DESC>End-to-end cursor rendering integration</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
