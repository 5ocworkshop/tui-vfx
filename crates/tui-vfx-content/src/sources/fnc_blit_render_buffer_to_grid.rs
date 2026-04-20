// <FILE>crates/tui-vfx-content/src/sources/fnc_blit_render_buffer_to_grid.rs</FILE> - <DESC>Shared helper that writes rocketsplash RenderBuffer cells into a tui-vfx Grid at a given offset. Used by both RocketsplashImage and RocketsplashFont source primitives since both produce the same RenderBuffer type.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1 of the splash library + VFX integration plan.</WCTX>
// <CLOG>0.1.0: initial; map RenderCell → tui-vfx Cell, honor opacity, clip against grid bounds.</CLOG>

use rocketsplash_rt::{RenderBuffer, RenderCell, TextStyle};
use tui_vfx_types::{Cell, Color, Grid, Modifiers};

/// Blit a rocketsplash [`RenderBuffer`] into a tui-vfx [`Grid`] starting at
/// `(offset_x, offset_y)`. Cells with zero opacity are skipped (preserving
/// the grid cell underneath, which matters when compositing a rocketsplash
/// asset on top of other content). Cells that fall outside the grid bounds
/// are silently clipped.
///
/// This is the shared primitive that every rocketsplash source type uses —
/// images ([`crate::sources::RocketsplashImage`]) and fonts
/// ([`crate::sources::RocketsplashFont`]) both produce [`RenderBuffer`]s,
/// so the blit path is written once.
pub fn blit_render_buffer_to_grid(
    buffer: &RenderBuffer,
    grid: &mut dyn Grid,
    offset_x: usize,
    offset_y: usize,
) {
    for src_y in 0..buffer.height {
        for src_x in 0..buffer.width {
            let idx = src_y * buffer.width + src_x;
            let Some(src_cell) = buffer.cells.get(idx) else {
                continue;
            };
            if src_cell.opacity == 0 {
                continue;
            }
            let dst_x = offset_x + src_x;
            let dst_y = offset_y + src_y;
            if !grid.in_bounds(dst_x, dst_y) {
                continue;
            }
            grid.set(dst_x, dst_y, render_cell_to_cell(src_cell));
        }
    }
}

fn render_cell_to_cell(src: &RenderCell) -> Cell {
    Cell {
        ch: src.ch,
        fg: src
            .fg
            .map(|rgb| Color::rgb(rgb.r, rgb.g, rgb.b))
            .unwrap_or(Color::TRANSPARENT),
        bg: src
            .bg
            .map(|rgb| Color::rgb(rgb.r, rgb.g, rgb.b))
            .unwrap_or(Color::TRANSPARENT),
        mods: text_style_to_modifiers(src.style),
        mod_alpha: None,
    }
}

fn text_style_to_modifiers(style: TextStyle) -> Modifiers {
    Modifiers {
        bold: style.contains(TextStyle::BOLD),
        italic: style.contains(TextStyle::ITALIC),
        underline: style.contains(TextStyle::UNDERLINE),
        reverse: style.contains(TextStyle::REVERSE),
        ..Modifiers::NONE
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::OwnedGrid;

    fn empty_grid(w: usize, h: usize) -> OwnedGrid {
        OwnedGrid::new(w, h)
    }

    fn opaque_render_cell(ch: char) -> RenderCell {
        let mut c = RenderCell::empty();
        c.ch = ch;
        c.opacity = 255;
        c
    }

    #[test]
    fn blit_copies_cells_at_origin() {
        let mut grid = empty_grid(10, 5);
        let mut buf = RenderBuffer::new(3, 2);
        buf.cells[0] = opaque_render_cell('A');
        buf.cells[1] = opaque_render_cell('B');
        buf.cells[3] = opaque_render_cell('C');
        blit_render_buffer_to_grid(&buf, &mut grid, 0, 0);
        assert_eq!(grid.get(0, 0).map(|c| c.ch), Some('A'));
        assert_eq!(grid.get(1, 0).map(|c| c.ch), Some('B'));
        assert_eq!(grid.get(0, 1).map(|c| c.ch), Some('C'));
    }

    #[test]
    fn blit_honors_offset() {
        let mut grid = empty_grid(10, 5);
        let mut buf = RenderBuffer::new(2, 1);
        buf.cells[0] = opaque_render_cell('X');
        blit_render_buffer_to_grid(&buf, &mut grid, 4, 2);
        assert_eq!(grid.get(4, 2).map(|c| c.ch), Some('X'));
        assert_eq!(grid.get(0, 0).map(|c| c.ch), Some(' '));
    }

    #[test]
    fn blit_skips_zero_opacity_cells() {
        let mut grid = empty_grid(5, 1);
        // Seed the grid with a known character so we can detect preservation.
        grid.set(0, 0, Cell::new('K'));
        let mut buf = RenderBuffer::new(1, 1);
        // opacity 0 → blit should skip this cell
        buf.cells[0] = RenderCell::empty();
        blit_render_buffer_to_grid(&buf, &mut grid, 0, 0);
        assert_eq!(grid.get(0, 0).map(|c| c.ch), Some('K'));
    }

    #[test]
    fn blit_clips_outside_bounds() {
        let mut grid = empty_grid(3, 3);
        let mut buf = RenderBuffer::new(5, 5);
        for cell in &mut buf.cells {
            *cell = opaque_render_cell('Z');
        }
        // Offset 2,2 with a 5x5 source — most cells fall off the 3x3 grid.
        blit_render_buffer_to_grid(&buf, &mut grid, 2, 2);
        // Only (2,2) should be populated; everything else should be unchanged.
        assert_eq!(grid.get(2, 2).map(|c| c.ch), Some('Z'));
        assert_eq!(grid.get(0, 0).map(|c| c.ch), Some(' '));
    }

    #[test]
    fn blit_maps_styles_to_modifiers() {
        let mut grid = empty_grid(1, 1);
        let mut buf = RenderBuffer::new(1, 1);
        let mut src = opaque_render_cell('B');
        src.style = TextStyle::BOLD | TextStyle::ITALIC;
        buf.cells[0] = src;
        blit_render_buffer_to_grid(&buf, &mut grid, 0, 0);
        let cell = grid.get(0, 0).copied().unwrap();
        assert!(cell.mods.bold);
        assert!(cell.mods.italic);
        assert!(!cell.mods.underline);
    }
}

// <FILE>crates/tui-vfx-content/src/sources/fnc_blit_render_buffer_to_grid.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
