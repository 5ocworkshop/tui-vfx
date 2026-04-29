// <FILE>crates/tui-vfx-player/src/fnc_fixture_qc_smoke_passed.rs</FILE> - <DESC>Classify fixture QC smoke report pass status</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: make fixture QC smoke booleans semantically honest.</WCTX>
// <CLOG>0.1.0: INIT — derive smoke pass/fail from rendered error-free frames.</CLOG>

use crate::{PlayerFrameDiffReport, PlayerFrameTimelineReport, PlayerStatus, PlayerVisualFrame};

/// Return true when timeline smoke frames all rendered without diagnostics.
pub(crate) fn timeline_smoke_passed(report: Option<&PlayerFrameTimelineReport>) -> bool {
    report.is_some_and(|report| !report.frames.is_empty() && report.frames.iter().all(frame_passed))
}

/// Return true when diff smoke endpoints rendered and diff diagnostics are clean.
pub(crate) fn diff_smoke_passed(report: Option<&PlayerFrameDiffReport>) -> bool {
    report.is_some_and(|report| frame_passed(&report.from_frame) && frame_passed(&report.to_frame))
}

fn frame_passed(frame: &PlayerVisualFrame) -> bool {
    frame.status == PlayerStatus::Rendered && frame.errors.is_empty()
}

// <FILE>crates/tui-vfx-player/src/fnc_fixture_qc_smoke_passed.rs</FILE> - <DESC>Classify fixture QC smoke report pass status</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
