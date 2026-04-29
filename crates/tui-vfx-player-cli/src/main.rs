// <FILE>crates/tui-vfx-player-cli/src/main.rs</FILE> - <DESC>Contract-native player CLI entrypoint</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: expose render-recipe command.</WCTX>
// <CLOG>0.1.0: INIT — add tiny CLI entrypoint for skeleton player smoke rendering.</CLOG>

mod cls_cli_options;
mod fnc_parse_cli_options;
mod fnc_print_usage;
mod fnc_run;

fn main() {
    std::process::exit(fnc_run::run(std::env::args()));
}

// <FILE>crates/tui-vfx-player-cli/src/main.rs</FILE> - <DESC>Contract-native player CLI entrypoint</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
