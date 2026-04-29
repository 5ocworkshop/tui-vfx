// <FILE>crates/tui-vfx-player/src/fnc_render_visual_frame_paths.rs</FILE> - <DESC>Render paths into visual-frame reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: reuse the player render path for stable visual-frame evidence.</WCTX>
// <CLOG>0.1.0: INIT — add aggregate visual-frame report builder over render_recipe_file.</CLOG>

use std::path::PathBuf;

use crate::{
    DescriptorPackReport, PlayerSampleRequest, PlayerVisualFrameReport, RecipePlayer,
    fnc_build_visual_frame::build_visual_frame, fnc_render_recipe_file::render_recipe_file,
    fnc_summarize_visual_frames::summarize_visual_frames,
};

/// Render collected recipe paths into a visual-frame report using the existing player renderer.
pub fn render_visual_frame_paths(
    player: &RecipePlayer,
    descriptor_packs: Vec<DescriptorPackReport>,
    paths: &[PathBuf],
    root: String,
    request: &PlayerSampleRequest,
) -> PlayerVisualFrameReport {
    let frames = paths
        .iter()
        .map(|path| build_visual_frame(render_recipe_file(player, path, request)))
        .collect::<Vec<_>>();
    let summary = summarize_visual_frames(&frames);
    PlayerVisualFrameReport::new(root, descriptor_packs, summary, frames)
}

// <FILE>crates/tui-vfx-player/src/fnc_render_visual_frame_paths.rs</FILE> - <DESC>Render paths into visual-frame reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
