// <FILE>crates/tui-vfx-player/src/lib.rs</FILE> - <DESC>Contract-native skeleton player exports</DESC>
// <VERS>VERSION: 0.7.1</VERS>
// <WCTX>Player crate de-slop: keep export metadata compact and current.</WCTX>
// <CLOG>0.7.1: PATCH — collapse historical export metadata into latest-change context.</CLOG>

//! Contract-native skeleton player for canonical v3.1 recipes.
//!
//! The skeleton player deliberately renders a tiny supported primitive subset and reports every
//! missing adapter explicitly. It does not depend on the legacy recipes runtime.

pub mod cls_player_error;
pub mod cls_player_frame;
pub mod cls_player_frame_diff_report;
pub mod cls_player_frame_report;
pub mod cls_player_frame_timeline_report;
pub mod cls_player_inventory_effect;
pub mod cls_player_inventory_recipe;
pub mod cls_player_inventory_report;
pub mod cls_player_inventory_source;
pub mod cls_player_inventory_summary;
pub mod cls_player_migration_gap_report;
pub mod cls_player_primitive_adapter_gap_entry;
pub mod cls_player_primitive_adapter_gap_report;
pub mod cls_player_primitive_adapter_gap_summary;
pub mod cls_player_primitive_field_coverage;
pub mod cls_player_run_report;
pub mod cls_player_sample_request;
pub mod cls_player_session;
pub mod cls_player_status;
pub mod cls_player_styled_cell;
pub mod cls_player_styled_grid;
pub mod cls_player_summary;
pub mod cls_player_visual_cell;
pub mod cls_player_visual_frame;
pub mod cls_player_visual_frame_report;
pub mod cls_player_warning;
mod cls_primitive_field_descriptor_coverage;
pub mod cls_recipe_player;
mod cls_resolved_color;
mod fnc_aggregate_player_inventory_effects;
mod fnc_aggregate_player_inventory_sources;
mod fnc_apply_filter_primitive;
pub mod fnc_apply_graph_effects;
mod fnc_apply_mask_checkers;
mod fnc_apply_mask_dissolve;
mod fnc_apply_mask_wipe;
mod fnc_apply_sampler_ripple;
mod fnc_apply_sampler_sine_wave;
mod fnc_apply_shader_primitive;
mod fnc_apply_style_primitive;
mod fnc_apply_styled_primitive;
pub mod fnc_build_frame_diff_report;
pub mod fnc_build_frame_timeline_report;
mod fnc_build_migration_gap_family;
pub mod fnc_build_migration_gap_report;
pub mod fnc_build_player_frame;
pub mod fnc_build_primitive_adapter_gap_report;
pub mod fnc_build_primitive_field_coverage_report;
mod fnc_build_primitive_field_instance;
mod fnc_build_visual_frame;
mod fnc_classify_debug_recipe_family;
mod fnc_classify_primitive_adapter_gap;
mod fnc_classify_primitive_field_coverage;
mod fnc_collect_debug_recipe_family_inventory;
mod fnc_collect_descriptor_inventory_ids;
mod fnc_collect_handled_primitive_inputs;
mod fnc_collect_migration_gap_family_names;
pub mod fnc_collect_recipe_paths;
mod fnc_collect_styled_grid_scope_cells;
mod fnc_collect_styled_visual_cells;
mod fnc_collect_unsupported_effect_ids;
mod fnc_diff_visual_frame_cells;
mod fnc_extract_recipe_inventory_ids;
pub mod fnc_inventory_recipe_file;
pub mod fnc_inventory_recipe_paths;
pub mod fnc_load_descriptor_catalog;
mod fnc_load_primitive_field_descriptor_coverage;
mod fnc_player_inventory_adapter_status;
mod fnc_player_inventory_file_error;
pub mod fnc_primitive_adapter_gap_paths;
mod fnc_recommend_migration_queue;
pub mod fnc_render_hash;
pub mod fnc_render_recipe_file;
pub mod fnc_render_scene;
pub mod fnc_render_visual_frame_paths;
mod fnc_resolve_effect_input;
pub mod fnc_resolve_value_source;
mod fnc_scan_primitive_field_recipe;
mod fnc_summarize_migration_gap_families;
mod fnc_summarize_player_inventory;
mod fnc_summarize_primitive_adapter_gaps;
mod fnc_summarize_primitive_field_coverage;
mod fnc_summarize_visual_frames;

