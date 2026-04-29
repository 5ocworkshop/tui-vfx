// <FILE>crates/tui-vfx-player-cli/src/fnc_run_migration_gap.rs</FILE> - <DESC>Run migration-gap CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate migration-gap command execution.</WCTX>
// <CLOG>0.1.0: INIT — split migration-gap runner from top-level dispatch.</CLOG>

use tui_vfx_player::{build_migration_gap_report, load_descriptor_catalog};

use crate::{
    cls_cli_options::CliOptions, fnc_validate_migration_gap_options::validate_migration_gap_options,
};

/// Run the migration-gap command and return a process exit code.
pub fn run_migration_gap(options: CliOptions) -> Result<(), String> {
    validate_migration_gap_options(&options)?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let legacy_root = required_option(options.legacy_root.as_deref(), "--legacy-root")?;
    let v31_root = required_option(options.v31_root.as_deref(), "--v31-root")?;
    let report = build_migration_gap_report(
        std::path::Path::new(legacy_root),
        std::path::Path::new(v31_root),
        descriptor_load.reports,
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("migration gap report serializes")
    );
    Ok(())
}

fn required_option<'a>(value: Option<&'a str>, option: &str) -> Result<&'a str, String> {
    value.ok_or_else(|| format!("missing required option `{option}`"))
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_migration_gap.rs</FILE> - <DESC>Run migration-gap CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
