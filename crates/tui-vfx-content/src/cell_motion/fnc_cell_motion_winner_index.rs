// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_cell_motion_winner_index.rs</FILE> - <DESC>Select cell-motion collision winner</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: deterministic collision policy evaluation.</WCTX>
// <CLOG>0.1.0: add collision winner helper.</CLOG>

use super::{CellCollisionMode, cls_cell_motion_candidate::CellMotionCandidate};

pub(crate) fn cell_motion_winner_index(
    candidates: &[CellMotionCandidate],
    mode: CellCollisionMode,
) -> usize {
    match mode {
        CellCollisionMode::ReverseSourceOrder => candidates
            .iter()
            .enumerate()
            .min_by_key(|(_, c)| c.actor.authored_index)
            .map(|(i, _)| i)
            .unwrap_or(0),
        CellCollisionMode::NearestToCompletion => candidates
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.local_t
                    .total_cmp(&b.local_t)
                    .then(a.actor.selected_ordinal.cmp(&b.actor.selected_ordinal))
                    .then(a.actor.authored_index.cmp(&b.actor.authored_index))
            })
            .map(|(i, _)| i)
            .unwrap_or(0),
        _ => candidates
            .iter()
            .enumerate()
            .max_by_key(|(_, c)| c.actor.authored_index)
            .map(|(i, _)| i)
            .unwrap_or(0),
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_cell_motion_winner_index.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
