// <FILE>crates/tui-vfx-player/src/lib.rs</FILE> - <DESC>Contract-native skeleton player exports</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase K2.2: expose stable visual-frame evidence APIs.</WCTX>
// <CLOG>0.5.0: MINOR — export visual-frame report DTOs and render helpers.</CLOG>

//! Contract-native skeleton player for canonical v3.1 recipes.
//!
//! The skeleton player deliberately renders a tiny supported primitive subset and reports every
//! missing adapter explicitly. It does not depend on the legacy recipes runtime.

pub mod cls_player_error;
pub mod cls_player_frame;
pub mod cls_player_frame_report;
pub mod cls_player_inventory_effect;
pub mod cls_player_inventory_recipe;
pub mod cls_player_inventory_report;
pub mod cls_player_inventory_source;
pub mod cls_player_inventory_summary;
pub mod cls_player_migration_gap_report;
pub mod cls_player_run_report;
pub mod cls_player_sample_request;
pub mod cls_player_session;
pub mod cls_player_status;
pub mod cls_player_summary;
pub mod cls_player_visual_cell;
pub mod cls_player_visual_frame;
pub mod cls_player_visual_frame_report;
pub mod cls_player_warning;
pub mod cls_recipe_player;
mod fnc_aggregate_player_inventory_effects;
mod fnc_aggregate_player_inventory_sources;
pub mod fnc_apply_graph_effects;
mod fnc_build_migration_gap_family;
pub mod fnc_build_migration_gap_report;
pub mod fnc_build_player_frame;
mod fnc_build_visual_frame;
mod fnc_classify_debug_recipe_family;
mod fnc_collect_debug_recipe_family_inventory;
mod fnc_collect_descriptor_inventory_ids;
mod fnc_collect_migration_gap_family_names;
pub mod fnc_collect_recipe_paths;
mod fnc_collect_unsupported_effect_ids;
mod fnc_collect_visual_cells;
mod fnc_extract_recipe_inventory_ids;
pub mod fnc_inventory_recipe_file;
pub mod fnc_inventory_recipe_paths;
pub mod fnc_load_descriptor_catalog;
mod fnc_player_inventory_adapter_status;
mod fnc_player_inventory_file_error;
mod fnc_recommend_migration_queue;
pub mod fnc_render_hash;
pub mod fnc_render_recipe_file;
pub mod fnc_render_scene;
pub mod fnc_render_visual_frame_paths;
pub mod fnc_resolve_value_source;
mod fnc_summarize_migration_gap_families;
mod fnc_summarize_player_inventory;
mod fnc_summarize_visual_frames;

pub use cls_player_error::PlayerError;
pub use cls_player_frame::PlayerFrame;
pub use cls_player_frame_report::PlayerFrameReport;
pub use cls_player_inventory_effect::PlayerInventoryEffect;
pub use cls_player_inventory_recipe::PlayerInventoryRecipe;
pub use cls_player_inventory_report::PlayerInventoryReport;
pub use cls_player_inventory_source::PlayerInventorySource;
pub use cls_player_inventory_summary::PlayerInventorySummary;
pub use cls_player_migration_gap_report::{
    PlayerMigrationGapFamily, PlayerMigrationGapReport, PlayerMigrationGapSummary,
    PlayerMigrationQueueItem,
};
pub use cls_player_run_report::PlayerRunReport;
pub use cls_player_sample_request::PlayerSampleRequest;
pub use cls_player_session::PlayerSession;
pub use cls_player_status::PlayerStatus;
pub use cls_player_summary::PlayerSummary;
pub use cls_player_visual_cell::PlayerVisualCell;
pub use cls_player_visual_frame::PlayerVisualFrame;
pub use cls_player_visual_frame_report::PlayerVisualFrameReport;
pub use cls_player_warning::PlayerWarning;
pub use cls_recipe_player::RecipePlayer;
pub use fnc_build_migration_gap_report::build_migration_gap_report;
pub use fnc_collect_recipe_paths::collect_recipe_paths;
pub use fnc_inventory_recipe_file::inventory_recipe_file;
pub use fnc_inventory_recipe_paths::inventory_recipe_paths;
pub use fnc_load_descriptor_catalog::{
    DescriptorPackReport, LoadedDescriptorCatalog, load_descriptor_catalog,
};
pub use fnc_render_recipe_file::render_recipe_file;
pub use fnc_render_visual_frame_paths::render_visual_frame_paths;
pub use fnc_resolve_value_source::resolve_value_source;

// <FILE>crates/tui-vfx-player/src/lib.rs</FILE> - <DESC>Contract-native skeleton player exports</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
