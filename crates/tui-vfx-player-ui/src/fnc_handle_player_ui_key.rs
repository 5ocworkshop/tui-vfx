// <FILE>crates/tui-vfx-player-ui/src/fnc_handle_player_ui_key.rs</FILE> - <DESC>Handle ratatui player UI keys</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: keep demo.rs-style browser navigation and preview controls separated by focus.</WCTX>
// <CLOG>0.1.0: INIT — route crossterm keys into fast-fs browser actions or K0 player commands.</CLOG>

use crossterm::event::KeyCode;

use crate::{PlayerUiApp, PlayerUiCommand, PlayerUiFocus};

/// Handle a crossterm key press. Returns false when the UI should quit.
pub async fn handle_player_ui_key(
    app: &mut PlayerUiApp,
    code: KeyCode,
    viewport_height: usize,
) -> bool {
    match code {
        KeyCode::Char('q') => false,
        KeyCode::Char('?') => app.player.apply_command(PlayerUiCommand::Help),
        KeyCode::Tab => {
            app.focus = match app.focus {
                PlayerUiFocus::Browser => PlayerUiFocus::Preview,
                PlayerUiFocus::Preview => PlayerUiFocus::Browser,
            };
            true
        }
        _ => match app.focus {
            PlayerUiFocus::Browser => handle_browser_key(app, code, viewport_height).await,
            PlayerUiFocus::Preview => handle_preview_key(app, code),
        },
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
        KeyCode::Char(' ') => PlayerUiCommand::TogglePause,
        KeyCode::Char('r') => PlayerUiCommand::Reset,
        KeyCode::Char('m') => PlayerUiCommand::ToggleMotionDisabled,
        KeyCode::Char('[') => PlayerUiCommand::PreviousPhase,
        KeyCode::Char(']') => PlayerUiCommand::NextPhase,
        KeyCode::Left | KeyCode::Char('h') => PlayerUiCommand::ScrubBackward,
        KeyCode::Right | KeyCode::Char('l') => PlayerUiCommand::ScrubForward,
        KeyCode::Char('t') => PlayerUiCommand::FireTrigger,
        _ => PlayerUiCommand::Render,
    };
    app.player.apply_command(command)
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_handle_player_ui_key.rs</FILE> - <DESC>Handle ratatui player UI keys</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
