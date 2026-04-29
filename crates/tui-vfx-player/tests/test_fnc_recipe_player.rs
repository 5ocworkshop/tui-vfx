// <FILE>crates/tui-vfx-player/tests/test_fnc_recipe_player.rs</FILE> - <DESC>Contract-native skeleton player regression tests</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Styled-cell substrate work: keep player evidence tests portable and explicit.</WCTX>
// <CLOG>0.6.0: MINOR — require K2.5 styled primitive adapters to emit real styled-cell evidence.
// 0.5.1: PATCH — clarify styled-grid proof naming and recipe repo override.
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
    build_visual_frame_from_styled_grid, render_visual_frame_paths,
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
fn test_fnc_player_renders_k25_styled_primitive_adapters() {
    let player = player();
    for relative in [
        "styles/style_color_fade.json",
        "styles/style_role_scope_border.json",
        "shaders/primitives/shader_linear_gradient.json",
        "shaders/compositions/shader_border_sweep.json",
    ] {
        let recipe = recipe(&v31_debug_recipe(relative));
        let report = player.render_recipe(&recipe, &PlayerSampleRequest::default());

        assert_eq!(report.status, PlayerStatus::Rendered, "{relative}");
        assert!(report.errors.is_empty(), "{relative}");
        assert!(report.styled_grid.is_some(), "{relative}");
    }
}

#[test]
fn test_fnc_player_styled_primitive_visual_frames_are_style_known() {
    let frames = render_visual_frame_paths(
        &player(),
        vec![],
        &[v31_debug_recipe(
            "shaders/primitives/shader_linear_gradient.json",
        )],
        "test".to_string(),
        &PlayerSampleRequest::default(),
    );
    let frame = frames.frames.into_iter().next().expect("visual frame");

    assert_eq!(frame.status, PlayerStatus::Rendered);
    assert_eq!(frame.substrate, "styledCell");
    assert_eq!(frame.cell_source, "styledCells");
    assert!(frame.style_known);
    assert!(!frame.rows.is_empty());
    assert!(frame.unsupported_effect_ids.is_empty());
    assert!(frame.cells.iter().any(|cell| {
        cell.foreground != "defaultForeground"
            || cell.background != "transparent"
            || !cell.modifiers.is_empty()
            || cell.role.is_some()
    }));
}

#[test]
fn test_fnc_player_style_evidence_changes_hash_without_row_changes() {
    let player = player();
    let styled_recipe = recipe(&v31_debug_recipe("styles/style_color_fade.json"));
    let mut plain_value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("styles/style_color_fade.json"))
            .expect("read style recipe"),
    )
    .expect("style recipe json");
    plain_value["id"] = serde_json::json!("debugStyleColorFadePlain");
    plain_value["graph"]["nodes"] = serde_json::json!({});
    plain_value["graph"]["order"] = serde_json::json!([]);
    let plain_recipe = serde_json::from_value(plain_value).expect("plain style recipe");
    let request = PlayerSampleRequest::default();
    let styled = player.render_recipe(&styled_recipe, &request);
    let plain = player.render_recipe(&plain_recipe, &request);

    assert_eq!(styled.rows, plain.rows);
    assert_ne!(styled.render_hash, plain.render_hash);
    assert!(styled.styled_grid.is_some());
    assert!(plain.styled_grid.is_none());
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

#[test]
fn source_ansi_fixture_strips_sgr_sequences() {
    let recipe = recipe(&v31_debug_recipe("sources/source_ansi_sgr_basic.json"));
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert_eq!(report.rows[0].trim_end(), "SGR RED");
    assert!(!report.rows[0].contains('\u{1b}'));
}

#[test]
fn source_image_fixture_emits_fallback_warning() {
    let recipe = recipe(&v31_debug_recipe(
        "sources/source_image_binding_missing_asset.json",
    ));
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert!(report.rows[0].contains("[image fallback: missing-bound-"));
    assert!(
        report
            .warnings
            .iter()
            .any(|warning| warning.code == "imageFallbackRendered")
    );
}

#[test]
fn source_procedural_fixture_uses_bound_dots_spinner() {
    let recipe = recipe(&v31_debug_recipe(
        "sources/source_procedural_dots_spinner_binding.json",
    ));
    let report = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            loop_t: Some(0.25),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert!(report.rows[0].contains("dots spinner"));
}

