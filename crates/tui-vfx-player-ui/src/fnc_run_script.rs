// <FILE>crates/tui-vfx-player-ui/src/fnc_run_script.rs</FILE> - <DESC>Run deterministic visual player command scripts</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Native compositor studio pilot: support semicolon scripts and generated control assignments.</WCTX>
// <CLOG>0.2.0: MINOR — parse `set control=value` script commands for studio control proof.
// 0.1.0: INIT — apply comma-separated commands and return rendered snapshots.</CLOG>

use tui_vfx_contract::Value;

use crate::{PlayerUiCommand, PlayerUiState, render_ui_snapshot};

/// Run a deterministic command script and return accumulated snapshots.
pub fn run_script(state: &mut PlayerUiState, script: &str, clear: bool) -> String {
    let mut out = render_ui_snapshot(state, clear);
    for token in script.split([',', ';']) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if let Some(rest) = token.strip_prefix("set ") {
            match apply_set_command(state, rest) {
                Ok(()) => out.push_str(&render_ui_snapshot(state, clear)),
                Err(error) => {
                    state.message = error;
                    out.push_str(&render_ui_snapshot(state, clear));
                }
            }
            continue;
        }
        let Some(command) = PlayerUiCommand::parse(token) else {
            state.message = format!("unknown command `{token}`");
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

fn apply_set_command(state: &mut PlayerUiState, assignment: &str) -> Result<(), String> {
    let (key, value) = assignment
        .split_once('=')
        .ok_or_else(|| format!("studio set command expects key=value, got `{assignment}`"))?;
    state.set_control_value(key.trim(), parse_value(value.trim()))
}

fn parse_value(value: &str) -> Value {
    if value.eq_ignore_ascii_case("true") {
        return Value::Boolean(true);
    }
    if value.eq_ignore_ascii_case("false") {
        return Value::Boolean(false);
    }
    if let Ok(integer) = value.parse::<i64>() {
        return Value::Integer(integer);
    }
    if let Ok(number) = value.parse::<f64>() {
        return Value::Number(number);
    }
    Value::String(value.to_string())
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_run_script.rs</FILE> - <DESC>Run deterministic visual player command scripts</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
