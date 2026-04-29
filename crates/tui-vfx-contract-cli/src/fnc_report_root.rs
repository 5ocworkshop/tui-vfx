// <FILE>crates/tui-vfx-contract-cli/src/fnc_report_root.rs</FILE> - <DESC>Choose validation report root label</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J1: emit deterministic root field for file and directory validation.</WCTX>
// <CLOG>0.1.0: INIT — add root selection helper.</CLOG>

use crate::cls_cli_options::CliOptions;

/// Select the report root field from parsed CLI options.
pub fn report_root(options: &CliOptions) -> String {
    if options.paths.len() == 1 {
        options.paths[0].clone()
    } else {
        "<multiple>".to_string()
    }
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_report_root.rs</FILE> - <DESC>Choose validation report root label</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
