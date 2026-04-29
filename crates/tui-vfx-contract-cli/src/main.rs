// <FILE>crates/tui-vfx-contract-cli/src/main.rs</FILE> - <DESC>Canonical v3.1 contract CLI entrypoint</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: expose validate-recipe command over tui-vfx-contract.</WCTX>
// <CLOG>0.1.0: INIT — add tiny CLI entrypoint.</CLOG>

mod cls_validation_error_report;
mod cls_validation_report;
mod fnc_print_usage;
mod fnc_run;
mod fnc_validate_recipe_file;

fn main() {
    std::process::exit(fnc_run::run(std::env::args()));
}

// <FILE>crates/tui-vfx-contract-cli/src/main.rs</FILE> - <DESC>Canonical v3.1 contract CLI entrypoint</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
