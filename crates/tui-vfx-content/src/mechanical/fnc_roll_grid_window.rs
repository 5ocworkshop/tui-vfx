// <FILE>crates/tui-vfx-content/src/mechanical/fnc_roll_grid_window.rs</FILE> - <DESC>Sample fixed-window roll transitions between paired mechanical grids</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 mechanical helpers for grid-first odometer tile roll.</WCTX>
// <CLOG>Add deterministic axis/full-window roll sampling for Odometer.</CLOG>

use super::{MechanicalSource, MechanicalTile};
use crate::types::{OdometerDirection, OdometerTravel};
use tui_vfx_types::{Cell, Grid, OwnedGrid};

pub(crate) fn roll_grid_window(
    source: &MechanicalSource,
    progress: f64,
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile: MechanicalTile,
) -> OwnedGrid {
    let width = source.to.width().max(source.from.width());
    let height = source.to.height().max(source.from.height());
    let mut out = OwnedGrid::new(width, height);
    let (travel_x, travel_y) = travel_distance(width, height, direction, travel, tile);
    let step_x = stepped_offset(progress, travel_x);
    let step_y = stepped_offset(progress, travel_y);

    for y in 0..height {
        for x in 0..width {
            let from_x = x as isize + step_x;
            let from_y = y as isize + step_y;
            let ch = sample_travel_stack(source, from_x, from_y, direction, width, height);
            out.set(x, y, Cell::new(ch));
        }
    }
    out
}

fn stepped_offset(progress: f64, distance: isize) -> isize {
    if distance == 0 {
        return 0;
    }
    let magnitude = ((distance.abs() as f64) * progress.clamp(0.0, 1.0)).floor() as isize;
    magnitude
        .saturating_mul(distance.signum())
        .clamp(distance.min(0), distance.max(0))
}

fn travel_distance(
    width: usize,
    height: usize,
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile: MechanicalTile,
) -> (isize, isize) {
    let cells = match travel {
        OdometerTravel::Axis => None,
        OdometerTravel::FullClear => Some(width.max(height) as u16),
        OdometerTravel::Cells { cells } => Some(cells),
    };
    let x_units = cells.unwrap_or(width as u16).max(tile.width.max(1));
    let y_units = cells.unwrap_or(height as u16).max(tile.height.max(1));
    match direction {
        OdometerDirection::Up => (0, y_units as isize),
        OdometerDirection::Down => (0, -(y_units as isize)),
        OdometerDirection::Left => (x_units as isize, 0),
        OdometerDirection::Right => (-(x_units as isize), 0),
        OdometerDirection::UpLeft => (x_units as isize, y_units as isize),
        OdometerDirection::UpRight => (-(x_units as isize), y_units as isize),
        OdometerDirection::DownLeft => (x_units as isize, -(y_units as isize)),
        OdometerDirection::DownRight => (-(x_units as isize), -(y_units as isize)),
    }
}

fn sample_travel_stack(
    source: &MechanicalSource,
    x: isize,
    y: isize,
    direction: OdometerDirection,
    width: usize,
    height: usize,
) -> char {
    if let Some(ch) = get_ch(&source.from, x, y) {
        return ch;
    }
    let (tx, ty) = target_coord(x, y, direction, width, height);
    get_ch(&source.to, tx, ty).unwrap_or(' ')
}

fn target_coord(
    x: isize,
    y: isize,
    direction: OdometerDirection,
    width: usize,
    height: usize,
) -> (isize, isize) {
    match direction {
        OdometerDirection::Up => (x, y - height as isize),
        OdometerDirection::Down => (x, y + height as isize),
        OdometerDirection::Left => (x - width as isize, y),
        OdometerDirection::Right => (x + width as isize, y),
        OdometerDirection::UpLeft => (x - width as isize, y - height as isize),
        OdometerDirection::UpRight => (x + width as isize, y - height as isize),
        OdometerDirection::DownLeft => (x - width as isize, y + height as isize),
        OdometerDirection::DownRight => (x + width as isize, y + height as isize),
    }
}

fn get_ch(grid: &OwnedGrid, x: isize, y: isize) -> Option<char> {
    let x = usize::try_from(x).ok()?;
    let y = usize::try_from(y).ok()?;
    grid.get(x, y).map(|cell| cell.ch)
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_roll_grid_window.rs</FILE> - <DESC>Sample fixed-window roll transitions between paired mechanical grids</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
