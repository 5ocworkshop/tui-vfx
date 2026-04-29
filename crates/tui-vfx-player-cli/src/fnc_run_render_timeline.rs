// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_timeline.rs</FILE> - <DESC>Run render-timeline CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: expose deterministic multi-frame timeline evidence.</WCTX>
// <CLOG>0.1.0: INIT — add render-timeline command runner.</CLOG>

use tui_vfx_player::{RecipePlayer, build_frame_timeline_report, load_descriptor_catalog};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths, fnc_report_root::report_root,
};

/// Run the render-timeline command.
pub fn run_render_timeline(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let path = paths
        .first()
        .ok_or_else(|| "render-timeline requires one recipe path".to_string())?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let report = build_frame_timeline_report(
        &player,
        descriptor_load.reports,
        path,
        report_root(&options.paths),
        &request,
        options.frames,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("timeline report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_timeline.rs</FILE> - <DESC>Run render-timeline CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
