// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_frame.rs</FILE> - <DESC>Run render-frame CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.2: expose visual-frame evidence through the CLI.</WCTX>
// <CLOG>0.1.0: INIT — add render-frame command runner over the existing player renderer.</CLOG>

use tui_vfx_player::{RecipePlayer, load_descriptor_catalog, render_visual_frame_paths};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths, fnc_report_root::report_root,
};

/// Run the render-frame command and return a process exit code.
pub fn run_render_frame(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let report = render_visual_frame_paths(
        &player,
        descriptor_load.reports,
        &paths,
        report_root(&options.paths),
        &request,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("visual frame report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_frame.rs</FILE> - <DESC>Run render-frame CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
