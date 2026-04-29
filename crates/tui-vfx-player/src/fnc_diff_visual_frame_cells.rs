// <FILE>crates/tui-vfx-player/src/fnc_diff_visual_frame_cells.rs</FILE> - <DESC>Diff visual frame row and styled-cell evidence</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Frame diff reporting: include styled-cell-only changes without overstating screenshot parity.</WCTX>
// <CLOG>0.1.0: INIT — split visual frame cell diffing from report construction.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::{PlayerFrameDiffCell, PlayerVisualCell, PlayerVisualFrame};

/// Diff row glyphs and sparse styled-cell evidence from two visual frames.
pub(crate) fn diff_visual_frame_cells(
    from_frame: &PlayerVisualFrame,
    to_frame: &PlayerVisualFrame,
) -> Vec<PlayerFrameDiffCell> {
    let mut changed_cells = diff_rows(&from_frame.rows, &to_frame.rows);
    let row_changed_coordinates = changed_cells
        .iter()
        .map(|cell| (cell.x, cell.y))
        .collect::<BTreeSet<_>>();
    changed_cells.extend(diff_sparse_cells(
        &from_frame.cells,
        &to_frame.cells,
        &row_changed_coordinates,
    ));
    changed_cells
}

fn diff_rows(from_rows: &[String], to_rows: &[String]) -> Vec<PlayerFrameDiffCell> {
    let height = from_rows.len().max(to_rows.len());
    let mut cells = Vec::new();
    for y in 0..height {
        let from_chars = row_chars(from_rows, y);
        let to_chars = row_chars(to_rows, y);
        for x in 0..from_chars.len().max(to_chars.len()) {
            let from = from_chars.get(x).copied().unwrap_or(' ');
            let to = to_chars.get(x).copied().unwrap_or(' ');
            if from != to {
                cells.push(PlayerFrameDiffCell {
                    x,
                    y,
                    from: from.to_string(),
                    to: to.to_string(),
                });
            }
        }
    }
    cells
}

fn row_chars(rows: &[String], y: usize) -> Vec<char> {
    rows.get(y)
        .map(|row| row.chars().collect::<Vec<_>>())
        .unwrap_or_default()
}

fn diff_sparse_cells(
    from_cells: &[PlayerVisualCell],
    to_cells: &[PlayerVisualCell],
    skipped_coordinates: &BTreeSet<(usize, usize)>,
) -> Vec<PlayerFrameDiffCell> {
    let from_by_coordinate = cells_by_coordinate(from_cells);
    let to_by_coordinate = cells_by_coordinate(to_cells);
    let coordinates = from_by_coordinate
        .keys()
        .chain(to_by_coordinate.keys())
        .copied()
        .collect::<BTreeSet<_>>();
    coordinates
        .into_iter()
        .filter(|coordinate| !skipped_coordinates.contains(coordinate))
        .filter_map(|coordinate| {
            diff_sparse_cell(coordinate, &from_by_coordinate, &to_by_coordinate)
        })
        .collect()
}

fn diff_sparse_cell(
    (x, y): (usize, usize),
    from_by_coordinate: &BTreeMap<(usize, usize), &PlayerVisualCell>,
    to_by_coordinate: &BTreeMap<(usize, usize), &PlayerVisualCell>,
) -> Option<PlayerFrameDiffCell> {
    let from = from_by_coordinate
        .get(&(x, y))
        .map(|cell| visual_cell_label(cell));
    let to = to_by_coordinate
        .get(&(x, y))
        .map(|cell| visual_cell_label(cell));
    (from != to).then(|| PlayerFrameDiffCell {
        x,
        y,
        from: from.unwrap_or_else(|| "default".to_string()),
        to: to.unwrap_or_else(|| "default".to_string()),
    })
}

fn cells_by_coordinate(cells: &[PlayerVisualCell]) -> BTreeMap<(usize, usize), &PlayerVisualCell> {
    cells.iter().map(|cell| ((cell.x, cell.y), cell)).collect()
}

fn visual_cell_label(cell: &PlayerVisualCell) -> String {
    format!(
        "glyph={} fg={} bg={} modifiers={} role={}",
        cell.glyph,
        cell.foreground,
        cell.background,
        cell.modifiers.join("+"),
        cell.role.as_deref().unwrap_or("none")
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_diff_visual_frame_cells.rs</FILE> - <DESC>Diff visual frame row and styled-cell evidence</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
