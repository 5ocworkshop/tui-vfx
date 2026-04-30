// <FILE>crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse player CLI options</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Native compositor lowering: parse composition mode and fallback policy.</WCTX>
// <CLOG>0.6.0: MINOR — parse --composition-mode and --fail-on-fallback.
// 0.5.0: MINOR — parse schema-readiness offender detail flag.
// 0.4.0: MINOR — parse migration mapping family filter.
// 0.3.1: PATCH — collapse historical parser metadata into latest-change context.</CLOG>

use tui_vfx_contract::LifecyclePhase;

use crate::cls_cli_options::CliOptions;

/// Parse player CLI options after the command token.
pub fn parse_cli_options(args: impl IntoIterator<Item = String>) -> Result<CliOptions, String> {
    let mut options = CliOptions::default();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--recursive" | "-r" => options.recursive = true,
            "--json" => options.json = true,
            "--recipe" => options.paths.push(next_value(&mut args, "--recipe")?),
            "--backend" => options.backend = next_value(&mut args, "--backend")?,
            "--format" => options.format = next_value(&mut args, "--format")?,
            "--composition-mode" => {
                options.composition_mode = next_value(&mut args, "--composition-mode")?
            }
            "--fail-on-fallback" => options.fail_on_fallback = true,
            "--descriptor-pack" => options
                .descriptor_packs
                .push(next_value(&mut args, "--descriptor-pack")?),
            "--descriptor-pack-dir" => options
                .descriptor_pack_dirs
                .push(next_value(&mut args, "--descriptor-pack-dir")?),
            "--phase" => options.phase = parse_phase(&next_value(&mut args, "--phase")?)?,
            "--phase-t" => options.phase_t = parse_f64(&next_value(&mut args, "--phase-t")?)?,
            "--loop-t" => options.loop_t = Some(parse_f64(&next_value(&mut args, "--loop-t")?)?),
            "--width" => options.width = Some(parse_usize(&next_value(&mut args, "--width")?)?),
            "--height" => options.height = Some(parse_usize(&next_value(&mut args, "--height")?)?),
            "--legacy-root" => options.legacy_root = Some(next_value(&mut args, "--legacy-root")?),
            "--v31-root" => options.v31_root = Some(next_value(&mut args, "--v31-root")?),
            "--family" => options.family = Some(next_value(&mut args, "--family")?),
            "--include-offenders" => options.include_offenders = true,
            "--include-blockers" => options.include_blockers = true,
            "--frames" => options.frames = parse_usize(&next_value(&mut args, "--frames")?)?,
            "--fps" => options.fps = parse_u64(&next_value(&mut args, "--fps")?)?,
            "--duration-ms" => {
                options.duration_ms = parse_u64(&next_value(&mut args, "--duration-ms")?)?
            }
            "--samples" => options.samples = parse_usize(&next_value(&mut args, "--samples")?)?,
            "--sample-ms" => {
                options.sample_ms = Some(parse_u64(&next_value(&mut args, "--sample-ms")?)?)
            }
            "--set" => options.sets.push(next_value(&mut args, "--set")?),
            "--no-clear" => options.no_clear = true,
            "--from-sample-t" => {
                options.from_sample_t = parse_f64(&next_value(&mut args, "--from-sample-t")?)?
            }
            "--to-sample-t" => {
                options.to_sample_t = parse_f64(&next_value(&mut args, "--to-sample-t")?)?
            }
            "--" => continue,
            value if value.starts_with("--recipe=") => {
                options.paths.push(value["--recipe=".len()..].to_string());
            }
            value if value.starts_with("--backend=") => {
                options.backend = value["--backend=".len()..].to_string();
            }
            value if value.starts_with("--format=") => {
                options.format = value["--format=".len()..].to_string();
            }
            value if value.starts_with("--composition-mode=") => {
                options.composition_mode = value["--composition-mode=".len()..].to_string();
            }
            value if value.starts_with("--descriptor-pack=") => options
                .descriptor_packs
                .push(value["--descriptor-pack=".len()..].to_string()),
            value if value.starts_with("--descriptor-pack-dir=") => options
                .descriptor_pack_dirs
                .push(value["--descriptor-pack-dir=".len()..].to_string()),
            value if value.starts_with("--phase=") => {
                options.phase = parse_phase(&value["--phase=".len()..])?;
            }
            value if value.starts_with("--phase-t=") => {
                options.phase_t = parse_f64(&value["--phase-t=".len()..])?;
            }
            value if value.starts_with("--loop-t=") => {
                options.loop_t = Some(parse_f64(&value["--loop-t=".len()..])?);
            }
            value if value.starts_with("--width=") => {
                options.width = Some(parse_usize(&value["--width=".len()..])?);
            }
            value if value.starts_with("--height=") => {
                options.height = Some(parse_usize(&value["--height=".len()..])?);
            }
            value if value.starts_with("--legacy-root=") => {
                options.legacy_root = Some(value["--legacy-root=".len()..].to_string());
            }
            value if value.starts_with("--v31-root=") => {
                options.v31_root = Some(value["--v31-root=".len()..].to_string());
            }
            value if value.starts_with("--family=") => {
                options.family = Some(value["--family=".len()..].to_string());
            }
            value if value.starts_with("--frames=") => {
                options.frames = parse_usize(&value["--frames=".len()..])?;
            }
            value if value.starts_with("--fps=") => {
                options.fps = parse_u64(&value["--fps=".len()..])?;
            }
            value if value.starts_with("--duration-ms=") => {
                options.duration_ms = parse_u64(&value["--duration-ms=".len()..])?;
            }
            value if value.starts_with("--samples=") => {
                options.samples = parse_usize(&value["--samples=".len()..])?;
            }
            value if value.starts_with("--sample-ms=") => {
                options.sample_ms = Some(parse_u64(&value["--sample-ms=".len()..])?);
            }
            value if value.starts_with("--set=") => {
                options.sets.push(value["--set=".len()..].to_string());
            }
            value if value.starts_with("--from-sample-t=") => {
                options.from_sample_t = parse_f64(&value["--from-sample-t=".len()..])?;
            }
            value if value.starts_with("--to-sample-t=") => {
                options.to_sample_t = parse_f64(&value["--to-sample-t=".len()..])?;
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            _ => options.paths.push(arg),
        }
    }
    Ok(options)
}

fn next_value(args: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    args.next()
        .filter(|value| !value.starts_with('-'))
        .ok_or_else(|| format!("missing value for `{option}`"))
}

fn parse_phase(value: &str) -> Result<LifecyclePhase, String> {
    match value {
        "enter" => Ok(LifecyclePhase::Enter),
        "dwell" => Ok(LifecyclePhase::Dwell),
        "exit" => Ok(LifecyclePhase::Exit),
        _ => Err(format!("unknown phase `{value}`")),
    }
}

fn parse_f64(value: &str) -> Result<f64, String> {
    value
        .parse::<f64>()
        .map_err(|_| format!("invalid number `{value}`"))
}

fn parse_usize(value: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|_| format!("invalid cell count `{value}`"))
}

fn parse_u64(value: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("invalid integer `{value}`"))
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs</FILE> - <DESC>Parse player CLI options</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
