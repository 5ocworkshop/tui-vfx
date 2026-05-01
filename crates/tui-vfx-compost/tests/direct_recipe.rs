// <FILE>crates/tui-vfx-compost/tests/direct_recipe.rs</FILE> - <DESC>Integration test harness for compost direct recipe tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Expose OFPF-split direct recipe tests, including scene substrate coverage, through one integration target.</WCTX>
// <CLOG>0.2.0: MINOR — add Phase 1 scene element substrate tests.
// 0.1.0: INIT — add direct_recipe integration test harness.</CLOG>

#[path = "direct_recipe/support.rs"]
mod support;

#[path = "direct_recipe/test_shader_linear_gradient.rs"]
mod test_shader_linear_gradient;

#[path = "direct_recipe/test_scene_elements.rs"]
mod test_scene_elements;

// <FILE>crates/tui-vfx-compost/tests/direct_recipe.rs</FILE> - <DESC>Integration test harness for compost direct recipe tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
