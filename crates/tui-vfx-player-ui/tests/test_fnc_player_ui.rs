// <FILE>crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs</FILE> - <DESC>Visual player UI regression tests</DESC>
// <VERS>VERSION: 0.7.2</VERS>
// <WCTX>Player UI: lock playback, drawer, wrapped recipe-summary, and local theme surface behavior for migrated primitive review.</WCTX>
// <CLOG>0.7.2: PATCH — assert phase/sample timing lives in the snapshot title.</CLOG>

use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{Terminal, backend::TestBackend, layout::Rect, style::Color};
use tui_vfx_player_ui::{
    CliOptions, PlayerUiApp, PlayerUiFocus, PlayerUiState, handle_player_ui_key,
    handle_player_ui_key_event, handle_player_ui_mouse_event, parse_cli_options, render_ratatui_ui,
    render_ui_snapshot, run_script,
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
fn test_fnc_ui_tick_uses_recipe_lifecycle_phase_duration() {
    let mut state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");

    let output = run_script(&mut state, "tick", false);

    assert!(output.contains("sample_t: 0.10"));
    assert_eq!(state.phase_t(), 0.1);
}

#[test]
fn test_fnc_ui_native_player_starts_enter_mask_playback() {
    let mut options = options(recipe_path("masks/mask_checkers.json"));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    options.fail_on_fallback = true;
    let mut state = PlayerUiState::load(&options).expect("load ui state");

    let output = run_script(
        &mut state,
        "tick,tick,tick,tick,tick,tick,tick,tick,tick,tick",
        false,
    );

    assert!(output.contains("phase: Enter"));
    assert_eq!(state.phase_t(), 0.5);
    assert_eq!(state.last_backend_output.composition_mode, "native");
    assert!(!state.last_backend_output.fallback_used);
    assert_eq!(
        state
            .last_backend_output
            .letter_cell_evidence
            .letter_cell_count,
        11
    );
    assert_eq!(
        state
            .last_backend_output
            .letter_cell_evidence
            .background_class_counts
            .get("rgba(50,20,50,255)")
            .copied(),
        Some(11)
    );
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
fn test_fnc_ui_script_fires_integer_event_dwell_trigger() {
    let mut state = PlayerUiState::load(&options(recipe_path(
        "event_driven_dwell/integer_binding_demo.json",
    )))
    .expect("load ui state");

    let output = run_script(&mut state, "t", false);

    assert!(output.contains("dwell trigger fired; next sample moved to exit"));
    assert!(output.contains("fired canonical signal `dismissCount`"));
    assert!(output.contains("phase: Exit"));
}

#[test]
fn test_fnc_ui_script_fires_text_event_dwell_trigger() {
    let mut state = PlayerUiState::load(&options(recipe_path(
        "event_driven_dwell/text_binding_demo.json",
    )))
    .expect("load ui state");

    let output = run_script(&mut state, "t", false);

    assert!(output.contains("dwell trigger fired; next sample moved to exit"));
    assert!(output.contains("fired canonical signal `dismissReason`"));
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
fn test_fnc_ratatui_stats_drawer_starts_open_and_owns_rapid_stats() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path(
        "shaders/primitives/shader_linear_gradient_apply_to_both.json",
    ));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let backend = TestBackend::new(140, 32);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal);

    assert!(app.stats_drawer_open);
    assert!(!rows[0].contains("phase="));
    assert!(!rows[0].contains("backend_hash="));
    assert!(rows.iter().any(|row| row.contains(" Stats ")));
    assert!(rows.iter().any(|row| row.contains("phase=Enter")));
    assert!(
        rows.iter().any(
            |row| row.contains("Player Snapshot │ 0.0 FPS (0.0ms) (phase=Enter sample_t=0.00)")
        )
    );
    assert!(rows.iter().any(|row| row.contains("backend_hash=")));
    assert!(rows.iter().any(|row| row.contains("hash=")
        && !row.contains("backend_hash=")
        && !row.contains("non_empty=")));
    assert!(
        rows.iter()
            .any(|row| row.contains("backend_hash=") && !row.contains("native_source="))
    );
}

