// <FILE>crates/tui-vfx-player/src/fnc_summarize_visual_frames.rs</FILE> - <DESC>Summarize visual-frame statuses</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: summarize render status counts for visual-frame reports.</WCTX>
// <CLOG>0.1.0: INIT — add aggregate counts for visual-frame entries.</CLOG>

use crate::{PlayerStatus, PlayerSummary, PlayerVisualFrame};

/// Summarize visual-frame render statuses.
pub(crate) fn summarize_visual_frames(frames: &[PlayerVisualFrame]) -> PlayerSummary {
    let mut summary = PlayerSummary {
        total: frames.len(),
        ..PlayerSummary::default()
    };
    for frame in frames {
        match frame.status {
            PlayerStatus::Rendered => summary.rendered += 1,
            PlayerStatus::Unsupported => summary.unsupported += 1,
            PlayerStatus::Error => summary.errors += 1,
        }
    }
    summary
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_visual_frames.rs</FILE> - <DESC>Summarize visual-frame statuses</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
