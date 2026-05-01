// <FILE>crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs</FILE> - <DESC>Pure v3.1 RecipeDocument to compositor-next rendering tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Compositor-next pure v3.1 path: verify load-validated canonical recipes enter compositor-next directly without transition-seam code.</WCTX>
// <CLOG>0.2.0: MINOR add strict v3.1 load and canonical gradient-stop coverage.
// 0.1.0: INIT add direct v3.1 linearGradient recipe render.</CLOG>

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31LoadError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId, RecipeDocument};
use tui_vfx_types::Color;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives under <repo>/crates/tui-vfx-compositor-next")
        .to_path_buf()
}

fn primitive_catalog() -> DescriptorCatalog {
    let pack_path = repo_root().join("descriptors/v3.1/packs/primitive.json");
    let pack: DescriptorPack =
        serde_json::from_str(&fs::read_to_string(pack_path).expect("read primitive pack"))
            .expect("deserialize primitive pack");
    let mut packs = BTreeMap::new();
    packs.insert(DescriptorPackId::new("v3.1.primitive"), pack);
    DescriptorCatalog { packs }
}

fn linear_gradient_recipe_value() -> serde_json::Value {
    serde_json::json!({
        "id": "compositorNextDirectLinearGradient",
        "version": "3.1",
        "metadata": {
            "title": "Compositor Next Direct Linear Gradient",
            "description": "Minimal load-validated v3.1 direct compositor-next fixture",
            "authors": [],
            "tags": ["v3.1", "compositor-next", "linear-gradient"],
            "expectedVisual": "foreground gradient applied by compositor-next"
        },
        "lifecycle": null,
        "assets": {},
        "descriptorPacks": [{ "id": "v3.1.primitive" }],
        "sourceDescriptors": {},
        "sources": {
            "mainCard": {
                "source": "source.card",
                "inputs": {
                    "message": { "kind": "literal", "value": { "kind": "text", "value": "DIRECT V31" } },
                    "width": { "kind": "literal", "value": { "kind": "integer", "value": 8 } },
                    "height": { "kind": "literal", "value": { "kind": "integer", "value": 2 } },
                    "foreground": { "kind": "literal", "value": { "kind": "color", "value": { "r": 255, "g": 255, "b": 255, "a": 255 } } },
                    "background": { "kind": "literal", "value": { "kind": "color", "value": { "r": 0, "g": 0, "b": 0, "a": 255 } } },
                    "borderStyle": { "kind": "literal", "value": { "kind": "enum", "value": "none" } },
                    "borderTrim": { "kind": "literal", "value": { "kind": "enum", "value": "none" } }
                },
                "assets": {}
            }
        },
        "graph": {
            "id": "mainGraph",
            "version": "3.1",
            "parameters": {},
            "signals": {},
            "bindings": [],
            "effects": {},
            "nodes": {
                "gradient": {
                    "id": "gradient",
                    "effect": "shader.linearGradient",
                    "inputs": {
                        "startColor": { "kind": "literal", "value": { "kind": "color", "value": { "r": 255, "g": 0, "b": 0, "a": 255 } } },
                        "endColor": { "kind": "literal", "value": { "kind": "color", "value": { "r": 0, "g": 0, "b": 255, "a": 255 } } },
                        "colorSpace": { "kind": "literal", "value": { "kind": "enum", "value": "rgb" } },
                        "angleDeg": { "kind": "literal", "value": { "kind": "number", "value": 0.0 } },
                        "intensity": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
                        "applyTo": { "kind": "literal", "value": { "kind": "enum", "value": "foreground" } }
                    },
                    "outputs": {},
                    "activePhases": [],
                    "scope": { "kind": "all" },
                    "cellWritePolicy": "writeCell",
                    "roleWritePolicy": { "kind": "preserveDestination" }
                }
            },
            "order": ["gradient"],
            "topology": null
        },
        "scenes": [{
            "id": "mainScene",
            "width": 8,
            "height": 2,
            "elements": [{
                "id": "mainElement",
                "layer": "primary",
                "zIndex": 0,
                "placement": { "x": 0, "y": 0 },
                "source": "mainCard",
                "pipeline": null,
                "clipPolicy": "clip",
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }]
        }]
    })
}

fn recipe_from_value(value: serde_json::Value) -> RecipeDocument {
    serde_json::from_value(value).expect("canonical v3.1 recipe")
}

fn linear_gradient_recipe() -> RecipeDocument {
    recipe_from_value(linear_gradient_recipe_value())
}

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