#[test]
fn test_fnc_ratatui_stats_drawer_uses_eichler_status_colors() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path(
        "shaders/primitives/shader_linear_gradient_apply_to_both.json",
    ));
    options.backend = "compositor".to_string();
    options.composition_mode = "native".to_string();
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let backend = TestBackend::new(140, 32);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");

    assert_eq!(
        cell_foreground_at_text(&terminal, " Stats ", 1),
        Some(Color::Rgb(80, 220, 205))
    );
    assert_eq!(
        cell_foreground_at_text(&terminal, "phase=Enter", "phase=".chars().count()),
        Some(Color::Rgb(80, 220, 205))
    );
    assert_eq!(
        cell_foreground_at_text(&terminal, "sample_t=0.00", 0),
        Some(Color::Rgb(255, 225, 80))
    );
    assert_eq!(
        cell_foreground_at_text(&terminal, "fallback=false", "fallback=".chars().count()),
        Some(Color::Rgb(80, 220, 205))
    );
    assert_eq!(
        cell_foreground_at_text(&terminal, "backend_hash=", 0),
        Some(Color::Rgb(255, 225, 80))
    );
}

#[test]
fn test_fnc_ratatui_preview_summary_wraps_long_recipe_description() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    state.recipe.metadata.description = Some(
        "Long description: Eichler-inspired recipe guidance should wrap inside the summary panel \
         with sunlit atrium context, humane scan rhythm, keyboard-safe metadata, and edge-case \
         text that must remain visible instead of running off the terminal canvas."
            .to_string(),
    );
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.stats_drawer_open = false;
    let backend = TestBackend::new(92, 28);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal);
    let description_rows = rows
        .iter()
        .filter(|row| {
            row.contains("Long description:")
                || row.contains("guidance should wrap")
                || row.contains("sunlit")
                || row.contains("atrium context")
                || row.contains("edge-case text")
        })
        .count();

    assert!(
        description_rows >= 3,
        "description should wrap across multiple visible rows:\n{}",
        rows.join("\n")
    );
    assert!(
        rows.iter().any(|row| row.contains("Player Snapshot")),
        "preview content should remain visible below wrapped metadata:\n{}",
        rows.join("\n")
    );
}

#[test]
fn test_fnc_ratatui_preview_summary_keeps_snapshot_anchor_for_normal_description_wraps() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut short_state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load short ui state");
    short_state.recipe.metadata.description = Some("Short stable preview summary.".to_string());
    let mut wrapped_state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load wrapped ui state");
    wrapped_state.recipe.metadata.description = Some(
        "Normal player guidance should wrap across the fixed summary allowance without moving the \
         playback surface up or down as terminal width changes the line breaks."
            .to_string(),
    );
    let mut short_app = runtime
        .block_on(PlayerUiApp::new(short_state))
        .expect("short player ui app");
    short_app.stats_drawer_open = false;
    let mut wrapped_app = runtime
        .block_on(PlayerUiApp::new(wrapped_state))
        .expect("wrapped player ui app");
    wrapped_app.stats_drawer_open = false;
    let mut short_terminal = Terminal::new(TestBackend::new(92, 28)).expect("short terminal");
    let mut wrapped_terminal = Terminal::new(TestBackend::new(92, 28)).expect("wrapped terminal");

    short_terminal
        .draw(|frame| render_ratatui_ui(&mut short_app, frame))
        .expect("short ratatui draw");
    wrapped_terminal
        .draw(|frame| render_ratatui_ui(&mut wrapped_app, frame))
        .expect("wrapped ratatui draw");

    assert_eq!(
        row_index_containing(&short_terminal, " Player Snapshot "),
        row_index_containing(&wrapped_terminal, " Player Snapshot "),
        "normal wrapped metadata should not bob the playback surface"
    );
}

