// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print player CLI usage</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase K2.1: document render, inventory, and migration-gap commands.</WCTX>
// <CLOG>0.3.0: MINOR — include migration-gap report-only command.</CLOG>

/// Print usage text for the player CLI.
pub fn print_usage() {
    eprintln!(
        "Usage:\n  tui-vfx-player-cli render-recipe [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--phase enter|dwell|exit] [--phase-t N] [--loop-t N] [--width N] [--height N] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli inventory-recipes [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli migration-gap --legacy-root PATH --v31-root PATH [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--json]"
    );
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print player CLI usage</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
