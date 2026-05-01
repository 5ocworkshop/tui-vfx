// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_highlighter.rs</FILE> - <DESC>Direct v3.1 highlighter tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct v3.1 tests are split by concern/primitive to preserve OFPF file size discipline.</WCTX>
// <CLOG>0.1.0: INIT — extract Direct v3.1 highlighter tests.</CLOG>

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31LoadError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_types::Color;

use super::support::*;

#[test]
fn load_validated_v31_highlighter_renders_directly_in_compositor_next() {
    let catalog = primitive_catalog();
    let loaded = LoadedV31Recipe::load(recipe_from_value(highlighter_recipe_value()), &catalog)
        .expect("highlighter recipe validates at load time");
    let frame = render_v31_recipe(&loaded, &V31SampleContext::default())
        .expect("direct compositor-next render");

    assert_eq!(frame.recipe_id, "compositorNextDirectHighlighter");
    assert_eq!(
        frame.applied_effect_kinds,
        vec!["shader.highlighter".to_string()]
    );
    assert_eq!(frame.grid.cell((0, 0)).unwrap().bg, Color::RED);
    assert_eq!(frame.grid.cell((4, 0)).unwrap().bg, Color::BLACK);
}

#[test]
fn rejects_unsupported_highlighter_inputs_at_load_time() {
    let catalog = primitive_catalog();
    let mut recipe = highlighter_recipe_value();
    recipe["graph"]["nodes"]["highlighter"]["inputs"]["textContrast"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "number", "value": 0.5 }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects unsupported highlighter textContrast");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedDirectInput {
            effect,
            input,
            ..
        } if effect == "shader.highlighter" && input == "textContrast"
    ));
}

#[test]
fn rejects_descriptor_valid_highlighter_modes_without_direct_support() {
    let catalog = primitive_catalog();
    for unsupported_mode in ["row", "centerOut"] {
        let mut recipe = highlighter_recipe_value();
        recipe["graph"]["nodes"]["highlighter"]["inputs"]["mode"] = serde_json::json!({
            "kind": "literal",
            "value": { "kind": "enum", "value": unsupported_mode }
        });

        let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
            .expect_err("direct v3.1 load rejects unsupported highlighter modes");

        assert!(matches!(
            err,
            V31LoadError::UnsupportedDirectInput {
                effect,
                input,
                ..
            } if effect == "shader.highlighter" && input == "mode"
        ));
    }
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_shader_highlighter.rs</FILE> - <DESC>Direct v3.1 highlighter tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
