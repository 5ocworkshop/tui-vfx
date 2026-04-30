// <FILE>crates/tui-vfx-player-ui/src/lib.rs</FILE> - <DESC>Contract-native visual player UI exports</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Player UI: expose ratatui visual shell, keyboard routing, snapshot rendering, and presentation helpers.</WCTX>
// <CLOG>0.6.0: MINOR — expose mouse routing for interactive studio controls.
// 0.5.0: MINOR — add focused browser and studio-control render modules.</CLOG>

//! Basic ratatui inspection shell layered on contract-native player snapshots.
//!
//! The UI crate owns browser navigation, presentation, and command handling only. Recipe loading,
//! descriptor catalog validation, sampled frames, lifecycle trigger latching,
//! backend-selected rendering, and unsupported diagnostics stay in `tui-vfx-player`.

pub mod cls_cli_options;
pub mod cls_player_ui_app;
pub mod cls_player_ui_command;
pub mod cls_player_ui_control;
pub mod cls_player_ui_state;
mod cls_player_ui_theme;
mod col_player_ui_recipe_summary;
mod col_player_ui_stats_drawer;
mod fnc_find_startup_recipe_path;
pub mod fnc_handle_player_ui_key;
pub mod fnc_parse_cli_options;
mod fnc_player_ui_state_support;
pub mod fnc_print_usage;
mod fnc_render_player_browser;
mod fnc_render_ratatui_help;
pub mod fnc_render_ratatui_ui;
mod fnc_render_stats_drawer;
mod fnc_render_studio_controls;
pub mod fnc_render_ui_snapshot;
pub mod fnc_run;
pub mod fnc_run_interactive;
pub mod fnc_run_script;

pub use cls_cli_options::CliOptions;
pub use cls_player_ui_app::{PlayerUiApp, PlayerUiFocus};
pub use cls_player_ui_command::PlayerUiCommand;
pub use cls_player_ui_control::PlayerUiControl;
pub use cls_player_ui_state::PlayerUiState;
pub use fnc_handle_player_ui_key::{
    handle_player_ui_key, handle_player_ui_key_event, handle_player_ui_mouse_event,
};
pub use fnc_parse_cli_options::parse_cli_options;
pub use fnc_render_ratatui_ui::render_ratatui_ui;
pub use fnc_render_ui_snapshot::render_ui_snapshot;
pub use fnc_run::run;
pub use fnc_run_script::run_script;

// <FILE>crates/tui-vfx-player-ui/src/lib.rs</FILE> - <DESC>Contract-native visual player UI exports</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
