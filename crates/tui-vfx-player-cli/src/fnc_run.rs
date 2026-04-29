// <FILE>crates/tui-vfx-player-cli/src/fnc_run.rs</FILE> - <DESC>Run player CLI commands</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: execute render-recipe smoke rendering.</WCTX>
// <CLOG>0.1.0: INIT — load catalog, collect recipes, render reports, and print JSON.</CLOG>

use tui_vfx_player::{
    PlayerRunReport, PlayerSampleRequest, RecipePlayer, collect_recipe_paths,
    load_descriptor_catalog, render_recipe_file,
};

use crate::{fnc_parse_cli_options::parse_cli_options, fnc_print_usage::print_usage};

/// Run the player CLI and return the process exit code.
pub fn run(args: impl IntoIterator<Item = String>) -> i32 {
    let mut args = args.into_iter();
    let _program = args.next();
    let Some(command) = args.next() else {
        print_usage();
        return 2;
    };
    if command != "render-recipe" {
        print_usage();
        return 2;
    }
    let options = match parse_cli_options(args) {
        Ok(options) => options,
        Err(error) => return usage_error(error),
    };
    let paths = match collect_recipe_paths(&options.paths, options.recursive) {
        Ok(paths) if !paths.is_empty() => paths,
        Ok(_) => return usage_error("no JSON recipes found".to_string()),
        Err(error) => return usage_error(error),
    };
    let descriptor_load =
        match load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs) {
            Ok(load) => load,
            Err(error) => return usage_error(error),
        };
    let player = RecipePlayer::new(descriptor_load.catalog);
    let request = PlayerSampleRequest {
        phase: options.phase,
        phase_t: options.phase_t,
        loop_t: options.loop_t,
        width: options.width,
        height: options.height,
        ..PlayerSampleRequest::default()
    };
    let frames = paths
        .iter()
        .map(|path| render_recipe_file(&player, path, &request))
        .collect::<Vec<_>>();
    print_report(&options.paths, &frames);
    0
}

fn print_report(paths: &[String], frames: &[tui_vfx_player::PlayerFrameReport]) {
    if frames.len() == 1 {
        println!(
            "{}",
            serde_json::to_string_pretty(&frames[0]).expect("frame report serializes")
        );
    } else {
        let root = if paths.len() == 1 {
            paths[0].clone()
        } else {
            "<multiple>".to_string()
        };
        let report = PlayerRunReport::new(root, frames.to_vec());
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("run report serializes")
        );
    }
}

fn usage_error(error: String) -> i32 {
    eprintln!("Error: {error}");
    print_usage();
    2
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run.rs</FILE> - <DESC>Run player CLI commands</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
