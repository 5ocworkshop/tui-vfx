// <FILE>tui-vfx-compositor/src/pipeline/mod.rs</FILE> - <DESC>Pipeline module</DESC>
// <VERS>VERSION: 7.3.0</VERS>
// <WCTX>Shadow integration into compositor pipeline</WCTX>
// <CLOG>Re-export ShadowSpec from types for convenient access</CLOG>

pub mod cls_composition_options;
pub mod cls_composition_spec;
mod cls_prepared_filter;
mod cls_prepared_mask;
mod cls_prepared_sampler;
pub mod cls_render_area;
pub mod cls_shader_layer_spec;
pub mod fnc_check_masks;
pub mod fnc_render_pipeline_with_spec;
pub mod fnc_render_pipeline_with_spec_area;
pub mod orc_render_pipeline;

pub use crate::types::ShadowSpec;
pub use cls_composition_options::{CompositionOptions, ShaderWithRegion};
pub use cls_composition_spec::CompositionSpec;
pub use cls_render_area::RenderArea;
pub use cls_shader_layer_spec::ShaderLayerSpec;
pub use fnc_check_masks::check_masks;
pub use fnc_render_pipeline_with_spec::render_pipeline_with_spec;
pub use fnc_render_pipeline_with_spec_area::render_pipeline_with_spec_area;
pub use orc_render_pipeline::{render_pipeline, render_pipeline_with_area};

// <FILE>tui-vfx-compositor/src/pipeline/mod.rs</FILE> - <DESC>Pipeline module</DESC>
// <VERS>END OF VERSION: 7.3.0</VERS>
