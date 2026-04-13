// <FILE>crates/tui-vfx-probe/src/fnc_select_cells.rs</FILE> - <DESC>Select which probe cells to emit</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase-1 pipeline probe implementation</WCTX>
// <CLOG>MINOR: Implement all/non-empty/modified selectors while preserving row-major order</CLOG>

use crate::cls_probe_cell::ProbeCell;
use crate::cls_probe_request::ProbeCellSelector;

pub fn select_cells(
    cells: Vec<(ProbeCell, bool, bool)>,
    selector: ProbeCellSelector,
) -> Vec<ProbeCell> {
    cells
        .into_iter()
        .filter_map(|(cell, is_non_empty, is_modified)| match selector {
            ProbeCellSelector::All => Some(cell),
            ProbeCellSelector::NonEmpty if is_non_empty => Some(cell),
            ProbeCellSelector::Modified if is_modified => Some(cell),
            _ => None,
        })
        .collect()
}

// <FILE>crates/tui-vfx-probe/src/fnc_select_cells.rs</FILE> - <DESC>Select which probe cells to emit</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
