// <FILE>crates/tui-vfx-contract-cli/src/main.rs</FILE> - <DESC>Canonical v3.1 contract CLI entrypoint</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase J2: register descriptor-pack catalog helpers.</WCTX>
// <CLOG>0.4.0: MINOR — register descriptor pack loading/reporting helpers.
// 0.3.0: MINOR — register top-level report helpers.
// 0.2.0: MINOR — register recursive validation helpers.
// 0.1.0: INIT — add tiny CLI entrypoint.</CLOG>

mod cls_cli_options;
mod cls_descriptor_pack_load;
mod cls_descriptor_pack_report;
mod cls_validation_error_report;
mod cls_validation_report;
mod cls_validation_run_report;
mod cls_validation_summary;
mod cls_validation_warning_report;
mod fnc_build_run_report;
mod fnc_collect_descriptor_pack_paths;
mod fnc_collect_recipe_paths;
mod fnc_load_descriptor_catalog;
mod fnc_parse_cli_options;
mod fnc_print_usage;
mod fnc_report_root;
mod fnc_run;
mod fnc_validate_recipe_file;

fn main() {
    std::process::exit(fnc_run::run(std::env::args()));
}

// <FILE>crates/tui-vfx-contract-cli/src/main.rs</FILE> - <DESC>Canonical v3.1 contract CLI entrypoint</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
