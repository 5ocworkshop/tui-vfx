// <FILE>crates/tui-vfx-content/src/mechanical/fnc_tile_rects.rs</FILE> - <DESC>Iterate the tile rectangles that tile a target/source grid for non-Pair mechanical cycles</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3 of mechanical circular content cycles plan: tile-rect geometry that lets Odometer route each tile independently through a resolved cycle.</WCTX>
// <CLOG>0.1.0: introduce tile_rects helper plus extract_tile_text and blit_tile_grid utilities.</CLOG>

use tui_vfx_types::{Cell, Grid, OwnedGrid};

use super::types::MechanicalTile;

/// Coordinate of one tile rectangle inside a larger grid.
///
/// `x` and `y` are top-left grid coordinates; `tile_index` is the
/// linear index used by cascade scheduling. Edge tiles whose grid
/// extents are smaller than `tile.width × tile.height` still report
/// the full tile size — extraction pads, and blit clips back to the
/// output viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TileRect {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) tile_col: usize,
    pub(crate) tile_row: usize,
    pub(crate) tile_index: usize,
}

/// Iterate the tile rectangles that cover `(grid_w, grid_h)` at
/// `tile.width × tile.height`, returning indices in row-major order
/// (left-to-right, top-to-bottom).
///
/// If the grid extent is not divisible by the tile extent, the last
/// column/row of tiles will overhang. Callers handle the overhang in
/// `blit_tile_grid` by clipping into the output viewport.
pub(crate) fn tile_rects(grid_w: usize, grid_h: usize, tile: MechanicalTile) -> Vec<TileRect> {
    let tw = tile.width as usize;
    let th = tile.height as usize;
    if tw == 0 || th == 0 {
        return Vec::new();
    }
    let cols = grid_w.div_ceil(tw);
    let rows = grid_h.div_ceil(th);
    let mut rects = Vec::with_capacity(cols.saturating_mul(rows));
    let mut idx = 0usize;
    for row in 0..rows {
        for col in 0..cols {
            rects.push(TileRect {
                x: col * tw,
                y: row * th,
                tile_col: col,
                tile_row: row,
                tile_index: idx,
            });
            idx += 1;
        }
    }
    rects
}

/// Extract the text content of one tile from a source grid as a
/// newline-joined string padded to `tile.width × tile.height`.
///
/// Cells outside the source grid extent (overhang on the right or
/// bottom edge) are emitted as spaces.
pub(crate) fn extract_tile_text(grid: &OwnedGrid, rect: TileRect, tile: MechanicalTile) -> String {
    let tw = tile.width as usize;
    let th = tile.height as usize;
    let mut out = String::with_capacity(tw.saturating_mul(th).saturating_add(th.saturating_sub(1)));
    for dy in 0..th {
        if dy > 0 {
            out.push('\n');
        }
        for dx in 0..tw {
            let x = rect.x + dx;
            let y = rect.y + dy;
            let ch = grid.get(x, y).map(|cell| cell.ch).unwrap_or(' ');
            out.push(ch);
        }
    }
    out
}

