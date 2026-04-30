// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print player CLI usage</DESC>
// <VERS>VERSION: 0.9.0</VERS>
// <WCTX>K2.12 schema lock: document offender-ledger usage flag.</WCTX>
// <CLOG>0.9.0: MINOR — document schema-readiness offender output flag.
// 0.8.0: MINOR — document schema-readiness usage.</CLOG>

/// Print usage text for the player CLI.
pub fn print_usage() {
    eprintln!(
        "Usage:\n  tui-vfx-player-cli render-recipe [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--phase enter|dwell|exit] [--phase-t N] [--loop-t N] [--width N] [--height N] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli render-frame [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--phase enter|dwell|exit] [--phase-t N] [--loop-t N] [--width N] [--height N] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli render-ir [--json] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--phase enter|dwell|exit] [--phase-t N] [--loop-t N] [--width N] [--height N] [--recipe PATH]\n  tui-vfx-player-cli inventory-recipes [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli primitive-adapter-gap [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli primitive-field-coverage [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli fixture-qc [--json] [--recursive] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--recipe PATH] [PATH ...]\n  tui-vfx-player-cli render-timeline [--json] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--frames N] [--recipe PATH]\n  tui-vfx-player-cli render-frame-diff [--json] [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--from-sample-t N] [--to-sample-t N] [--recipe PATH]\n  tui-vfx-player-cli migration-gap --legacy-root PATH --v31-root PATH [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--json]\n  tui-vfx-player-cli migration-mapping-batch --legacy-root PATH --v31-root PATH [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--family NAME | --recursive] [--json]\n  tui-vfx-player-cli schema-readiness --legacy-root PATH --v31-root PATH [--descriptor-pack PATH] [--descriptor-pack-dir DIR] [--family NAME | --recursive] [--json] [--include-offenders]"
    );
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_print_usage.rs</FILE> - <DESC>Print player CLI usage</DESC>
// <VERS>END OF VERSION: 0.9.0</VERS>