pub use cls_player_error::PlayerError;
pub use cls_player_frame::PlayerFrame;
pub use cls_player_frame_diff_report::{PlayerFrameDiffCell, PlayerFrameDiffReport};
pub use cls_player_frame_report::PlayerFrameReport;
pub use cls_player_frame_timeline_report::PlayerFrameTimelineReport;
pub use cls_player_inventory_effect::PlayerInventoryEffect;
pub use cls_player_inventory_recipe::PlayerInventoryRecipe;
pub use cls_player_inventory_report::PlayerInventoryReport;
pub use cls_player_inventory_source::PlayerInventorySource;
pub use cls_player_inventory_summary::PlayerInventorySummary;
pub use cls_player_migration_gap_report::{
    PlayerMigrationGapFamily, PlayerMigrationGapReport, PlayerMigrationGapSummary,
    PlayerMigrationQueueItem,
};
pub use cls_player_primitive_adapter_gap_entry::PlayerPrimitiveAdapterGapEntry;
pub use cls_player_primitive_adapter_gap_report::PlayerPrimitiveAdapterGapReport;
pub use cls_player_primitive_adapter_gap_summary::PlayerPrimitiveAdapterGapSummary;
pub use cls_player_primitive_field_coverage::{
    PlayerPrimitiveFieldCoverageInstance, PlayerPrimitiveFieldCoverageRecipe,
    PlayerPrimitiveFieldCoverageReport, PlayerPrimitiveFieldCoverageSummary,
};
pub use cls_player_run_report::PlayerRunReport;
pub use cls_player_sample_request::PlayerSampleRequest;
pub use cls_player_session::PlayerSession;
pub use cls_player_status::PlayerStatus;
pub use cls_player_styled_cell::PlayerStyledCell;
pub use cls_player_styled_grid::PlayerStyledGrid;
pub use cls_player_summary::PlayerSummary;
pub use cls_player_visual_cell::PlayerVisualCell;
pub use cls_player_visual_frame::PlayerVisualFrame;
pub use cls_player_visual_frame_report::PlayerVisualFrameReport;
pub use cls_player_warning::PlayerWarning;
pub(crate) use cls_primitive_field_descriptor_coverage::PrimitiveFieldDescriptorCoverage;
pub use cls_recipe_player::RecipePlayer;
pub use fnc_build_frame_diff_report::build_frame_diff_report;
pub use fnc_build_frame_timeline_report::build_frame_timeline_report;
pub use fnc_build_migration_gap_report::build_migration_gap_report;
pub use fnc_build_primitive_adapter_gap_report::build_primitive_adapter_gap_report;
pub use fnc_build_primitive_field_coverage_report::build_primitive_field_coverage_report;
pub use fnc_build_visual_frame::build_visual_frame_from_styled_grid;
pub use fnc_collect_recipe_paths::collect_recipe_paths;
pub use fnc_inventory_recipe_file::inventory_recipe_file;
pub use fnc_inventory_recipe_paths::inventory_recipe_paths;
pub use fnc_load_descriptor_catalog::{
    DescriptorPackReport, LoadedDescriptorCatalog, load_descriptor_catalog,
};
pub use fnc_primitive_adapter_gap_paths::primitive_adapter_gap_paths;
pub use fnc_render_recipe_file::render_recipe_file;
pub use fnc_render_visual_frame_paths::render_visual_frame_paths;
pub use fnc_resolve_value_source::resolve_value_source;

// <FILE>crates/tui-vfx-player/src/lib.rs</FILE> - <DESC>Contract-native skeleton player exports</DESC>
// <VERS>END OF VERSION: 0.7.1</VERS>
