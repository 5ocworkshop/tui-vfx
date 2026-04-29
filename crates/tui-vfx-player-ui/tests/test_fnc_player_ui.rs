// <FILE>crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs</FILE> - <DESC>Visual player UI regression tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel player UI: lock one-shot, script, trigger, and rendered diagnostic behavior.</WCTX>
// <CLOG>0.2.0: MINOR — expect styled primitive fixtures to render after adapter burn-down.
// 0.1.0: INIT — add UI smoke tests over the player path.</CLOG>

use std::{path::PathBuf, process::Command};

use ratatui::{Terminal, backend::TestBackend};
use tui_vfx_player_ui::{
    CliOptions, PlayerUiApp, PlayerUiState, parse_cli_options, render_ratatui_ui, run_script,
};

#[test]
fn test_fnc_ui_parses_recipe_root_and_startup_recipe_options() {
    let options = parse_cli_options(vec![
        "--descriptor-pack".to_string(),
        descriptor_pack().display().to_string(),
        "--recipes-root".to_string(),
        debug_recipe_root().display().to_string(),
        "--recipe".to_string(),
        recipe_path("baseline.json").display().to_string(),
    ])
    .expect("parse ui options");

    assert_eq!(options.recipe_path, recipe_path("baseline.json"));
    assert_eq!(options.recipes_root, Some(debug_recipe_root()));
    assert!(options.descriptor_packs[0].ends_with("descriptors/v3.1/packs/primitive.json"));
}

#[test]
fn test_fnc_ui_binary_renders_baseline_once() {
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-ui"))
        .args([
            "--once",
            "--no-clear",
            "--descriptor-pack",
            descriptor_pack().to_str().expect("descriptor pack path"),
            "--recipes-root",
            debug_recipe_root().to_str().expect("debug recipe root"),
            "--recipe",
            recipe_path("baseline.json").to_str().expect("recipe path"),
        ])
        .output()
        .expect("run player ui");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("tui-vfx contract-native player UI"));
    assert!(stdout.contains("BASELINE TEST - No Effects"));
    assert!(stdout.contains("render_hash:"));
    assert!(stdout.contains("non_empty_cells:"));
}

#[test]
fn test_fnc_ui_script_fires_event_dwell_trigger() {
    let mut state = PlayerUiState::load(&options(recipe_path(
        "event_driven_dwell/bool_binding_demo.json",
    )))
    .expect("load ui state");

    let output = run_script(&mut state, "t", false);

    assert!(output.contains("dwell trigger fired; next sample moved to exit"));
    assert!(output.contains("dwell: terminated by canonical trigger"));
    assert!(output.contains("phase: Exit"));
}

#[test]
fn test_fnc_ui_reports_styled_primitives_visibly() {
    let mut state = PlayerUiState::load(&options(recipe_path(
        "shaders/primitives/shader_linear_gradient.json",
    )))
    .expect("load ui state");

    let output = run_script(&mut state, "render", false);

    assert!(output.contains("status: Rendered"));
    assert!(output.contains("render_hash:"));
    assert!(output.contains("Shader: Linear Gradient"));
}

#[test]
fn test_fnc_ratatui_renderer_draws_without_terminal_io() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
}

fn options(recipe_path: PathBuf) -> CliOptions {
    CliOptions {
        recipe_path,
        recipes_root: Some(debug_recipe_root()),
        descriptor_packs: vec![descriptor_pack().display().to_string()],
        descriptor_pack_dirs: vec![],
        width: None,
        height: None,
        once: false,
        script: None,
        no_clear: true,
    }
}

fn descriptor_pack() -> PathBuf {
    workspace_root().join("descriptors/v3.1/packs/primitive.json")
}

fn recipe_path(relative: &str) -> PathBuf {
    debug_recipe_root().join(relative)
}

fn debug_recipe_root() -> PathBuf {
    recipe_repo_root().join("recipes/v3.1/debug_recipes")
}

fn recipe_repo_root() -> PathBuf {
    if let Ok(path) = std::env::var("RECIPE_REPO") {
        return PathBuf::from(path);
    }

    workspace_root()
        .parent()
        .expect("workspace parent")
        .join("tui-vfx-recipes")
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

// <FILE>crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs</FILE> - <DESC>Visual player UI regression tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
