// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_source_contract.rs</FILE> - <DESC>Compost source substrate tests</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Source substrate tests ensure source descriptors validate at load and materialization stays separate from scene dimensions.</WCTX>
// <CLOG>0.3.0: MINOR — align source.card tests with descriptor optional color defaults and border style vocabulary.
// 0.2.1: PATCH — tighten source-input diagnostic assertions.
// 0.2.0: PATCH — add source.card dimension bounds, literal shape, and multiline regressions.
// 0.1.1: PATCH — name repeated source message fixture mutation.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe, SampleContext, render_recipe};
use tui_vfx_types::{Color, Modifiers};

fn load_recipe_error(recipe: serde_json::Value) -> LoadError {
    let catalog = primitive_catalog();
    LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect_err("recipe should fail load")
}

fn render_recipe_value(recipe: serde_json::Value) -> tui_vfx_compost::Frame {
    let catalog = primitive_catalog();
    let loaded = LoadedRecipe::load(recipe_from_value(recipe), &catalog).expect("load recipe");
    render_recipe(&loaded, &SampleContext::default()).expect("render recipe")
}

fn set_source_dimension(recipe: &mut serde_json::Value, width: i64, height: i64) {
    recipe["sources"]["mainCard"]["inputs"]["width"]["value"]["value"] =
        serde_json::Value::Number(width.into());
    recipe["sources"]["mainCard"]["inputs"]["height"]["value"]["value"] =
        serde_json::Value::Number(height.into());
}

fn set_scene_dimension(recipe: &mut serde_json::Value, width: i64, height: i64) {
    recipe["scenes"][0]["width"] = serde_json::Value::Number(width.into());
    recipe["scenes"][0]["height"] = serde_json::Value::Number(height.into());
}

fn set_source_message(recipe: &mut serde_json::Value, message: &str) {
    recipe["sources"]["mainCard"]["inputs"]["message"]["value"]["value"] =
        serde_json::Value::String(message.to_string());
}

fn assert_source_input_error(error: LoadError, expected_input: &str) {
    let is_expected_source_input = matches!(
        &error,
        LoadError::UnsupportedSourceInput {
            source_id,
            input,
            ..
        } if source_id == "mainCard" && input == expected_input
    );

    assert!(
        is_expected_source_input,
        "expected source input `{expected_input}` rejection for `mainCard`, got: {error}"
    );
}

fn assert_contract_error_contains(error: LoadError, expected: &str) {
    assert!(
        matches!(&error, LoadError::Contract { message } if message.contains(expected)),
        "expected descriptor contract error containing `{expected}`, got: {error}"
    );
}

fn assert_source_descriptor_error(error: LoadError, expected_descriptor: &str) {
    let is_expected_descriptor = matches!(
        &error,
        LoadError::UnsupportedSourceDescriptor {
            source_id,
            descriptor,
            ..
        } if source_id == "mainCard" && descriptor == expected_descriptor
    );

    assert!(
        is_expected_descriptor,
        "expected source descriptor `{expected_descriptor}` rejection for `mainCard`, got: {error}"
    );
}

#[test]
fn literal_source_card_remains_supported() {
    let frame = render_recipe_value(linear_gradient_recipe_value());

    assert_eq!(frame.width, 3);
    assert_eq!(frame.height, 1);
    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'B');
    assert_eq!(frame.grid.cell((2, 0)).unwrap().ch, 'C');
}

