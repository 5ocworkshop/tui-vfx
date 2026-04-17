// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_paint_ops.rs</FILE> - <DESC>Tests for CursorPaintOps</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: CursorPaintOps tests</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_content::cursor::{CursorPaintOps, PrimaryOp, TrailOp};

#[test]
fn empty_has_no_primary_and_no_trail() {
    let ops = CursorPaintOps::default();
    assert!(ops.primary.is_none());
    assert!(ops.trail.is_empty());
}

#[test]
fn builds_primary_and_trail_ops() {
    let ops = CursorPaintOps {
        primary: Some(PrimaryOp { position: (2, 5), glyph: "▄".into(), alpha: 0.5 }),
        trail: vec![
            TrailOp { position: (2, 4), glyph: None, alpha: 0.3 },
            TrailOp { position: (2, 3), glyph: Some("█".into()), alpha: 0.1 },
        ],
    };
    assert_eq!(ops.primary.as_ref().unwrap().position, (2, 5));
    assert_eq!(ops.trail.len(), 2);
    assert!(ops.trail[0].glyph.is_none()); // Tint mode
    assert!(ops.trail[1].glyph.is_some()); // Ghost mode
}

// <FILE>tui-vfx-content/tests/cursor/test_cls_cursor_paint_ops.rs</FILE> - <DESC>Tests for CursorPaintOps</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
