// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_cell_motion_visibility_position.rs</FILE> - <DESC>Resolve cell-motion visibility gate positions</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: keep visibility and reduced-motion gates explicit.</WCTX>
// <CLOG>0.1.0: add visibility and reduced-motion position helpers.</CLOG>

use super::{
    CellActor, CellMotionPhase, CellMotionStats, CellMotionVisibility, CellVisibilityMode,
};
use tui_vfx_geometry::types::Position;

pub(crate) fn cell_motion_visibility_position(
    mode: CellVisibilityMode,
    _actor: &CellActor,
    from: Position,
    to: Position,
    stats: &mut CellMotionStats,
    before: bool,
) -> Option<Position> {
    match mode {
        CellVisibilityMode::Hidden => {
            if before {
                stats.hidden_before_start_count += 1
            } else {
                stats.hidden_after_complete_count += 1
            };
            None
        }
        CellVisibilityMode::AtFrom | CellVisibilityMode::Hold => Some(from),
        CellVisibilityMode::AtTo => Some(to),
    }
}

pub(crate) fn reduced_cell_motion_position(
    phase: CellMotionPhase,
    before_start: bool,
    actor: &CellActor,
    visibility: &CellMotionVisibility,
    from: Position,
    to: Position,
    stats: &mut CellMotionStats,
) -> Option<Position> {
    if before_start {
        return cell_motion_visibility_position(
            visibility.before_start,
            actor,
            from,
            to,
            stats,
            true,
        );
    }
    match phase {
        CellMotionPhase::Enter => Some(to),
        CellMotionPhase::Exit => {
            stats.hidden_after_complete_count += 1;
            None
        }
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_cell_motion_visibility_position.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
