// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_load_contract.rs</FILE> - <DESC>Direct v3.1 load/source contract tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Direct v3.1 tests are split by concern/primitive to preserve OFPF file size discipline.</WCTX>
// <CLOG>0.1.0: INIT — extract Direct v3.1 load/source contract tests.</CLOG>

use tui_vfx_compositor_next::v31::{LoadedV31Recipe, V31LoadError};

use super::support::*;

#[test]
fn rejects_non_v31_recipe_before_rendering() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["version"] = serde_json::Value::String("3.2".to_string());

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects future recipe versions");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedVersion {
            recipe_version,
            graph_version,
        } if recipe_version == "3.2" && graph_version == "3.1"
    ));
}

#[test]
fn rejects_runtime_sourced_source_style_inputs_at_load_time() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["foregroundColor"] = serde_json::json!({
        "id": "foregroundColor",
        "displayName": "Foreground Color",
        "description": null,
        "value": {
            "kind": "color",
            "default": { "kind": "color", "value": { "r": 255, "g": 255, "b": 255, "a": 255 } },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": "foreground"
        },
        "bindable": true
    });
    recipe["sources"]["mainCard"]["inputs"]["foreground"] = serde_json::json!({
        "kind": "parameter",
        "id": "foregroundColor",
        "fallback": { "kind": "color", "value": { "r": 255, "g": 255, "b": 255, "a": 255 } }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects runtime-sourced source styling inputs");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedSourceInput {
            source_id,
            input,
            ..
        } if source_id == "mainCard" && input == "foreground"
    ));
}

#[test]
fn rejects_runtime_sourced_source_inputs_at_load_time() {
    let catalog = primitive_catalog();
    let mut recipe = linear_gradient_recipe_value();
    recipe["graph"]["parameters"]["messageText"] = serde_json::json!({
        "id": "messageText",
        "displayName": "Message Text",
        "description": null,
        "value": {
            "kind": "text",
            "default": { "kind": "text", "value": "DIRECT V31" },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": "label"
        },
        "bindable": true
    });
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "parameter",
        "id": "messageText",
        "fallback": { "kind": "text", "value": "DIRECT V31" }
    });

    let err = LoadedV31Recipe::load(recipe_from_value(recipe), &catalog)
        .expect_err("direct v3.1 load rejects unresolved source inputs");

    assert!(matches!(
        err,
        V31LoadError::UnsupportedSourceInput {
            source_id,
            input,
            ..
        } if source_id == "mainCard" && input == "message"
    ));
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/test_load_contract.rs</FILE> - <DESC>Direct v3.1 load/source contract tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
