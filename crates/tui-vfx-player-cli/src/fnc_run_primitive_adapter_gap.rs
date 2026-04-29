// <FILE>crates/tui-vfx-player-cli/src/fnc_run_primitive_adapter_gap.rs</FILE> - <DESC>Run primitive-adapter-gap CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: expose focused support classification command.</WCTX>
// <CLOG>0.1.0: INIT — add adapter gap command runner over inventory evidence.</CLOG>

use tui_vfx_player::{RecipePlayer, load_descriptor_catalog, primitive_adapter_gap_paths};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths, fnc_report_root::report_root,
};

/// Run the primitive-adapter-gap command and return a process exit code.
pub fn run_primitive_adapter_gap(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let catalog = descriptor_load.catalog.clone();
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = cli_sample_request(&options);
    let report = primitive_adapter_gap_paths(
        &player,
        &catalog,
        descriptor_load.reports,
        &paths,
        report_root(&options.paths),
        &request,
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("primitive adapter gap report serializes")
    );
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_primitive_adapter_gap.rs</FILE> - <DESC>Run primitive-adapter-gap CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
