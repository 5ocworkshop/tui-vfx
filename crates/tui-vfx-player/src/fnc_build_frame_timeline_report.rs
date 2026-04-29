// <FILE>crates/tui-vfx-player/src/fnc_build_frame_timeline_report.rs</FILE> - <DESC>Build deterministic frame timeline reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: sample one recipe across normalized time.</WCTX>
// <CLOG>0.1.0: INIT — add timeline builder over visual frame rendering.</CLOG>

use std::path::Path;

use crate::{
    DescriptorPackReport, PlayerFrameTimelineReport, PlayerSampleRequest, PlayerVisualFrame,
    RecipePlayer, fnc_build_visual_frame::build_visual_frame,
    fnc_render_recipe_file::render_recipe_file,
};

/// Build a deterministic frame timeline report for one recipe path.
pub fn build_frame_timeline_report(
    player: &RecipePlayer,
    descriptor_packs: Vec<DescriptorPackReport>,
    path: &Path,
    root: String,
    request: &PlayerSampleRequest,
    frame_count: usize,
) -> PlayerFrameTimelineReport {
    let count = frame_count.max(1);
    let frames = (0..count)
        .map(|index| timeline_frame(player, path, request, index, count))
        .collect();
    PlayerFrameTimelineReport::new(root, descriptor_packs, frames)
}

fn timeline_frame(
    player: &RecipePlayer,
    path: &Path,
    request: &PlayerSampleRequest,
    index: usize,
    count: usize,
) -> PlayerVisualFrame {
    let sample_t = if count <= 1 {
        request.phase_t
    } else {
        index as f64 / count.saturating_sub(1) as f64
    };
    let mut sample_request = request.clone();
    sample_request.phase_t = sample_t;
    let mut frame = build_visual_frame(render_recipe_file(player, path, &sample_request));
    frame.absolute_time_ms = (sample_t * 1000.0).round() as u64;
    frame
}

// <FILE>crates/tui-vfx-player/src/fnc_build_frame_timeline_report.rs</FILE> - <DESC>Build deterministic frame timeline reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