#[test]
fn test_fnc_ratatui_canvas_and_panels_use_eichler_theme_surfaces() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let backend = TestBackend::new(110, 30);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");

    assert_eq!(
        terminal.backend().buffer()[(0, 0)].bg,
        Color::Rgb(16, 22, 28)
    );
    assert_eq!(
        cell_background_at_text(&terminal, " Browser ", 1),
        Some(Color::Rgb(32, 42, 50))
    );
    assert_eq!(
        cell_background_at_text(&terminal, " Recipe ", 1),
        Some(Color::Rgb(42, 54, 64))
    );
    assert_eq!(
        cell_foreground_at_text(&terminal, "tui-vfx player ", 0),
        Some(Color::Rgb(80, 220, 205))
    );
}

#[test]
fn test_fnc_ratatui_recipe_summary_shows_active_filename() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let mut terminal = Terminal::new(TestBackend::new(120, 30)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(
        rows.contains("file: baseline.json"),
        "recipe summary should identify the active filename:\n{rows}"
    );
}

#[test]
fn test_fnc_ratatui_stats_drawer_width_includes_hash_padding() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let backend = TestBackend::new(140, 32);
    let mut terminal = Terminal::new(backend).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal);
    let stats_left_edge = rows
        .iter()
        .find_map(|row| {
            row.find(" Stats ")
                .map(|byte_index| row[..byte_index].chars().count())
        })
        .expect("stats drawer title");
    let expected_width = expected_stats_lines(&app)
        .iter()
        .map(|line| line.chars().count())
        .max()
        .expect("stats lines")
        + 6;

    assert_eq!(140 - stats_left_edge + 1, expected_width);
}

#[test]
fn test_fnc_ratatui_stats_drawer_ctrl_arrows_toggle_without_playback_change() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let before_phase_t = app.player.phase_t();
    let before_hash = app.player.last_backend_output.backend_hash;

    assert!(runtime.block_on(handle_player_ui_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Right, KeyModifiers::CONTROL),
        10,
    )));
    assert!(!app.stats_drawer_open);
    assert_eq!(app.player.phase_t(), before_phase_t);
    assert_eq!(app.player.last_backend_output.backend_hash, before_hash);

    assert!(runtime.block_on(handle_player_ui_key_event(
        &mut app,
        KeyEvent::new(KeyCode::Left, KeyModifiers::CONTROL),
        10,
    )));
    assert!(app.stats_drawer_open);
    assert_eq!(app.player.phase_t(), before_phase_t);
    assert_eq!(app.player.last_backend_output.backend_hash, before_hash);
}

#[test]
fn test_fnc_ui_script_toggles_black_canvas_command() {
    let mut state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");

    let output = run_script(&mut state, "b,b", false);

    assert!(output.contains("black_canvas: true"));
    assert!(output.contains("black_canvas: false"));
    assert!(!state.black_canvas_enabled());
}

#[test]
fn test_fnc_ratatui_b_key_toggles_black_canvas_without_playback_change() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let before_phase_t = app.player.phase_t();
    let before_hash = app.player.last_backend_output.backend_hash;

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('b'), 10)));

    assert!(app.player.black_canvas_enabled());
    assert_eq!(app.player.phase_t(), before_phase_t);
    assert_eq!(app.player.last_backend_output.backend_hash, before_hash);
}

#[test]
fn test_fnc_ratatui_documented_playback_keys_work_outside_preview_focus() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Browser;

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char(' '), 10)));
    assert!(
        app.player.paused,
        "Space should pause even when browser has focus"
    );

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char(']'), 10)));
    assert_eq!(
        format!("{:?}", app.player.phase()),
        "Dwell",
        "] should advance phase even when browser has focus"
    );

    app.focus = PlayerUiFocus::Studio;
    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('['), 10)));
    assert_eq!(
        format!("{:?}", app.player.phase()),
        "Enter",
        "[ should rewind phase even when studio has focus"
    );
}

