// <FILE>crates/tui-vfx-contract-cli/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse validate-recipe CLI flags</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J1: parse recursive/json smoke-harness flags without adding dependencies.</WCTX>
// <CLOG>0.1.0: INIT — add small flag parser for validate-recipe.</CLOG>

use crate::cls_cli_options::CliOptions;

/// Parse validate-recipe options after the command token.
pub fn parse_cli_options(args: impl IntoIterator<Item = String>) -> Result<CliOptions, String> {
    let mut options = CliOptions {
        json: true,
        ..CliOptions::default()
    };
    for arg in args {
        match arg.as_str() {
            "--recursive" | "-r" => options.recursive = true,
            "--json" => options.json = true,
            "--" => continue,
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            _ => options.paths.push(arg),
        }
    }
    if options.paths.is_empty() {
        return Err("missing recipe path".to_string());
    }
    Ok(options)
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse validate-recipe CLI flags</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