#[test]
fn source_text_materializes_text_bounds_and_style() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["sourceDescriptor"] =
        serde_json::Value::String("source.text".to_string());
    recipe["sources"]["mainCard"]["inputs"] = serde_json::json!({
        "message": { "kind": "literal", "value": { "kind": "text", "value": "Hi\nZ" } },
        "foreground": { "kind": "literal", "value": { "kind": "color", "value": { "r": 10, "g": 20, "b": 30, "a": 255 } } },
        "background": { "kind": "literal", "value": { "kind": "color", "value": { "r": 1, "g": 2, "b": 3, "a": 255 } } },
        "bold": { "kind": "literal", "value": { "kind": "boolean", "value": true } }
    });
    recipe["graph"]["nodes"] = serde_json::json!({});
    recipe["graph"]["order"] = serde_json::json!([]);
    recipe["scenes"][0]["height"] = serde_json::Value::Number(2.into());

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'H');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'i');
    assert_eq!(frame.grid.cell((0, 1)).unwrap().ch, 'Z');
    assert_eq!(frame.grid.cell((0, 0)).unwrap().fg, Color::rgb(10, 20, 30));
    assert_eq!(frame.grid.cell((0, 0)).unwrap().bg, Color::rgb(1, 2, 3));
    assert_eq!(frame.grid.cell((0, 0)).unwrap().mods, Modifiers::bold());
}

#[test]
fn source_procedural_materializes_braille_flag_asset_cells() {
    let mut recipe = linear_gradient_recipe_value();
    let asset_path = crate::support::repo_root()
        .join("crates/tui-vfx-compost/tests/fixtures/braille_flag_asset_minimal.json");
    recipe["assets"] = serde_json::json!({
        "flagDots": {
            "id": "flagDots",
            "kind": { "kind": "brailleDotfield" },
            "format": "tui-vfx.braille_flag_asset.v1",
            "locator": { "kind": "path", "path": asset_path.to_string_lossy() },
            "description": null
        }
    });
    recipe["sources"]["mainCard"] = serde_json::json!({
        "sourceDescriptor": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "braille_flag_field" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "params": {
                "kind": "literal",
                "value": { "kind": "structured", "value": { "asset": "$asset:flagDots" } }
            }
        },
        "assets": {}
    });
    recipe["graph"]["nodes"] = serde_json::json!({});
    recipe["graph"]["order"] = serde_json::json!([]);
    recipe["scenes"][0]["elements"][0]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });

    let frame = render_recipe_value(recipe);

    let cell = frame.grid.cell((0, 0)).unwrap();
    assert_eq!(cell.ch, '⠁');
    assert_eq!(cell.fg, Color::RED);
    assert_eq!(
        frame.grid.role((0, 0)),
        Some(tui_vfx_types::RoleTag::Procedural)
    );
}

#[test]
fn source_procedural_requires_braille_asset_params_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"] = serde_json::json!({
        "sourceDescriptor": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "braille_flag_field" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
        },
        "assets": {}
    });

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "params");
}

#[test]
fn source_procedural_rejects_unknown_generator_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"] = serde_json::json!({
        "sourceDescriptor": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "particle_fountain" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "params": { "kind": "literal", "value": { "kind": "structured", "value": {} } }
        },
        "assets": {}
    });

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "generator");
}

#[test]
fn source_procedural_rejects_unknown_braille_asset_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"] = serde_json::json!({
        "sourceDescriptor": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "braille_flag_field" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "params": {
                "kind": "literal",
                "value": { "kind": "structured", "value": { "asset": "$asset:missingDots" } }
            }
        },
        "assets": {}
    });

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "params");
}

#[test]
fn source_procedural_rejects_logical_braille_asset_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["assets"] = serde_json::json!({
        "flagDots": {
            "id": "flagDots",
            "kind": { "kind": "brailleDotfield" },
            "format": "tui-vfx.braille_flag_asset.v1",
            "locator": { "kind": "logical", "locator": "flags.canada" },
            "description": null
        }
    });
    recipe["sources"]["mainCard"] = serde_json::json!({
        "sourceDescriptor": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "braille_flag_field" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
            "params": {
                "kind": "literal",
                "value": { "kind": "structured", "value": { "asset": "$asset:flagDots" } }
            }
        },
        "assets": {}
    });

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "params");
}

