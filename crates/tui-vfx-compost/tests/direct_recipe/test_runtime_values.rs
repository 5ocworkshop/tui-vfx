// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_runtime_values.rs</FILE> - <DESC>Compost runtime value resolver substrate tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime value tests prove mature-reference prepare-context semantics resolve non-literal sources through compost substrate.</WCTX>
// <CLOG>0.2.0: MINOR — expect parameter fallback and mapped values to render through native runtime resolver.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadedRecipe, SampleContext, render_recipe};

fn load_recipe(recipe: serde_json::Value) -> LoadedRecipe {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("recipe should load")
}

#[test]
fn resolves_parameter_fallback_through_runtime_resolver() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["angle"] = serde_json::json!({
        "id": "angle",
        "displayName": "Angle",
        "description": null,
        "value": {
            "kind": "number",
            "default": { "kind": "number", "value": 0.0 },
            "range": { "min": 0.0, "max": 360.0 },
            "allowedValues": [],
            "unit": "degrees",
            "semantic": null
        },
        "bindable": true
    });
    recipe["graph"]["nodes"]["gradient"]["inputs"]["angleDeg"] = serde_json::json!({
        "kind": "parameter",
        "id": "angle",
        "fallback": { "kind": "number", "value": 0.0 }
    });

    let loaded = load_recipe(recipe);
    let frame =
        render_recipe(&loaded, &SampleContext::new(0.0)).expect("render parameter fallback");

    assert_eq!(frame.width, 3);
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

#[test]
fn resolves_parameter_default_from_graph_declaration() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["angle"] = serde_json::json!({
        "id": "angle",
        "displayName": "Angle",
        "description": null,
        "value": {
            "kind": "number",
            "default": { "kind": "number", "value": 0.0 },
            "range": { "min": 0.0, "max": 360.0 },
            "allowedValues": [],
            "unit": "degrees",
            "semantic": null
        },
        "bindable": true
    });
    recipe["graph"]["nodes"]["gradient"]["inputs"]["angleDeg"] = serde_json::json!({
        "kind": "parameter",
        "id": "angle",
        "fallback": null
    });

    let loaded = load_recipe(recipe);
    let frame = render_recipe(&loaded, &SampleContext::new(0.0)).expect("render parameter default");

    assert_eq!(frame.width, 3);
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

#[test]
fn resolves_signal_default_from_graph_declaration() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["signals"]["intensitySignal"] = serde_json::json!({
        "id": "intensitySignal",
        "displayName": "Intensity",
        "description": null,
        "value": {
            "kind": "number",
            "default": { "kind": "number", "value": 1.0 },
            "range": { "min": 0.0, "max": 1.0 },
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "previewLoopback": null,
        "required": false
    });
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"] = serde_json::json!({
        "kind": "signal",
        "id": "intensitySignal",
        "fallback": null
    });

    let loaded = load_recipe(recipe);
    let frame = render_recipe(&loaded, &SampleContext::new(0.0)).expect("render signal default");

    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

#[test]
fn resolves_phase_clock_and_signal_expression_sources() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["angleDeg"] = serde_json::json!({
        "kind": "phaseProgress",
        "phase": "enter"
    });
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"] = serde_json::json!({
        "kind": "signalExpression",
        "expression": { "kind": "constant", "value": 1.0 },
        "fallback": null
    });

    let loaded = load_recipe(recipe);
    let frame = render_recipe(
        &loaded,
        &SampleContext::new(0.25)
            .with_loop_t(0.5)
            .with_absolute_time_ms(750),
    )
    .expect("render phase and expression sources");

    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

#[test]
fn resolves_clock_source_for_numeric_inputs() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"] = serde_json::json!({
        "kind": "clock",
        "clock": "loopSeconds"
    });

    let loaded = load_recipe(recipe);
    let frame = render_recipe(
        &loaded,
        &SampleContext::new(0.25)
            .with_loop_t(1.0)
            .with_loop_time_ms(1_000),
    )
    .expect("render clock source");

    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

#[test]
fn resolves_supported_sampled_field_sources() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["angleDeg"] = serde_json::json!({
        "kind": "sampledField",
        "field": "surfaceAngleFrom",
        "x": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
        "y": { "kind": "literal", "value": { "kind": "number", "value": 0.0 } },
        "fallback": null
    });

    let loaded = load_recipe(recipe);
    let frame =
        render_recipe(&loaded, &SampleContext::new(0.0)).expect("render sampled field source");

    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

#[test]
fn resolves_mapped_value_source_through_runtime_resolver() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"] = serde_json::json!({
        "kind": "map",
        "from": { "kind": "literal", "value": { "kind": "number", "value": 0.5 } },
        "input": { "min": 0.0, "max": 1.0 },
        "output": { "min": 0.0, "max": 1.0 },
        "clamp": true
    });

    let loaded = load_recipe(recipe);
    let frame = render_recipe(&loaded, &SampleContext::new(0.0)).expect("render mapped input");

    assert_eq!(frame.width, 3);
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_runtime_values.rs</FILE> - <DESC>Compost runtime value resolver substrate tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
