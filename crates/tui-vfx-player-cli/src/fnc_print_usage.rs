// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print player CLI usage</DESC>
// <VERS>VERSION: 0.6.1</VERS>
// <WCTX>Player CLI de-slop: keep usage metadata compact and current.</WCTX>
// <CLOG>0.6.1: PATCH — collapse historical usage metadata into latest-change context.</CLOG>

/// Print usage text for the player CLI.
pub fn print_usage() {
    eprintln!(
        "Usage:\n  tui-vfx-player-cli render-recipe [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--phase enter|dwell|exit] [--phase-t N] [--loop-t N] [--width N] [--height N] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli render-frame [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--phase enter|dwell|exit] [--phase-t N] [--loop-t N] [--width N] [--height N] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli inventory-recipes [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli primitive-adapter-gap [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli primitive-field-coverage [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli render-timeline [--json] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--frames N] [--recipe PATH]\n  tui-vfx-player-cli render-frame-diff [--json] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--from-sample-t N] [--to-sample-t N] [--recipe PATH]\n  tui-vfx-player-cli migration-gap --legacy-root PATH --v31-root PATH [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--json]"
    );
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print player CLI usage</DESC>
// <VERS>END OF VERSION: 0.6.1</VERS>
