// <FILE>crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs</FILE> - <DESC>Visual player UI regression tests</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>New kernel player UI: lock one-shot, script, trigger, and rendered diagnostic behavior.</WCTX>
// <CLOG>0.2.1: PATCH — lock browser focus startup and recipe-selection behavior.
// 0.2.0: MINOR — expect styled primitive fixtures to render after adapter burn-down.
// 0.1.0: INIT — add UI smoke tests over the player path.</CLOG>

use std::{fs, path::PathBuf, process::Command};

use crossterm::event::KeyCode;
use ratatui::{Terminal, backend::TestBackend, style::Color};
use tui_vfx_player_ui::{
    CliOptions, PlayerUiApp, PlayerUiFocus, PlayerUiState, handle_player_ui_key, parse_cli_options,
    render_ratatui_ui, render_ui_snapshot, run_script,
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
fn test_fnc_ui_parses_backend_selector() {
    let options = parse_cli_options(vec![
        "--descriptor-pack".to_string(),
        descriptor_pack().display().to_string(),
        "--recipes-root".to_string(),
        debug_recipe_root().display().to_string(),
        "--recipe".to_string(),
        recipe_path("shaders/primitives/shader_linear_gradient_apply_to_both.json")
            .display()
            .to_string(),
        "--backend".to_string(),
        "compositor".to_string(),
        "--once".to_string(),
    ])
    .expect("parse ui options");

    assert_eq!(options.backend, "compositor");
    assert!(options.once);
}

#[test]
fn test_fnc_ui_binary_renders_compositor_backend_once() {
    let output = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-ui"))
        .args([
            "--once",
            "--no-clear",
            "--descriptor-pack",
            descriptor_pack().to_str().expect("descriptor pack path"),
            "--recipes-root",
            debug_recipe_root().to_str().expect("debug recipe root"),
            "--recipe",
            recipe_path("shaders/primitives/shader_linear_gradient_apply_to_both.json")
                .to_str()
                .expect("recipe path"),
            "--backend",
            "compositor",
        ])
        .output()
        .expect("run player ui");

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("backend: compositor"));
    assert!(stdout.contains("backend_hash:"));
    assert!(stdout.contains("\u{1b}[38;2;") || stdout.contains("\u{1b}[48;2;"));
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
fn test_fnc_ui_studio_descriptor_runtime_control_changes_backend_hash() {
    let mut options = options(recipe_path(
        "filters/filter_pill_button_progress_binding.json",
    ));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    options.fail_on_fallback = true;
    options.studio = true;
    let mut state = PlayerUiState::load(&options).expect("load ui state");
    let before_hash = state.last_backend_output.backend_hash;

    let output = run_script(
        &mut state,
        "set effect:filter.pillButton:effectNode:activeColor=#ff0000; render",
        false,
    );

    assert_ne!(before_hash, state.last_backend_output.backend_hash);
    assert!(output.contains("control: colorPicker"));
    assert!(output.contains("target: runtimeInputOverride"));
    assert!(output.contains("runtime: effect:filter.pillButton:effectNode:activeColor"));
}

#[test]
fn test_fnc_ui_studio_source_control_changes_backend_hash() {
    let mut options = options(recipe_path("baseline.json"));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    options.fail_on_fallback = true;
    options.studio = true;
    let mut state = PlayerUiState::load(&options).expect("load ui state");
    let before_hash = state.last_backend_output.backend_hash;

    let output = run_script(
        &mut state,
        "set source:source.card:mainCard:message=SOURCE OVERRIDE; render",
        false,
    );

    assert_ne!(before_hash, state.last_backend_output.backend_hash);
    assert!(output.contains("source:source.card:mainCard:message"));
    assert!(output.contains("control: textInput"));
    assert!(output.contains("current:"));
    assert!(
        state
            .last_backend_output
            .rows
            .iter()
            .any(|row| row.contains("SOURCE OVERRIDE"))
    );
}

#[test]
fn test_fnc_ratatui_studio_keyboard_toggle_uses_effective_current_value() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path("masks/mask_wipe_right_to_left.json"));
    options.studio = true;
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Studio;
    app.studio_control_index = app
        .player
        .controls
        .iter()
        .position(|control| control.id == "effect:mask.wipe:maskWipeEnter:softEdge")
        .expect("soft edge studio control");

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Enter, 10)));
    let first_value = app
        .player
        .controls
        .iter()
        .find(|control| control.id == "effect:mask.wipe:maskWipeEnter:softEdge")
        .and_then(|control| control.current_value.as_ref())
        .expect("first effective control value");
    assert_eq!(first_value["kind"], "boolean");
    assert_eq!(first_value["value"], false);

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Enter, 10)));
    let second_value = app
        .player
        .controls
        .iter()
        .find(|control| control.id == "effect:mask.wipe:maskWipeEnter:softEdge")
        .and_then(|control| control.current_value.as_ref())
        .expect("second effective control value");
    assert_eq!(second_value["kind"], "boolean");
    assert_eq!(second_value["value"], true);
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

