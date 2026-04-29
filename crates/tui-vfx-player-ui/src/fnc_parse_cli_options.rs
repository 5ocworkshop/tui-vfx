// <FILE>crates/tui-vfx-player-ui/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse visual player UI CLI options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: parse one-recipe UI flags without adding dependencies.</WCTX>
// <CLOG>0.1.0: INIT — add hand-rolled parser for descriptor packs, dimensions, and script mode.</CLOG>

use std::path::PathBuf;

use crate::CliOptions;

/// Parse visual player UI options after the binary name.
pub fn parse_cli_options(args: impl IntoIterator<Item = String>) -> Result<CliOptions, String> {
    let mut descriptor_packs = Vec::new();
    let mut descriptor_pack_dirs = Vec::new();
    let mut width = None;
    let mut height = None;
    let mut once = false;
    let mut script = None;
    let mut no_clear = false;
    let mut paths = Vec::new();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--descriptor-pack" => {
                descriptor_packs.push(next_value(&mut args, "--descriptor-pack")?)
            }
            "--descriptor-pack-dir" => {
                descriptor_pack_dirs.push(next_value(&mut args, "--descriptor-pack-dir")?);
            }
            "--width" => width = Some(parse_usize(&next_value(&mut args, "--width")?)?),
            "--height" => height = Some(parse_usize(&next_value(&mut args, "--height")?)?),
            "--once" => once = true,
            "--script" => script = Some(next_value(&mut args, "--script")?),
            "--no-clear" => no_clear = true,
            value if value.starts_with("--descriptor-pack=") => {
                descriptor_packs.push(value["--descriptor-pack=".len()..].to_string());
            }
            value if value.starts_with("--descriptor-pack-dir=") => {
                descriptor_pack_dirs.push(value["--descriptor-pack-dir=".len()..].to_string());
            }
            value if value.starts_with("--width=") => {
                width = Some(parse_usize(&value["--width=".len()..])?);
            }
            value if value.starts_with("--height=") => {
                height = Some(parse_usize(&value["--height=".len()..])?);
            }
            value if value.starts_with("--script=") => {
                script = Some(value["--script=".len()..].to_string());
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            _ => paths.push(arg),
        }
    }
    let Some(recipe_path) = paths.pop() else {
        return Err("missing recipe path".to_string());
    };
    if !paths.is_empty() {
        return Err("only one recipe path is supported in K1".to_string());
    }
    Ok(CliOptions {
        recipe_path: PathBuf::from(recipe_path),
        descriptor_packs,
        descriptor_pack_dirs,
        width,
        height,
        once,
        script,
        no_clear,
    })
}

fn next_value(args: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    args.next()
        .filter(|value| !value.starts_with('-'))
        .ok_or_else(|| format!("missing value for `{option}`"))
}

fn parse_usize(value: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|_| format!("invalid cell count `{value}`"))
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse visual player UI CLI options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
