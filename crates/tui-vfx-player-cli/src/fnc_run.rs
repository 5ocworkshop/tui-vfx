// <FILE>crates/tui-vfx-player-cli/src/fnc_run.rs</FILE> - <DESC>Run player CLI commands</DESC>
// <VERS>VERSION: 0.10.0</VERS>
// <WCTX>K2.11 schema readiness: dispatch blocker ledger command.</WCTX>
// <CLOG>0.10.0: MINOR — dispatch schema-readiness reports.</CLOG>

use crate::{
    cls_cli_options::CliOptions, fnc_parse_cli_options::parse_cli_options,
    fnc_print_usage::print_usage, fnc_run_control_catalog::run_control_catalog,
    fnc_run_fixture_qc::run_fixture_qc,
    fnc_run_implementation_readiness::run_implementation_readiness,
    fnc_run_inventory_recipes::run_inventory_recipes, fnc_run_migration_gap::run_migration_gap,
    fnc_run_migration_mapping_batch::run_migration_mapping_batch,
    fnc_run_play_backend::run_play_backend,
    fnc_run_primitive_adapter_gap::run_primitive_adapter_gap,
    fnc_run_primitive_field_coverage::run_primitive_field_coverage,
    fnc_run_render_backend::run_render_backend,
    fnc_run_render_backend_timeline::run_render_backend_timeline,
    fnc_run_render_frame::run_render_frame, fnc_run_render_frame_diff::run_render_frame_diff,
    fnc_run_render_ir::run_render_ir, fnc_run_render_recipe::run_render_recipe,
    fnc_run_render_timeline::run_render_timeline, fnc_run_schema_readiness::run_schema_readiness,
    fnc_run_studio_snapshot::run_studio_snapshot,
};

/// Run the player CLI and return the process exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let _program = args.next();
    let Some(command) = args.next() else {
        print_usage();
        return 2;
    };
    if !is_known_command(&command) {
        print_usage();
        return 2;
    }
    let options = match parse_cli_options(args) {
        Ok(options) => options,
        Err(error) => return usage_error(error),
    };
    dispatch_command(&command, options).map_or_else(usage_error, |_| 0)
}

fn is_known_command(command: &str) -> bool {
    matches!(
        command,
        "render-recipe"
            | "render-frame"
            | "render-ir"
            | "render-backend"
            | "render-backend-timeline"
            | "play-backend"
            | "inventory-recipes"
            | "migration-gap"
            | "migration-mapping-batch"
            | "schema-readiness"
            | "primitive-adapter-gap"
            | "primitive-field-coverage"
            | "fixture-qc"
            | "render-timeline"
            | "render-frame-diff"
            | "implementation-readiness"
            | "control-catalog"
            | "studio-snapshot"
    )
}

fn dispatch_command(command: &str, options: CliOptions) -> Result<(), String> {
    match command {
        "render-recipe" => run_render_recipe(options),
        "render-frame" => run_render_frame(options),
        "render-ir" => run_render_ir(options),
        "render-backend" => run_render_backend(options),
        "render-backend-timeline" => run_render_backend_timeline(options),
        "play-backend" => run_play_backend(options),
        "inventory-recipes" => run_inventory_recipes(options),
        "migration-gap" => run_migration_gap(options),
        "migration-mapping-batch" => run_migration_mapping_batch(options),
        "schema-readiness" => run_schema_readiness(options),
        "primitive-adapter-gap" => run_primitive_adapter_gap(options),
        "primitive-field-coverage" => run_primitive_field_coverage(options),
        "fixture-qc" => run_fixture_qc(options),
        "render-timeline" => run_render_timeline(options),
        "render-frame-diff" => run_render_frame_diff(options),
        "implementation-readiness" => run_implementation_readiness(options),
        "control-catalog" => run_control_catalog(options),
        "studio-snapshot" => run_studio_snapshot(options),
        _ => unreachable!("command already validated"),
    }
}

fn usage_error(error: String) -> i32 {
    eprintln!("Error: {error}");
    print_usage();
    2
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run.rs</FILE> - <DESC>Run player CLI commands</DESC>
// <VERS>END OF VERSION: 0.10.0</VERS>
