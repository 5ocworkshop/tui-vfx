// <FILE>crates/tui-vfx-player-cli/src/fnc_run_primitive_field_coverage.rs</FILE> - <DESC>Run primitive-field-coverage CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: expose primitive input field coverage.</WCTX>
// <CLOG>0.1.0: INIT — add primitive-field-coverage command runner.</CLOG>

use tui_vfx_player::{build_primitive_field_coverage_report, load_descriptor_catalog};

use crate::{
    cls_cli_options::CliOptions, fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
    fnc_report_root::report_root,
};

/// Run the primitive-field-coverage command.
pub fn run_primitive_field_coverage(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let report = build_primitive_field_coverage_report(
        report_root(&options.paths),
        descriptor_load.reports,
        &paths,
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("primitive field coverage report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_primitive_field_coverage.rs</FILE> - <DESC>Run primitive-field-coverage CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