#[test]
fn test_fnc_ratatui_help_overlay_dispatches_documented_playback_commands() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.player.show_help = true;

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('b'), 10)));

    assert!(!app.player.show_help);
    assert!(
        app.player.black_canvas_enabled(),
        "b from the help overlay should toggle the black canvas instead of only dismissing help"
    );
}

#[test]
fn test_fnc_ratatui_s_key_toggles_studio_pane_at_runtime() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    assert!(!app.player.studio);

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('s'), 10)));

    assert!(app.player.studio);
    assert_eq!(app.player.message, "studio controls enabled");
    let mut terminal = Terminal::new(TestBackend::new(150, 30)).expect("terminal");
    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("studio ratatui draw");
    assert!(
        terminal_rows(&terminal)
            .join("\n")
            .contains("Studio controls")
    );

    app.focus = PlayerUiFocus::Preview;
    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Tab, 10)));
    assert_eq!(app.focus, PlayerUiFocus::Studio);

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('s'), 10)));

    assert!(!app.player.studio);
    assert_eq!(app.focus, PlayerUiFocus::Preview);
    assert_eq!(app.player.message, "studio controls hidden");
}

#[test]
fn test_fnc_ratatui_help_overlay_s_toggles_studio_pane() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.player.show_help = true;

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('s'), 10)));

    assert!(!app.player.show_help);
    assert!(app.player.studio);
    assert_eq!(app.player.message, "studio controls enabled");
}

#[test]
fn test_fnc_ratatui_help_overlay_r_reload_is_documented_and_shows_notice() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let root = isolated_recipe_browser_root();
    let recipe = root.join("baseline.json");
    let state =
        PlayerUiState::load(&options_with_root(recipe.clone(), root)).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.player.show_help = true;
    let text = fs::read_to_string(&recipe).expect("read isolated recipe");
    fs::write(
        &recipe,
        text.replace("BASELINE TEST - No Effects", "RELOADED FROM HELP R"),
    )
    .expect("mutate isolated recipe");

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('r'), 10)));

    assert!(!app.player.show_help);
    assert!(app.player.message.contains("reloaded active recipe JSON"));

    let mut terminal = Terminal::new(TestBackend::new(150, 30)).expect("terminal");
    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(rows.contains("RELOADED FROM HELP R"));
    assert!(rows.contains("notice=reloaded active recipe JSON"));
}

#[test]
fn test_fnc_ratatui_help_lists_black_canvas_toggle() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.player.show_help = true;
    let mut terminal = Terminal::new(TestBackend::new(110, 30)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(
        rows.contains("b black"),
        "help overlay should list the black-canvas toggle"
    );
    assert!(
        rows.contains("s studio"),
        "help overlay should list the runtime studio toggle"
    );
    assert!(
        rows.contains("r reload"),
        "help overlay should document that r reloads from disk"
    );
    assert!(
        rows.contains("Studio: click selects"),
        "help overlay should document studio mouse controls"
    );
    assert!(
        rows.contains("Space, r, b"),
        "help overlay should document r as an active help-overlay command"
    );
}

#[test]
fn test_fnc_ratatui_black_canvas_changes_playback_virtual_canvas_only() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let mut terminal = Terminal::new(TestBackend::new(110, 30)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("default ratatui draw");
    assert_eq!(
        terminal.backend().buffer()[(0, 0)].bg,
        Color::Rgb(16, 22, 28)
    );
    let default_virtual_canvas = cell_background_below_text(&terminal, " Player Snapshot ", 4, 3);
    assert_ne!(default_virtual_canvas, Some(Color::Black));

    app.player
        .apply_command(tui_vfx_player_ui::PlayerUiCommand::ToggleBlackCanvas);
    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("black canvas ratatui draw");

    assert_eq!(
        terminal.backend().buffer()[(0, 0)].bg,
        Color::Rgb(16, 22, 28),
        "black-canvas mode must not recolor the whole player shell"
    );
    assert_eq!(
        { cell_background_below_text(&terminal, " Player Snapshot ", 4, 3) },
        Some(Color::Black),
        "black-canvas mode should recolor the playback virtual canvas"
    );
}

