// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_linear_gradient.rs</FILE> - <DESC>Direct v3.1 linearGradient tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct v3.1 tests are split by concern/primitive to preserve OFPF file size discipline.</WCTX>
// <CLOG>0.1.0: INIT — extract Direct v3.1 linearGradient tests.</CLOG>

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31LoadError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_types::Color;

use super::support::*;

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

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects unresolved runtime inputs");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            node_id,
            effect,
            input,
            ..
        } if node_id == "gradient" && effect == "shader.linearGradient" && input == "angleDeg"
    ));
}

#[test]
fn load_validated_v31_linear_gradient_uses_canonical_gradient_stops() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "text", "value": "ABC" }
    });
    recipe["sources"]["mainCard"]["inputs"]["width"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 3 }
    });
    recipe["scenes"][0]["width"] = serde_json::Value::from(3);
    let inputs = recipe["graph"]["nodes"]["gradient"]["inputs"]
        .as_object_mut()
        .expect("node inputs object");
    inputs.remove("startColor");
    inputs.remove("endColor");
    inputs.remove("colorSpace");
    inputs.insert(
        "gradient".to_string(),
        serde_json::json!({
            "kind": "literal",
            "value": {
                "kind": "gradient",
                "value": {
                    "space": "rgb",
                    "stops": [
                        { "position": 0.0, "color": { "r": 255, "g": 0, "b": 0, "a": 255 } },
                        { "position": 0.5, "color": { "r": 0, "g": 255, "b": 0, "a": 255 } },
                        { "position": 1.0, "color": { "r": 0, "g": 0, "b": 255, "a": 255 } }
                    ]
                }
            }
        }),
    );

    let loaded = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect("recipe validates at load time");
    let frame = render_v31_recipe(&loaded, &V31SampleContext::default())
        .expect("direct compositor-next render");

    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
    assert_eq!(frame.grid.cell((1, 0)).unwrap().fg, Color::GREEN);
    assert_eq!(frame.grid.cell((2, 0)).unwrap().fg, Color::BLUE);
}

#[test]
fn load_validated_v31_linear_gradient_renders_directly_in_compositor_next() {
    let catalog = primitive_catalog();
    let loaded = LoadedV31Recipe::load(linear_gradient_recipe(), &catalog)
        .expect("recipe validates at load time");

    let frame = render_v31_recipe(&loaded, &V31SampleContext::default())
        .expect("direct compositor-next render");

    assert_eq!(frame.recipe_id, "compositorNextDirectLinearGradient");
    assert_eq!(frame.width, 8);
    assert_eq!(frame.height, 2);
    assert_eq!(frame.diagnostics, Vec::<String>::new());
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.linearGradient".to_string()]
    );
    assert_ne!(frame.grid.cell((0, 0)).unwrap().fg, Color::WHITE);
    assert_ne!(frame.grid.cell((7, 0)).unwrap().fg, Color::WHITE);
    assert_ne!(
        frame.grid.cell((0, 0)).unwrap().fg,
        frame.grid.cell((7, 0)).unwrap().fg,
        "horizontal gradient should produce different edge colors"
    );
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_linear_gradient.rs</FILE> - <DESC>Direct v3.1 linearGradient tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
