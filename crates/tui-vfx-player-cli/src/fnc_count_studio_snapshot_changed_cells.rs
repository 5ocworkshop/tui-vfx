// <FILE>crates/tui-vfx-player-cli/src/fnc_count_studio_snapshot_changed_cells.rs</FILE> - <DESC>Count changed rendered cells between studio snapshot backend outputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>OFPF LOC burn-down: isolate studio snapshot cell-diff accounting from command orchestration.</WCTX>
// <CLOG>0.1.0: INIT — extract studio snapshot changed-cell counting.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_player::PlayerRenderBackendOutput;

/// Count rendered backend cell positions whose visible content or style changed.
pub(crate) fn count_studio_snapshot_changed_cells(
    before: &PlayerRenderBackendOutput,
    after: &PlayerRenderBackendOutput,
) -> usize {
    let before_cells = cell_map(before);
    let after_cells = cell_map(after);
    let mut keys = before_cells
        .keys()
        .chain(after_cells.keys())
        .collect::<Vec<_>>();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|key| before_cells.get(key) != after_cells.get(key))
        .count()
}

fn cell_map(output: &PlayerRenderBackendOutput) -> BTreeMap<(usize, usize), String> {
    let mut cells = BTreeMap::new();
    for (y, row) in output.rows.iter().enumerate() {
        for (x, glyph) in row.chars().enumerate() {
            cells.insert((x, y), format!("{glyph}|transparent|transparent||"));
        }
    }
    for cell in &output.styled_cells {
        cells.insert(
            (cell.x, cell.y),
            format!(
                "{}|{}|{}|{}|{}",
                cell.glyph,
                cell.foreground,
                cell.background,
                cell.modifiers.join(","),
                cell.role.clone().unwrap_or_default()
            ),
        );
    }
    cells
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_count_studio_snapshot_changed_cells.rs</FILE> - <DESC>Count changed rendered cells between studio snapshot backend outputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
