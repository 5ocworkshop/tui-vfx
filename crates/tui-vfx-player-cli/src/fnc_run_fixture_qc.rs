// <FILE>crates/tui-vfx-player-cli/src/fnc_run_fixture_qc.rs</FILE> - <DESC>Run fixture-qc CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: expose composed fixture QC report.</WCTX>
// <CLOG>0.1.0: INIT — add fixture-qc command runner over existing player reports.</CLOG>

use tui_vfx_player::{RecipePlayer, build_fixture_qc_report, load_descriptor_catalog};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths, fnc_report_root::report_root,
};

/// Run the fixture-qc command.
pub fn run_fixture_qc(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let catalog = descriptor_load.catalog.clone();
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let report = build_fixture_qc_report(
        &player,
        &catalog,
        descriptor_load.reports,
        &paths,
        report_root(&options.paths),
        &request,
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("fixture QC report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_fixture_qc.rs</FILE> - <DESC>Run fixture-qc CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
