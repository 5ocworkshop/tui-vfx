// <FILE>tui-vfx-compositor/src/pipeline/mod.rs</FILE> - <DESC>Pipeline module</DESC>
// <VERS>VERSION: 7.5.1</VERS>
// <WCTX>Phase 1a perf — register cls_grid_pool for render-pipeline buffer reuse</WCTX>
// <CLOG>7.5.1: PATCH — register cls_grid_pool module (internal, thread-local buffer pool)</CLOG>

pub mod cls_composition_options;
pub mod cls_composition_playback_timing;
pub mod cls_composition_spec;
mod cls_grid_pool;
mod cls_prepare_context;
mod cls_prepared_filter;
mod cls_prepared_mask;
mod cls_prepared_sampler;
pub mod cls_render_area;
pub mod cls_shader_layer_spec;
pub mod fnc_check_masks;
pub mod fnc_grade_shadow_cell;
pub mod fnc_render_pipeline_with_spec;
pub mod fnc_render_pipeline_with_spec_area;
pub mod orc_render_pipeline;

pub use crate::types::ShadowSpec;
pub use cls_composition_options::{CompositionOptions, ShaderWithRegion};
pub use cls_composition_playback_timing::CompositionPlaybackTiming;
pub use cls_composition_spec::CompositionSpec;
pub use cls_render_area::RenderArea;
pub use cls_shader_layer_spec::ShaderLayerSpec;
pub use fnc_check_masks::check_masks;
pub use fnc_render_pipeline_with_spec::render_pipeline_with_spec;
pub use fnc_render_pipeline_with_spec_area::render_pipeline_with_spec_area;
pub use orc_render_pipeline::{render_pipeline, render_pipeline_with_area};

// <FILE>tui-vfx-compositor/src/pipeline/mod.rs</FILE> - <DESC>Pipeline module</DESC>
// <VERS>END OF VERSION: 7.5.1</VERS>
