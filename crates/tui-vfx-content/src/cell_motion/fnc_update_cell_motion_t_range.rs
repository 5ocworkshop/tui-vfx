// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_update_cell_motion_t_range.rs</FILE> - <DESC>Update cell-motion local progress range stats</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: centralize min/max local_t stats updates.</WCTX>
// <CLOG>0.1.0: add local_t range update helper.</CLOG>

use super::CellMotionStats;

pub(crate) fn update_cell_motion_t_range(stats: &mut CellMotionStats, t: f32, active: bool) {
    if !active {
        return;
    }
    if stats.min_local_t == 0.0 && stats.max_local_t == 0.0 {
        stats.min_local_t = t;
        stats.max_local_t = t;
    } else {
        stats.min_local_t = stats.min_local_t.min(t);
        stats.max_local_t = stats.max_local_t.max(t);
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_update_cell_motion_t_range.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
