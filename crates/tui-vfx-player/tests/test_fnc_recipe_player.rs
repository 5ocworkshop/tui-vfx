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
    DescriptorCatalog, DescriptorPack, DescriptorPackId, GraphValueId, RecipeDocument, SignalId,
    Value,
};
use tui_vfx_player::{
    PlayerRenderBackend, PlayerSampleRequest, PlayerSession, PlayerStatus, PlayerStyledGrid,
    RecipePlayer, StyledCellRenderBackend, TextGridRenderBackend,
    build_visual_frame_from_styled_grid,
    fnc_render_scene::render_scene_with_source_asset_resolver,
    fnc_resolve_source_asset::{
        PlayerSourceAssetRequest, PlayerSourceAssetResolution, PlayerSourceAssetResolver,
    },
    render_visual_frame_paths,
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
fn test_fnc_player_render_ir_carries_rows_styles_provenance_and_graph_values() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe(
        "complex/graph_io_sequence_filter_to_tint.json",
    ));
    let report = player.render_recipe_ir(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.schema_version, "v3.1.player.renderIr.1");
    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(!report.rows.is_empty());
    assert_eq!(
        report.render_hash,
        player
            .render_recipe(&recipe, &PlayerSampleRequest::default())
            .render_hash
    );
    assert!(
        report
            .styled_cells
            .iter()
            .any(|cell| cell.foreground != "defaultForeground"),
        "render IR should carry styled-cell evidence"
    );
    assert!(
        report
            .provenance
            .iter()
            .any(|entry| entry.source_id.as_deref() == Some("mainText")
                && entry.source_descriptor_id.as_deref() == Some("source.text")),
        "render IR should carry scene/source provenance"
    );
    assert!(
        report
            .graph_values
            .iter()
            .any(|value| value.id == "sharedStrength"),
        "render IR should carry graph value snapshots"
    );
}

#[test]
fn test_fnc_player_source_render_ir_excludes_recipe_graph_effects() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("filters/filter_tint.json"));
    let post_effect = player.render_recipe_ir(&recipe, &PlayerSampleRequest::default());
    let source_only = player.render_recipe_source_ir(&recipe, &PlayerSampleRequest::default());

    assert_eq!(source_only.schema_version, "v3.1.player.renderIr.1");
    assert_eq!(source_only.status, PlayerStatus::Rendered);
    assert_eq!(source_only.rows, post_effect.rows);
    assert!(
        post_effect
            .styled_cells
            .iter()
            .any(|cell| cell.foreground != "defaultForeground"),
        "post-effect IR should include filter tint styled evidence"
    );
    assert!(
        source_only
            .styled_cells
            .iter()
            .all(|cell| cell.foreground == "defaultForeground"),
        "source-only IR must not include recipe-level filter tint styled evidence"
    );
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
fn filter_fixture_set_emits_styled_player_evidence() {
    let player = player();
    for relative in [
        "filters/filter_vignette.json",
        "filters/filter_bracket_emphasis.json",
        "filters/filter_dot_indicator.json",
        "filters/filter_edge_grow_left.json",
        "filters/filter_hover_bar.json",
        "filters/filter_kitt_scanner.json",
        "filters/filter_underline_wipe.json",
        "filters/filter_sub_pixel_bar.json",
    ] {
        let report = player.render_recipe(
            &recipe(&v31_debug_recipe(relative)),
            &PlayerSampleRequest {
                phase_t: 0.5,
                loop_t: Some(0.25),
                ..PlayerSampleRequest::default()
            },
        );

        assert_eq!(report.status, PlayerStatus::Rendered, "{relative}");
        assert!(report.errors.is_empty(), "{relative}");
        let grid = report.styled_grid.expect("styled filter output");
        assert!(
            grid.cells().iter().any(|cell| {
                cell.foreground != "defaultForeground"
                    || cell.background != "transparent"
                    || !cell.modifiers.is_empty()
                    || cell.role.is_some()
            }),
            "expected styled evidence for {relative}"
        );
    }
}