#[test]
fn test_fnc_ratatui_b_key_shows_notice_without_recoloring_ui_panels() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    let mut terminal = Terminal::new(TestBackend::new(150, 30)).expect("terminal");

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('b'), 10)));
    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("black canvas ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(rows.contains("notice=black player canvas enabled"));
    assert_eq!(
        terminal.backend().buffer()[(0, 0)].bg,
        Color::Rgb(16, 22, 28),
        "black-canvas mode must leave the player shell canvas alone"
    );
    assert_eq!(
        cell_background_below_text(&terminal, " Player Snapshot ", 4, 3),
        Some(Color::Black),
        "black-canvas mode should affect the playback virtual canvas"
    );
    assert_ne!(
        cell_background_at_text(&terminal, " Browser ", 1),
        Some(Color::Black),
        "black-canvas mode is intentionally not applied to browser chrome"
    );
}

#[test]
fn test_fnc_ratatui_snapshot_title_shows_fps_in_demo_location() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    state.advance_time(16);
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.stats_drawer_open = false;
    let mut terminal = Terminal::new(TestBackend::new(110, 30)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal);

    assert!(
        rows.iter()
            .any(|row| row.contains("Player Snapshot │ 62.5 FPS (16.0ms)")),
        "snapshot title should put FPS immediately after title like demo.rs:
{}",
        rows.join(
            "
"
        )
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

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('x'), 10)));

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
fn test_fnc_ratatui_help_overlay_does_not_scrub_with_arrow_keys() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.player.show_help = true;
    app.player
        .apply_command(tui_vfx_player_ui::PlayerUiCommand::ScrubForward);
    let before_phase_t = app.player.phase_t();
    let before_message = app.player.message.clone();
    app.player.show_help = true;

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Left, 10)));

    assert!(!app.player.show_help);
    assert_eq!(app.player.phase_t(), before_phase_t);
    assert_eq!(app.player.message, before_message);
}

#[test]
fn test_fnc_ratatui_r_key_reloads_globally_and_shows_notice() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let root = isolated_recipe_browser_root();
    let recipe = root.join("baseline.json");
    let state =
        PlayerUiState::load(&options_with_root(recipe.clone(), root)).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Browser;
    let before_hash = app.player.last_backend_output.backend_hash;
    let text = fs::read_to_string(&recipe).expect("read isolated recipe");
    fs::write(
        &recipe,
        text.replace("BASELINE TEST - No Effects", "RELOADED FROM GLOBAL R"),
    )
    .expect("mutate isolated recipe");

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Char('r'), 10)));

    assert_ne!(before_hash, app.player.last_backend_output.backend_hash);
    assert!(app.player.message.contains("reloaded active recipe JSON"));

    let mut terminal = Terminal::new(TestBackend::new(150, 30)).expect("terminal");
    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(rows.contains("RELOADED FROM GLOBAL R"));
    assert!(rows.contains("notice=reloaded active recipe JSON"));
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
fn test_fnc_ui_snapshot_command_summary_calls_r_reload() {
    let state = PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");

    let output = render_ui_snapshot(&state, false);

    assert!(output.contains("r reload"));
    assert!(!output.contains("r reset"));
}

