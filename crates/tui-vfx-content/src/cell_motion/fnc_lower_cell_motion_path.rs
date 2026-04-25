// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_lower_cell_motion_path.rs</FILE> - <DESC>Lower cell-motion route plus dynamics</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: reuse geometry PathType composition for cell-motion path semantics.</WCTX>
// <CLOG>0.1.0: add route+dynamics lowering helper.</CLOG>

use tui_vfx_geometry::types::PathType;

pub(crate) fn lower_cell_motion_path(route: &PathType, dynamics: &[PathType]) -> PathType {
    if dynamics.is_empty() {
        route.clone()
    } else {
        PathType::Composed {
            route: Box::new(route.clone()),
            dynamics: dynamics.to_vec(),
        }
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_lower_cell_motion_path.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