#[test]
fn scene_layer_local_pipeline_preserves_placed_style_evidence() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_layer_surface_base_style.json",
    ));
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert_eq!(report.rows[0].trim_end(), "BASE STYLE LOCAL");
    let grid = report.styled_grid.expect("styled scene output");
    assert_any_foreground(&grid, "rgba(255,80,160,255)");
}

#[test]
fn graph_sequence_value_output_is_consumed_by_later_node() {
    let recipe = graph_recipe(
        serde_json::json!({
            "publishStrength": {
                "id": "publishStrength",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.85),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "tintStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "consumeStrength": {
                "id": "consumeStrength",
                "effect": "filter.tint",
                "inputs": {
                    "strength": graph_number_source("tintStrength", 0.0),
                    "color": color_source(255, 0, 0),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {},
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!(["publishStrength", "consumeStrength"]),
        Some(serde_json::json!({
            "kind": "sequence",
            "children": [
                { "kind": "node", "node": "publishStrength" },
                { "kind": "node", "node": "consumeStrength" }
            ]
        })),
        &["filter.dim", "filter.tint"],
    );

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert!(report.warnings.is_empty());
    let grid = report.styled_grid.expect("styled graph output");
    assert_any_foreground(&grid, "rgba(255,38,38,255)");
}

#[test]
fn graph_parallel_branch_value_is_visible_after_join() {
    let recipe = graph_recipe(
        serde_json::json!({
            "publishStrength": {
                "id": "publishStrength",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.75),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "joinedStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "consumeAfterJoin": {
                "id": "consumeAfterJoin",
                "effect": "filter.tint",
                "inputs": {
                    "strength": graph_number_source("joinedStrength", 0.0),
                    "color": color_source(0, 255, 0),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {},
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!(["publishStrength", "consumeAfterJoin"]),
        Some(serde_json::json!({
            "kind": "sequence",
            "children": [
                {
                    "kind": "parallel",
                    "mergePolicy": "childOrderLastWriterWins",
                    "valueMergePolicy": "childOrderLastWriterWins",
                    "children": [{ "kind": "node", "node": "publishStrength" }]
                },
                { "kind": "node", "node": "consumeAfterJoin" }
            ]
        })),
        &["filter.dim", "filter.tint"],
    );

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    let grid = report.styled_grid.expect("styled graph output");
    assert_any_foreground(&grid, "rgba(64,255,64,255)");
}

#[test]
fn graph_parallel_branch_output_is_not_visible_inside_sibling_branch() {
    let recipe = graph_recipe(
        serde_json::json!({
            "publishStrength": {
                "id": "publishStrength",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.8),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "branchStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "siblingProbe": {
                "id": "siblingProbe",
                "effect": "filter.tint",
                "inputs": {
                    "strength": graph_number_source("branchStrength", 0.0),
                    "color": color_source(0, 0, 255),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "siblingObservedStrength": { "source": { "kind": "input", "id": "strength" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "consumeSiblingObservation": {
                "id": "consumeSiblingObservation",
                "effect": "filter.tint",
                "inputs": {
                    "strength": graph_number_source("siblingObservedStrength", 1.0),
                    "color": color_source(255, 0, 0),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {},
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!([
            "publishStrength",
            "siblingProbe",
            "consumeSiblingObservation"
        ]),
        Some(serde_json::json!({
            "kind": "sequence",
            "children": [
                {
                    "kind": "parallel",
                    "mergePolicy": "childOrderLastWriterWins",
                    "valueMergePolicy": "childOrderLastWriterWins",
                    "children": [
                        { "kind": "node", "node": "publishStrength" },
                        { "kind": "node", "node": "siblingProbe" }
                    ]
                },
                { "kind": "node", "node": "consumeSiblingObservation" }
            ]
        })),
        &["filter.dim", "filter.tint"],
    );

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    let grid = report.styled_grid.expect("styled graph output");
    assert_any_foreground(&grid, "rgba(255,255,255,255)");
    assert_no_foreground(&grid, "rgba(255,51,51,255)");
}

#[test]
fn graph_parallel_conflicts_emit_deterministic_warnings() {
    let recipe = graph_recipe(
        serde_json::json!({
            "first": {
                "id": "first",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.2),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "sharedStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "second": {
                "id": "second",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.9),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "sharedStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!(["first", "second"]),
        Some(serde_json::json!({
            "kind": "parallel",
            "mergePolicy": "childOrderLastWriterWins",
            "valueMergePolicy": "childOrderLastWriterWins",
            "children": [
                { "kind": "node", "node": "first" },
                { "kind": "node", "node": "second" }
            ]
        })),
        &["filter.dim"],
    );

    let first = player().render_recipe(&recipe, &PlayerSampleRequest::default());
    let second = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(first.status, PlayerStatus::Rendered);
    assert!(first.errors.is_empty());
    assert!(
        first
            .warnings
            .iter()
            .any(|warning| warning.code == "parallelGraphValueConflict")
    );
    assert!(
        first
            .warnings
            .iter()
            .any(|warning| warning.code == "parallelSurfaceConflict")
    );
    assert_eq!(first.warnings, second.warnings);
}

#[test]
fn graph_order_fallback_still_executes_without_topology() {
    let recipe = graph_recipe(
        serde_json::json!({
            "orderedTint": {
                "id": "orderedTint",
                "effect": "filter.tint",
                "inputs": {
                    "strength": number_source(1.0),
                    "color": color_source(255, 0, 0),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {},
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!(["orderedTint"]),
        None,
        &["filter.tint"],
    );

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert!(report.styled_grid.is_some());
}

#[test]
fn unknown_topology_node_fails_with_structured_contract_diagnostic() {
    let mut value = graph_recipe_value(
        serde_json::json!({}),
        serde_json::json!([]),
        Some(serde_json::json!({ "kind": "node", "node": "missingNode" })),
        &[],
    );
    value["graph"]["nodes"] = serde_json::json!({});
    let recipe: RecipeDocument = serde_json::from_value(value).expect("recipe shape");

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Error);
    assert_eq!(report.errors[0].code, "contractValidationFailed");
    assert!(report.errors[0].message.contains("UnknownOrderNode"));
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

fn assert_any_foreground(grid: &PlayerStyledGrid, expected: &str) {
    assert!(
        grid.cells().iter().any(|cell| cell.foreground == expected),
        "expected at least one foreground {expected}; actual foregrounds: {:?}",
        grid.cells()
            .iter()
            .map(|cell| cell.foreground.as_str())
            .collect::<Vec<_>>()
    );
}

fn assert_no_foreground(grid: &PlayerStyledGrid, forbidden: &str) {
    assert!(
        grid.cells().iter().all(|cell| cell.foreground != forbidden),
        "did not expect foreground {forbidden}"
    );
}

fn graph_recipe(
    nodes: serde_json::Value,
    order: serde_json::Value,
    topology: Option<serde_json::Value>,
    effect_ids: &[&str],
) -> RecipeDocument {
    serde_json::from_value(graph_recipe_value(nodes, order, topology, effect_ids))
        .expect("graph recipe")
}

fn graph_recipe_value(
    nodes: serde_json::Value,
    order: serde_json::Value,
    topology: Option<serde_json::Value>,
    effect_ids: &[&str],
) -> serde_json::Value {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("baseline.json")).expect("read baseline recipe"),
    )
    .expect("baseline json");
    let pack: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(descriptor_pack_path()).expect("pack json"))
            .expect("descriptor pack json");
    let effects = effect_ids
        .iter()
        .map(|id| ((*id).to_string(), pack["effects"][*id].clone()))
        .collect::<serde_json::Map<_, _>>();
    value["id"] = serde_json::json!("debugGraphExecutorTest");
    value["graph"]["effects"] = serde_json::Value::Object(effects);
    value["graph"]["nodes"] = nodes;
    value["graph"]["order"] = order;
    value["graph"]["topology"] = topology.unwrap_or(serde_json::Value::Null);
    value
}

fn descriptor_pack_path() -> PathBuf {
    workspace_root().join("descriptors/v3.1/packs/primitive.json")
}

fn number_source(value: f64) -> serde_json::Value {
    serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": value } })
}

fn enum_source(value: &str) -> serde_json::Value {
    serde_json::json!({ "kind": "literal", "value": { "kind": "enum", "value": value } })
}

fn color_source(r: u8, g: u8, b: u8) -> serde_json::Value {
    serde_json::json!({
        "kind": "literal",
        "value": { "kind": "color", "value": { "r": r, "g": g, "b": b, "a": 255 } }
    })
}

fn graph_number_source(id: &str, fallback: f64) -> serde_json::Value {
    serde_json::json!({
        "kind": "graphValue",
        "id": id,
        "fallback": { "kind": "number", "value": fallback }
    })
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
// <VERS>END OF VERSION: 0.6.0</VERS>
