// <FILE>crates/tui-vfx-content/src/mechanical/fnc_grid_text.rs</FILE> - <DESC>Convert mechanical display text to and from character-cell grids</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 mechanical helpers for grid-first odometer tile roll.</WCTX>
// <CLOG>Add newline-aware grid conversion with PadToMax pairing.</CLOG>

use super::{MechanicalSizing, MechanicalSource};
use tui_vfx_types::{Cell, Grid, OwnedGrid};

pub(crate) fn grid_from_text(text: &str, _policy: MechanicalSizing) -> OwnedGrid {
    let rows: Vec<&str> = text.split('\n').collect();
    let height = rows.len();
    let width = rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0);
    let mut grid = OwnedGrid::new(width, height);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            grid.set(x, y, Cell::new(ch));
        }
    }
    grid
}

pub(crate) fn grid_to_text(grid: &OwnedGrid) -> String {
    let height = grid.height();
    let width = grid.width();
    let mut out = String::with_capacity(width.saturating_mul(height) + height.saturating_sub(1));
    for y in 0..height {
        if y > 0 {
            out.push('\n');
        }
        for x in 0..width {
            out.push(grid.get(x, y).map(|cell| cell.ch).unwrap_or(' '));
        }
    }
    out
}

pub(crate) fn paired_grids(
    from: Option<&str>,
    to: &str,
    policy: MechanicalSizing,
) -> MechanicalSource {
    let to_grid = grid_from_text(to, policy);
    let from_grid = from
        .map(|from| grid_from_text(from, policy))
        .unwrap_or_else(|| OwnedGrid::new(to_grid.width(), to_grid.height()));
    let width = from_grid.width().max(to_grid.width());
    let height = from_grid.height().max(to_grid.height());
    MechanicalSource {
        from: pad_grid(&from_grid, width, height),
        to: pad_grid(&to_grid, width, height),
    }
}

fn pad_grid(source: &OwnedGrid, width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height.min(source.height()) {
        for x in 0..width.min(source.width()) {
            if let Some(cell) = source.get(x, y) {
                grid.set(x, y, *cell);
            }
        }
    }
    grid
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_grid_text.rs</FILE> - <DESC>Convert mechanical display text to and from character-cell grids</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
