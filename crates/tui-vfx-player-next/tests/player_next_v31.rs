// <FILE>crates/tui-vfx-player-next/tests/player_next_v31.rs</FILE> - <DESC>Player-next direct v3.1 vertical path tests</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Lock player-next to compositor-next v31 load/render without legacy acceptance.</WCTX>
// <CLOG>0.1.0: INIT — cover successful v3.1 render and rejection of non-v3.1/runtime-sourced inputs.</CLOG>

use std::{collections::BTreeMap, fs, path::PathBuf};

use tui_vfx_compositor_next::v31::{V31LoadError, V31SampleContext};
use tui_vfx_contract::{DescriptorCatalog, DescriptorPack, DescriptorPackId, RecipeDocument};
use tui_vfx_player_next::{load_player_next_recipe, render_player_next_recipe};

#[test]
fn player_next_loads_and_renders_v31_recipe_with_descriptor_catalog() {
    let catalog = descriptor_catalog();
    let loaded = load_player_next_recipe(v31_recipe(), &catalog).expect("v3.1 recipe loads");

    assert_eq!(loaded.recipe().version, "3.1");

    let frame = render_player_next_recipe(&loaded, &V31SampleContext { phase_t: 0.25 })
        .expect("v3.1 recipe renders");

    assert_eq!(frame.recipe_id, "playerNextGradientSmoke");
    assert_eq!((frame.width, frame.height), (4, 1));
    assert_eq!(frame.applied_effect_kinds, ["shader.linearGradient"]);
}

#[test]
fn player_next_rejects_non_v31_recipe() {
    let catalog = descriptor_catalog();
    let mut recipe = v31_recipe();
    recipe.version = "3.0".to_string();

    let error = load_player_next_recipe(recipe, &catalog).expect_err("non-v3.1 is rejected");

    assert!(matches!(
        error,
        V31LoadError::UnsupportedVersion {
            recipe_version,
            graph_version
        } if recipe_version == "3.0" && graph_version == "3.1"
    ));
}

#[test]
fn player_next_rejects_runtime_sourced_inputs_at_load() {
    let catalog = descriptor_catalog();
    let mut value = v31_recipe_json();
    value["graph"]["nodes"]["gradient"]["inputs"]["intensity"] = serde_json::json!({
        "kind": "signal",
        "id": "hostIntensity",
        "fallback": { "kind": "number", "value": 1.0 }
    });
    value["graph"]["signals"] = serde_json::json!({
        "hostIntensity": {
            "id": "hostIntensity",
            "displayName": "Host intensity",
            "description": null,
            "value": { "kind": "number", "default": { "kind": "number", "value": 1.0 }, "range": { "min": 0.0, "max": 1.0 }, "allowedValues": [], "unit": null, "semantic": null },
            "required": false
        }
    });
    let recipe: RecipeDocument = serde_json::from_value(value).expect("runtime-sourced recipe");

    let error = load_player_next_recipe(recipe, &catalog).expect_err("runtime input is rejected");

    assert!(matches!(
        error,
        V31LoadError::UnsupportedDirectInput { input, .. } if input == "intensity"
    ));
}

fn descriptor_catalog() -> DescriptorCatalog {
    let path = repo_path("descriptors/v3.1/packs/primitive.json");
    let pack: DescriptorPack =
        serde_json::from_str(&fs::read_to_string(path).expect("read primitive descriptor pack"))
            .expect("primitive descriptor pack json");
    let mut packs = BTreeMap::new();
    packs.insert(DescriptorPackId::new("v3.1.primitive"), pack);
    DescriptorCatalog { packs }
}

fn v31_recipe() -> RecipeDocument {
    serde_json::from_value(v31_recipe_json()).expect("v3.1 recipe json")
}

fn v31_recipe_json() -> serde_json::Value {
    serde_json::json!({
        "id": "playerNextGradientSmoke",
        "version": "3.1",
        "metadata": { "title": "Player Next Gradient Smoke", "description": null, "authors": [], "expectedVisual": null, "tags": [] },
        "lifecycle": null,
        "assets": {},
        "descriptorPacks": [{ "id": "v3.1.primitive" }],
        "sourceDescriptors": {},
        "sources": {
            "message": {
                "source": "source.text",
                "inputs": {
                    "text": { "kind": "literal", "value": { "kind": "text", "value": "TEST" } },
                    "width": { "kind": "literal", "value": { "kind": "integer", "value": 4 } },
                    "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
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
                    "scope": null,
                    "cellWritePolicy": null,
                    "roleWritePolicy": null
                }
            },
            "order": ["gradient"],
            "topology": null
        },
        "scenes": [{
            "id": "mainScene",
            "width": 4,
            "height": 1,
            "elements": [{
                "id": "textElement",
                "layer": null,
                "zIndex": 0,
                "placement": { "x": 0, "y": 0 },
                "source": "message",
                "pipeline": null,
                "clipPolicy": "clip",
                "cellWritePolicy": "skipTransparentEmpty",
                "roleWritePolicy": { "kind": "copySampledSource" }
            }]
        }]
    })
}

fn repo_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

// <FILE>crates/tui-vfx-player-next/tests/player_next_v31.rs</FILE> - <DESC>Player-next direct v3.1 vertical path tests</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
