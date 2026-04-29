// <FILE>crates/tui-vfx-contract-cli/src/fnc_run.rs</FILE> - <DESC>Run canonical contract CLI commands</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase J1: emit top-level recursive validation run reports.</WCTX>
// <CLOG>0.3.0: MINOR — wrap recipe reports in schemaVersion/root/summary output.
// 0.2.0: MINOR — add recursive path collection and stable JSON diagnostics.
// 0.1.0: INIT — add command dispatcher.</CLOG>

use crate::{
    fnc_build_run_report::build_run_report, fnc_collect_recipe_paths::collect_recipe_paths,
    fnc_parse_cli_options::parse_cli_options, fnc_print_usage::print_usage,
    fnc_report_root::report_root, fnc_validate_recipe_file::validate_recipe_file,
};

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
    let options = match parse_cli_options(args) {
        Ok(options) => options,
        Err(error) => return usage_error(error),
    };
    let paths = match collect_recipe_paths(&options.paths, options.recursive) {
        Ok(paths) if !paths.is_empty() => paths,
        Ok(_) => return usage_error("no JSON recipes found".to_string()),
        Err(error) => return usage_error(error),
    };
    let recipes = paths
        .iter()
        .map(|path| validate_recipe_file(path))
        .collect::<Vec<_>>();
    let report = build_run_report(report_root(&options), recipes);
    let valid = report.summary.invalid == 0;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("validation report serializes")
    );
    if valid { 0 } else { 1 }
}

fn usage_error(error: String) -> i32 {
    eprintln!("Error: {error}");
    print_usage();
    2
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_run.rs</FILE> - <DESC>Run canonical contract CLI commands</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
