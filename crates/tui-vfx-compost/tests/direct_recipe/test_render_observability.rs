// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_render_observability.rs</FILE> - <DESC>Compost render observability tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Frame observability tests record native trace events and skipped-work diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — add RED coverage for trace event identity and clipping diagnostics.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadedRecipe, SampleContext, render_recipe};

fn render_recipe_value(recipe: serde_json::Value) -> tui_vfx_compost::Frame {
    let catalog = primitive_catalog();
    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    render_recipe(&loaded, &SampleContext::default()).expect("render recipe")
}

#[test]
fn trace_events_identify_scene_element_stage_and_effect() {
    let frame = render_recipe_value(linear_gradient_recipe_value());

    assert_eq!(frame.trace_events.len(), 1);
    let event = &frame.trace_events[0];
    assert_eq!(event.scene_id, "mainScene");
    assert_eq!(event.element_id, "mainElement");
    assert_eq!(event.stage_index, 0);
    assert_eq!(event.effect, "shader.linearGradient");
}

#[test]
fn diagnostics_explain_fully_clipped_elements() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": 100, "y": 100 });

    let frame = render_recipe_value(recipe);

    assert!(frame.applied_effect_kinds.is_empty());
    assert!(
        frame
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.message.contains("fully clipped"))
    );
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_render_observability.rs</FILE> - <DESC>Compost render observability tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
