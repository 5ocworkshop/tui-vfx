// <FILE>crates/tui-vfx-player-cli/src/main.rs</FILE> - <DESC>Contract-native player CLI entrypoint</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Player CLI de-slop: keep entrypoint metadata compact and current.</WCTX>
// <CLOG>0.6.0: MINOR — register migration mapping command runner module.
// 0.5.1: PATCH — collapse historical entrypoint metadata into latest-change context.</CLOG>

mod cls_cli_options;
mod fnc_cli_sample_request;
mod fnc_collect_cli_recipe_paths;
mod fnc_parse_cli_options;
mod fnc_print_render_report;
mod fnc_print_usage;
mod fnc_report_root;
mod fnc_run;
mod fnc_run_fixture_qc;
mod fnc_run_inventory_recipes;
mod fnc_run_migration_gap;
mod fnc_run_migration_mapping_batch;
mod fnc_run_primitive_adapter_gap;
mod fnc_run_primitive_field_coverage;
mod fnc_run_render_frame;
mod fnc_run_render_frame_diff;
mod fnc_run_render_recipe;
mod fnc_run_render_timeline;
mod fnc_validate_migration_gap_options;

fn main() {
    std::process::exit(fnc_run::run(std::env::args()));
}

// <FILE>crates/tui-vfx-player-cli/src/main.rs</FILE> - <DESC>Contract-native player CLI entrypoint</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
