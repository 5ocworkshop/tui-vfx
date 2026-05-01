// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/support.rs</FILE> - <DESC>Shared fixtures for direct v3.1 recipe tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep large direct recipe fixtures out of per-primitive test files while preserving one test harness entrypoint.</WCTX>
// <CLOG>0.1.0: INIT — extract direct v3.1 recipe fixtures.</CLOG>

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId, RecipeDocument};

pub(crate) fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives under <repo>/crates/tui-vfx-compositor-next")
        .to_path_buf()
}

pub(crate) fn primitive_catalog() -> DescriptorCatalog {
    let pack_path = repo_root().join("descriptors/v3.1/packs/primitive.json");
    let pack: DescriptorPack =
        serde_json::from_str(&fs::read_to_string(pack_path).expect("read primitive pack"))
            .expect("deserialize primitive pack");
    let mut packs = BTreeMap::new();
    packs.insert(DescriptorPackId::new("v3.1.primitive"), pack);
    DescriptorCatalog { packs }
}

pub(crate) fn linear_gradient_recipe_value() -> serde_json::Value {
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

pub(crate) fn recipe_from_value(value: serde_json::Value) -> RecipeDocument {
    serde_json::from_value(value).expect("canonical v3.1 recipe")
}

pub(crate) fn linear_gradient_recipe() -> RecipeDocument {
    recipe_from_value(linear_gradient_recipe_value())
}

pub(crate) fn highlighter_recipe_value() -> serde_json::Value {
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

pub(crate) fn glisten_band_recipe_value() -> serde_json::Value {
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
pub(crate) fn focus_field_recipe_value() -> serde_json::Value {
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

pub(crate) fn border_sweep_recipe_value() -> serde_json::Value {
    let mut recipe = linear_gradient_recipe_value();
    recipe["id"] = serde_json::Value::String("compositorNextDirectBorderSweep".to_string());
    recipe["sources"]["mainCard"]["inputs"]["message"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "text", "value": "ABCDE\nFGHIJ\nKLMNO" }
    });
    recipe["sources"]["mainCard"]["inputs"]["width"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 5 }
    });
    recipe["sources"]["mainCard"]["inputs"]["height"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "integer", "value": 3 }
    });
    recipe["scenes"][0]["width"] = serde_json::Value::from(5);
    recipe["scenes"][0]["height"] = serde_json::Value::from(3);
    recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("nodes object")
        .remove("gradient");
    recipe["graph"]["nodes"]["borderSweep"] = serde_json::json!({
        "id": "borderSweep",
        "effect": "shader.borderSweep",
        "inputs": {
            "color": { "kind": "literal", "value": { "kind": "color", "value": { "r": 0, "g": 255, "b": 255, "a": 255 } } },
            "speed": { "kind": "literal", "value": { "kind": "number", "value": 1.0 } },
            "length": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
        },
        "outputs": {},
        "activePhases": [],
        "scope": { "kind": "all" },
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    });
    recipe["graph"]["order"] = serde_json::json!(["borderSweep"]);
    recipe
}

// <FILE>crates/tui-vfx-compositor-next/tests/v31_direct_recipe/support.rs</FILE> - <DESC>Shared fixtures for direct v3.1 recipe tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
