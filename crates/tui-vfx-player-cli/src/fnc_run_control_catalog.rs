// <FILE>crates/tui-vfx-player-cli/src/fnc_run_control_catalog.rs</FILE> - <DESC>Run control-catalog CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Control catalog CLI: route descriptor-derived studio catalog command.</WCTX>
// <CLOG>0.1.0: INIT — add control-catalog command runner.</CLOG>

use tui_vfx_player::{build_control_catalog_report, load_descriptor_catalog};

use crate::cls_cli_options::CliOptions;

/// Run the control-catalog command and print its JSON report.
pub fn run_control_catalog(options: CliOptions) -> Result<(), String> {
    validate_options(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let recipe_path = options.paths.first().map(std::path::Path::new);
    let report = build_control_catalog_report(
        descriptor_load.reports,
        &descriptor_load.catalog,
        recipe_path,
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("control catalog report serializes")
    );
    Ok(())
}

fn validate_options(options: &CliOptions) -> Result<(), String> {
    if options.paths.len() > 1 {
        return Err("control-catalog accepts at most one --recipe path".to_string());
    }
    if options.recursive {
        return Err("control-catalog does not accept --recursive".to_string());
    }
    if options.family.is_some() || options.legacy_root.is_some() || options.v31_root.is_some() {
        return Err("control-catalog does not accept migration options".to_string());
    }
    if options.width.is_some() || options.height.is_some() {
        return Err("control-catalog does not accept frame size options".to_string());
    }
    if options.loop_t.is_some() {
        return Err("control-catalog does not accept --loop-t".to_string());
    }
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_control_catalog.rs</FILE> - <DESC>Run control-catalog CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
