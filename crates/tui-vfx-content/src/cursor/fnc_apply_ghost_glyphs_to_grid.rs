// <FILE>tui-vfx-content/src/cursor/fnc_apply_ghost_glyphs_to_grid.rs</FILE> - <DESC>Overwrite grid glyphs at ghost-mode trail positions</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive T27: ghost glyph painter — walks CursorPaintOps.trail and overwrites grid cells whose TrailOp.glyph is Some(_). The CursorShader paints the alpha-blended tint on top of the freshly-overwritten glyph on the style layer; Tint-mode entries (glyph=None) are skipped because they only color-tint whatever is already beneath.</WCTX>
// <CLOG>Initial impl — iterate ops.trail, on Some(glyph) take first char and splice into the grid cell at (col, row), preserving the existing cell's style. Out-of-bounds positions are silently ignored.</CLOG>

use super::CursorPaintOps;
use tui_vfx_types::{Cell, Grid};

/// Overwrite grid cells at ghost-mode wake trail positions with their ghost
/// glyph, preserving the existing cell's styling.
///
/// Walks `ops.trail` and, for every entry whose `glyph` is `Some(_)`, takes
/// the first character of the glyph string and writes it into the grid cell
/// at the trail position, keeping the original `fg` / `bg` / `mods`. Tint-mode
/// entries (`glyph == None`) are skipped — their visual effect is delivered
/// purely by [`tui_vfx_style::models::CursorShader`] at the style layer.
///
/// Positions follow the [`CursorPaintOps`] convention: `(row, col) = (y, x)`.
/// Grid indexing is `(x, y)`. Cells outside the grid are silently ignored
/// (no panic).
///
/// # Limitations
///
/// See spec E9: this helper ignores wide-glyph content already present in
/// the grid — a wide char being overwritten by a ghost glyph will leave the
/// adjacent continuation cell untouched. The consumer is responsible for
/// normalising the grid before invoking this helper if that matters.
pub fn fnc_apply_ghost_glyphs_to_grid(grid: &mut dyn Grid, ops: &CursorPaintOps) {
    for t in &ops.trail {
        let Some(glyph) = t.glyph.as_ref() else {
            continue;
        };
        let Some(ch) = glyph.chars().next() else {
            continue;
        };
        let (row, col) = t.position;
        let x = col as usize;
        let y = row as usize;
        let Some(existing) = grid.get(x, y).copied() else {
            continue;
        };
        grid.set(x, y, Cell { ch, ..existing });
    }
}

// <FILE>tui-vfx-content/src/cursor/fnc_apply_ghost_glyphs_to_grid.rs</FILE> - <DESC>Overwrite grid glyphs at ghost-mode trail positions</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
