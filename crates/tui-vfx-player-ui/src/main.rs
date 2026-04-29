// <FILE>crates/tui-vfx-player-ui/src/main.rs</FILE> - <DESC>Contract-native visual player UI entrypoint</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: expose tui-vfx-player-ui binary.</WCTX>
// <CLOG>0.1.0: INIT — delegate process execution to the UI library.</CLOG>

fn main() {
    std::process::exit(tui_vfx_player_ui::run(std::env::args()));
}

// <FILE>crates/tui-vfx-player-ui/src/main.rs</FILE> - <DESC>Contract-native visual player UI entrypoint</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
