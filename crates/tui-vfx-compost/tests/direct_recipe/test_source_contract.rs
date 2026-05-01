// <FILE>crates/tui-vfx-compost/tests/direct_recipe/test_source_contract.rs</FILE> - <DESC>Compost source substrate tests</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Source substrate tests ensure source descriptors validate at load and materialization stays separate from scene dimensions.</WCTX>
// <CLOG>0.2.1: PATCH — tighten source-input diagnostic assertions.
// 0.2.0: PATCH — add source.card dimension bounds, literal shape, and multiline regressions.
// 0.1.1: PATCH — name repeated source message fixture mutation.</CLOG>

use crate::support::{linear_gradient_recipe_value, primitive_catalog, recipe_from_value};
use tui_vfx_compost::{LoadError, LoadedRecipe, SampleContext, render_recipe};

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
fn rejects_unsupported_source_descriptor_id_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["sourceDescriptor"] =
        serde_json::Value::String("source.text".to_string());
    recipe["sources"]["mainCard"]["inputs"] = serde_json::json!({
        "text": { "kind": "literal", "value": { "kind": "text", "value": "text source" } },
        "width": { "kind": "literal", "value": { "kind": "integer", "value": 3 } },
        "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
    });

    let error = load_recipe_error(recipe);

    assert!(matches!(
        error,
        LoadError::UnsupportedSourceDescriptor {
            source_id,
            descriptor,
            ..
        } if source_id == "mainCard" && descriptor == "source.text"
    ));
}

#[test]
fn rejects_runtime_sourced_source_inputs_at_load_time() {
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

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "message");
}

#[test]
fn rejects_unsupported_source_card_inputs_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]["borderStyle"]["value"]["value"] =
        serde_json::Value::String("rounded".to_string());

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "borderStyle");
}

#[test]
fn rejects_missing_required_source_card_input_at_load_time() {
    let mut recipe = linear_gradient_recipe_value();
    recipe["sources"]["mainCard"]["inputs"]
        .as_object_mut()
        .expect("source inputs object")
        .remove("foreground");

    let error = load_recipe_error(recipe);

    assert_source_input_error(error, "foreground");
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
// <VERS>END OF VERSION: 0.2.1</VERS>