#[test]
fn unsupported_existing_source_descriptor_rejects_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"] = serde_json::json!({
        "sourceDescriptor": "source.ansi",
        "inputs": {
            "ansiText": { "kind": "literal", "value": { "kind": "text", "value": "\\u001b[31mred" } }
        },
        "assets": {}
    });

    let error = load_recipe_error(recipe);

    assert_source_descriptor_error(error, "source.ansi");
}

#[test]
fn resolves_runtime_sourced_source_inputs_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["messageParam"] = serde_json::json!({
        "id": "messageParam",
        "displayName": "Message",
        "description": null,
        "value": {
            "kind": "text",
            "default": { "kind": "text", "value": "runtime" },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "bindable": true
    });
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "parameter",
        "id": "messageParam",
        "fallback": { "kind": "text", "value": "fallback" }
    });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'r');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'u');
}

#[test]
fn rejects_unknown_source_card_inputs_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]["padding"] =
        serde_json::json!({ "kind": "literal", "value": { "kind": "integer", "value": 1 } });

    let error = load_recipe_error(recipe);

    assert_contract_error_contains(error, "UnknownSourceInput");
}

#[test]
fn rejects_missing_required_source_card_input_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]
        .as_object_mut()
        .expect("source inputs object")
        .remove("message");

    let error = load_recipe_error(recipe);

    assert_contract_error_contains(error, "MissingRequiredSourceInput");
}

#[test]
fn source_card_renders_rounded_border_cells_with_border_roles() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]["borderStyle"]["value"]["value"] =
        serde_json::Value::String("rounded".to_string());
    recipe["sources"]["mainCard"]["inputs"]["width"]["value"]["value"] =
        serde_json::Value::Number(5.into());
    recipe["sources"]["mainCard"]["inputs"]["height"]["value"]["value"] =
        serde_json::Value::Number(3.into());
    recipe["graph"]["nodes"] = serde_json::json!({});
    recipe["graph"]["order"] = serde_json::json!([]);
    recipe["scenes"][0]["width"] = serde_json::Value::Number(5.into());
    recipe["scenes"][0]["height"] = serde_json::Value::Number(3.into());
    recipe["scenes"][0]["elements"][0]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, '╭');
    assert_eq!(frame.grid.cell((4, 0)).unwrap().ch, '╮');
    assert_eq!(frame.grid.cell((1, 1)).unwrap().ch, 'A');
    assert_eq!(
        frame.grid.role((0, 0)),
        Some(tui_vfx_types::RoleTag::Border)
    );
}

#[test]
fn source_card_renders_custom_border_frame() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]["borderStyle"]["value"]["value"] =
        serde_json::Value::String("custom".to_string());
    recipe["sources"]["mainCard"]["inputs"]["borderConfig"] = serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "structured",
            "value": {
                "frame": {
                    "corners": ["◢", "◣", "◥", "◤"],
                    "edges": ["▀", "█", "▄", "▌"]
                }
            }
        }
    });
    recipe["sources"]["mainCard"]["inputs"]["width"]["value"]["value"] =
        serde_json::Value::Number(5.into());
    recipe["sources"]["mainCard"]["inputs"]["height"]["value"]["value"] =
        serde_json::Value::Number(3.into());
    recipe["graph"]["nodes"] = serde_json::json!({});
    recipe["graph"]["order"] = serde_json::json!([]);
    recipe["scenes"][0]["width"] = serde_json::Value::Number(5.into());
    recipe["scenes"][0]["height"] = serde_json::Value::Number(3.into());
    recipe["scenes"][0]["elements"][0]["roleWritePolicy"] =
        serde_json::json!({ "kind": "copySampledSource" });

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, '◢');
    assert_eq!(frame.grid.cell((4, 0)).unwrap().ch, '◣');
    assert_eq!(frame.grid.cell((0, 2)).unwrap().ch, '◥');
    assert_eq!(frame.grid.cell((4, 2)).unwrap().ch, '◤');
    assert_eq!(frame.grid.cell((2, 0)).unwrap().ch, '▀');
    assert_eq!(frame.grid.cell((4, 1)).unwrap().ch, '█');
    assert_eq!(frame.grid.cell((2, 2)).unwrap().ch, '▄');
    assert_eq!(frame.grid.cell((0, 1)).unwrap().ch, '▌');
}

