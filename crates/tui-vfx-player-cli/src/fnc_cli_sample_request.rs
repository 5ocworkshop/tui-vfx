// <FILE>crates/tui-vfx-player-cli/src/fnc_cli_sample_request.rs</FILE> - <DESC>Build player sample requests from CLI options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep CLI command runners OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split shared sample request construction from fnc_run.</CLOG>

use tui_vfx_player::PlayerSampleRequest;

use crate::cls_cli_options::CliOptions;

/// Build a player sample request from parsed CLI options.
pub fn cli_sample_request(options: &CliOptions) -> PlayerSampleRequest {
    let (phase_t, loop_t) = match options.sample_ms {
        Some(sample_ms) => sample_time_from_millis(sample_ms, options.duration_ms),
        None => (options.phase_t, options.loop_t),
    };
    PlayerSampleRequest {
        phase: options.phase,
        phase_t,
        loop_t,
        absolute_t_ms: options.sample_ms.map(|sample_ms| sample_ms as f64),
        width: options.width,
        height: options.height,
        ..PlayerSampleRequest::default()
    }
}

/// Convert elapsed milliseconds into normalized phase/loop time.
pub fn sample_time_from_millis(sample_ms: u64, duration_ms: u64) -> (f64, Option<f64>) {
    let duration_ms = duration_ms.max(1);
    let phase_t = (sample_ms % duration_ms) as f64 / duration_ms as f64;
    (phase_t, Some(phase_t))
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_cli_sample_request.rs</FILE> - <DESC>Build player sample requests from CLI options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
