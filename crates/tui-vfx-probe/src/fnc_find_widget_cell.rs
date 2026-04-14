// <FILE>crates/tui-vfx-probe/src/fnc_find_widget_cell.rs</FILE> - <DESC>Find one widget-local cell inside a probe report</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Root-cause-first CLI focus flow</WCTX>
// <CLOG>NEW: Add a helper that returns one widget-local cell from a probe report so command-line tools can expose focused debugging without forcing users to scan full-frame payloads</CLOG>

use crate::{ProbeCell, ProbeReport};

pub fn find_widget_cell(report: &ProbeReport, x: u16, y: u16) -> Option<ProbeCell> {
    report
        .cells
        .iter()
        .find(|cell| cell.widget_local.x == x && cell.widget_local.y == y)
        .cloned()
}

// <FILE>crates/tui-vfx-probe/src/fnc_find_widget_cell.rs</FILE> - <DESC>Find one widget-local cell inside a probe report</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
