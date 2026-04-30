// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_ir.rs</FILE> - <DESC>Run render-ir CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player render evidence work: expose player-owned render IR without UI/compositor coupling.</WCTX>
// <CLOG>0.1.0: INIT — add render-ir command over player render IR API.</CLOG>

use tui_vfx_player::{RecipePlayer, load_descriptor_catalog, render_recipe_file_ir};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
};

/// Run the render-ir command and print a JSON render IR report.
pub fn run_render_ir(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let Some(path) = paths.first() else {
        return Err("render-ir requires one recipe path".to_string());
    };
    if paths.len() > 1 {
        return Err("render-ir currently accepts exactly one recipe path".to_string());
    }
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let report = render_recipe_file_ir(&player, path, &request);
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("render IR report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_ir.rs</FILE> - <DESC>Run render-ir CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
