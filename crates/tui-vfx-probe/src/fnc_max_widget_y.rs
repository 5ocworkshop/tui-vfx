// <FILE>crates/tui-vfx-probe/src/fnc_max_widget_y.rs</FILE> - <DESC>Find the maximum widget-local Y present in a probe report</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>NEW: Add a helper that finds the bottommost emitted widget row so diagnostics can reason about borders without assuming full frame occupancy</CLOG>

use crate::ProbeReport;

pub fn max_widget_y(report: &ProbeReport) -> u16 {
    report
        .cells
        .iter()
        .map(|cell| cell.widget_local.y)
        .max()
        .unwrap_or_default()
}

// <FILE>crates/tui-vfx-probe/src/fnc_max_widget_y.rs</FILE> - <DESC>Find the maximum widget-local Y present in a probe report</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