#[test]
fn mask_and_sampler_fixture_set_changes_row_evidence() {
    let player = player();
    for relative in [
        "masks/mask_cellular.json",
        "masks/mask_materialize_corner.json",
        "masks/mask_wipe_corner_out_from_top_left.json",
        "samplers/sampler_crt.json",
        "samplers/sampler_crt_jitter.json",
    ] {
        let recipe = recipe(&v31_debug_recipe(relative));
        let early = player.render_recipe(
            &recipe,
            &PlayerSampleRequest {
                phase_t: 0.2,
                loop_t: Some(0.0),
                ..PlayerSampleRequest::default()
            },
        );
        let later = player.render_recipe(
            &recipe,
            &PlayerSampleRequest {
                phase_t: 0.8,
                loop_t: Some(0.35),
                ..PlayerSampleRequest::default()
            },
        );

        assert_eq!(
            early.status,
            PlayerStatus::Rendered,
            "{relative}: {:?}",
            early.errors
        );
        assert_eq!(
            later.status,
            PlayerStatus::Rendered,
            "{relative}: {:?}",
            later.errors
        );
        assert!(early.errors.is_empty(), "{relative}");
        assert!(later.errors.is_empty(), "{relative}");
        assert_ne!(
            early.rows, later.rows,
            "expected row evidence change for {relative}"
        );
        assert_ne!(
            early.render_hash, later.render_hash,
            "expected hash evidence change for {relative}"
        );
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
fn test_fnc_player_renders_added_shader_and_style_fixtures() {
    let player = player();
    for (relative, effect_id, expectation) in [
        (
            "styles/style_fade_in_apply_to_both.json",
            "style.fadeIn",
            "foregroundAndBackground",
        ),
        (
            "styles/style_fade_out_to_canvas_color.json",
            "style.fadeOut",
            "backgroundOnly",
        ),
        (
            "styles/style_pulse_runtime_frequency.json",
            "style.pulse",
            "foregroundOnly",
        ),
        (
            "styles/style_italic_window.json",
            "style.italicWindow",
            "italicModifier",
        ),
        (
            "styles/style_neon_flicker_modifier.json",
            "style.neonFlicker",
            "foregroundAndItalic",
        ),
        (
            "shaders/primitives/shader_barber_pole.json",
            "shader.barberPole",
            "backgroundOnly",
        ),
        (
            "shaders/primitives/shader_diffusion_background.json",
            "shader.diffusion",
            "backgroundOnly",
        ),
        (
            "shaders/primitives/shader_radar_sweep.json",
            "shader.radar",
            "foregroundOnly",
        ),
    ] {
        let recipe = recipe_with_descriptor_fixture_effect(relative, effect_id);
        let report = player.render_recipe(
            &recipe,
            &PlayerSampleRequest {
                phase_t: 0.5,
                loop_t: Some(0.25),
                ..PlayerSampleRequest::default()
            },
        );

        assert_eq!(report.status, PlayerStatus::Rendered, "{relative}");
        assert!(report.errors.is_empty(), "{relative}");
        let grid = report.styled_grid.expect("styled descriptor fixture");
        match expectation {
            "foregroundAndBackground" => assert!(grid.cells().iter().any(|cell| {
                cell.foreground != "defaultForeground" && cell.background != "transparent"
            })),
            "backgroundOnly" => {
                assert!(
                    grid.cells()
                        .iter()
                        .any(|cell| cell.background != "transparent")
                );
                assert!(
                    grid.cells()
                        .iter()
                        .all(|cell| cell.foreground == "defaultForeground")
                );
            }
            "foregroundOnly" => {
                assert!(
                    grid.cells()
                        .iter()
                        .any(|cell| cell.foreground != "defaultForeground")
                );
                assert!(
                    grid.cells()
                        .iter()
                        .all(|cell| cell.background == "transparent")
                );
            }
            "italicModifier" => {
                assert!(grid.cells().iter().any(|cell| cell.modifiers == ["italic"]))
            }
            "foregroundAndItalic" => assert!(grid.cells().iter().any(|cell| {
                cell.foreground != "defaultForeground" && cell.modifiers == ["italic"]
            })),
            _ => unreachable!("unknown expectation"),
        }

        if effect_id == "style.pulse" {
            let mut slow_request = PlayerSampleRequest {
                phase_t: 0.5,
                loop_t: Some(0.1),
                ..PlayerSampleRequest::default()
            };
            slow_request
                .signals
                .insert(SignalId::new("pulseFrequency"), Value::Number(1.0));
            let mut fast_request = slow_request.clone();
            fast_request
                .signals
                .insert(SignalId::new("pulseFrequency"), Value::Number(3.0));
            let slow = player.render_recipe(&recipe, &slow_request);
            let fast = player.render_recipe(&recipe, &fast_request);
            assert_eq!(slow.status, PlayerStatus::Rendered);
            assert_eq!(fast.status, PlayerStatus::Rendered);
            assert_ne!(slow.render_hash, fast.render_hash);
        }

        if effect_id == "style.italicWindow" {
            let outside_window = player.render_recipe(
                &recipe,
                &PlayerSampleRequest {
                    phase_t: 0.95,
                    ..PlayerSampleRequest::default()
                },
            );
            assert_eq!(outside_window.status, PlayerStatus::Rendered);
            if let Some(outside_grid) = outside_window.styled_grid {
                assert!(
                    outside_grid
                        .cells()
                        .iter()
                        .all(|cell| !cell.modifiers.iter().any(|modifier| modifier == "italic"))
                );
            }
        }
    }
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
fn source_ansi_fixture_carries_bounded_sgr_style_cells() {
    let recipe = recipe(&v31_debug_recipe("sources/source_ansi_sgr_basic.json"));
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    let grid = report.styled_grid.expect("ansi styled-cell evidence");
    assert_any_foreground(&grid, "ansi.red");
    assert!(
        grid.cells().iter().any(|cell| cell.glyph == "S"
            && cell.modifiers.iter().any(|modifier| modifier == "bold")
            && cell.modifiers.iter().any(|modifier| modifier == "italic")),
        "expected bold and italic SGR modifiers on styled ANSI glyph"
    );
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
fn source_image_resolver_seam_can_supply_grid_material() {
    let recipe = recipe(&v31_debug_recipe(
        "sources/source_image_resolver_grid_smoke.json",
    ));
    let resolver = SmokeImageResolver;
    let (rows, styled_grid, errors, warnings) = render_scene_with_source_asset_resolver(
        &recipe,
        &PlayerSampleRequest::default(),
        &resolver,
    );

    assert!(errors.is_empty());
    assert!(warnings.is_empty());
    assert_eq!(rows[0], "IMG!");
    assert!(styled_grid.style_known());
    assert_any_foreground(&styled_grid, "ansi.cyan");
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
fn source_procedural_fixture_uses_checkerboard_registry_entry() {
    let recipe = recipe(&v31_debug_recipe(
        "sources/source_procedural_checkerboard.json",
    ));
    let report = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            loop_t: Some(0.5),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert_eq!(report.rows[0], "░█░█░█");
    assert_eq!(report.rows[1], "█░█░█░");
}

#[test]
fn source_procedural_fixture_uses_progress_bar_registry_entry() {
    let recipe = recipe(&v31_debug_recipe(
        "sources/source_procedural_progress_bar.json",
    ));
    let report = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            loop_t: Some(0.5),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert_eq!(report.rows[0], "████░░░░");
}

#[test]
fn source_procedural_fixture_uses_subcell_shape_atlas_registry_entry() {
    let recipe = recipe(&v31_debug_recipe(
        "sources/source_procedural_subcell_shape_atlas.json",
    ));
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    assert_eq!(report.rows[0], "▘▝▖▗▀▄▌▐");
}

#[test]
fn source_procedural_unknown_generator_is_unsupported() {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe(
            "sources/source_procedural_checkerboard.json",
        ))
        .expect("read checkerboard recipe"),
    )
    .expect("checkerboard json");
    value["sources"]["proceduralLayer"]["inputs"]["generator"]["value"]["value"] =
        serde_json::json!("missing_generator");
    let recipe = serde_json::from_value(value).expect("unknown procedural recipe");
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Unsupported);
    assert!(
        report
            .errors
            .iter()
            .any(|error| error.code == "unsupportedProceduralGenerator")
    );
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
fn scene_layer_visibility_default_true_renders_layer() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_layer_signal_binding_io.json",
    ));
    let report = player().render_recipe_ir(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.rows[0].contains("VISIBLE BY FALLBACK"));
    let layer = report
        .layers
        .iter()
        .find(|layer| layer.element_id == "visibleElement")
        .expect("layer runtime result");
    assert!(layer.visible);
    assert!(!layer.skipped);
}

