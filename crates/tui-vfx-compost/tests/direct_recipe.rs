// <FILE>crates/tui-vfx-compost/tests/direct_recipe.rs</FILE> - <DESC>Integration test harness for compost direct recipe tests</DESC>
// <VERS>VERSION: 0.9.0</VERS>
// <WCTX>Expose OFPF-split direct recipe tests, including scene/source/effect-stack/timing/write/runtime/observability substrate coverage.</WCTX>
// <CLOG>0.9.0: MINOR — add node-local write policy execution coverage.
// 0.8.1: PATCH — cover node-local write policy rejection and final-cell skip policy.</CLOG>

#[path = "direct_recipe/support.rs"]
mod support;

#[path = "direct_recipe/test_shader_linear_gradient.rs"]
mod test_shader_linear_gradient;

#[path = "direct_recipe/test_scene_elements.rs"]
mod test_scene_elements;

#[path = "direct_recipe/test_source_contract.rs"]
mod test_source_contract;

#[path = "direct_recipe/test_effect_stack_contract.rs"]
mod test_effect_stack_contract;

#[path = "direct_recipe/test_timing_lifecycle.rs"]
mod test_timing_lifecycle;

#[path = "direct_recipe/test_write_merge_policy.rs"]
mod test_write_merge_policy;

#[path = "direct_recipe/test_node_write_policy.rs"]
mod test_node_write_policy;

#[path = "direct_recipe/test_runtime_values.rs"]
mod test_runtime_values;

#[path = "direct_recipe/test_render_observability.rs"]
mod test_render_observability;

#[path = "direct_recipe/test_shadow_surface.rs"]
mod test_shadow_surface;

// <FILE>crates/tui-vfx-compost/tests/direct_recipe.rs</FILE> - <DESC>Integration test harness for compost direct recipe tests</DESC>
// <VERS>END OF VERSION: 0.9.0</VERS>
