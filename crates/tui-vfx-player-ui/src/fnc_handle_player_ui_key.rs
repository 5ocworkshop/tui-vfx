// <FILE>crates/tui-vfx-player-ui/src/fnc_handle_player_ui_key.rs</FILE> - <DESC>Handle ratatui player UI keys</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Player UI: route keyboard input across browser, preview, studio, and presentation drawers.</WCTX>
// <CLOG>0.7.0: MINOR — make studio Left/Right adjust the focused slider by one unit.
// 0.6.1: PATCH — mirror the studio-only preview/control layout in mouse hit testing and focus cycling.
// 0.6.0: MINOR — route mouse clicks and drags to visible studio controls.
// 0.5.1: PATCH — remove duplicated unreachable focus-local handlers for global shortcuts.
// 0.5.0: PATCH — route documented playback keys globally and let help-overlay keys execute safe playback toggles instead of only dismissing help.
// 0.4.0: MINOR — add global black-canvas toggle key.
// 0.3.0: MINOR — add Ctrl+Arrow stats drawer toggles without changing playback commands.</CLOG>

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::{PlayerUiApp, PlayerUiCommand, PlayerUiFocus};

/// Handle a crossterm key press. Returns false when the UI should quit.
pub async fn handle_player_ui_key(
    app: &mut PlayerUiApp,
    code: KeyCode,
    viewport_height: usize,
) -> bool {
    handle_player_ui_key_event(
        app,
        KeyEvent::new(code, KeyModifiers::NONE),
        viewport_height,
    )
    .await
}

/// Handle a full crossterm key event, including modifiers for presentation toggles.
pub async fn handle_player_ui_key_event(
    app: &mut PlayerUiApp,
    key: KeyEvent,
    viewport_height: usize,
) -> bool {
    if app.player.show_help {
        return handle_help_overlay_key(app, key.code);
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Right => {
                app.stats_drawer_open = false;
                return true;
            }
            KeyCode::Left => {
                app.stats_drawer_open = true;
                return true;
            }
            _ => {}
        }
    }
    match key.code {
        KeyCode::Char('q') => return false,
        KeyCode::Char('?') => return app.player.apply_command(PlayerUiCommand::Help),
        KeyCode::Tab => {
            app.focus = next_focus(app.focus, app.player.studio);
            return true;
        }
        KeyCode::Char('s') | KeyCode::Char('S') => return toggle_studio(app),
        _ => {}
    }

    if let Some(command) = global_key_command(key.code) {
        return app.player.apply_command(command);
    }

    match app.focus {
        PlayerUiFocus::Browser => handle_browser_key(app, key.code, viewport_height).await,
        PlayerUiFocus::Preview => handle_preview_key(app, key.code),
        PlayerUiFocus::Studio => handle_studio_key(app, key.code),
    }
}

/// Handle a crossterm mouse event. Returns false when the UI should quit.
pub fn handle_player_ui_mouse_event(
    app: &mut PlayerUiApp,
    mouse: MouseEvent,
    frame_area: Rect,
) -> bool {
    if app.player.show_help {
        app.player.show_help = false;
        return true;
    }
    let Some(studio_area) = studio_area(app, frame_area) else {
        return true;
    };
    if !contains(studio_area, mouse.column, mouse.row) {
        return true;
    }
    let Some(index) = studio_control_index_at(app, studio_area, mouse.row) else {
        return true;
    };
    app.focus = PlayerUiFocus::Studio;
    app.studio_control_index = index;
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left)
        | MouseEventKind::Drag(MouseButton::Left)
        | MouseEventKind::Up(MouseButton::Left) => {
            let ratio = horizontal_ratio(studio_area, mouse.column);
            app.player.adjust_studio_control_from_ratio(index, ratio);
        }
        MouseEventKind::ScrollDown => move_studio_cursor(app, 1),
        MouseEventKind::ScrollUp => move_studio_cursor(app, -1),
        _ => {}
    }
    true
}

fn handle_help_overlay_key(app: &mut PlayerUiApp, code: KeyCode) -> bool {
    if matches!(code, KeyCode::Char('q')) {
        return false;
    }

    if matches!(code, KeyCode::Esc | KeyCode::Char('?')) {
        app.player.show_help = false;
        return true;
    }

    if matches!(code, KeyCode::Char('s') | KeyCode::Char('S')) {
        app.player.show_help = false;
        return toggle_studio(app);
    }

    if let Some(command) = global_key_command(code) {
        app.player.show_help = false;
        return app.player.apply_command(command);
    }

    app.player.show_help = false;
    true
}

fn global_key_command(code: KeyCode) -> Option<PlayerUiCommand> {
    match code {
        KeyCode::Char(' ') => Some(PlayerUiCommand::TogglePause),
        KeyCode::Char('r') => Some(PlayerUiCommand::Reset),
        KeyCode::Char('m') | KeyCode::Char('M') => Some(PlayerUiCommand::ToggleMotionDisabled),
        KeyCode::Char('b') | KeyCode::Char('B') => Some(PlayerUiCommand::ToggleBlackCanvas),
        KeyCode::Char('[') => Some(PlayerUiCommand::PreviousPhase),
        KeyCode::Char(']') => Some(PlayerUiCommand::NextPhase),
        KeyCode::Char('t') | KeyCode::Char('T') => Some(PlayerUiCommand::FireTrigger),
        _ => None,
    }
}

fn toggle_studio(app: &mut PlayerUiApp) -> bool {
    app.player.apply_command(PlayerUiCommand::ToggleStudio);
    if app.player.studio {
        app.focus = PlayerUiFocus::Studio;
    } else if app.focus == PlayerUiFocus::Studio {
        app.focus = PlayerUiFocus::Preview;
    }
    true
}

