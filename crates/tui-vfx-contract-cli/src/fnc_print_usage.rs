// <FILE>crates/tui-vfx-contract-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print contract CLI usage text</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J0: document validate-recipe command shape.</WCTX>
// <CLOG>0.1.0: INIT — add usage helper.</CLOG>

/// Print CLI usage to stderr.
pub fn print_usage() {
    eprintln!("Usage: tui-vfx-contract-cli validate-recipe <recipe.json> [more-recipe.json ...]");
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print contract CLI usage text</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
