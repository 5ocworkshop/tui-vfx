// <FILE>crates/tui-vfx-player-cli/src/fnc_run.rs</FILE> - <DESC>Run player CLI commands</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>New kernel Phase K2.2: dispatch render-frame visual evidence command.</WCTX>
// <CLOG>0.6.0: MINOR — add render-frame to top-level command dispatch.</CLOG>

use crate::{
    cls_cli_options::CliOptions, fnc_parse_cli_options::parse_cli_options,
    fnc_print_usage::print_usage, fnc_run_inventory_recipes::run_inventory_recipes,
    fnc_run_migration_gap::run_migration_gap, fnc_run_render_frame::run_render_frame,
    fnc_run_render_recipe::run_render_recipe,
};

/// Run the player CLI and return the process exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let _program = args.next();
    let Some(command) = args.next() else {
        print_usage();
        return 2;
    };
    if !is_known_command(&command) {
        print_usage();
        return 2;
    }
    let options = match parse_cli_options(args) {
        Ok(options) => options,
        Err(error) => return usage_error(error),
    };
    dispatch_command(&command, options).map_or_else(usage_error, |_| 0)
}

fn is_known_command(command: &str) -> bool {
    matches!(
        command,
        "render-recipe" | "render-frame" | "inventory-recipes" | "migration-gap"
    )
}

fn dispatch_command(command: &str, options: CliOptions) -> Result<(), String> {
    match command {
        "render-recipe" => run_render_recipe(options),
        "render-frame" => run_render_frame(options),
        "inventory-recipes" => run_inventory_recipes(options),
        "migration-gap" => run_migration_gap(options),
        _ => unreachable!("command already validated"),
    }
}

fn usage_error(error: String) -> i32 {
    eprintln!("Error: {error}");
    print_usage();
    2
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run.rs</FILE> - <DESC>Run player CLI commands</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