#[test]
fn test_fnc_ui_script_toggles_studio_command() {
    let mut state =
        PlayerUiState::load(&options(recipe_path("baseline.json"))).expect("load ui state");

    let output = run_script(&mut state, "s", false);

    assert!(state.studio);
    assert!(output.contains("Controls"));
    assert!(output.contains("studio controls enabled"));
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

#[test]
fn test_fnc_ratatui_studio_left_right_adjust_selected_slider_by_one() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path("baseline.json"));
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
        .position(|control| {
            control.source_instance_id.as_deref() == Some("mainCard")
                && control.input_name == "width"
        })
        .expect("source width control");

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Right, 10)));
    assert_eq!(
        app.player.controls[app.studio_control_index]
            .current_value
            .as_ref()
            .and_then(|value| value.get("value"))
            .and_then(|value| value.as_i64()),
        Some(36)
    );

    assert!(runtime.block_on(handle_player_ui_key(&mut app, KeyCode::Left, 10)));
    assert_eq!(
        app.player.controls[app.studio_control_index]
            .current_value
            .as_ref()
            .and_then(|value| value.get("value"))
            .and_then(|value| value.as_i64()),
        Some(35)
    );
}

#[test]
fn test_fnc_ratatui_studio_controls_draw_ranged_numeric_slider() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path(
        "filters/filter_kitt_scanner_progress_binding.json",
    ));
    options.studio = true;
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Studio;
    app.stats_drawer_open = false;
    app.studio_control_index = app
        .player
        .controls
        .iter()
        .position(|control| control.range.is_some())
        .expect("ranged studio control");
    let control_label = app.player.controls[app.studio_control_index].label.clone();
    let mut terminal = Terminal::new(TestBackend::new(150, 30)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(
        rows.contains(&control_label),
        "selected ranged control should be visible:\n{rows}"
    );
    assert!(
        rows.contains("range"),
        "slider should show min/max context:\n{rows}"
    );
    assert!(
        rows.contains("━") && rows.contains("●"),
        "slider should draw a filled gauge with thumb:\n{rows}"
    );
}

#[test]
fn test_fnc_ratatui_studio_controls_draw_toggle_enum_and_color_affordances() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut tint_options = options(recipe_path("filters/filter_tint.json"));
    tint_options.studio = true;
    let state = PlayerUiState::load(&tint_options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Studio;
    app.stats_drawer_open = false;
    let mut terminal = Terminal::new(TestBackend::new(170, 34)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(
        rows.contains("opts"),
        "enum controls should render compact options:\n{rows}"
    );
    assert!(
        rows.contains("▣") && rows.contains("#"),
        "color controls should render a swatch and color label:\n{rows}"
    );

    let mut boolean_options = options(recipe_path("masks/mask_wipe_right_to_left.json"));
    boolean_options.studio = true;
    let boolean_state = PlayerUiState::load(&boolean_options).expect("load boolean ui state");
    let mut boolean_app = runtime
        .block_on(PlayerUiApp::new(boolean_state))
        .expect("boolean player ui app");
    boolean_app.focus = PlayerUiFocus::Studio;
    boolean_app.stats_drawer_open = false;
    boolean_app.studio_control_index = boolean_app
        .player
        .controls
        .iter()
        .position(|control| control.value_kind == "boolean")
        .expect("boolean studio control");
    let mut boolean_terminal = Terminal::new(TestBackend::new(150, 30)).expect("boolean terminal");

    boolean_terminal
        .draw(|frame| render_ratatui_ui(&mut boolean_app, frame))
        .expect("boolean ratatui draw");
    let boolean_rows = terminal_rows(&boolean_terminal).join("\n");

    assert!(
        boolean_rows.contains("toggle") && boolean_rows.contains("ON"),
        "boolean controls should render a toggle state:\n{boolean_rows}"
    );
}

#[test]
fn test_fnc_ratatui_studio_controls_keep_selected_row_visible() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path("filters/filter_tint.json"));
    options.studio = true;
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Studio;
    app.stats_drawer_open = false;
    app.studio_control_index = app.player.controls.len().saturating_sub(1);
    let selected_label = app.player.controls[app.studio_control_index].label.clone();
    let mut terminal = Terminal::new(TestBackend::new(120, 14)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(
        rows.contains(&selected_label),
        "selected studio control should be scrolled into view:\n{rows}"
    );
    assert!(
        rows.contains("▶"),
        "selected row marker should remain visible:\n{rows}"
    );
}

