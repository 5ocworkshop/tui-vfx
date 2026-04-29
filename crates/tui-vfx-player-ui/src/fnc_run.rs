// <FILE>crates/tui-vfx-player-ui/src/fnc_run.rs</FILE> - <DESC>Run visual player UI commands</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: route one-shot, script, and interactive UI modes.</WCTX>
// <CLOG>0.1.0: INIT — parse CLI, load K0 player state, and render the visual shell.</CLOG>

use crate::{
    PlayerUiState, fnc_parse_cli_options::parse_cli_options, fnc_print_usage::print_usage,
    fnc_run_interactive::run_interactive, render_ui_snapshot, run_script,
};

/// Run the player UI process and return an exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let _program = args.next();
    let options = match parse_cli_options(args) {
        Ok(options) => options,
        Err(error) => return usage_error(error),
    };
    let mut state = match PlayerUiState::load(&options) {
        Ok(state) => state,
        Err(error) => return runtime_error(error),
    };
    let clear = !options.no_clear;
    if options.once {
        println!("{}", render_ui_snapshot(&state, clear));
        return 0;
    }
    if let Some(script) = &options.script {
        print!("{}", run_script(&mut state, script, clear));
        return 0;
    }
    match run_interactive(state) {
        Ok(()) => 0,
        Err(error) => runtime_error(error),
    }
}

fn usage_error(error: String) -> i32 {
    eprintln!("Error: {error}");
    print_usage();
    2
}

fn runtime_error(error: String) -> i32 {
    eprintln!("Error: {error}");
    1
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_run.rs</FILE> - <DESC>Run visual player UI commands</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
