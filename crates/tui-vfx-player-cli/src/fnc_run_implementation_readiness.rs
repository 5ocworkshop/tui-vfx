// <FILE>crates/tui-vfx-player-cli/src/fnc_run_implementation_readiness.rs</FILE> - <DESC>Run implementation-readiness CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Implementation readiness: route disposition-first report command.</WCTX>
// <CLOG>0.1.0: INIT — add implementation-readiness command runner.</CLOG>

use tui_vfx_player::{build_implementation_readiness_report, load_descriptor_catalog};

use crate::cls_cli_options::CliOptions;

/// Run the implementation-readiness command and print its JSON report.
pub fn run_implementation_readiness(options: CliOptions) -> Result<(), String> {
    validate_options(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let legacy_root = required_option(options.legacy_root.as_deref(), "--legacy-root")?;
    let v31_root = required_option(options.v31_root.as_deref(), "--v31-root")?;
    let report = build_implementation_readiness_report(
        std::path::Path::new(legacy_root),
        std::path::Path::new(v31_root),
        descriptor_load.reports,
        &descriptor_load.catalog,
        options.family.as_deref(),
        options.recursive,
        options.include_blockers,
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("implementation readiness report serializes")
    );
    Ok(())
}

fn validate_options(options: &CliOptions) -> Result<(), String> {
    if !options.paths.is_empty() {
        return Err("implementation-readiness does not accept recipe paths".to_string());
    }
    if options.family.is_none() && !options.recursive {
        return Err("implementation-readiness requires --family or --recursive".to_string());
    }
    if options.width.is_some() || options.height.is_some() {
        return Err("implementation-readiness does not accept frame size options".to_string());
    }
    if options.loop_t.is_some() {
        return Err("implementation-readiness does not accept --loop-t".to_string());
    }
    Ok(())
}

fn required_option<'a>(value: Option<&'a str>, option: &str) -> Result<&'a str, String> {
    value.ok_or_else(|| format!("missing required option `{option}`"))
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_implementation_readiness.rs</FILE> - <DESC>Run implementation-readiness CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