#[test]
fn scene_layer_visibility_default_false_skips_layer_with_ir_diagnostic() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_layer_visibility_false_skips_layer.json",
    ));
    let report = player().render_recipe_ir(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.rows[0].contains("LOWER CONTENT"));
    assert!(!report.rows[0].contains("HIDDEN TOP"));
    assert!(report.warnings.iter().any(|warning| {
        warning.code == "sceneLayerSkipped"
            && warning.path.contains("hiddenElement")
            && warning.message.contains("primary")
    }));
    let hidden_layer = report
        .layers
        .iter()
        .find(|layer| layer.element_id == "hiddenElement")
        .expect("hidden layer runtime result");
    assert_eq!(hidden_layer.layer_id.as_deref(), Some("primary"));
    assert!(!hidden_layer.visible);
    assert!(hidden_layer.skipped);
    assert_eq!(hidden_layer.skip_reason.as_deref(), Some("visibilityFalse"));
    assert!(report.provenance.iter().any(|entry| {
        entry.element_id == "hiddenElement"
            && !entry.rendered
            && entry.skip_reason.as_deref() == Some("visibilityFalse")
    }));
}

#[test]
fn scene_layer_visibility_binding_override_true_renders_layer() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_layer_visibility_false_skips_layer.json",
    ));
    let mut request = PlayerSampleRequest::default();
    request
        .signals
        .insert(SignalId::new("showPrimaryLayer"), Value::Boolean(true));
    let report = player().render_recipe_ir(&recipe, &request);

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.rows[0].contains("HIDDEN TOP"));
    let hidden_layer = report
        .layers
        .iter()
        .find(|layer| layer.element_id == "hiddenElement")
        .expect("hidden layer runtime result");
    assert!(hidden_layer.visible);
    assert!(!hidden_layer.skipped);
}