fn highlighter_recipe_value() -> serde_json::Value {
    let mut recipe = linear_gradient_recipe_value();
    recipe["id"] = serde_json::Value::String("compositorNextDirectHighlighter".to_string());
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "text", "value": "HELLO" }
    });
    recipe["sources"]["mainCard"]["inputs"]["width"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 5 }
    });
    recipe["sources"]["mainCard"]["inputs"]["height"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 1 }
    });
    recipe["scenes"][0]["width"] = serde_json::Value::from(5);
    recipe["scenes"][0]["height"] = serde_json::Value::from(1);
    recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("nodes object")
        .remove("gradient");
    recipe["graph"]["nodes"]["highlighter"] = serde_json::json!({
        "id": "highlighter",
        "effect": "shader.highlighter",
        "inputs": {
            "color": { "kind": "literal", "value": { "kind": "color", "value": { "r": 255, "g": 0, "b": 0, "a": 255 } } },
            "bandWidth": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
            "blendStrength": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
            "textContrast": { "kind": "literal", "value": { "kind": "number", "value": 0.0 } },
            "mode": { "kind": "literal", "value": { "kind": "enum", "value": "band" } },
            "softEdge": { "kind": "literal", "value": { "kind": "boolean", "value": false } },
            "direction": { "kind": "literal", "value": { "kind": "enum", "value": "leftToRight" } },
            "rowMask": { "kind": "literal", "value": { "kind": "integer", "value": 0 } },
            "applyTo": { "kind": "literal", "value": { "kind": "enum", "value": "background" } }
        },
        "outputs": {},
        "activePhases": [],
        "scope": { "kind": "all" },
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    });
    recipe["graph"]["order"] = serde_json::json!(["highlighter"]);
    recipe
}

fn glisten_band_recipe_value() -> serde_json::Value {
    let mut recipe = linear_gradient_recipe_value();
    recipe["id"] = serde_json::Value::String("compositorNextDirectGlistenBand".to_string());
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "text", "value": "GLISTEN" }
    });
    recipe["sources"]["mainCard"]["inputs"]["width"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 7 }
    });
    recipe["sources"]["mainCard"]["inputs"]["height"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 1 }
    });
    recipe["scenes"][0]["width"] = serde_json::Value::from(7);
    recipe["scenes"][0]["height"] = serde_json::Value::from(1);
    recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("nodes object")
        .remove("gradient");
    recipe["graph"]["nodes"]["glisten"] = serde_json::json!({
        "id": "glisten",
        "effect": "shader.glistenBand",
        "inputs": {
            "color": { "kind": "literal", "value": { "kind": "color", "value": { "r": 255, "g": 0, "b": 0, "a": 255 } } },
            "bandWidth": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
            "direction": { "kind": "literal", "value": { "kind": "enum", "value": "leftToRight" } },
            "blendStrength": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
            "angleDeg": { "kind": "literal", "value": { "kind": "number", "value": 0.0 } },
            "speed": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } }
        },
        "outputs": {},
        "activePhases": [],
        "scope": { "kind": "all" },
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    });
    recipe["graph"]["order"] = serde_json::json!(["glisten"]);
    recipe
}

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

fn focus_field_recipe_value() -> serde_json::Value {
    let mut recipe = linear_gradient_recipe_value();
    recipe["id"] = serde_json::Value::String("compositorNextDirectFocusField".to_string());
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "text", "value": "FOCUS" }
    });
    recipe["sources"]["mainCard"]["inputs"]["width"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 5 }
    });
    recipe["sources"]["mainCard"]["inputs"]["height"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 1 }
    });
    recipe["scenes"][0]["width"] = serde_json::Value::from(5);
    recipe["scenes"][0]["height"] = serde_json::Value::from(1);
    recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("nodes object")
        .remove("gradient");
    recipe["graph"]["nodes"]["focusField"] = serde_json::json!({
        "id": "focusField",
        "effect": "shader.focusField",
        "inputs": {
            "color": { "kind": "literal", "value": { "kind": "color", "value": { "r": 0, "g": 0, "b": 255, "a": 255 } } },
            "centerX": { "kind": "literal", "value": { "kind": "number", "value": 2.0 } },
            "centerY": { "kind": "literal", "value": { "kind": "number", "value": 0.0 } },
            "radius": { "kind": "literal", "value": { "kind": "number", "value": 2.0 } },
            "intensity": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
            "applyTo": { "kind": "literal", "value": { "kind": "enum", "value": "background" } }
        },
        "outputs": {},
        "activePhases": [],
        "scope": { "kind": "all" },
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    });
    recipe["graph"]["order"] = serde_json::json!(["focusField"]);
    recipe
}

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

// <FILE>crates/tui-vfx-compositor-next/tests/test_v31_direct_recipe.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
