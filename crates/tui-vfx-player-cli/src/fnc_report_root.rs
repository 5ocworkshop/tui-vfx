// <FILE>crates/tui-vfx-player-cli/src/fnc_report_root.rs</FILE> - <DESC>Resolve CLI report root labels</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep CLI report helpers focused.</WCTX>
// <CLOG>0.1.0: INIT — split report root labeling from command runners.</CLOG>

/// Return a single root label or a multi-input sentinel for aggregate reports.
pub fn report_root(paths: &[String]) -> String {
    if paths.len() == 1 {
        paths[0].clone()
    } else {
        "<multiple>".to_string()
    }
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_report_root.rs</FILE> - <DESC>Resolve CLI report root labels</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