#[test]
fn scene_layer_visibility_binding_override_false_skips_layer() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_layer_visibility_binding_io.json",
    ));
    let mut request = PlayerSampleRequest::default();
    request
        .signals
        .insert(SignalId::new("showPrimaryLayer"), Value::Boolean(false));
    let report = player().render_recipe_ir(&recipe, &request);

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(!report.rows[0].contains("VISIBLE BY FALLBACK"));
    assert!(
        report
            .layers
            .iter()
            .any(|layer| layer.element_id == "visibleElement" && layer.skipped)
    );
}

#[test]
fn scene_skip_transparent_empty_preserves_lower_content() {
    let recipe = transparent_overlay_recipe();
    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0].trim_end(), "LOWER");
}

#[test]
fn text_grid_backend_consumes_player_render_ir_rows() {
    let recipe = recipe(&v31_debug_recipe("baseline.json"));
    let ir = player().render_recipe_ir(&recipe, &PlayerSampleRequest::default());
    let backend = TextGridRenderBackend;
    let output = backend.render(&ir);

    assert_eq!(output.schema_version, "v3.1.player.renderBackend.1");
    assert_eq!(output.backend, "textGrid");
    assert_eq!(output.rows, ir.rows);
    assert!(output.diagnostics.is_empty());
}

