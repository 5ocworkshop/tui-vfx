// <FILE>crates/tui-vfx-player-cli/src/fnc_validate_migration_gap_options.rs</FILE> - <DESC>Validate migration-gap CLI options</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep migration-gap command surface strict.</WCTX>
// <CLOG>0.1.0: INIT — reject render/inventory-only options for migration-gap.</CLOG>

use crate::cls_cli_options::CliOptions;

/// Reject options that do not belong to the migration-gap command.
pub fn validate_migration_gap_options(options: &CliOptions) -> Result<(), String> {
    if !options.paths.is_empty() {
        return Err("migration-gap does not accept recipe paths".to_string());
    }
    if options.recursive {
        return Err("migration-gap does not accept --recursive".to_string());
    }
    if options.width.is_some() || options.height.is_some() {
        return Err("migration-gap does not accept frame size options".to_string());
    }
    if options.loop_t.is_some() {
        return Err("migration-gap does not accept --loop-t".to_string());
    }
    let defaults = CliOptions::default();
    if options.phase != defaults.phase || (options.phase_t - defaults.phase_t).abs() > f64::EPSILON
    {
        return Err("migration-gap does not accept lifecycle sampling options".to_string());
    }
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_validate_migration_gap_options.rs</FILE> - <DESC>Validate migration-gap CLI options</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
