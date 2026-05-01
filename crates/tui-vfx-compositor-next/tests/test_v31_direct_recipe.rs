// <FILE>crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs</FILE> - <DESC>Pure v3.1 RecipeDocument to compositor-next rendering test harness</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Compositor-next pure v3.1 path tests are split into OFPF-sized support and per-primitive modules.</WCTX>
// <CLOG>0.3.0: MINOR — split direct v3.1 recipe tests into OFPF-sized modules.
// 0.2.0: MINOR add strict v3.1 load and canonical gradient-stop coverage.</CLOG>

#[path = "v31_direct_recipe/support.rs"]
mod support;
#[path = "v31_direct_recipe/test_load_contract.rs"]
mod test_load_contract;
#[path = "v31_direct_recipe/test_shader_border_sweep.rs"]
mod test_shader_border_sweep;
#[path = "v31_direct_recipe/test_shader_focus_field.rs"]
mod test_shader_focus_field;
#[path = "v31_direct_recipe/test_shader_glisten_band.rs"]
mod test_shader_glisten_band;
#[path = "v31_direct_recipe/test_shader_highlighter.rs"]
mod test_shader_highlighter;
#[path = "v31_direct_recipe/test_shader_linear_gradient.rs"]
mod test_shader_linear_gradient;

// <FILE>crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs</FILE> - <DESC>Pure v3.1 RecipeDocument to compositor-next rendering test harness</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
