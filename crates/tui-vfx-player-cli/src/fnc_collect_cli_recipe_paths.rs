// <FILE>crates/tui-vfx-player-cli/src/fnc_collect_cli_recipe_paths.rs</FILE> - <DESC>Collect recipe paths from CLI options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep CLI command runners OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split shared recipe path collection from fnc_run.</CLOG>

use tui_vfx_player::collect_recipe_paths;

use crate::cls_cli_options::CliOptions;

/// Collect JSON recipe paths requested by render/inventory commands.
pub fn collect_cli_recipe_paths(options: &CliOptions) -> Result<Vec<std::path::PathBuf>, String> {
    if options.paths.is_empty() {
        return Err("missing recipe path".to_string());
    }
    match collect_recipe_paths(&options.paths, options.recursive) {
        Ok(paths) if !paths.is_empty() => Ok(paths),
        Ok(_) => Err("no JSON recipes found".to_string()),
        Err(error) => Err(error),
    }
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_collect_cli_recipe_paths.rs</FILE> - <DESC>Collect recipe paths from CLI options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
