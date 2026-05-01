// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/source/col_source_grid_from_text.rs</FILE> - <DESC>Build a grid from literal source text</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pure leaf helper for direct v3.1 source rendering.</WCTX>
// <CLOG>0.1.0: INIT — extract text-to-grid leaf helper.</CLOG>

use tui_vfx_types::{Cell, Color, Grid, OwnedGrid};

pub(crate) fn source_grid_from_text(text: &str, width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    let lines: Vec<&str> = text.lines().collect();
    let lines = if lines.is_empty() { vec![text] } else { lines };
    for (y, line) in lines.into_iter().take(height).enumerate() {
        for (x, ch) in line.chars().take(width).enumerate() {
            grid.set(
                x,
                y,
                Cell {
                    ch,
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                    ..Default::default()
                },
            );
        }
    }
    grid
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/source/col_source_grid_from_text.rs</FILE> - <DESC>Build a grid from literal source text</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
