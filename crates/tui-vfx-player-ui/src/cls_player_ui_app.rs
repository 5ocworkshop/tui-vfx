// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_app.rs</FILE> - <DESC>Ratatui app state with fast-fs browser</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player UI: mirror demo.rs navigation while tracking lightweight presentation state.</WCTX>
// <CLOG>0.2.0: MINOR — add default-open stats drawer presentation state.</CLOG>

use std::path::{Path, PathBuf};

use fast_fs::nav::{ActionResult, Browser, BrowserConfig, KeyInput};
use ratatui::widgets::ListState;

use crate::PlayerUiState;

/// Focus target for demo-style keyboard routing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerUiFocus {
    /// Left recipe browser pane.
    Browser,
    /// Right preview pane.
    Preview,
    /// Descriptor-derived studio controls pane.
    Studio,
}

/// Interactive ratatui app state.
pub struct PlayerUiApp {
    /// Current keyboard focus.
    pub focus: PlayerUiFocus,
    /// fast-fs directory browser for canonical recipe selection.
    pub browser: Browser,
    /// Stateful list cursor for ratatui rendering.
    pub list_state: ListState,
    /// Active player-backed UI state.
    pub player: PlayerUiState,
    /// Browser root selected for the session.
    pub browser_root: PathBuf,
    /// Selected studio control row for keyboard mutation.
    pub studio_control_index: usize,
    /// Whether the rapidly changing stats drawer is visible.
    pub stats_drawer_open: bool,
}

impl PlayerUiApp {
    /// Create the app around an already-loaded recipe.
    pub async fn new(player: PlayerUiState) -> Result<Self, String> {
        let browser_root = player
            .recipes_root
            .clone()
            .unwrap_or_else(|| browser_root_for(&player.recipe_path));
        let browser = Browser::at_path(&browser_root, BrowserConfig::default())
            .await
            .map_err(|error| error.to_string())?;
        let mut list_state = ListState::default();
        list_state.select(Some(browser.cursor()));
        let studio_control_index = default_studio_control_index(&player);
        Ok(Self {
            focus: PlayerUiFocus::Browser,
            browser,
            list_state,
            player,
            browser_root,
            studio_control_index,
            stats_drawer_open: true,
        })
    }

    /// Synchronize the ratatui list cursor with the fast-fs browser cursor.
    pub fn sync_cursor(&mut self) {
        self.list_state.select(Some(self.browser.cursor()));
    }

    /// Select or enter the focused browser item using fast-fs semantics.
    pub async fn open_focused_entry(&mut self) {
        match self.browser.handle_key(KeyInput::Enter).await {
            ActionResult::FileSelected(path) => self.load_selected_file(path),
            ActionResult::DirectoryChanged => self.sync_cursor(),
            ActionResult::Done
            | ActionResult::Unhandled
            | ActionResult::NeedsConfirmation(_)
            | ActionResult::NeedsInput(_)
            | ActionResult::Clipboard(_) => {}
        }
    }

    /// Move to the parent directory using fast-fs semantics.
    pub async fn parent_directory(&mut self) {
        let _ = self.browser.handle_key(KeyInput::Backspace).await;
        self.sync_cursor();
    }

    /// Refresh the current browser directory from disk.
    pub async fn refresh_browser(&mut self) {
        match self.browser.refresh().await {
            Ok(()) => self.player.message = "browser refreshed from disk".to_string(),
            Err(error) => self.player.message = format!("browser refresh failed: {error}"),
        }
        self.sync_cursor();
    }

    fn load_selected_file(&mut self, path: PathBuf) {
        if path.extension().is_none_or(|extension| extension != "json") {
            self.player.message = "selected file is not JSON".to_string();
            return;
        }
        match self.player.load_recipe_path(path) {
            Ok(()) => {}
            Err(error) => self.player.message = format!("recipe load failed: {error}"),
        }
    }
}

fn browser_root_for(recipe_path: &Path) -> PathBuf {
    let mut current = recipe_path.parent();
    while let Some(path) = current {
        if path.ends_with("recipes/v3.1/debug_recipes") {
            return path.to_path_buf();
        }
        current = path.parent();
    }
    recipe_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

// <FILE>crates/tui-vfx-player-ui/src/cls_player_ui_app.rs</FILE> - <DESC>Ratatui app state with fast-fs browser</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>

fn default_studio_control_index(player: &PlayerUiState) -> usize {
    player
        .controls
        .iter()
        .position(|control| control.value_kind == "text" || control.value_kind == "string")
        .unwrap_or(0)
}
