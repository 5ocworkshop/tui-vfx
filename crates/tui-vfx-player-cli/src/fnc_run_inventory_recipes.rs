// <FILE>crates/tui-vfx-player-cli/src/fnc_run_inventory_recipes.rs</FILE> - <DESC>Run inventory-recipes CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate inventory-recipes command execution.</WCTX>
// <CLOG>0.1.0: INIT — split inventory command runner from top-level dispatch.</CLOG>

use tui_vfx_player::{RecipePlayer, inventory_recipe_paths, load_descriptor_catalog};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths, fnc_report_root::report_root,
};

/// Run the inventory-recipes command and return a process exit code.
pub fn run_inventory_recipes(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let catalog = descriptor_load.catalog.clone();
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let report = inventory_recipe_paths(
        &player,
        &catalog,
        descriptor_load.reports,
        &paths,
        report_root(&options.paths),
        &request,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("inventory report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_inventory_recipes.rs</FILE> - <DESC>Run inventory-recipes CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
