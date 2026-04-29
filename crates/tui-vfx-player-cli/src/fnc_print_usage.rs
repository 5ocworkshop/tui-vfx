// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Player CLI usage text</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: document render-recipe command shape.</WCTX>
// <CLOG>0.1.0: INIT — add concise usage output.</CLOG>

/// Print player CLI usage to stderr.
pub fn print_usage() {
    eprintln!(
        "Usage: tui-vfx-player-cli render-recipe [--json] [--recursive] [--recipe <file>] [--descriptor-pack <file>] [--descriptor-pack-dir <dir>] [--phase enter|dwell|exit] [--phase-t <0..1>] [--loop-t <0..1>] [--width <cells>] [--height <cells>] <recipe-or-dir> [...]"
    );
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Player CLI usage text</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
