// <FILE>tui-vfx-content/src/cursor/cls_cursor_paint_ops.rs</FILE> - <DESC>Per-frame cursor paint ops</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: paint ops</WCTX>
// <CLOG>Initial impl</CLOG>

/// Primary cursor paint op at the current position.
///
/// `glyph` is the grow-in-aware current glyph (possibly a 1/8th block during
/// grow-in; otherwise the base [`crate::cursor::Cursor::character`]).
/// `alpha` is the effective visibility in `0..=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct PrimaryOp {
    pub position: (u16, u16),
    pub glyph: String,
    pub alpha: f32,
}

/// Trail paint op at a previous cursor position.
///
/// `glyph` is `None` for [`crate::cursor::WakeMode::Tint`] (consumer paints
/// tint on whatever is beneath) and `Some(character)` for
/// [`crate::cursor::WakeMode::Ghost`].
/// `alpha` is the curve-mapped decay in `0..=1`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrailOp {
    pub position: (u16, u16),
    pub glyph: Option<String>,
    pub alpha: f32,
}

/// Per-frame paint output from [`crate::cursor::fnc_render_cursor()`].
///
/// Viewport clipping is the consumer's responsibility — ops may reference
/// cells outside the visible area (see spec E8).
#[derive(Debug, Clone, Default)]
pub struct CursorPaintOps {
    pub primary: Option<PrimaryOp>,
    pub trail: Vec<TrailOp>,
}

// <FILE>tui-vfx-content/src/cursor/cls_cursor_paint_ops.rs</FILE> - <DESC>Per-frame cursor paint ops</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
