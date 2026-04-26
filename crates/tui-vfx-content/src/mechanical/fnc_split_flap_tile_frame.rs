// <FILE>crates/tui-vfx-content/src/mechanical/fnc_split_flap_tile_frame.rs</FILE> - <DESC>Render center-hinged multi-cell SplitFlap tile frames</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 3 SplitFlap multi-cell Solari tile support.</WCTX>
// <CLOG>0.2.0: allow callers to schedule per-tile local progress for cascade/dispersion controls.
// 0.1.0: add deterministic center-hinge frame renderer for even-height tiles.</CLOG>

use super::{MechanicalSource, MechanicalTile};
use tui_vfx_types::{Cell, Grid, OwnedGrid};

pub(crate) fn split_flap_tile_frame<F>(
    source: &MechanicalSource,
    tile: MechanicalTile,
    mut tile_progress: F,
) -> OwnedGrid
where
    F: FnMut(usize, usize) -> f64,
{
    let width = source.to.width().max(source.from.width());
    let height = source.to.height().max(source.from.height());
    let mut out = OwnedGrid::new(width, height);
    let tile_w = tile.width as usize;
    let tile_h = tile.height as usize;

    for y in 0..height {
        for x in 0..width {
            let tile_x = x - (x % tile_w);
            let tile_y = y - (y % tile_h);
            let local_y = y - tile_y;
            let tile_col = tile_x / tile_w;
            let tile_row = tile_y / tile_h;
            let p = tile_progress(tile_col, tile_row).clamp(0.0, 1.0);
            let ch = if p <= 0.0 {
                get_ch(&source.from, x, y)
            } else if p >= 1.0 || !tile_changed(source, tile_x, tile_y, tile_w, tile_h) {
                get_ch(&source.to, x, y)
            } else {
                center_hinge_ch(source, x, y, local_y, tile_h, p)
            };
            out.set(x, y, Cell::new(ch));
        }
    }
    out
}

fn center_hinge_ch(
    source: &MechanicalSource,
    x: usize,
    y: usize,
    local_y: usize,
    tile_h: usize,
    progress: f64,
) -> char {
    let half = tile_h / 2;
    if progress < 0.5 {
        if local_y < half {
            get_ch(&source.from, x, y)
        } else {
            get_ch(&source.to, x, y)
        }
    } else {
        let reveal_steps = (((progress - 0.5) / 0.5) * half as f64).floor() as usize;
        if local_y >= half {
            return get_ch(&source.to, x, y);
        }
        let distance = half - 1 - local_y;
        if distance < reveal_steps {
            get_ch(&source.to, x, y)
        } else {
            get_ch(&source.from, x, y)
        }
    }
}

fn tile_changed(
    source: &MechanicalSource,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) -> bool {
    for y in start_y..start_y.saturating_add(height) {
        for x in start_x..start_x.saturating_add(width) {
            if get_ch(&source.from, x, y) != get_ch(&source.to, x, y) {
                return true;
            }
        }
    }
    false
}

fn get_ch(grid: &OwnedGrid, x: usize, y: usize) -> char {
    grid.get(x, y).map(|cell| cell.ch).unwrap_or(' ')
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_split_flap_tile_frame.rs</FILE> - <DESC>Render center-hinged multi-cell SplitFlap tile frames</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
