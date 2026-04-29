// <FILE>crates/tui-vfx-player/tests/test_fnc_recipe_player.rs</FILE> - <DESC>Contract-native skeleton player regression tests</DESC>
// <VERS>VERSION: 0.5.1</VERS>
// <WCTX>Styled-cell substrate work: keep player evidence tests portable and explicit.</WCTX>
// <CLOG>0.5.1: PATCH — clarify styled-grid proof naming and recipe repo override.
// 0.5.0: MINOR — assert styled-cell visual frames can carry non-default style evidence.
// 0.4.0: MINOR — assert newly supported text-grid adapters produce player row evidence.
// 0.3.0: PATCH — use project-derived recipe paths and switch unsupported adapter regression away from newly supported dissolve.
// 0.2.0: PATCH — add source.text text-input regression coverage.
// 0.1.0: INIT — add primitive render, deterministic hash, unsupported effect, and session latch coverage.</CLOG>

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use tui_vfx_contract::{
    DescriptorCatalog, DescriptorPack, DescriptorPackId, RecipeDocument, SignalId, Value,
};
use tui_vfx_player::{
    PlayerSampleRequest, PlayerSession, PlayerStatus, PlayerStyledGrid, RecipePlayer,
    build_visual_frame_from_styled_grid,
};

#[test]
fn test_fnc_player_renders_baseline_with_stable_hash() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("baseline.json"));
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
fn test_fnc_player_styled_visual_frame_carries_real_style_evidence() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("baseline.json"));
    let report = player.render_recipe(&recipe, &PlayerSampleRequest::default());
    let mut styled_grid = PlayerStyledGrid::from_rows(&report.rows);

    styled_grid.set_cell_style(
        0,
        0,
        "ansi.red",
        "ansi.blue",
        vec!["bold".to_string()],
        Some("Title".to_string()),
    );

    let frame = build_visual_frame_from_styled_grid(report, styled_grid);
    let styled_cell = frame
        .cells
        .iter()
        .find(|cell| cell.x == 0 && cell.y == 0)
        .expect("styled cell evidence");

    assert_eq!(frame.substrate, "styledCell");
    assert_eq!(frame.cell_source, "styledCells");
    assert!(frame.style_known);
    assert!(!frame.rows.is_empty());
    assert_eq!(styled_cell.foreground, "ansi.red");
    assert_eq!(styled_cell.background, "ansi.blue");
    assert_eq!(styled_cell.modifiers, vec!["bold".to_string()]);
    assert_eq!(styled_cell.role.as_deref(), Some("Title"));
}

#[test]
fn test_fnc_player_dissolve_adapter_changes_row_evidence_by_phase() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("masks/mask_dissolve.json"));
    let hidden = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase_t: 0.0,
            ..PlayerSampleRequest::default()
        },
    );
    let revealed = player.render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(hidden.status, PlayerStatus::Rendered);
    assert_eq!(revealed.status, PlayerStatus::Rendered);
    assert_eq!(hidden.non_empty_cells, 0);
    assert!(revealed.non_empty_cells > hidden.non_empty_cells);
    assert_ne!(hidden.rows, revealed.rows);
    assert_ne!(hidden.render_hash, revealed.render_hash);
}

#[test]
fn test_fnc_player_ripple_adapter_changes_row_evidence_by_loop_time() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("samplers/sampler_ripple.json"));
    let first = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            loop_t: Some(0.0),
            ..PlayerSampleRequest::default()
        },
    );
    let second = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            loop_t: Some(0.25),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(first.status, PlayerStatus::Rendered);
    assert_eq!(second.status, PlayerStatus::Rendered);
    assert!(first.non_empty_cells > 0);
    assert!(second.non_empty_cells > 0);
    assert_ne!(first.rows, second.rows);
    assert_ne!(first.render_hash, second.render_hash);
}

#[test]
fn test_fnc_player_reports_unsupported_effect_adapter() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe(
        "shaders/primitives/shader_linear_gradient.json",
    ));

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
    let recipe = recipe(&v31_debug_recipe(
        "event_driven_dwell/bool_binding_demo.json",
    ));
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
        &fs::read_to_string(v31_debug_recipe("baseline.json")).expect("read baseline recipe"),
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

fn recipe(path: &Path) -> RecipeDocument {
    serde_json::from_str(&fs::read_to_string(path).expect("read recipe"))
        .expect("deserialize recipe")
}

fn v31_debug_recipe(relative: &str) -> PathBuf {
    recipe_repo_root()
        .join("recipes/v3.1/debug_recipes")
        .join(relative)
}

fn recipe_repo_root() -> PathBuf {
    if let Ok(path) = std::env::var("RECIPE_REPO") {
        return PathBuf::from(path);
    }

    workspace_root()
        .parent()
        .expect("workspace parent")
        .join("tui-vfx-recipes")
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

// <FILE>crates/tui-vfx-player/tests/test_fnc_recipe_player.rs</FILE> - <DESC>Contract-native skeleton player regression tests</DESC>
// <VERS>END OF VERSION: 0.5.1</VERS>
