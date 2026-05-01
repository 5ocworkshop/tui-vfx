// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_glisten_band.rs</FILE> - <DESC>Direct v3.1 glistenBand tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct v3.1 tests are split by concern/primitive to preserve OFPF file size discipline.</WCTX>
// <CLOG>0.1.0: INIT — extract Direct v3.1 glistenBand tests.</CLOG>

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31LoadError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_types::Color;

use super::support::*;

#[test]
fn load_validated_v31_glisten_band_renders_directly_in_compositor_next() {
    let catalog = primitive_catalog();
    let loaded = LoadedV31Recipe::load(recipe_from_value(glisten_band_recipe_value()), &catalog)
        .expect("glisten band recipe validates at load time");

    let frame = render_v31_recipe(&loaded, &V31SampleContext { phase_t: 0.0625 })
        .expect("direct compositor-next render");

    assert_eq!(frame.recipe_id, "compositorNextDirectGlistenBand");
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.glistenBand".to_string()]
    );
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::RED);
    assert_eq!(frame.grid.cell((6, 0)).unwrap().fg, Color::WHITE);
}

#[test]
fn rejects_descriptor_valid_glisten_band_tail_input_without_direct_support() {
    let catalog = primitive_catalog();
    let mut recipe = glisten_band_recipe_value();
    recipe["graph"]["nodes"]["glisten"]["inputs"]["tail"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "number", "value": 0.5 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects unsupported glisten band tail input");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.glistenBand" && input == "tail"
    ));
}

#[test]
fn rejects_fractional_glisten_band_width_without_direct_support() {
    let catalog = primitive_catalog();
    let mut recipe = glisten_band_recipe_value();
    recipe["graph"]["nodes"]["glisten"]["inputs"]["bandWidth"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "number", "value": 1.5 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects fractional glisten band width");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.glistenBand" && input == "bandWidth"
    ));
}

#[test]
fn rejects_runtime_sourced_glisten_band_inputs_at_load_time() {
    let catalog = primitive_catalog();
    let mut recipe = glisten_band_recipe_value();
    recipe["graph"]["parameters"]["glistenColor"] = serde_json::json!({
        "id": "glistenColor",
        "displayName": "Glisten Color",
        "description": null,
        "value": {
            "kind": "color",
            "default": { "kind": "color", "value": { "r": 255, "g": 0, "b": 0, "a": 255 } },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "bindable": true
    });
    recipe["graph"]["nodes"]["glisten"]["inputs"]["color"] = serde_json::json!({
        "kind": "parameter",
        "id": "glistenColor",
        "fallback": { "kind": "color", "value": { "r": 255, "g": 0, "b": 0, "a": 255 } }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects runtime-sourced glisten band inputs");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.glistenBand" && input == "color"
    ));
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_glisten_band.rs</FILE> - <DESC>Direct v3.1 glistenBand tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
