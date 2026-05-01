// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_shader_linear_gradient.rs</FILE> - <DESC>Compost direct linearGradient tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>First compost vertical slice proves canonical v3.1 load + render without v31 path or legacy lowering.</WCTX>
// <CLOG>0.1.0: INIT — add RED tests for native compost linearGradient slice.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_types::Color;

#[test]
fn rejects_unsupported_recipe_version_before_rendering() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["version"] = serde_json::Value::String("3.2".to_string());

    let err = LoadedRecipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("compost rejects non-v3.1 recipes at load time");

    assert!(matches!(
        err,
        LoadError::UnsupportedVersion {
            recipe_version,
            graph_version,
        } if recipe_version == "3.2" && graph_version == "3.1"
    ));
}

#[test]
fn rejects_runtime_sourced_linear_gradient_inputs_at_load_time() {
    let catalog = primitive_catalog();
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

    let err = LoadedRecipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("compost rejects unresolved runtime inputs");

    assert!(matches!(
        err,
        LoadError::UnsupportedInput {
            node_id,
            effect,
            input,
            ..
        } if node_id == "gradient" && effect == "shader.linearGradient" && input == "angleDeg"
    ));
}

#[test]
fn load_validated_linear_gradient_renders_without_legacy_lowering() {
    let catalog = primitive_catalog();
    let loaded = LoadedRecipe::load(recipe_from_value(linear_gradient_recipe_value()), &catalog)
        .expect("recipe validates at load time");

    let frame = render_recipe(&loaded, &SampleContext::default()).expect("compost render");

    assert_eq!(frame.recipe_id, "compostDirectLinearGradient");
    assert_eq!(frame.width, 3);
    assert_eq!(frame.height, 1);
    assert_eq!(frame.applied_effect_kinds, vec!["shader.linearGradient"]);
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
    assert_eq!(frame.grid.cell((1, 0)).unwrap().fg, Color::GREEN);
    assert_eq!(frame.grid.cell((2, 0)).unwrap().fg, Color::BLUE);
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_shader_linear_gradient.rs</FILE> - <DESC>Compost direct linearGradient tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
