// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_focus_field.rs</FILE> - <DESC>Direct v3.1 focusField tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct v3.1 tests are split by concern/primitive to preserve OFPF file size discipline.</WCTX>
// <CLOG>0.1.0: INIT — extract Direct v3.1 focusField tests.</CLOG>

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31LoadError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_types::Color;

use super::support::*;

#[test]
fn load_validated_v31_focus_field_renders_directly_in_compositor_next() {
    let catalog = primitive_catalog();
    let loaded = LoadedV31Recipe::load(recipe_from_value(focus_field_recipe_value()), &catalog)
        .expect("focus field recipe validates at load time");
    let frame = render_v31_recipe(&loaded, &V31SampleContext::default())
        .expect("direct compositor-next render");

    assert_eq!(frame.recipe_id, "compositorNextDirectFocusField");
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.focusField".to_string()]
    );
    assert_eq!(frame.grid.cell((2, 0)).unwrap().bg, Color::BLUE);
    assert_eq!(frame.grid.cell((4, 0)).unwrap().bg, Color::BLACK);
}

#[test]
fn rejects_descriptor_valid_focus_field_rect_shape_without_direct_support() {
    let catalog = primitive_catalog();
    let mut recipe = focus_field_recipe_value();
    recipe["graph"]["nodes"]["focusField"]["inputs"]["shape"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "enum", "value": "rect" }
    });
    recipe["graph"]["nodes"]["focusField"]["inputs"]["rectWidth"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "number", "value": 2.0 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects unsupported focusField rect semantics");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.focusField" && input == "shape"
    ));
}

#[test]
fn rejects_runtime_sourced_focus_field_inputs_at_load_time() {
    let catalog = primitive_catalog();
    let mut recipe = focus_field_recipe_value();
    recipe["graph"]["parameters"]["focusX"] = serde_json::json!({
        "id": "focusX",
        "displayName": "Focus X",
        "description": null,
        "value": {
            "kind": "number",
            "default": { "kind": "number", "value": 2.0 },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "bindable": true
    });
    recipe["graph"]["nodes"]["focusField"]["inputs"]["centerX"] = serde_json::json!({
        "kind": "parameter",
        "id": "focusX",
        "fallback": { "kind": "number", "value": 2.0 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects runtime-sourced focusField inputs");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.focusField" && input == "centerX"
    ));
}

#[test]
fn rejects_fractional_focus_field_geometry_at_load_time() {
    let catalog = primitive_catalog();
    for input in ["centerX", "centerY", "radius"] {
        let mut recipe = focus_field_recipe_value();
        recipe["graph"]["nodes"]["focusField"]["inputs"][input] = serde_json::json!({
            "kind": "literal",
            "value": { "kind": "number", "value": 1.5 }
        });

        let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
            .expect_err("direct v3.1 load rejects fractional focus field geometry");

        assert!(matches!(
            err,
            V31LoadError::UnsupportedDirectInput {
                effect,
                input: rejected_input,
                ..
            } if effect == "shader.focusField" && rejected_input == input
        ));
    }
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_focus_field.rs</FILE> - <DESC>Direct v3.1 focusField tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
