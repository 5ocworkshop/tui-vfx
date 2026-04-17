// <FILE>tui-vfx-content/tests/cursor/test_fnc_apply_ghost_glyphs_to_grid.rs</FILE> - <DESC>Tests for ghost-glyph grid overwrite helper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive T27: tests for fnc_apply_ghost_glyphs_to_grid — ghost-mode trail entries overwrite grid glyphs, tint-mode entries (glyph=None) are skipped, out-of-bounds entries are no-ops</WCTX>
// <CLOG>Initial tests — overwrites on Some(glyph), skips None, out-of-bounds no-op</CLOG>

use tui_vfx_content::cursor::{CursorPaintOps, TrailOp, fnc_apply_ghost_glyphs_to_grid};
use tui_vfx_types::{Cell, Grid, OwnedGrid};

#[test]
fn overwrites_glyph_on_ghost_trail_cells() {
    let mut grid = OwnedGrid::new(10, 1);
    grid.set(
        3,
        0,
        Cell {
            ch: ' ',
            ..Cell::default()
        },
    );
    let ops = CursorPaintOps {
        primary: None,
        trail: vec![TrailOp {
            position: (0, 3),
            glyph: Some("▓".into()),
            alpha: 0.5,
        }],
    };
    fnc_apply_ghost_glyphs_to_grid(&mut grid, &ops);
    assert_eq!(grid.get(3, 0).unwrap().ch, '▓');
}

#[test]
fn skips_tint_trail_entries() {
    let mut grid = OwnedGrid::new(10, 1);
    grid.set(
        3,
        0,
        Cell {
            ch: 'x',
            ..Cell::default()
        },
    );
    let ops = CursorPaintOps {
        primary: None,
        trail: vec![TrailOp {
            position: (0, 3),
            glyph: None,
            alpha: 0.5,
        }],
    };
    fnc_apply_ghost_glyphs_to_grid(&mut grid, &ops);
    assert_eq!(grid.get(3, 0).unwrap().ch, 'x'); // unchanged
}

#[test]
fn out_of_bounds_trail_is_ignored() {
    let mut grid = OwnedGrid::new(5, 1);
    let ops = CursorPaintOps {
        primary: None,
        trail: vec![TrailOp {
            position: (0, 99), // col beyond width
            glyph: Some("▓".into()),
            alpha: 0.5,
        }],
    };
    // Should not panic or mutate valid cells.
    fnc_apply_ghost_glyphs_to_grid(&mut grid, &ops);
    for x in 0..5 {
        assert_eq!(grid.get(x, 0).unwrap().ch, ' ');
    }
}

#[test]
fn preserves_existing_cell_styling() {
    use tui_vfx_types::Color;
    let mut grid = OwnedGrid::new(10, 1);
    let original = Cell {
        ch: 'a',
        fg: Color::rgb(200, 100, 50),
        bg: Color::rgb(10, 20, 30),
        ..Cell::default()
    };
    grid.set(4, 0, original);
    let ops = CursorPaintOps {
        primary: None,
        trail: vec![TrailOp {
            position: (0, 4),
            glyph: Some("▓".into()),
            alpha: 0.7,
        }],
    };
    fnc_apply_ghost_glyphs_to_grid(&mut grid, &ops);
    let after = grid.get(4, 0).unwrap();
    assert_eq!(after.ch, '▓');
    assert_eq!(after.fg, original.fg);
    assert_eq!(after.bg, original.bg);
}

// <FILE>tui-vfx-content/tests/cursor/test_fnc_apply_ghost_glyphs_to_grid.rs</FILE> - <DESC>Tests for ghost-glyph grid overwrite helper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