#[test]
fn source_card_uses_descriptor_color_defaults_when_optional_inputs_are_absent() {
    let mut recipe = linear_gradient_recipe_value();
    let inputs = recipe["sources"]["mainCard"]["inputs"]
        .as_object_mut()
        .expect("source inputs object");
    inputs.remove("foreground");
    inputs.remove("background");
    recipe["graph"]["nodes"] = serde_json::json!({});
    recipe["graph"]["order"] = serde_json::json!([]);

    let frame = render_recipe_value(recipe);

    let cell = frame.grid.cell((0, 0)).unwrap();
    assert_eq!(cell.fg, Color::WHITE);
    assert_eq!(cell.bg, Color::TRANSPARENT);
}

#[test]
fn rejects_wrong_source_card_literal_kinds_during_load() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]["width"]["value"] =
        serde_json::json!({ "kind": "text", "value": "wide" });

    let error = load_recipe_error(recipe);

    assert!(
        matches!(error, LoadError::Contract { .. }),
        "expected descriptor contract rejection for wrong width literal kind, got: {error}"
    );
}

#[test]
fn rejects_out_of_range_source_card_dimensions_at_load_time() {
    let mut wide_recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut wide_recipe, 513, 1);

    let wide_error = load_recipe_error(wide_recipe);

    assert_source_input_error(wide_error, "width");

    let mut tall_recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut tall_recipe, 1, 257);

    let tall_error = load_recipe_error(tall_recipe);

    assert_source_input_error(tall_error, "height");
}

#[test]
fn rejects_zero_and_negative_source_card_dimensions_at_load_time() {
    let mut zero_recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut zero_recipe, 0, 1);

    let zero_error = load_recipe_error(zero_recipe);

    assert_source_input_error(zero_error, "width");

    let mut negative_recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut negative_recipe, 1, -1);

    let negative_error = load_recipe_error(negative_recipe);

    assert_source_input_error(negative_error, "height");
}

#[test]
fn source_width_clips_independently_from_scene_width() {
    let mut recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut recipe, 4, 1);
    set_scene_dimension(&mut recipe, 2, 1);
    set_source_message(&mut recipe, "WXYZ");

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.width, 2);
    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'W');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'X');
}

#[test]
fn scene_width_can_exceed_source_width_without_source_reflow() {
    let mut recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut recipe, 2, 1);
    set_scene_dimension(&mut recipe, 4, 1);
    set_source_message(&mut recipe, "ABCD");

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.width, 4);
    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, 'B');
    assert_eq!(frame.grid.cell((2, 0)).unwrap().ch, ' ');
    assert_eq!(frame.grid.cell((3, 0)).unwrap().ch, ' ');
}

#[test]
fn source_card_message_preserves_line_boundaries() {
    let mut recipe = linear_gradient_recipe_value();
    set_source_dimension(&mut recipe, 2, 2);
    set_scene_dimension(&mut recipe, 2, 2);
    set_source_message(&mut recipe, "A\nB");

    let frame = render_recipe_value(recipe);

    assert_eq!(frame.grid.cell((0, 0)).unwrap().ch, 'A');
    assert_eq!(frame.grid.cell((1, 0)).unwrap().ch, ' ');
    assert_eq!(frame.grid.cell((0, 1)).unwrap().ch, 'B');
    assert_eq!(frame.grid.cell((1, 1)).unwrap().ch, ' ');
}

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_source_contract.rs</FILE> - <DESC>Compost source substrate tests</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
