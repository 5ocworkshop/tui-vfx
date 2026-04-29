// <FILE>crates/tui-vfx-player-cli/src/fnc_cli_sample_request.rs</FILE> - <DESC>Build player sample requests from CLI options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep CLI command runners OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split shared sample request construction from fnc_run.</CLOG>

use tui_vfx_player::PlayerSampleRequest;

use crate::cls_cli_options::CliOptions;

/// Build a player sample request from parsed CLI options.
pub fn cli_sample_request(options: &CliOptions) -> PlayerSampleRequest {
    PlayerSampleRequest {
        phase: options.phase,
        phase_t: options.phase_t,
        loop_t: options.loop_t,
        width: options.width,
        height: options.height,
        ..PlayerSampleRequest::default()
    }
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_cli_sample_request.rs</FILE> - <DESC>Build player sample requests from CLI options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