#[test]
fn test_fnc_ratatui_mouse_click_adjusts_visible_studio_slider() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path(
        "filters/filter_kitt_scanner_progress_binding.json",
    ));
    options.studio = true;
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Preview;
    app.stats_drawer_open = false;
    app.studio_control_index = 0;

    assert!(handle_player_ui_mouse_event(
        &mut app,
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 146,
            row: 3,
            modifiers: KeyModifiers::NONE,
        },
        Rect::new(0, 0, 150, 30),
    ));

    assert_eq!(app.focus, PlayerUiFocus::Studio);
    assert_eq!(app.studio_control_index, 1);
    assert!(
        app.player.message.contains("mouse-adjusted studio control"),
        "message was: {}",
        app.player.message
    );
    let adjusted_value = app.player.controls[1]
        .current_value
        .as_ref()
        .and_then(|value| value.get("value"))
        .and_then(|value| value.as_f64())
        .expect("numeric control value");
    assert!(
        adjusted_value > 0.8,
        "right-edge mouse click should move slider toward max, got {adjusted_value}"
    );
}

#[test]
fn test_fnc_ratatui_mouse_click_toggles_visible_studio_boolean() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path(
        "filters/filter_kitt_scanner_progress_binding.json",
    ));
    options.studio = true;
    let state = PlayerUiState::load(&options).expect("load ui state");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Preview;
    app.stats_drawer_open = false;
    app.studio_control_index = 0;

    assert!(handle_player_ui_mouse_event(
        &mut app,
        MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 114,
            row: 6,
            modifiers: KeyModifiers::NONE,
        },
        Rect::new(0, 0, 150, 30),
    ));

    assert_eq!(app.focus, PlayerUiFocus::Studio);
    assert_eq!(app.studio_control_index, 4);
    assert!(
        app.player.message.contains("PowerlineMode")
            || app
                .player
                .message
                .contains("effect:filter.kittScanner:effectNode:powerlineMode"),
        "message was: {}",
        app.player.message
    );
}

#[test]
fn test_fnc_player_ui_source_width_control_expands_studio_preview_canvas() {
    let mut options = options(recipe_path("baseline.json"));
    options.studio = true;
    let mut state = PlayerUiState::load(&options).expect("load ui state");

    state
        .set_control_value(
            "source:source.card:mainCard:width",
            tui_vfx_contract::Value::Integer(80),
        )
        .expect("set source width control");

    assert!(
        state
            .last_backend_output
            .rows
            .iter()
            .any(|row| row.len() >= 80),
        "studio source width changes should not be clipped by the original scene canvas"
    );
}

#[test]
fn test_fnc_ratatui_studio_layout_gives_expanded_preview_room() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let mut options = options(recipe_path("baseline.json"));
    options.studio = true;
    let mut state = PlayerUiState::load(&options).expect("load ui state");
    state
        .set_control_value(
            "source:source.card:mainCard:width",
            tui_vfx_contract::Value::Integer(80),
        )
        .expect("set source width control");
    let mut app = runtime
        .block_on(PlayerUiApp::new(state))
        .expect("player ui app");
    app.focus = PlayerUiFocus::Studio;
    app.stats_drawer_open = false;
    let mut terminal = Terminal::new(TestBackend::new(150, 30)).expect("terminal");

    terminal
        .draw(|frame| render_ratatui_ui(&mut app, frame))
        .expect("ratatui draw");
    let rows = terminal_rows(&terminal).join("\n");

    assert!(
        !rows.contains(" Browser "),
        "studio mode should trade the browser pane for wider preview/control panes:\n{rows}"
    );
    assert!(
        rows.contains("╮"),
        "expanded source card should have enough preview room to show its right border:\n{rows}"
    );
}

