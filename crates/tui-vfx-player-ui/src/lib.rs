// <FILE>crates/tui-vfx-player-ui/src/lib.rs</FILE> - <DESC>Contract-native visual player UI exports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K1: expose ratatui visual shell over K0 player reports.</WCTX>
// <CLOG>0.1.0: INIT — export CLI parsing, app state, command handling, and snapshot rendering.</CLOG>

//! Basic ratatui inspection shell layered on K0 contract-native snapshots.
//!
//! The UI crate owns browser navigation, presentation, and command handling only. Recipe loading,
//! descriptor catalog validation, sampled frames, lifecycle trigger latching,
//! and unsupported diagnostics stay in `tui-vfx-player`. It is not compositor-backed playback yet.

pub mod cls_cli_options;
pub mod cls_player_ui_app;
pub mod cls_player_ui_command;
pub mod cls_player_ui_state;
pub mod fnc_handle_player_ui_key;
pub mod fnc_parse_cli_options;
mod fnc_player_ui_state_support;
pub mod fnc_print_usage;
mod fnc_render_ratatui_help;
pub mod fnc_render_ratatui_ui;
pub mod fnc_render_ui_snapshot;
pub mod fnc_run;
pub mod fnc_run_interactive;
pub mod fnc_run_script;

pub use cls_cli_options::CliOptions;
pub use cls_player_ui_app::{PlayerUiApp, PlayerUiFocus};
pub use cls_player_ui_command::PlayerUiCommand;
pub use cls_player_ui_state::PlayerUiState;
pub use fnc_handle_player_ui_key::handle_player_ui_key;
pub use fnc_parse_cli_options::parse_cli_options;
pub use fnc_render_ratatui_ui::render_ratatui_ui;
pub use fnc_render_ui_snapshot::render_ui_snapshot;
pub use fnc_run::run;
pub use fnc_run_script::run_script;

// <FILE>crates/tui-vfx-player-ui/src/lib.rs</FILE> - <DESC>Contract-native visual player UI exports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
