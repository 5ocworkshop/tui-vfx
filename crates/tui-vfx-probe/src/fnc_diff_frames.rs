// <FILE>crates/tui-vfx-probe/src/fnc_diff_frames.rs</FILE> - <DESC>Compute changed cells between two frame dumps</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe diff support</WCTX>
// <CLOG>NEW: Add a helper that turns two frame dumps into a structured list of changed cells</CLOG>

use std::collections::HashMap;

use crate::cls_probe_diff_cell::ProbeDiffCell;
use crate::cls_probe_report::ProbeReport;
use crate::cls_probe_state_snapshot::ProbeStateSnapshot;

pub fn diff_frames(from: &ProbeReport, to: &ProbeReport) -> Vec<ProbeDiffCell> {
    let to_cells_by_pos: HashMap<(u16, u16), _> = to
        .cells
        .iter()
        .map(|cell| ((cell.widget_local.x, cell.widget_local.y), cell))
        .collect();

    from.cells
        .iter()
        .filter_map(|before_cell| {
            let key = (before_cell.widget_local.x, before_cell.widget_local.y);
            let after_cell = to_cells_by_pos.get(&key)?;
            if before_cell.ch == after_cell.ch
                && before_cell.fg == after_cell.fg
                && before_cell.bg == after_cell.bg
                && before_cell.modifiers == after_cell.modifiers
            {
                return None;
            }
            Some(ProbeDiffCell {
                abs: after_cell.abs,
                widget_local: after_cell.widget_local,
                before: ProbeStateSnapshot {
                    ch: Some(before_cell.ch),
                    fg: before_cell.fg.clone(),
                    bg: before_cell.bg.clone(),
                    modifiers: before_cell.modifiers.clone(),
                },
                after: ProbeStateSnapshot {
                    ch: Some(after_cell.ch),
                    fg: after_cell.fg.clone(),
                    bg: after_cell.bg.clone(),
                    modifiers: after_cell.modifiers.clone(),
                },
                last_touch: after_cell.last_touch.clone(),
                trace: after_cell.trace.clone(),
            })
        })
        .collect()
}

// <FILE>crates/tui-vfx-probe/src/fnc_diff_frames.rs</FILE> - <DESC>Compute changed cells between two frame dumps</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
