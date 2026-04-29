// <FILE>crates/tui-vfx-player-ui/src/fnc_print_usage.rs</FILE> - <DESC>Visual player UI usage text</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: document ratatui visual shell command shape.</WCTX>
// <CLOG>0.1.0: INIT — add concise stderr usage text.</CLOG>

/// Print player UI usage to stderr.
pub fn print_usage() {
    eprintln!(
        "Usage: tui-vfx-player-ui [--descriptor-pack <file>] [--descriptor-pack-dir <dir>] [--recipes-root <dir>] [--recipe <recipe.json>] [--width <cells>] [--height <cells>] [--once] [--script <commands>] [--no-clear] [recipe.json]"
    );
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_print_usage.rs</FILE> - <DESC>Visual player UI usage text</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
