// <FILE>crates/tui-vfx-contract-cli/src/fnc_run.rs</FILE> - <DESC>Run canonical contract CLI commands</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: implement validate-recipe command without legacy tooling.</WCTX>
// <CLOG>0.1.0: INIT — add command dispatcher.</CLOG>

use std::path::Path;

use crate::{fnc_print_usage::print_usage, fnc_validate_recipe_file::validate_recipe_file};

/// Run the contract CLI and return the process exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let _program = args.next();
    let Some(command) = args.next() else {
        print_usage();
        return 2;
    };
    if command != "validate-recipe" {
        print_usage();
        return 2;
    }
    let paths = args.collect::<Vec<_>>();
    if paths.is_empty() {
        print_usage();
        return 2;
    }
    let reports = paths
        .iter()
        .map(|path| validate_recipe_file(Path::new(path)))
        .collect::<Vec<_>>();
    let valid = reports.iter().all(|report| report.valid);
    println!(
        "{}",
        serde_json::to_string_pretty(&reports).expect("validation reports serialize")
    );
    if valid { 0 } else { 1 }
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_run.rs</FILE> - <DESC>Run canonical contract CLI commands</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
