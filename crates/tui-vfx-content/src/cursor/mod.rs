// <FILE>tui-vfx-content/src/cursor/mod.rs</FILE> - <DESC>General Cursor primitive module</DESC>
// <VERS>VERSION: 0.5.1</VERS>
// <WCTX>feat/cursor-primitive T31: disambiguate rustdoc links — the fnc_advance_cursor and fnc_render_cursor submodules have the same name as the re-exported functions they carry, which makes bare intradoc links ambiguous. Append `()` so rustdoc resolves to the function variant. Also fix the stale END OF VERSION footer left over from T28's 0.4.0 → 0.5.0 bump.</WCTX>
// <CLOG>PATCH: append `()` to fnc_render_cursor/fnc_advance_cursor intradoc links in this module's doc comment; sync footer to 0.5.1</CLOG>

//! General-purpose cursor primitive with optional grow-in and wake animations.
//!
//! See `docs/superpowers/specs/2026-04-17-cursor-primitive-design.md` for the
//! full design. Short version:
//!
//! - [`Cursor`] is pure config: glyph, blink, grow-in, wake.
//! - [`CursorState`] is runtime bookkeeping the caller owns per cursor.
//! - [`CursorPaintOps`] is what [`fnc_render_cursor()`] returns each frame.
//! - [`fnc_advance_cursor()`] mutates state given a new position and dt.
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
pub mod fnc_build_cursor_shader;
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
pub use fnc_build_cursor_shader::fnc_build_cursor_shader;
pub use fnc_cursor_grow_in_glyph::fnc_cursor_grow_in_glyph;
pub use fnc_render_cursor::fnc_render_cursor;
pub use fnc_splice_cursor_into_text::fnc_splice_cursor_into_text;
pub use fnc_typewriter_cursor_position::fnc_typewriter_cursor_position;

// <FILE>tui-vfx-content/src/cursor/mod.rs</FILE> - <DESC>General Cursor primitive module</DESC>
// <VERS>END OF VERSION: 0.5.1</VERS>
