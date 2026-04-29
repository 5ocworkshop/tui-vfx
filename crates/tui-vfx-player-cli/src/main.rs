// <FILE>crates/tui-vfx-player-cli/src/main.rs</FILE> - <DESC>Contract-native player CLI entrypoint</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: register focused player CLI command modules.</WCTX>
// <CLOG>0.2.0: PATCH — add split render, inventory, and migration-gap runner modules.</CLOG>

mod cls_cli_options;
mod fnc_cli_sample_request;
mod fnc_collect_cli_recipe_paths;
mod fnc_parse_cli_options;
mod fnc_print_render_report;
mod fnc_print_usage;
mod fnc_report_root;
mod fnc_run;
mod fnc_run_inventory_recipes;
mod fnc_run_migration_gap;
mod fnc_run_render_recipe;
mod fnc_validate_migration_gap_options;

fn main() {
    std::process::exit(fnc_run::run(std::env::args()));
}

// <FILE>crates/tui-vfx-player-cli/src/main.rs</FILE> - <DESC>Contract-native player CLI entrypoint</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
