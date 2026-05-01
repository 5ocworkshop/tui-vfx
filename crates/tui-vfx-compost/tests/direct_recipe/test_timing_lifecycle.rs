// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_timing_lifecycle.rs</FILE> - <DESC>Compost timing and lifecycle substrate tests</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Timing substrate tests keep sample clocks explicit and verify lifecycle active-node gating.</WCTX>
// <CLOG>0.2.1: PATCH — keep external tests focused on sample data and render-observable lifecycle behavior.
// 0.2.0: MINOR — verify activePhases render only during matching lifecycle samples.
// 0.1.0: INIT — add RED coverage for loop clocks, absolute time, and activePhases rejection.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_types::Color;

fn load_recipe(recipe: serde_json::Value) -> LoadedRecipe {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("recipe should load")
}

#[test]
fn sample_context_carries_explicit_loop_and_absolute_time() {
    let sample = SampleContext::new(1.25)
        .with_loop_t(-0.25)
        .with_absolute_time_ms(1_250);

    assert_eq!(sample.phase_t, 1.25);
    assert_eq!(sample.loop_t, Some(-0.25));
    assert_eq!(sample.absolute_time_ms, Some(1_250));
}

#[test]
fn sample_context_uses_phase_time_when_loop_time_is_absent() {
    let sample = SampleContext::new(0.75);

    assert_eq!(sample.loop_t, None);
    assert_eq!(sample.absolute_time_ms, None);
}

fn render_recipe_value(recipe: serde_json::Value, sample: SampleContext) -> tui_vfx_compost::Frame {
    let loaded = load_recipe(recipe);
    render_recipe(&loaded, &sample).expect("recipe should render")
}

#[test]
fn active_phases_apply_node_only_when_sample_phase_matches() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["activePhases"] = serde_json::json!(["enter"]);

    let inactive_frame = render_recipe_value(recipe.clone(), SampleContext::default());
    assert_eq!(inactive_frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
    assert!(inactive_frame.applied_effect_kinds.is_empty());

    let active_frame = render_recipe_value(
        recipe,
        SampleContext::default().with_lifecycle_phase(tui_vfx_contract::LifecyclePhase::Enter),
    );
    assert_eq!(active_frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
    assert_eq!(
        active_frame.applied_effect_kinds,
        vec!["shader.linearGradient"]
    );
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_timing_lifecycle.rs</FILE> - <DESC>Compost timing and lifecycle substrate tests</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