/// Blit the cells of `tile_grid` into `output` at `rect`, clipping to
/// `output`'s extents. `tile_grid` is assumed to be exactly
/// `tile.width × tile.height`; cells past the output viewport are
/// silently dropped (intentional — overhang tiles are how the iterator
/// handles non-divisible grid sizes).
pub(crate) fn blit_tile_grid(
    output: &mut OwnedGrid,
    tile_grid: &OwnedGrid,
    rect: TileRect,
    tile: MechanicalTile,
) {
    let tw = tile.width as usize;
    let th = tile.height as usize;
    for dy in 0..th {
        for dx in 0..tw {
            let ox = rect.x + dx;
            let oy = rect.y + dy;
            if ox >= output.width() || oy >= output.height() {
                continue;
            }
            let cell = tile_grid
                .get(dx, dy)
                .copied()
                .unwrap_or_else(|| Cell::new(' '));
            output.set(ox, oy, cell);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::fnc_grid_text::grid_from_text;
    use super::*;

    fn tile(w: u16, h: u16) -> MechanicalTile {
        MechanicalTile::new(w, h).unwrap()
    }

    #[test]
    fn divides_grid_into_three_one_by_one_tiles() {
        let rects = tile_rects(3, 1, tile(1, 1));
        assert_eq!(rects.len(), 3);
        assert_eq!(rects[0].x, 0);
        assert_eq!(rects[1].x, 1);
        assert_eq!(rects[2].x, 2);
        assert_eq!(rects[0].tile_index, 0);
        assert_eq!(rects[2].tile_index, 2);
    }

    #[test]
    fn iterates_row_major() {
        let rects = tile_rects(6, 2, tile(2, 1));
        assert_eq!(rects.len(), 6);
        for (i, rect) in rects.iter().enumerate() {
            assert_eq!(rect.tile_index, i);
        }
        assert_eq!((rects[0].x, rects[0].y), (0, 0));
        assert_eq!((rects[1].x, rects[1].y), (2, 0));
        assert_eq!((rects[2].x, rects[2].y), (4, 0));
        assert_eq!((rects[3].x, rects[3].y), (0, 1));
        assert_eq!((rects[5].x, rects[5].y), (4, 1));
    }

    #[test]
    fn non_divisible_grid_includes_overhang_tile() {
        // 5 wide tiled by 2-wide tiles → 3 tiles (last overhangs by 1).
        let rects = tile_rects(5, 1, tile(2, 1));
        assert_eq!(rects.len(), 3);
        assert_eq!(rects[2].x, 4);
    }

    #[test]
    fn zero_dimension_yields_empty_iterator() {
        let rects = tile_rects(10, 0, tile(2, 2));
        assert!(rects.is_empty());
    }

    #[test]
    fn extract_one_by_one_tile_reads_single_char() {
        let grid = grid_from_text("ABC", super::super::types::MechanicalSizing::PadToMax);
        let rects = tile_rects(3, 1, tile(1, 1));
        assert_eq!(extract_tile_text(&grid, rects[0], tile(1, 1)), "A");
        assert_eq!(extract_tile_text(&grid, rects[1], tile(1, 1)), "B");
        assert_eq!(extract_tile_text(&grid, rects[2], tile(1, 1)), "C");
    }

    #[test]
    fn extract_three_by_three_tile_returns_newline_joined_face() {
        let grid = grid_from_text(
            "ABCDEF\nGHIJKL\nMNOPQR\nSTUVWX",
            super::super::types::MechanicalSizing::PadToMax,
        );
        let rects = tile_rects(6, 4, tile(3, 2));
        assert_eq!(rects.len(), 4);
        assert_eq!(extract_tile_text(&grid, rects[0], tile(3, 2)), "ABC\nGHI");
        assert_eq!(extract_tile_text(&grid, rects[1], tile(3, 2)), "DEF\nJKL");
        assert_eq!(extract_tile_text(&grid, rects[2], tile(3, 2)), "MNO\nSTU");
        assert_eq!(extract_tile_text(&grid, rects[3], tile(3, 2)), "PQR\nVWX");
    }

    #[test]
    fn extract_overhang_tile_pads_with_spaces() {
        let grid = grid_from_text("12345", super::super::types::MechanicalSizing::PadToMax);
        let rects = tile_rects(5, 1, tile(2, 1));
        assert_eq!(extract_tile_text(&grid, rects[2], tile(2, 1)), "5 ");
    }

    #[test]
    fn blit_writes_tile_into_output() {
        let mut output = OwnedGrid::new(3, 1);
        let tile_grid = grid_from_text("X", super::super::types::MechanicalSizing::PadToMax);
        let rect = TileRect {
            x: 1,
            y: 0,
            tile_col: 1,
            tile_row: 0,
            tile_index: 1,
        };
        blit_tile_grid(&mut output, &tile_grid, rect, tile(1, 1));
        assert_eq!(output.get(0, 0).map(|c| c.ch), Some(' '));
        assert_eq!(output.get(1, 0).map(|c| c.ch), Some('X'));
        assert_eq!(output.get(2, 0).map(|c| c.ch), Some(' '));
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_tile_rects.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
