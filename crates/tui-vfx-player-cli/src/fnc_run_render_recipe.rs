// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_recipe.rs</FILE> - <DESC>Run render-recipe CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate render-recipe command execution.</WCTX>
// <CLOG>0.1.0: INIT — split render command runner from top-level dispatch.</CLOG>

use tui_vfx_player::{RecipePlayer, load_descriptor_catalog, render_recipe_file};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
    fnc_print_render_report::print_render_report,
};

/// Run the render-recipe command and return a process exit code.
pub fn run_render_recipe(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let frames = paths
        .iter()
        .map(|path| render_recipe_file(&player, path, &request))
        .collect::<Vec<_>>();
    print_render_report(&options.paths, &frames);
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_recipe.rs</FILE> - <DESC>Run render-recipe CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
