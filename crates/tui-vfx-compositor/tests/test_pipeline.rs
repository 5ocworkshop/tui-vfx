// <FILE>tui-vfx-compositor/tests/test_pipeline.rs</FILE>
// <DESC>Test linker for pipeline module</DESC>
// <VERS>VERSION: 2.5.0</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement and glyph-preserving alpha shadow blending.</WCTX>
// <CLOG>2.5.0: link blend_underlying_shadow_cell tests.</CLOG>

#[path = "pipeline/test_orc_render_pipeline.rs"]
mod test_orc_render_pipeline;

#[path = "pipeline/test_multiple_effects.rs"]
mod test_multiple_effects;

#[path = "pipeline/test_render_pipeline_with_spec.rs"]
mod test_render_pipeline_with_spec;

#[path = "pipeline/test_fnc_grade_shadow_cell.rs"]
mod test_fnc_grade_shadow_cell;

#[path = "pipeline/test_fnc_blend_underlying_shadow_cell.rs"]
mod test_fnc_blend_underlying_shadow_cell;

// <FILE>tui-vfx-compositor/tests/test_pipeline.rs</FILE>
// <DESC>Test linker for pipeline module</DESC>
// <VERS>END OF VERSION: 2.5.0</VERS>
