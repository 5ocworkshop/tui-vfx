// <FILE>crates/tui-vfx-player/src/fnc_build_frame_diff_report.rs</FILE> - <DESC>Build deterministic frame diff reports</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Player evidence tooling: compare two sampled visual frames.</WCTX>
// <CLOG>0.1.1: PATCH — delegate row/style cell diffing to focused helper.</CLOG>

use std::path::Path;

use crate::{
    DescriptorPackReport, PlayerFrameDiffReport, PlayerSampleRequest, PlayerVisualFrame,
    RecipePlayer, fnc_build_visual_frame::build_visual_frame,
    fnc_diff_visual_frame_cells::diff_visual_frame_cells,
    fnc_render_recipe_file::render_recipe_file,
};

/// Build a deterministic diff between two sampled frames for one recipe path.
pub fn build_frame_diff_report(
    player: &RecipePlayer,
    descriptor_packs: Vec<DescriptorPackReport>,
    path: &Path,
    root: String,
    request: &PlayerSampleRequest,
    from_sample_t: f64,
    to_sample_t: f64,
) -> PlayerFrameDiffReport {
    let from_frame = sampled_frame(player, path, request, from_sample_t);
    let to_frame = sampled_frame(player, path, request, to_sample_t);
    let changed_cells = diff_visual_frame_cells(&from_frame, &to_frame);
    PlayerFrameDiffReport::new(root, descriptor_packs, from_frame, to_frame, changed_cells)
}

fn sampled_frame(
    player: &RecipePlayer,
    path: &Path,
    request: &PlayerSampleRequest,
    sample_t: f64,
) -> PlayerVisualFrame {
    let mut sample_request = request.clone();
    sample_request.phase_t = sample_t;
    let mut frame = build_visual_frame(render_recipe_file(player, path, &sample_request));
    frame.absolute_time_ms = (sample_t * 1000.0).round() as u64;
    frame
}

// <FILE>crates/tui-vfx-player/src/fnc_build_frame_diff_report.rs</FILE> - <DESC>Build deterministic frame diff reports</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
