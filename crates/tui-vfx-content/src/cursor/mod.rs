// <FILE>tui-vfx-content/src/cursor/mod.rs</FILE> - <DESC>General Cursor primitive module</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>feat/cursor-primitive T27: register fnc_apply_ghost_glyphs_to_grid module + re-export so consumers can paint ghost-mode wake glyphs into a source grid before composition</WCTX>
// <CLOG>Register fnc_apply_ghost_glyphs_to_grid (T27)</CLOG>

//! General-purpose cursor primitive with optional grow-in and wake animations.
//!
//! See `docs/superpowers/specs/2026-04-17-cursor-primitive-design.md` for the
//! full design. Short version:
//!
//! - [`Cursor`] is pure config: glyph, blink, grow-in, wake.
//! - [`CursorState`] is runtime bookkeeping the caller owns per cursor.
//! - [`CursorPaintOps`] is what [`fnc_render_cursor`] returns each frame.
//! - [`fnc_advance_cursor`] mutates state given a new position and dt.
//! - All animation fields default to "do nothing" — `Cursor::default()` renders
//!   identical to a plain static block cursor.

pub mod cls_cursor;
pub mod cls_cursor_blink;
pub mod cls_cursor_grow_in;
pub mod cls_cursor_paint_ops;
pub mod cls_cursor_state;
pub mod cls_cursor_wake;
pub mod fnc_advance_cursor;
pub mod fnc_apply_ghost_glyphs_to_grid;
pub mod fnc_cursor_grow_in_glyph;
pub mod fnc_render_cursor;
pub mod fnc_splice_cursor_into_text;
pub mod fnc_typewriter_cursor_position;

pub use cls_cursor::Cursor;
pub use cls_cursor_blink::CursorBlink;
pub use cls_cursor_grow_in::{GrowDirection, GrowIn, GrowInMode};
pub use cls_cursor_paint_ops::{CursorPaintOps, PrimaryOp, TrailOp};
pub use cls_cursor_state::{CursorState, GrowInPhase};
pub use cls_cursor_wake::{Wake, WakeMode};
pub use fnc_advance_cursor::fnc_advance_cursor;
pub use fnc_apply_ghost_glyphs_to_grid::fnc_apply_ghost_glyphs_to_grid;
pub use fnc_cursor_grow_in_glyph::fnc_cursor_grow_in_glyph;
pub use fnc_render_cursor::fnc_render_cursor;
pub use fnc_splice_cursor_into_text::fnc_splice_cursor_into_text;
pub use fnc_typewriter_cursor_position::fnc_typewriter_cursor_position;

// <FILE>tui-vfx-content/src/cursor/mod.rs</FILE> - <DESC>General Cursor primitive module</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
