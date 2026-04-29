// <FILE>crates/tui-vfx-player-ui/src/fnc_run_script.rs</FILE> - <DESC>Run deterministic visual player command scripts</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: make UI behavior testable without raw terminal dependencies.</WCTX>
// <CLOG>0.1.0: INIT — apply comma-separated commands and return rendered snapshots.</CLOG>

use crate::{PlayerUiCommand, PlayerUiState, render_ui_snapshot};

/// Run a deterministic command script and return accumulated snapshots.
pub fn run_script(state: &mut PlayerUiState, script: &str, clear: bool) -> String {
    let mut out = render_ui_snapshot(state, clear);
    for token in script.split(',') {
        let Some(command) = PlayerUiCommand::parse(token) else {
            state.message = format!("unknown command `{}`", token.trim());
            out.push_str(&render_ui_snapshot(state, clear));
            continue;
        };
        if !state.apply_command(command) {
            out.push_str("\nquit\n");
            break;
        }
        out.push_str(&render_ui_snapshot(state, clear));
    }
    out
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_run_script.rs</FILE> - <DESC>Run deterministic visual player command scripts</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
