// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_timing_lifecycle.rs</FILE> - <DESC>Compost timing and lifecycle substrate tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Timing substrate tests keep sample clocks explicit and reject unsupported lifecycle gating.</WCTX>
// <CLOG>0.1.0: INIT — add RED coverage for loop clocks, absolute time, and activePhases rejection.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe, SampleContext};

fn load_recipe_error(recipe: serde_json::Value) -> LoadError {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect_err("recipe should fail load")
}

#[test]
fn sample_context_carries_explicit_loop_and_absolute_time() {
    let sample = SampleContext::new(1.25)
        .with_loop_t(-0.25)
        .with_absolute_time_ms(1_250);

    assert_eq!(sample.phase_t, 1.25);
    assert_eq!(sample.loop_t, Some(-0.25));
    assert_eq!(sample.absolute_time_ms, Some(1_250));
    assert_eq!(sample.effective_loop_t(), 0.0);
    assert_eq!(sample.shader_phase_t(), 0.0);
}

#[test]
fn sample_context_uses_phase_time_when_loop_time_is_absent() {
    let sample = SampleContext::new(0.75);

    assert_eq!(sample.loop_t, None);
    assert_eq!(sample.absolute_time_ms, None);
    assert_eq!(sample.effective_loop_t(), 0.75);
    assert_eq!(sample.shader_phase_t(), 0.75);
}

#[test]
fn rejects_node_active_phases_until_lifecycle_resolution_exists() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["activePhases"] = serde_json::json!(["enter"]);

    let error = load_recipe_error(recipe);

    assert!(matches!(
        error,
        LoadError::UnsupportedNodeTiming {
            node_id,
            effect,
            field,
            ..
        } if node_id == "gradient"
            && effect == "shader.linearGradient"
            && field == "activePhases"
    ));
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_timing_lifecycle.rs</FILE> - <DESC>Compost timing and lifecycle substrate tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
