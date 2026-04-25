// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_candidate.rs</FILE> - <DESC>Internal moved actor collision candidate</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: keep cell-motion collision candidate state out of the scheduler entry file.</WCTX>
// <CLOG>0.1.0: add internal Candidate for destination bucket resolution.</CLOG>

use super::CellActor;

#[derive(Clone, Debug)]
pub(crate) struct CellMotionCandidate {
    pub(crate) actor: CellActor,
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) local_t: f32,
}

// <FILE>crates/tui-vfx-content/src/cell_motion/cls_cell_motion_candidate.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
