// <FILE>crates/tui-vfx-player-cli/src/fnc_print_render_report.rs</FILE> - <DESC>Print render-recipe JSON reports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep render report printing focused.</WCTX>
// <CLOG>0.1.0: INIT — split frame/run JSON printing from command runner.</CLOG>

use tui_vfx_player::{PlayerFrameReport, PlayerRunReport};

use crate::fnc_report_root::report_root;

/// Print a single frame or aggregate run report as pretty JSON.
pub fn print_render_report(paths: &[String], frames: &[PlayerFrameReport]) {
    if frames.len() == 1 {
        println!(
            "{}",
            serde_json::to_string_pretty(&frames[0]).expect("frame report serializes")
        );
    } else {
        let report = PlayerRunReport::new(report_root(paths), frames.to_vec());
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("run report serializes")
        );
    }
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_print_render_report.rs</FILE> - <DESC>Print render-recipe JSON reports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
