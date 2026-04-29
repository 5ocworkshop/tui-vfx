// <FILE>crates/tui-vfx-player/src/lib.rs</FILE> - <DESC>Contract-native skeleton player exports</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: expose minimal contract-native sampled frame API.</WCTX>
// <CLOG>0.1.0: INIT — export RecipePlayer, PlayerSession, frame reports, and catalog loading helpers.</CLOG>

//! Contract-native skeleton player for canonical v3.1 recipes.
//!
//! K0 deliberately renders a tiny supported primitive subset and reports every
//! missing adapter explicitly. It does not depend on the legacy recipes runtime.

pub mod cls_player_error;
pub mod cls_player_frame;
pub mod cls_player_frame_report;
pub mod cls_player_run_report;
pub mod cls_player_sample_request;
pub mod cls_player_session;
pub mod cls_player_status;
pub mod cls_player_summary;
pub mod cls_player_warning;
pub mod cls_recipe_player;
pub mod fnc_apply_graph_effects;
pub mod fnc_build_player_frame;
pub mod fnc_collect_recipe_paths;
pub mod fnc_load_descriptor_catalog;
pub mod fnc_render_hash;
pub mod fnc_render_recipe_file;
pub mod fnc_render_scene;
pub mod fnc_resolve_value_source;

pub use cls_player_error::PlayerError;
pub use cls_player_frame::PlayerFrame;
pub use cls_player_frame_report::PlayerFrameReport;
pub use cls_player_run_report::PlayerRunReport;
pub use cls_player_sample_request::PlayerSampleRequest;
pub use cls_player_session::PlayerSession;
pub use cls_player_status::PlayerStatus;
pub use cls_player_summary::PlayerSummary;
pub use cls_player_warning::PlayerWarning;
pub use cls_recipe_player::RecipePlayer;
pub use fnc_collect_recipe_paths::collect_recipe_paths;
pub use fnc_load_descriptor_catalog::{
    DescriptorPackReport, LoadedDescriptorCatalog, load_descriptor_catalog,
};
pub use fnc_render_recipe_file::render_recipe_file;
pub use fnc_resolve_value_source::resolve_value_source;

// <FILE>crates/tui-vfx-player/src/lib.rs</FILE> - <DESC>Contract-native skeleton player exports</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