fn row_index_containing(terminal: &Terminal<TestBackend>, needle: &str) -> Option<usize> {
    terminal_rows(terminal)
        .iter()
        .position(|row| row.contains(needle))
}

fn terminal_rows(terminal: &Terminal<TestBackend>) -> Vec<String> {
    let buffer = terminal.backend().buffer();
    let area = *buffer.area();
    buffer
        .content()
        .chunks(area.width as usize)
        .map(|cells| cells.iter().map(|cell| cell.symbol()).collect::<String>())
        .collect()
}

fn cell_foreground_at_text(
    terminal: &Terminal<TestBackend>,
    needle: &str,
    character_offset: usize,
) -> Option<Color> {
    let rows = terminal_rows(terminal);
    let row_index = rows.iter().position(|row| row.contains(needle))?;
    let byte_index = rows[row_index].find(needle)?;
    let column = rows[row_index][..byte_index].chars().count() + character_offset;
    let buffer = terminal.backend().buffer();
    let area = *buffer.area();
    Some(buffer[(column as u16, area.y + row_index as u16)].fg)
}

fn cell_background_at_text(
    terminal: &Terminal<TestBackend>,
    needle: &str,
    character_offset: usize,
) -> Option<Color> {
    let rows = terminal_rows(terminal);
    let row_index = rows.iter().position(|row| row.contains(needle))?;
    let byte_index = rows[row_index].find(needle)?;
    let column = rows[row_index][..byte_index].chars().count() + character_offset;
    let buffer = terminal.backend().buffer();
    let area = *buffer.area();
    Some(buffer[(column as u16, area.y + row_index as u16)].bg)
}

fn cell_background_below_text(
    terminal: &Terminal<TestBackend>,
    needle: &str,
    row_offset: usize,
    character_offset: usize,
) -> Option<Color> {
    let rows = terminal_rows(terminal);
    let title_row = rows.iter().position(|row| row.contains(needle))?;
    let row_index = title_row + row_offset;
    let byte_index = rows[title_row].find(needle)?;
    let column = rows[title_row][..byte_index].chars().count() + character_offset;
    let buffer = terminal.backend().buffer();
    let area = *buffer.area();
    Some(buffer[(column as u16, area.y + row_index as u16)].bg)
}

fn expected_stats_lines(app: &PlayerUiApp) -> Vec<String> {
    let report = app.player.report();
    vec![
        format!(
            "loop_t={} elapsed={}ms",
            app.player
                .loop_t()
                .map(|value| format!("{value:.2}"))
                .unwrap_or_else(|| "none".to_string()),
            app.player.elapsed_ms
        ),
        format!("hash={}", expected_hash_text(report.render_hash)),
        format!("non_empty={}", report.non_empty_cells),
        format!(
            "backend={} mode={}",
            app.player.last_backend_output.backend, app.player.last_backend_output.composition_mode
        ),
        format!(
            "source={} fallback={}",
            app.player.last_backend_output.source_render_mode,
            app.player.last_backend_output.fallback_used
        ),
        format!(
            "native_source={}",
            app.player.last_backend_output.native_source_isolated
        ),
        format!(
            "backend_hash={}",
            expected_hash_text(app.player.last_backend_output.backend_hash)
        ),
    ]
}

fn expected_hash_text(value: u64) -> String {
    format!("{value:>20}")
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
    static NEXT_ROOT_ID: AtomicUsize = AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "tui-vfx-player-ui-browser-focus-{}-{}",
        std::process::id(),
        NEXT_ROOT_ID.fetch_add(1, Ordering::Relaxed)
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
// <VERS>END OF VERSION: 0.7.2</VERS>
