// <FILE>crates/tui-vfx-contract-cli/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse validate-recipe CLI flags</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase J2: parse descriptor-pack flags without adding dependencies.</WCTX>
// <CLOG>0.2.0: MINOR — add descriptor pack file/dir parsing.
// 0.1.0: INIT — add small flag parser for validate-recipe.</CLOG>

use crate::cls_cli_options::CliOptions;

/// Parse validate-recipe options after the command token.
pub fn parse_cli_options(args: impl IntoIterator<Item = String>) -> Result<CliOptions, String> {
    let mut options = CliOptions {
        json: true,
        ..CliOptions::default()
    };
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--recursive" | "-r" => options.recursive = true,
            "--json" => options.json = true,
            "--descriptor-pack" => options
                .descriptor_packs
                .push(next_option_value(&mut args, "--descriptor-pack")?),
            "--descriptor-pack-dir" => options
                .descriptor_pack_dirs
                .push(next_option_value(&mut args, "--descriptor-pack-dir")?),
            "--" => continue,
            value if value.starts_with("--descriptor-pack=") => {
                options
                    .descriptor_packs
                    .push(value["--descriptor-pack=".len()..].to_string());
            }
            value if value.starts_with("--descriptor-pack-dir=") => {
                options
                    .descriptor_pack_dirs
                    .push(value["--descriptor-pack-dir=".len()..].to_string());
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            _ => options.paths.push(arg),
        }
    }
    if options.paths.is_empty() {
        return Err("missing recipe path".to_string());
    }
    Ok(options)
}

fn next_option_value(
    args: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<String, String> {
    args.next()
        .filter(|value| !value.starts_with('-'))
        .ok_or_else(|| format!("missing value for `{option}`"))
}

// <FILE>crates/tui-vfx-contract-cli/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse validate-recipe CLI flags</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
