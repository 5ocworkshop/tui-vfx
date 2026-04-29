// <FILE>crates/tui-vfx-player/tests/test_fnc_recipe_player.rs</FILE> - <DESC>Contract-native skeleton player regression tests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K0: lock canonical source.text rendering after acceptance rejection.</WCTX>
// <CLOG>0.2.0: PATCH — add source.text text-input regression coverage.
// 0.1.0: INIT — add primitive render, deterministic hash, unsupported effect, and session latch coverage.</CLOG>

use std::{collections::BTreeMap, fs, path::Path};

use tui_vfx_contract::{
    DescriptorCatalog, DescriptorPack, DescriptorPackId, RecipeDocument, SignalId, Value,
};
use tui_vfx_player::{PlayerSampleRequest, PlayerSession, PlayerStatus, RecipePlayer};

#[test]
fn test_fnc_player_renders_baseline_with_stable_hash() {
    let player = player();
    let recipe = recipe("/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json");
    let request = PlayerSampleRequest::default();

    let first = player.render_recipe(&recipe, &request);
    let second = player.render_recipe(&recipe, &request);

    assert_eq!(first.status, PlayerStatus::Rendered);
    assert!(first.non_empty_cells > 0);
    assert_eq!(first.render_hash, second.render_hash);
    assert_eq!(first.rows, second.rows);
}

#[test]
fn test_fnc_player_renders_source_text_from_text_input() {
    let report = player().render_recipe(&source_text_recipe(), &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.non_empty_cells > 0);
    assert_eq!(report.rows[0].trim_end(), "HELLO TEXT");
}

#[test]
fn test_fnc_player_reports_unsupported_effect_adapter() {
    let player = player();
    let recipe =
        recipe("/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_dissolve.json");

    let report = player.render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Unsupported);
    assert!(
        report
            .errors
            .iter()
            .any(|error| error.code == "unsupportedEffectAdapter")
    );
}

#[test]
fn test_fnc_player_session_latches_event_driven_dwell() {
    let player = player();
    let recipe = recipe(
        "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json",
    );
    let mut session = PlayerSession::new();
    let mut request = PlayerSampleRequest::default();

    let first = session.render(&player, &recipe, &request);
    assert!(!first.dwell_terminated);

    request
        .signals
        .insert(SignalId::new("userDismissed"), Value::Boolean(true));
    let fired = session.render(&player, &recipe, &request);
    assert!(fired.dwell_terminated);

    request.signals = BTreeMap::new();
    let latched = session.render(&player, &recipe, &request);
    assert!(latched.dwell_terminated);

    session.reset();
    let reset = session.render(&player, &recipe, &request);
    assert!(!reset.dwell_terminated);
}

fn source_text_recipe() -> RecipeDocument {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(
            "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json",
        )
        .expect("read baseline recipe"),
    )
    .expect("baseline json");
    value["id"] = serde_json::json!("debugTextSource");
    value["sourceDescriptors"] = serde_json::json!({
        "source.text": {
            "id": "source.text",
            "version": "0.1.0",
            "displayName": "Text Source",
            "category": "debug",
            "kind": { "kind": "text" },
            "inputs": {
                "text": {
                    "displayName": "Text",
                    "description": "Text rendered into a source-produced surface.",
                    "value": {
                        "kind": "text",
                        "default": null,
                        "range": null,
                        "allowedValues": [],
                        "unit": null,
                        "semantic": null
                    },
                    "bindable": true,
                    "runtimeMutability": "runtime"
                }
            },
            "assets": {},
            "output": {
                "size": { "kind": "inputDriven" },
                "roles": { "kind": "defaultRole", "role": "Text" }
            },
            "lifecycle": {
                "deterministicWithSeed": true,
                "timeAware": false,
                "resizeAware": true
            }
        }
    });
    value["sources"]["mainCard"] = serde_json::json!({
        "source": "source.text",
        "inputs": {
            "text": { "kind": "literal", "value": { "kind": "text", "value": "HELLO TEXT" } }
        },
        "assets": {}
    });
    serde_json::from_value(value).expect("source.text recipe")
}

fn player() -> RecipePlayer {
    RecipePlayer::new(catalog())
}

fn catalog() -> DescriptorCatalog {
    let pack = descriptor_pack(&workspace_root().join("descriptors/v3.1/packs/primitive.json"));
    let mut packs = BTreeMap::new();
    packs.insert(DescriptorPackId::new("v3.1.primitive"), pack);
    DescriptorCatalog { packs }
}

fn descriptor_pack(path: &Path) -> DescriptorPack {
    serde_json::from_str(&fs::read_to_string(path).expect("read descriptor pack"))
        .expect("deserialize descriptor pack")
}

fn recipe(path: &str) -> RecipeDocument {
    serde_json::from_str(&fs::read_to_string(Path::new(path)).expect("read recipe"))
        .expect("deserialize recipe")
}

fn workspace_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

// <FILE>crates/tui-vfx-player/tests/test_fnc_recipe_player.rs</FILE> - <DESC>Contract-native skeleton player regression tests</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
