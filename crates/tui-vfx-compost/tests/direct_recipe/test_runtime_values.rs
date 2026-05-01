// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_runtime_values.rs</FILE> - <DESC>Compost runtime value resolver substrate tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Runtime value tests centralize non-literal ValueSource rejection before runtime bindings are implemented.</WCTX>
// <CLOG>0.1.0: INIT — add RED coverage for central runtime resolver diagnostics.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe};

fn load_recipe_error(recipe: serde_json::Value) -> LoadError {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect_err("recipe should fail load")
}

#[test]
fn rejects_parameter_value_source_through_runtime_resolver() {
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

    let error = load_recipe_error(recipe);

    assert!(matches!(
        &error,
        LoadError::UnsupportedInput {
            node_id,
            effect,
            input,
            reason,
        } if node_id == "gradient"
            && effect == "shader.linearGradient"
            && input == "angleDeg"
            && reason.contains("runtime value resolver")
            && reason.contains("parameter")
    ));
}

#[test]
fn rejects_mapped_value_source_through_runtime_resolver() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["nodes"]["gradient"]["inputs"]["intensity"] = serde_json::json!({
        "kind": "map",
        "from": { "kind": "literal", "value": { "kind": "number", "value": 0.5 } },
        "input": { "min": 0.0, "max": 1.0 },
        "output": { "min": 0.0, "max": 1.0 },
        "clamp": true
    });

    let error = load_recipe_error(recipe);

    assert!(matches!(
        &error,
        LoadError::UnsupportedInput {
            node_id,
            effect,
            input,
            reason,
        } if node_id == "gradient"
            && effect == "shader.linearGradient"
            && input == "intensity"
            && reason.contains("runtime value resolver")
            && reason.contains("map")
    ));
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_runtime_values.rs</FILE> - <DESC>Compost runtime value resolver substrate tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
