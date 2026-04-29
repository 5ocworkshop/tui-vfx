// <FILE>crates/tui-vfx-contract-cli/src/main.rs</FILE> - <DESC>Canonical v3.1 contract CLI entrypoint</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase J1: expose stable recursive validation run reports.</WCTX>
// <CLOG>0.3.0: MINOR — register top-level report helpers.
// 0.2.0: MINOR — register recursive validation helpers.
// 0.1.0: INIT — add tiny CLI entrypoint.</CLOG>

mod cls_cli_options;
mod cls_validation_error_report;
mod cls_validation_report;
mod cls_validation_run_report;
mod cls_validation_summary;
mod cls_validation_warning_report;
mod fnc_build_run_report;
mod fnc_collect_recipe_paths;
mod fnc_parse_cli_options;
mod fnc_print_usage;
mod fnc_report_root;
mod fnc_run;
mod fnc_validate_recipe_file;

fn main() {
    std::process::exit(fnc_run::run(std::env::args()));
}

// <FILE>crates/tui-vfx-contract-cli/src/main.rs</FILE> - <DESC>Canonical v3.1 contract CLI entrypoint</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
