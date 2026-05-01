// <FILE>tui-vfx-compositor-next/tests/test_pipeline.rs</FILE>
// <DESC>Test linker for pipeline module</DESC>
// <VERS>VERSION: 2.5.1</VERS>
// <WCTX>Color-inert glyph detection for shadow grading replacement and glyph-preserving alpha shadow blending.</WCTX>
// <CLOG>2.5.1: declare test_helpers at the test-binary root once and have sub-modules `use crate::test_helpers::*` instead of each redeclaring `mod test_helpers;` (clears clippy::duplicate_mod under -D warnings).</CLOG>

// Shared test helpers for the pipeline test binary. Declared once at the
// crate root so the three sub-modules below can reference it via
// `crate::test_helpers::...` without each redeclaring `mod test_helpers;`
// (which triggers clippy::duplicate_mod).
#[path = "pipeline/test_helpers.rs"]
mod test_helpers;

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

// <FILE>tui-vfx-compositor-next/tests/test_pipeline.rs</FILE>
// <DESC>Test linker for pipeline module</DESC>
// <VERS>END OF VERSION: 2.5.0</VERS>