#[test]
fn styled_cell_backend_consumes_player_render_ir_cells_deterministically() {
    let recipe = recipe(&v31_debug_recipe("styles/style_color_fade.json"));
    let ir = player().render_recipe_ir(&recipe, &PlayerSampleRequest::default());
    let backend = StyledCellRenderBackend;
    let first = backend.render(&ir);
    let second = backend.render(&ir);

    assert_eq!(first, second);
    assert_eq!(first.backend, "styledCell");
    assert!(!first.styled_cells.is_empty());
    assert_eq!(first.rows, ir.rows);
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
fn graph_missing_required_value_emits_structured_player_diagnostic() {
    let recipe = graph_recipe(
        serde_json::json!({
            "consumeBeforePublish": {
                "id": "consumeBeforePublish",
                "effect": "filter.tint",
                "inputs": {
                    "strength": graph_required_source("lateStrength"),
                    "color": color_source(255, 0, 0),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {},
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "publishLate": {
                "id": "publishLate",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.5),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "lateStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!(["consumeBeforePublish", "publishLate"]),
        Some(serde_json::json!({
            "kind": "sequence",
            "children": [
                { "kind": "node", "node": "consumeBeforePublish" },
                { "kind": "node", "node": "publishLate" }
            ]
        })),
        &["filter.tint", "filter.dim"],
    );

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Unsupported);
    assert!(
        report
            .errors
            .iter()
            .any(|error| error.code == "missingGraphValue")
    );
}

#[test]
fn graph_value_kind_mismatch_emits_structured_player_diagnostic() {
    let recipe = graph_recipe(
        serde_json::json!({
            "consumeWrongKind": {
                "id": "consumeWrongKind",
                "effect": "filter.dim",
                "inputs": {
                    "factor": graph_required_source("lateStrength"),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {},
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            },
            "publishLate": {
                "id": "publishLate",
                "effect": "filter.dim",
                "inputs": {
                    "factor": number_source(0.5),
                    "applyTo": enum_source("foreground")
                },
                "outputs": {
                    "lateStrength": { "source": { "kind": "input", "id": "factor" } }
                },
                "scope": { "kind": "all" },
                "cellWritePolicy": "writeCell",
                "roleWritePolicy": { "kind": "preserveDestination" }
            }
        }),
        serde_json::json!(["consumeWrongKind", "publishLate"]),
        Some(serde_json::json!({
            "kind": "sequence",
            "children": [
                { "kind": "node", "node": "consumeWrongKind" },
                { "kind": "node", "node": "publishLate" }
            ]
        })),
        &["filter.dim"],
    );
    let mut request = PlayerSampleRequest::default();
    request.graph_values.insert(
        GraphValueId::new("lateStrength"),
        Value::Text("wrong".to_string()),
    );

    let report = player().render_recipe(&recipe, &request);

    assert_eq!(report.status, PlayerStatus::Unsupported);
    let error = report
        .errors
        .iter()
        .find(|error| error.code == "graphValueKindMismatch")
        .expect("kind mismatch diagnostic");
    assert!(
        error
            .message
            .contains("expected graph value `lateStrength` to be number")
    );
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

fn transparent_overlay_recipe() -> RecipeDocument {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("scene/scene_layer_signal_binding_io.json"))
            .expect("read scene fixture"),
    )
    .expect("baseline json");
    value["id"] = serde_json::json!("debugTransparentOverlayPreservesLower");
    value["sources"] = serde_json::json!({
        "lowerText": {
            "source": "source.text",
            "inputs": {
                "text": { "kind": "literal", "value": { "kind": "text", "value": "LOWER" } },
                "width": { "kind": "literal", "value": { "kind": "integer", "value": 5 } },
                "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
            },
            "assets": {}
        },
        "emptyOverlay": {
            "source": "source.text",
            "inputs": {
                "text": { "kind": "literal", "value": { "kind": "text", "value": "     " } },
                "width": { "kind": "literal", "value": { "kind": "integer", "value": 5 } },
                "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
            },
            "assets": {}
        }
    });
    value["scenes"][0]["elements"] = serde_json::json!([
        {
            "id": "lowerElement",
            "layer": "base",
            "zIndex": 0,
            "placement": { "x": 0, "y": 0 },
            "source": "lowerText",
            "pipeline": null,
            "clipPolicy": "clip",
            "cellWritePolicy": "writeCell",
            "roleWritePolicy": { "kind": "preserveDestination" }
        },
        {
            "id": "emptyOverlayElement",
            "layer": "overlay",
            "zIndex": 1,
            "placement": { "x": 0, "y": 0 },
            "source": "emptyOverlay",
            "pipeline": null,
            "clipPolicy": "clip",
            "cellWritePolicy": "skipTransparentEmpty",
            "roleWritePolicy": { "kind": "preserveDestination" }
        }
    ]);
    serde_json::from_value(value).expect("transparent overlay recipe")
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

struct SmokeImageResolver;

impl PlayerSourceAssetResolver for SmokeImageResolver {
    fn resolve_image_asset(
        &self,
        request: PlayerSourceAssetRequest<'_>,
    ) -> PlayerSourceAssetResolution {
        assert_eq!(request.asset_id, "smoke-image");
        let rows = vec!["IMG!".to_string()];
        let mut styled_grid = PlayerStyledGrid::from_rows(&rows);
        styled_grid.set_cell_style(
            0,
            0,
            "ansi.cyan",
            "transparent",
            vec![],
            Some("Image".into()),
        );
        PlayerSourceAssetResolution::ResolvedGrid { rows, styled_grid }
    }
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

fn graph_required_source(id: &str) -> serde_json::Value {
    serde_json::json!({
        "kind": "graphValue",
        "id": id,
        "fallback": null
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

fn recipe_with_descriptor_fixture_effect(relative: &str, effect_id: &str) -> RecipeDocument {
    let mut value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(v31_debug_recipe(relative)).expect("read recipe"))
            .expect("recipe json");
    let pack: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(descriptor_pack_path()).expect("pack json"))
            .expect("descriptor pack json");
    if pack["effects"].get(effect_id).is_none() {
        value["graph"]["effects"][effect_id] = descriptor_fixture_effect(effect_id);
    }
    serde_json::from_value(value).expect("recipe with descriptor fixture effect")
}

fn descriptor_fixture_effect(effect_id: &str) -> serde_json::Value {
    let inputs = match effect_id {
        "style.fadeIn" | "style.fadeOut" => serde_json::json!({
            "from": optional_color_input("From Color"),
            "to": optional_color_input("To Color"),
            "applyTo": optional_enum_input("Apply To", &["foreground", "background", "both"], "foreground"),
            "ease": optional_enum_input("Ease", &["linear", "easeIn", "easeOut", "easeInOut"], "linear"),
            "colorSpace": optional_enum_input("Color Space", &["rgb", "hct"], "rgb")
        }),
        "style.pulse" => serde_json::json!({
            "color": optional_color_input("Color"),
            "frequency": optional_number_input("Frequency", 1.0, Some((0.0, 60.0))),
            "applyTo": optional_enum_input("Apply To", &["foreground", "background", "both"], "foreground")
        }),
        "style.italicWindow" => serde_json::json!({
            "start": optional_number_input("Start", 0.0, Some((0.0, 1.0))),
            "end": optional_number_input("End", 1.0, Some((0.0, 1.0)))
        }),
        "style.neonFlicker" => serde_json::json!({
            "color": optional_color_input("Color"),
            "stability": optional_number_input("Stability", 0.7, Some((0.0, 1.0))),
            "dimAmount": optional_number_input("Dim Amount", 0.5, Some((0.0, 1.0))),
            "italicWindow": optional_boolean_input("Italic Window", false)
        }),
        "shader.barberPole" => serde_json::json!({
            "stripeColor": optional_color_input("Stripe Color"),
            "backgroundColor": optional_color_input("Background Color"),
            "stripeWidth": optional_integer_input("Stripe Width", 3, Some((1, 64))),
            "gapWidth": optional_integer_input("Gap Width", 2, Some((1, 64))),
            "angleDeg": optional_number_input("Angle Degrees", 45.0, None),
            "speed": optional_number_input("Speed", 0.0, Some((0.0, 60.0))),
            "applyTo": optional_enum_input("Apply To", &["foreground", "background", "both"], "background")
        }),
        "shader.diffusion" => serde_json::json!({
            "color": optional_color_input("Color"),
            "centerX": optional_number_input("Center X", 20.0, None),
            "centerY": optional_number_input("Center Y", 2.0, None),
            "radius": optional_number_input("Radius", 8.0, Some((0.0, 128.0))),
            "intensity": optional_number_input("Intensity", 1.0, Some((0.0, 1.0))),
            "applyTo": optional_enum_input("Apply To", &["foreground", "background", "both"], "background")
        }),
        "shader.radar" => serde_json::json!({
            "color": optional_color_input("Color"),
            "speed": optional_number_input("Speed", 1.0, Some((0.0, 60.0))),
            "tailLength": optional_number_input("Tail Length", 0.25, Some((0.0, 1.0))),
            "applyTo": optional_enum_input("Apply To", &["foreground", "background", "both"], "foreground")
        }),
        _ => unreachable!("unsupported descriptor fixture effect"),
    };

    serde_json::json!({
        "id": effect_id,
        "version": "0.1.0",
        "displayName": effect_id.replace('.', " "),
        "category": "debug primitive",
        "domain": "cellShader",
        "cellAccess": {
            "reads": ["glyph", "foreground", "background", "modifiers", "modifierAlpha", "role"],
            "writes": ["glyph", "foreground", "background", "modifiers", "modifierAlpha", "role"]
        },
        "scopeSupport": {
            "kinds": ["all", "role", "outerBand", "inner"],
            "coordinateSpaces": ["destinationLocal", "sampledSource"],
            "roleSpaces": ["sampledSource", "destination"]
        },
        "writeSupport": {
            "cellPolicies": ["writeCell", "skipTransparentEmpty"],
            "rolePolicies": ["preserveDestination", "copySampledSource"]
        },
        "inputs": inputs,
        "outputs": {},
        "lifecycle": {
            "completion": "instant",
            "resettable": true,
            "seekable": true,
            "deterministicWithSeed": true
        }
    })
}

fn optional_color_input(display_name: &str) -> serde_json::Value {
    serde_json::json!({
        "displayName": display_name,
        "description": "Proposed Lane F styled-cell player descriptor input.",
        "value": {
            "kind": "color",
            "default": { "kind": "color", "value": { "r": 255, "g": 255, "b": 255, "a": 255 } },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "optional": true,
        "bindable": true,
        "runtimeMutability": "phaseStart"
    })
}

fn optional_number_input(
    display_name: &str,
    default: f64,
    range: Option<(f64, f64)>,
) -> serde_json::Value {
    let range = range
        .map(|(min, max)| serde_json::json!({ "min": min, "max": max }))
        .unwrap_or(serde_json::Value::Null);
    serde_json::json!({
        "displayName": display_name,
        "description": "Proposed Lane F styled-cell player descriptor input.",
        "value": {
            "kind": "number",
            "default": { "kind": "number", "value": default },
            "range": range,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "optional": true,
        "bindable": true,
        "runtimeMutability": "phaseStart"
    })
}

fn optional_integer_input(
    display_name: &str,
    default: i64,
    range: Option<(i64, i64)>,
) -> serde_json::Value {
    let range = range
        .map(|(min, max)| serde_json::json!({ "min": min, "max": max }))
        .unwrap_or(serde_json::Value::Null);
    serde_json::json!({
        "displayName": display_name,
        "description": "Proposed Lane F styled-cell player descriptor input.",
        "value": {
            "kind": "integer",
            "default": { "kind": "integer", "value": default },
            "range": range,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "optional": true,
        "bindable": true,
        "runtimeMutability": "phaseStart"
    })
}

fn optional_enum_input(
    display_name: &str,
    allowed_values: &[&str],
    default: &str,
) -> serde_json::Value {
    serde_json::json!({
        "displayName": display_name,
        "description": "Proposed Lane F styled-cell player descriptor input.",
        "value": {
            "kind": "enum",
            "default": { "kind": "enum", "value": default },
            "range": null,
            "allowedValues": allowed_values,
            "unit": null,
            "semantic": null
        },
        "optional": true,
        "bindable": true,
        "runtimeMutability": "phaseStart"
    })
}

fn optional_boolean_input(display_name: &str, default: bool) -> serde_json::Value {
    serde_json::json!({
        "displayName": display_name,
        "description": "Proposed Lane F styled-cell player descriptor input.",
        "value": {
            "kind": "boolean",
            "default": { "kind": "boolean", "value": default },
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "optional": true,
        "bindable": true,
        "runtimeMutability": "phaseStart"
    })
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
