// <FILE>crates/tui-vfx-contract-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print contract CLI usage text</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J1: document recursive validation smoke-harness flags.</WCTX>
// <CLOG>0.2.0: MINOR — add recursive/json usage text.
// 0.1.0: INIT — add usage helper.</CLOG>

/// Print CLI usage to stderr.
pub fn print_usage() {
    eprintln!(
        "Usage: tui-vfx-contract-cli validate-recipe [--json] [--recursive] <recipe-or-dir> [...]"
    );
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print contract CLI usage text</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
