// <FILE>crates/tui-vfx-compost/tests/direct_recipe/support.rs</FILE> - <DESC>Shared fixtures for compost direct recipe tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep canonical v3.1 recipe fixtures separate from per-primitive tests.</WCTX>
// <CLOG>0.1.0: INIT — add linearGradient fixture for compost RED tests.</CLOG>

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId, RecipeDocument};

pub(crate) fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate lives under <repo>/crates/tui-vfx-compost")
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

pub(crate) fn recipe_from_value(value: serde_json::Value) -> RecipeDocument {
    serde_json::from_value(value).expect("canonical v3.1 recipe")
}

pub(crate) fn linear_gradient_recipe_value() -> serde_json::Value {
    serde_json::json!({
        "id": "compostDirectLinearGradient",
        "version": "3.1",
        "metadata": {
            "title": "Compost Direct Linear Gradient",
            "description": "Minimal v3.1 native compost fixture",
            "authors": [],
            "tags": ["v3.1", "compost", "linear-gradient"],
            "expectedVisual": "foreground gradient applied by compost"
        },
        "lifecycle": null,
        "assets": {},
        "descriptorPacks": [{ "id": "v3.1.primitive" }],
        "sourceDescriptors": {},
        "sources": {
            "mainCard": {
                "source": "source.card",
                "inputs": {
                    "message": { "kind": "literal", "value": { "kind": "text", "value": "ABC" } },
                    "width": { "kind": "literal", "value": { "kind": "integer", "value": 3 } },
                    "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } },
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
                        "gradient": {
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
                        },
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
            "width": 3,
            "height": 1,
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

// <FILE>crates/tui-vfx-compost/tests/direct_recipe/support.rs</FILE> - <DESC>Shared fixtures for compost direct recipe tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