#[test]
fn test_fnc_ratatui_renderer_draws_compositor_styled_cells() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path(
        "shaders/primitives/shader_linear_gradient_apply_to_both.json",
    ));
    options.backend = "compositor".to_string();
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let backend = TestBackend::new(100, 30);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");

    assert!(
        terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .any(|cell| cell.fg != Color::Reset || cell.bg != Color::Reset)
    );
}

#[test]
fn test_fnc_ratatui_app_starts_with_browser_focus() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");

    let app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");

    assert_eq!(app.focus, PlayerUiFocus::Browser);
}

#[test]
fn test_fnc_ratatui_browser_load_keeps_browser_focus() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let browser_root = isolated_recipe_browser_root();
    let state = PlayerUiState::load(&options_with_root(
        browser_root.join("baseline.json"),
        browser_root.clone(),
    ))
    .expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");

    runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Enter, 10));

    assert_eq!(app.focus, PlayerUiFocus::Browser);
}

#[test]
fn test_fnc_ratatui_help_overlay_dismisses_non_quit_input_without_state_mutation() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Preview;
    app.player.show_help = true;
    let before_elapsed = app.player.elapsed_ms;
    let before_hash = app.player.last_backend_output.backend_hash;
    let before_message = app.player.message.clone();

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('r'), 10)));

    assert!(!app.player.show_help);
    assert_eq!(app.player.elapsed_ms, before_elapsed);
    assert_eq!(app.player.last_backend_output.backend_hash, before_hash);
    assert_eq!(app.player.message, before_message);

    app.player.show_help = true;
    app.player.advance_time(250);
    assert_eq!(app.player.elapsed_ms, before_elapsed);
    assert_eq!(app.player.last_backend_output.backend_hash, before_hash);
    assert_eq!(app.player.message, before_message);

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Esc, 10)));
    assert!(!app.player.show_help);
    app.player.show_help = true;
    assert!(!runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('q'), 10)));
}

#[test]
fn test_fnc_ui_reset_command_reloads_active_recipe_from_disk() {
    let root = isolated_recipe_browser_root();
    let recipe = root.join("baseline.json");
    let mut state =
        PlayerUiState::load(&options_with_root(recipe.clone(), root)).expect("load ui state");
    let before_hash = state.last_backend_output.backend_hash;
    let text = fs::read_to_string(&recipe).expect("read isolated recipe");
    fs::write(
        &recipe,
        text.replace("BASELINE TEST - No Effects", "RELOADED FROM DISK"),
    )
    .expect("mutate isolated recipe");

    let output = run_script(&mut state, "r", false);

    assert_ne!(before_hash, state.last_backend_output.backend_hash);
    assert!(output.contains("RELOADED FROM DISK"));
    assert!(output.contains("reloaded active recipe JSON from disk"));
}

#[test]
fn test_fnc_ui_compositor_status_reports_source_and_native_evidence() {
    let mut options = options(recipe_path(
        "shaders/primitives/shader_linear_gradient_apply_to_both.json",
    ));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    let state = PlayerUiState::load(&options).expect("load ui state");

    let output = render_ui_snapshot(&state, false);

    assert!(output.contains("composition_mode: native"));
    assert!(output.contains("fallback_used: false"));
    assert!(output.contains("source_render_mode: sourceOnly"));
    assert!(output.contains("native_source_isolated: true"));
    assert!(output.contains("native_lowering_succeeded:"));
    assert!(output.contains("lowered_nodes:"));
}

#[test]
fn test_fnc_ratatui_studio_keyboard_mutation_changes_source_control() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path("baseline.json"));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    options.fail_on_fallback = true;
    options.studio = true;
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Studio;
    let before_hash = app.player.last_backend_output.backend_hash;

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('e'), 10)));

    assert_ne!(before_hash, app.player.last_backend_output.backend_hash);
    assert!(
        app.player
            .last_backend_output
            .rows
            .iter()
            .any(|row| row.contains("STUDIO KEYBOARD OVERRIDE"))
    );
}

fn options(recipe_path: PathBuf) -> CliOptions {
    options_with_root(recipe_path, debug_recipe_root())
}

fn options_with_root(recipe_path: PathBuf, recipes_root: PathBuf) -> CliOptions {
    CliOptions {
        recipe_path,
        recipes_root: Some(recipes_root),
        descriptor_packs: vec![descriptor_pack().display().to_string()],
        descriptor_pack_dirs: vec![],
        backend: "styledCell".to_string(),
        composition_mode: "irResolved".to_string(),
        fail_on_fallback: false,
        studio: false,
        width: None,
        height: None,
        once: false,
        script: None,
        no_clear: true,
    }
}

fn isolated_recipe_browser_root() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "tui-vfx-player-ui-browser-focus-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create isolated recipe browser root");
    fs::copy(recipe_path("baseline.json"), root.join("baseline.json")).expect("copy baseline");
    root
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
// <VERS>END OF VERSION: 0.2.1</VERS>
