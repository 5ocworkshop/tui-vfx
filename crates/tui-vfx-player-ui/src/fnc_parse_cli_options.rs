// <FILE>crates/tui-vfx-player-ui/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse visual player UI CLI options</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Native compositor lowering: parse composition mode, fallback policy, and studio mode.</WCTX>
// <CLOG>0.3.0: MINOR — parse --composition-mode, --fail-on-fallback, and --studio.
// 0.2.0: MINOR — support --recipes-root and --recipe startup forms.</CLOG>

use std::path::PathBuf;

use crate::{CliOptions, fnc_find_startup_recipe_path::find_startup_recipe_path};

/// Parse visual player UI options after the binary name.
pub fn parse_cli_options(args: impl IntoIterator<Item = String>) -> Result<CliOptions, String> {
    let mut descriptor_packs = Vec::new();
    let mut descriptor_pack_dirs = Vec::new();
    let mut backend = "styledCell".to_string();
    let mut composition_mode = "irResolved".to_string();
    let mut fail_on_fallback = false;
    let mut studio = false;
    let mut width = None;
    let mut height = None;
    let mut once = false;
    let mut script = None;
    let mut no_clear = false;
    let mut recipes_root = None;
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
            "--recipes-root" => {
                recipes_root = Some(PathBuf::from(next_value(&mut args, "--recipes-root")?))
            }
            "--recipe" => paths.push(next_value(&mut args, "--recipe")?),
            "--backend" => backend = next_value(&mut args, "--backend")?,
            "--composition-mode" => composition_mode = next_value(&mut args, "--composition-mode")?,
            "--fail-on-fallback" => fail_on_fallback = true,
            "--studio" => studio = true,
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
            value if value.starts_with("--recipes-root=") => {
                recipes_root = Some(PathBuf::from(&value["--recipes-root=".len()..]));
            }
            value if value.starts_with("--recipe=") => {
                paths.push(value["--recipe=".len()..].to_string());
            }
            value if value.starts_with("--backend=") => {
                backend = value["--backend=".len()..].to_string();
            }
            value if value.starts_with("--composition-mode=") => {
                composition_mode = value["--composition-mode=".len()..].to_string();
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
    let recipe_path = match paths.pop() {
        Some(path) => PathBuf::from(path),
        None => find_startup_recipe_path(recipes_root.as_deref())?,
    };
    if !paths.is_empty() {
        return Err("only one startup recipe path is supported".to_string());
    }
    Ok(CliOptions {
        recipe_path,
        recipes_root,
        descriptor_packs,
        descriptor_pack_dirs,
        backend,
        composition_mode,
        fail_on_fallback,
        studio,
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
// <VERS>END OF VERSION: 0.3.0</VERS>
