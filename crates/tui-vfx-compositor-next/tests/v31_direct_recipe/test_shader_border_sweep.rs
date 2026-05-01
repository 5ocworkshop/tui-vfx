// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_border_sweep.rs</FILE> - <DESC>Direct v3.1 borderSweep tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct v3.1 tests are split by concern/primitive to preserve OFPF file size discipline.</WCTX>
// <CLOG>0.1.0: INIT — extract Direct v3.1 borderSweep tests.</CLOG>

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31LoadError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_types::Color;

use super::support::*;

#[test]
fn load_validated_v31_border_sweep_renders_directly_in_compositor_next() {
    let catalog = primitive_catalog();
    let loaded = LoadedV31Recipe::load(recipe_from_value(border_sweep_recipe_value()), &catalog)
        .expect("border sweep recipe validates at load time");
    let frame = render_v31_recipe(&loaded, &V31SampleContext::default())
        .expect("direct compositor-next render");

    assert_eq!(frame.recipe_id, "compositorNextDirectBorderSweep");
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.borderSweep".to_string()]
    );
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::rgb(0, 255, 255));
    assert_eq!(frame.grid.cell((1, 1)).unwrap().fg, Color::WHITE);
}

#[test]
fn rejects_descriptor_valid_border_sweep_position_without_direct_support() {
    let catalog = primitive_catalog();
    let mut recipe = border_sweep_recipe_value();
    recipe["graph"]["nodes"]["borderSweep"]["inputs"]["position"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "number", "value": 0.0 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects unsupported border sweep position");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.borderSweep" && input == "position"
    ));
}

#[test]
fn rejects_runtime_sourced_border_sweep_inputs_at_load_time() {
    let catalog = primitive_catalog();
    let mut recipe = border_sweep_recipe_value();
    recipe["graph"]["parameters"]["borderSweepSpeed"] = serde_json::json!({
        "id": "borderSweepSpeed",
        "displayName": "Border Sweep Speed",
        "description": null,
        "value": {
            "kind": "number",
            "default": { "kind": "number", "value": 1.0 },
            "range": { "min": 0.0, "max": null },
            "allowedValues": [],
            "unit": "loops-per-second",
            "semantic": null
        },
        "bindable": true
    });
    recipe["graph"]["nodes"]["borderSweep"]["inputs"]["speed"] = serde_json::json!({
        "kind": "parameter",
        "id": "borderSweepSpeed",
        "fallback": { "kind": "number", "value": 1.0 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects runtime-sourced border sweep inputs");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.borderSweep" && input == "speed"
    ));
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_border_sweep.rs</FILE> - <DESC>Direct v3.1 borderSweep tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