fn next_focus(focus: PlayerUiFocus, studio_enabled: bool) -> PlayerUiFocus {
    match (focus, studio_enabled) {
        (PlayerUiFocus::Preview, true) => PlayerUiFocus::Studio,
        (PlayerUiFocus::Studio, true) => PlayerUiFocus::Preview,
        (PlayerUiFocus::Browser, _) => PlayerUiFocus::Preview,
        (PlayerUiFocus::Preview, false) | (PlayerUiFocus::Studio, false) => PlayerUiFocus::Browser,
    }
}

async fn handle_browser_key(app: &mut PlayerUiApp, code: KeyCode, viewport_height: usize) -> bool {
    match code {
        KeyCode::Char('j') | KeyCode::Down => app.browser.move_down(),
        KeyCode::Char('k') | KeyCode::Up => app.browser.move_up(),
        KeyCode::PageDown | KeyCode::Char('d') => app.browser.page_down(viewport_height),
        KeyCode::PageUp | KeyCode::Char('u') => app.browser.page_up(viewport_height),
        KeyCode::Char('g') | KeyCode::Home => app.browser.set_cursor(0),
        KeyCode::Char('G') | KeyCode::End => {
            let count = app.browser.filtered_count();
            if count > 0 {
                app.browser.set_cursor(count - 1);
            }
        }
        KeyCode::Enter | KeyCode::Right => app.open_focused_entry().await,
        KeyCode::Backspace | KeyCode::Left => app.parent_directory().await,
        KeyCode::Char('.') => app.browser.toggle_hidden(),
        KeyCode::Char('R') => app.refresh_browser().await,
        KeyCode::Esc => app.focus = PlayerUiFocus::Preview,
        _ => {}
    }
    app.sync_cursor();
    true
}

fn handle_preview_key(app: &mut PlayerUiApp, code: KeyCode) -> bool {
    let command = match code {
        KeyCode::Esc => {
            app.focus = PlayerUiFocus::Browser;
            return true;
        }
        KeyCode::Left | KeyCode::Char('h') => PlayerUiCommand::ScrubBackward,
        KeyCode::Right | KeyCode::Char('l') => PlayerUiCommand::ScrubForward,
        _ => PlayerUiCommand::Render,
    };
    app.player.apply_command(command)
}

fn handle_studio_key(app: &mut PlayerUiApp, code: KeyCode) -> bool {
    match code {
        KeyCode::Esc => app.focus = PlayerUiFocus::Preview,
        KeyCode::Char('j') | KeyCode::Down => move_studio_cursor(app, 1),
        KeyCode::Char('k') | KeyCode::Up => move_studio_cursor(app, -1),
        KeyCode::Left | KeyCode::Char('h') => app
            .player
            .adjust_studio_control_by_units(app.studio_control_index, -1),
        KeyCode::Right | KeyCode::Char('l') => app
            .player
            .adjust_studio_control_by_units(app.studio_control_index, 1),
        KeyCode::Enter | KeyCode::Char('e') => app
            .player
            .mutate_studio_control_interactively(app.studio_control_index),
        _ => {}
    }
    true
}

fn move_studio_cursor(app: &mut PlayerUiApp, delta: isize) {
    let count = app.player.controls.len();
    if count == 0 {
        app.studio_control_index = 0;
        return;
    }
    app.studio_control_index =
        (app.studio_control_index as isize + delta).rem_euclid(count as isize) as usize;
}

fn studio_area(app: &PlayerUiApp, frame_area: Rect) -> Option<Rect> {
    if !app.player.studio {
        return None;
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(frame_area);
    let body = if app.stats_drawer_open {
        let drawer_width = crate::col_player_ui_stats_drawer::player_ui_stats_drawer_width(app);
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(drawer_width)])
            .split(chunks[1])[0]
    } else {
        chunks[1]
    };
    Some(
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(66), Constraint::Percentage(34)])
            .split(body)[1],
    )
}

fn contains(area: Rect, column: u16, row: u16) -> bool {
    column >= area.x
        && column < area.x.saturating_add(area.width)
        && row >= area.y
        && row < area.y.saturating_add(area.height)
}

fn studio_control_index_at(app: &PlayerUiApp, area: Rect, row: u16) -> Option<usize> {
    let content_top = area.y.saturating_add(1);
    let content_bottom = area.y.saturating_add(area.height.saturating_sub(1));
    if row < content_top || row >= content_bottom || app.player.controls.is_empty() {
        return None;
    }
    let visible_rows = area.height.saturating_sub(2);
    let visible_control_count = usize::from(visible_rows.max(1)).saturating_div(3).max(1);
    let selected_index = app
        .studio_control_index
        .min(app.player.controls.len().saturating_sub(1));
    let start_index = if app.player.controls.len() <= visible_control_count {
        0
    } else {
        selected_index
            .saturating_sub(visible_control_count / 2)
            .min(
                app.player
                    .controls
                    .len()
                    .saturating_sub(visible_control_count),
            )
    };
    let row_offset = usize::from(row.saturating_sub(content_top));
    (start_index + row_offset < app.player.controls.len()).then_some(start_index + row_offset)
}

fn horizontal_ratio(area: Rect, column: u16) -> f64 {
    let content_left = area.x.saturating_add(1);
    let content_width = area.width.saturating_sub(2).max(1);
    let offset = column.saturating_sub(content_left).min(content_width);
    f64::from(offset) / f64::from(content_width)
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_handle_player_ui_key.rs</FILE> - <DESC>Handle ratatui player UI keys</DESC>
// <VERS>END OF VERSION: 0.7.0</VERS>
