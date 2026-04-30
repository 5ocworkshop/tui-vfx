// <FILE>crates/tui-vfx-player-ui/src/fnc_run_interactive.rs</FILE> - <DESC>Run ratatui visual player interaction loop</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Player UI: run the raw-mode ratatui event loop with browser, playback, and presentation keys.</WCTX>
// <CLOG>0.2.1: PATCH — pass full key events so Ctrl+Arrow drawer toggles are visible.</CLOG>

use std::{
    io::stdout,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::runtime::Builder;

use crate::{PlayerUiApp, PlayerUiState, handle_player_ui_key_event, render_ratatui_ui};

/// Run the raw terminal ratatui player UI loop.
pub fn run_interactive(state: PlayerUiState) -> Result<(), String> {
    let runtime = Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())?;
    runtime.block_on(run_interactive_async(state))
}

async fn run_interactive_async(state: PlayerUiState) -> Result<(), String> {
    let mut app = PlayerUiApp::new(state).await?;
    enable_raw_mode().map_err(|error| error.to_string())?;
    stdout()
        .execute(EnterAlternateScreen)
        .map_err(|error| error.to_string())?;
    let _guard = TerminalGuard;
    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout())).map_err(|error| error.to_string())?;
    let target_frame_time = Duration::from_millis(16);
    let mut last_frame = Instant::now();
    loop {
        let now = Instant::now();
        let delta_ms = now.duration_since(last_frame).as_millis() as u64;
        last_frame = now;
        app.player.advance_time(delta_ms.max(1));
        terminal
            .draw(|frame| render_ratatui_ui(&mut app, frame))
            .map_err(|error| error.to_string())?;
        let viewport_height = terminal
            .size()
            .map_err(|error| error.to_string())?
            .height
            .saturating_sub(4) as usize;
        let frame_elapsed = last_frame.elapsed();
        let poll_timeout = target_frame_time.saturating_sub(frame_elapsed);
        if event::poll(poll_timeout).map_err(|error| error.to_string())?
            && let Event::Key(key) = event::read().map_err(|error| error.to_string())?
            && key.kind == KeyEventKind::Press
            && !handle_player_ui_key_event(&mut app, key, viewport_height).await
        {
            break;
        }
    }
    Ok(())
}

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
    }
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_run_interactive.rs</FILE> - <DESC>Run ratatui visual player interaction loop</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
