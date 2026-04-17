// <FILE>tui-vfx-content/src/cursor/mod.rs</FILE> - <DESC>General Cursor primitive module</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: module skeleton + all re-exports (Tasks 1–11)</WCTX>
// <CLOG>Initial module with all T1–T11 types wired</CLOG>

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
pub mod fnc_cursor_grow_in_glyph;
pub mod fnc_render_cursor;

pub use cls_cursor::Cursor;
pub use cls_cursor_blink::CursorBlink;
pub use cls_cursor_grow_in::{GrowDirection, GrowIn, GrowInMode};
pub use cls_cursor_paint_ops::{CursorPaintOps, PrimaryOp, TrailOp};
pub use cls_cursor_state::{CursorState, GrowInPhase};
pub use cls_cursor_wake::{Wake, WakeMode};
pub use fnc_advance_cursor::fnc_advance_cursor;
pub use fnc_cursor_grow_in_glyph::fnc_cursor_grow_in_glyph;
pub use fnc_render_cursor::fnc_render_cursor;

// <FILE>tui-vfx-content/src/cursor/mod.rs</FILE> - <DESC>General Cursor primitive module</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
