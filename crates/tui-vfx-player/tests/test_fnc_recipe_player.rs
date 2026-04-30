// <FILE>crates/tui-vfx-player/tests/test_fnc_recipe_player.rs</FILE> - <DESC>Contract-native skeleton player regression tests</DESC>
// <VERS>VERSION: 0.6.6</VERS>
// <WCTX>Styled-cell substrate work: keep player evidence tests portable and explicit.</WCTX>
// <CLOG>0.6.6: PATCH — assert vertical KittScanner varies by row rather than by column.</CLOG>

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use tui_vfx_contract::{
    DescriptorCatalog, DescriptorPack, DescriptorPackId, GraphValueId, LifecyclePhase,
    RecipeDocument, SignalId, SourceInputId, SourceInstanceId, Value, ValueSource,
};
use tui_vfx_player::{
    PlayerLoopbackStrictness, PlayerRenderBackend, PlayerSampleRequest, PlayerSession,
    PlayerStatus, PlayerStyledGrid, RecipePlayer, StyledCellRenderBackend, TextGridRenderBackend,
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
fn source_card_fixture_renders_surface_chrome_and_roles() {
    let report = player().render_recipe(
        &source_card_chrome_recipe(),
        &PlayerSampleRequest::default(),
    );

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0].trim_end(), "╭────────╮");
    assert_eq!(report.rows[1].trim_end(), "│CARD    │");
    assert_eq!(report.rows[2].trim_end(), "╰────────╯");
    let grid = report
        .styled_grid
        .expect("card chrome styled-cell evidence");
    assert!(
        grid.cells()
            .iter()
            .any(|cell| cell.role.as_deref() == Some("Border"))
    );
    assert!(
        grid.cells()
            .iter()
            .any(|cell| cell.role.as_deref() == Some("Background"))
    );
    assert!(
        grid.cells()
            .iter()
            .any(|cell| cell.role.as_deref() == Some("Text"))
    );
    assert_any_foreground(&grid, "rgba(255,0,255,255)");
    assert!(
        grid.cells()
            .iter()
            .any(|cell| cell.background == "rgba(50,20,50,255)")
    );
}

#[test]
fn source_card_plain_border_uses_single_line_box_drawing_glyphs() {
    let mut recipe = source_card_chrome_recipe();
    recipe
        .sources
        .get_mut(&SourceInstanceId::new("mainCard"))
        .expect("mainCard source")
        .inputs
        .insert(
            SourceInputId::new("borderStyle"),
            serde_json::from_value::<ValueSource>(enum_source("plain"))
                .expect("plain border value source"),
        );

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0].trim_end(), "┌────────┐");
    assert_eq!(report.rows[1].trim_end(), "│CARD    │");
    assert_eq!(report.rows[2].trim_end(), "└────────┘");
}

#[test]
fn filter_tint_matches_v2_deprecated_oracle_colors_by_phase() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("filters/filter_tint.json"));
    let enter = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase: LifecyclePhase::Enter,
            phase_t: 1.0,
            ..PlayerSampleRequest::default()
        },
    );
    let dwell = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase: LifecyclePhase::Dwell,
            phase_t: 1.0,
            ..PlayerSampleRequest::default()
        },
    );
    let exit = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase: LifecyclePhase::Exit,
            phase_t: 1.0,
            ..PlayerSampleRequest::default()
        },
    );

    for report in [&enter, &dwell, &exit] {
        assert_eq!(report.status, PlayerStatus::Rendered);
        assert_eq!(
            report.rows[0].trim_end(),
            "╭─────────────────────────────────╮"
        );
        assert_eq!(
            report.rows[1].trim_end(),
            "│FILTER TEST: Tint Effect         │"
        );
        assert_eq!(
            report.rows[2].trim_end(),
            "╰─────────────────────────────────╯"
        );
    }

    assert_letter_color_count(
        &enter.styled_grid.expect("enter styled grid"),
        "rgba(255,193,173,255)",
        "rgba(126,64,44,255)",
        20,
    );
    assert_letter_color_count(
        &dwell.styled_grid.expect("dwell styled grid"),
        "rgba(255,255,255,255)",
        "rgba(40,40,40,255)",
        20,
    );
    assert_letter_color_count(
        &exit.styled_grid.expect("exit styled grid"),
        "rgba(132,162,255,255)",
        "rgba(46,76,169,255)",
        20,
    );
}

#[test]
fn basic_filter_primitives_match_v2_deprecated_oracle_colors_by_phase() {
    assert_filter_phase_colors(
        "filters/filter_dim.json",
        "│FILTER TEST: Dim Effect          │",
        [
            (
                LifecyclePhase::Enter,
                "rgba(179,179,179,255)",
                "rgba(42,42,56,255)",
                19,
            ),
            (
                LifecyclePhase::Dwell,
                "rgba(255,255,255,255)",
                "rgba(60,60,80,255)",
                19,
            ),
            (
                LifecyclePhase::Exit,
                "rgba(128,128,128,255)",
                "rgba(60,60,80,255)",
                19,
            ),
        ],
    );
    assert_filter_phase_colors(
        "filters/filter_invert.json",
        "│FILTER TEST: Invert Effect       │",
        [
            (
                LifecyclePhase::Enter,
                "rgba(20,40,60,255)",
                "rgba(0,255,255,255)",
                22,
            ),
            (
                LifecyclePhase::Dwell,
                "rgba(0,255,255,255)",
                "rgba(20,40,60,255)",
                22,
            ),
            (
                LifecyclePhase::Exit,
                "rgba(20,40,60,255)",
                "rgba(20,40,60,255)",
                22,
            ),
        ],
    );
    assert_filter_phase_colors(
        "filters/filter_greyscale.json",
        "│GREYSCALE                       │",
        [
            (
                LifecyclePhase::Enter,
                "rgba(255,100,100,255)",
                "rgba(20,60,180,255)",
                9,
            ),
            (
                LifecyclePhase::Dwell,
                "rgba(168,137,137,255)",
                "rgba(54,62,86,255)",
                9,
            ),
            (
                LifecyclePhase::Exit,
                "rgba(255,100,100,255)",
                "rgba(20,60,180,255)",
                9,
            ),
        ],
    );
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
    let request = PlayerSampleRequest {
        phase: LifecyclePhase::Enter,
        phase_t: 1.0,
        ..PlayerSampleRequest::default()
    };
    let post_effect = player.render_recipe_ir(&recipe, &request);
    let source_only = player.render_recipe_source_ir(&recipe, &request);

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
            .all(|cell| cell.role.as_deref() != Some("FilterTint")),
        "source-only IR must not include recipe-level filter tint styled evidence"
    );
    assert!(
        source_only
            .styled_cells
            .iter()
            .any(|cell| cell.foreground == "rgba(255,255,255,255)"
                && cell.background == "rgba(40,40,40,255)"),
        "source-only IR should preserve source.card base chrome styles"
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
            phase: LifecyclePhase::Enter,
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
fn kitt_scanner_vertical_fixture_sweeps_by_row_not_column() {
    let report = player().render_recipe(
        &recipe(&v31_debug_recipe(
            "filters/filter_kitt_scanner_vertical.json",
        )),
        &PlayerSampleRequest {
            phase_t: 0.5,
            loop_t: Some(0.25),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty());
    let grid = report
        .styled_grid
        .expect("vertical KittScanner styled output");

    let row_zero_foregrounds = foregrounds_at_row(&grid, 0);
    let column_zero_foregrounds = foregrounds_at_column(&grid, 0);

    assert_eq!(
        row_zero_foregrounds.len(),
        1,
        "vertical KittScanner should keep a row color-uniform; foregrounds: {row_zero_foregrounds:?}"
    );
    assert!(
        column_zero_foregrounds.len() > 1,
        "vertical KittScanner should vary across rows; foregrounds: {column_zero_foregrounds:?}"
    );
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
fn mask_checkers_vertical_slice_preserves_card_chrome_and_phase_gating() {
    let player = player();
    let recipe = recipe(&v31_debug_recipe("masks/mask_checkers.json"));
    let dwell = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase: LifecyclePhase::Dwell,
            phase_t: 0.5,
            ..PlayerSampleRequest::default()
        },
    );
    let enter = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase: LifecyclePhase::Enter,
            phase_t: 0.5,
            ..PlayerSampleRequest::default()
        },
    );
    let exit = player.render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase: LifecyclePhase::Exit,
            phase_t: 0.5,
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(dwell.status, PlayerStatus::Rendered);
    assert_eq!(enter.status, PlayerStatus::Rendered);
    assert_eq!(exit.status, PlayerStatus::Rendered);
    assert_eq!(
        dwell.rows[0].trim_end(),
        "╭─────────────────────────────────╮"
    );
    assert!(dwell.rows[1].contains("MASK TEST: Checkers Effect"));
    assert_eq!(
        dwell.rows[2].trim_end(),
        "╰─────────────────────────────────╯"
    );
    assert_ne!(
        enter.rows, dwell.rows,
        "enter phase should apply the enter checkers mask"
    );
    assert_ne!(
        exit.rows, dwell.rows,
        "exit phase should apply the exit checkers mask"
    );
    assert_ne!(
        enter.rows, exit.rows,
        "enter and exit masks should keep their distinct cell sizes"
    );
    let grid = dwell.styled_grid.expect("card chrome styled cells");
    assert_any_foreground(&grid, "rgba(255,0,255,255)");
    assert!(
        grid.cells()
            .iter()
            .any(|cell| cell.background == "rgba(50,20,50,255)")
    );
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
    for (relative, effect_id, expectation, phase) in [
        (
            "styles/style_fade_in_apply_to_both.json",
            "style.fadeIn",
            "foregroundAndBackground",
            LifecyclePhase::Dwell,
        ),
        (
            "styles/style_fade_out_to_canvas_color.json",
            "style.fadeOut",
            "foregroundAndBackground",
            LifecyclePhase::Dwell,
        ),
        (
            "styles/style_pulse_runtime_frequency.json",
            "style.pulse",
            "foregroundAndBackground",
            LifecyclePhase::Dwell,
        ),
        (
            "styles/style_italic_window.json",
            "style.italicWindow",
            "italicModifier",
            LifecyclePhase::Enter,
        ),
        (
            "styles/style_neon_flicker_modifier.json",
            "style.neonFlicker",
            "foregroundAndItalic",
            LifecyclePhase::Dwell,
        ),
        (
            "shaders/primitives/shader_barber_pole.json",
            "shader.barberPole",
            "backgroundOnly",
            LifecyclePhase::Dwell,
        ),
        (
            "shaders/primitives/shader_diffusion_background.json",
            "shader.diffusion",
            "backgroundOnly",
            LifecyclePhase::Dwell,
        ),
        (
            "shaders/primitives/shader_radar_sweep.json",
            "shader.radar",
            "foregroundOnly",
            LifecyclePhase::Dwell,
        ),
    ] {
        let recipe = recipe_with_descriptor_fixture_effect(relative, effect_id);
        let report = player.render_recipe(
            &recipe,
            &PlayerSampleRequest {
                phase,
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

        if effect_id == "style.italicWindow" {
            let outside_window = player.render_recipe(
                &recipe,
                &PlayerSampleRequest {
                    phase: LifecyclePhase::Enter,
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
    let request = PlayerSampleRequest {
        phase: LifecyclePhase::Enter,
        phase_t: 0.5,
        ..PlayerSampleRequest::default()
    };
    let styled = player.render_recipe(&styled_recipe, &request);
    let plain = player.render_recipe(&plain_recipe, &request);

    assert_eq!(styled.rows, plain.rows);
    assert_ne!(styled.render_hash, plain.render_hash);
    assert!(styled.styled_grid.is_some());
    assert!(plain.styled_grid.is_some());
    assert_ne!(styled.styled_grid, plain.styled_grid);
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
fn scene_layer_visibility_uses_preview_loopback_when_host_signal_missing() {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe(
            "scene/scene_layer_visibility_false_skips_layer.json",
        ))
        .expect("read visibility fixture"),
    )
    .expect("visibility json");
    value["graph"]["signals"]["showPrimaryLayer"]["previewLoopback"] = serde_json::json!({
        "kind": "literal",
        "value": { "kind": "boolean", "value": true }
    });
    let recipe = serde_json::from_value(value).expect("preview loopback recipe");

    let report = player().render_recipe_ir(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.rows[0].contains("HIDDEN TOP"));
    assert!(
        report
            .layers
            .iter()
            .any(|layer| layer.element_id == "hiddenElement" && layer.visible)
    );
}

#[test]
fn authored_loopback_indicator_marks_missing_host_signal_sample() {
    let mut value = source_text_recipe_json("LOOPBACK OK");
    value["graph"]["signals"]["demoLoopback"] = serde_json::json!({
        "id": "demoLoopback",
        "displayName": "Demo Loopback",
        "description": "Test-only authored loopback signal.",
        "value": {
            "kind": "boolean",
            "default": null,
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "previewLoopback": {
            "kind": "literal",
            "value": { "kind": "boolean", "value": true }
        },
        "required": false
    });
    let recipe = serde_json::from_value(value).expect("loopback indicator recipe");

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(
        report.rows[0].ends_with("[LB]"),
        "authored loopback usage should be visible in the top-right indicator row: {:?}",
        report.rows
    );
}

#[test]
fn authored_loopback_indicator_is_suppressed_by_host_signal() {
    let mut value = source_text_recipe_json("HOST VALUE");
    value["graph"]["signals"]["demoLoopback"] = serde_json::json!({
        "id": "demoLoopback",
        "displayName": "Demo Loopback",
        "description": "Test-only authored loopback signal.",
        "value": {
            "kind": "boolean",
            "default": null,
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "previewLoopback": {
            "kind": "literal",
            "value": { "kind": "boolean", "value": true }
        },
        "required": false
    });
    let recipe = serde_json::from_value(value).expect("loopback indicator recipe");
    let mut request = PlayerSampleRequest::default();
    request
        .signals
        .insert(SignalId::new("demoLoopback"), Value::Boolean(false));

    let report = player().render_recipe(&recipe, &request);

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0].trim_end(), "HOST VALUE");
    assert!(!report.rows[0].contains("[LB]"));
}

#[test]
fn authored_loopback_strict_mode_records_intent_without_indicator_merge() {
    let mut value = source_text_recipe_json("STRICT");
    value["graph"]["signals"]["demoLoopback"] = serde_json::json!({
        "id": "demoLoopback",
        "displayName": "Demo Loopback",
        "description": "Test-only authored loopback signal.",
        "value": {
            "kind": "boolean",
            "default": null,
            "range": null,
            "allowedValues": [],
            "unit": null,
            "semantic": null
        },
        "previewLoopback": {
            "kind": "literal",
            "value": { "kind": "boolean", "value": true }
        },
        "required": false
    });
    let recipe = serde_json::from_value(value).expect("loopback indicator recipe");
    let request = PlayerSampleRequest {
        loopback_strictness: PlayerLoopbackStrictness::Strict,
        ..PlayerSampleRequest::default()
    };

    let report = player().render_recipe(&recipe, &request);

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0].trim_end(), "STRICT");
    assert!(report.warnings.iter().any(|warning| {
        warning.code == "authoredLoopbackSuppressed" && warning.path.contains("demoLoopback")
    }));
}

#[test]
fn scene_anchor_placement_rule_centers_element_at_render_time() {
    let mut value = source_text_recipe_json("HI");
    value["scenes"][0]["width"] = serde_json::json!(6);
    value["scenes"][0]["height"] = serde_json::json!(1);
    value["scenes"][0]["elements"][0]["placementRule"] = serde_json::json!({
        "kind": "anchor",
        "anchor": "center",
        "offsetRows": 0,
        "offsetColumns": 0,
        "siblingLayer": null,
        "motion": null
    });
    let recipe = serde_json::from_value(value).expect("anchored scene recipe");

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0], "  HI  ");
}

#[test]
fn scene_overflow_wrap_places_out_of_bounds_cells_inside_scene() {
    let mut value = source_text_recipe_json("ABC");
    value["scenes"][0]["width"] = serde_json::json!(3);
    value["scenes"][0]["height"] = serde_json::json!(1);
    value["scenes"][0]["elements"][0]["placement"] = serde_json::json!({ "x": -1, "y": 0 });
    value["scenes"][0]["elements"][0]["overflow"] = serde_json::json!("wrap");
    let recipe = serde_json::from_value(value).expect("wrapped scene recipe");

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(report.rows[0], "BCA");
}

#[test]
fn madeira_flag_procedural_source_uses_authored_loopback_and_host_signal() {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("baseline.json")).expect("read baseline recipe"),
    )
    .expect("baseline json");
    value["id"] = serde_json::json!("debugMadeiraFlagProcedural");
    value["graph"]["signals"]["wave_speed"] = serde_json::json!({
        "id": "wave_speed",
        "displayName": "Wave speed",
        "description": "Flag wave speed authored loopback.",
        "value": {
            "kind": "number",
            "default": null,
            "range": { "min": 0.0, "max": 4.0 },
            "allowedValues": [],
            "unit": null,
            "semantic": "waveSpeed"
        },
        "previewLoopback": {
            "kind": "numericSignal",
            "expression": { "type": "ramp", "start": 0.5, "end": 2.0, "duration": 4.0 },
            "fallback": { "kind": "number", "value": 1.0 }
        },
        "required": false
    });
    value["sources"]["mainCard"] = serde_json::json!({
        "source": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "braille_flag_field" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 40 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 17 } },
            "params": {
                "kind": "literal",
                "value": {
                    "kind": "structured",
                    "value": {
                        "layout": { "width_cells": 40, "flag_height_cells": 13, "overscan_rows": 2 },
                        "wave": {
                            "speed": { "binding": "wave_speed", "default": 1.0 },
                            "primary_cycles": 8.0,
                            "primary_rate": 2.4,
                            "secondary_cycles": 15.0,
                            "secondary_rate": 4.0,
                            "secondary_scale": 0.3,
                            "max_amplitude": 0.15
                        },
                        "asset": {
                            "path": "/usr/projects/tui-vfx-recipes/recipes/madeira_flag/assets/base_flag_dots.json",
                            "format": "tui-vfx.braille_flag_asset.v1"
                        }
                    }
                }
            }
        },
        "assets": {}
    });
    value["scenes"][0]["width"] = serde_json::json!(40);
    value["scenes"][0]["height"] = serde_json::json!(17);
    let recipe = serde_json::from_value(value).expect("madeira procedural recipe");
    let request = PlayerSampleRequest {
        loop_t: Some(0.5),
        phase_t: 0.5,
        ..PlayerSampleRequest::default()
    };

    let loopback_report = player().render_recipe(&recipe, &request);
    let mut host_request = request.clone();
    host_request
        .signals
        .insert(SignalId::new("wave_speed"), Value::Number(4.0));
    let host_report = player().render_recipe(&recipe, &host_request);

    assert_eq!(loopback_report.status, PlayerStatus::Rendered);
    assert_eq!(host_report.status, PlayerStatus::Rendered);
    assert!(
        loopback_report
            .rows
            .iter()
            .flat_map(|row| row.chars())
            .any(|ch| ('\u{2800}'..='\u{28ff}').contains(&ch)),
        "Madeira flag source should emit braille cells"
    );
    assert_ne!(
        loopback_report.rows, host_report.rows,
        "host-supplied wave_speed should win over authored loopback"
    );
}

#[test]
fn procedural_asset_ref_resolves_declared_asset_path_for_madeira_flag_source() {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("baseline.json")).expect("read baseline recipe"),
    )
    .expect("baseline json");
    value["id"] = serde_json::json!("debugMadeiraFlagAssetRef");
    value["assets"]["madeira_flag_base"] = serde_json::json!({
        "id": "madeira_flag_base",
        "kind": { "kind": "brailleDotfield" },
        "format": "tui-vfx.braille_flag_asset.v1",
        "locator": {
            "kind": "path",
            "path": "/usr/projects/tui-vfx-recipes/recipes/madeira_flag/assets/base_flag_dots.json"
        },
        "description": "Base Madeira flag dotfield artwork."
    });
    value["sources"]["mainCard"] = serde_json::json!({
        "source": "source.procedural",
        "inputs": {
            "generator": { "kind": "literal", "value": { "kind": "string", "value": "braille_flag_field" } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": 40 } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 17 } },
            "params": {
                "kind": "literal",
                "value": {
                    "kind": "structured",
                    "value": {
                        "layout": { "width_cells": 40, "flag_height_cells": 13, "overscan_rows": 2 },
                        "wave": { "speed": 0.75 },
                        "asset": {
                            "id": "madeira_flag_base",
                            "format": "tui-vfx.braille_flag_asset.v1"
                        }
                    }
                }
            }
        },
        "assets": {}
    });
    value["scenes"][0]["width"] = serde_json::json!(40);
    value["scenes"][0]["height"] = serde_json::json!(17);
    let recipe = serde_json::from_value(value).expect("asset ref procedural recipe");

    let report = player().render_recipe(&recipe, &PlayerSampleRequest::default());

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert!(
        report
            .rows
            .iter()
            .flat_map(|row| row.chars())
            .any(|ch| ('\u{2800}'..='\u{28ff}').contains(&ch)),
        "asset ref should resolve to the Madeira braille flag dotfield"
    );
    let grid = report
        .styled_grid
        .expect("Madeira braille flag should carry styled color evidence");
    assert!(
        grid.cells()
            .iter()
            .filter(|cell| ('\u{2800}'..='\u{28ff}')
                .contains(&cell.glyph.chars().next().unwrap_or(' ')))
            .map(|cell| cell.foreground.as_str())
            .any(|foreground| foreground.starts_with("rgba(")
                && foreground != "rgba(255,255,255,255)"),
        "Madeira flag braille cells should preserve asset palette/shading color detail; foregrounds: {:?}",
        grid.cells()
            .iter()
            .filter(|cell| ('\u{2800}'..='\u{28ff}')
                .contains(&cell.glyph.chars().next().unwrap_or(' ')))
            .map(|cell| cell.foreground.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn madeira_flag_full_scene_fixture_maps_source_bindings_assets_and_visibility() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_madeira_flag_full_scene.json",
    ));

    let source_start_report = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase_t: 0.0,
            loop_t: Some(0.0),
            absolute_t_ms: Some(0.0),
            ..PlayerSampleRequest::default()
        },
    );
    let loopback_report = player().render_recipe(&recipe, &PlayerSampleRequest::default());
    let mut host_request = PlayerSampleRequest::default();
    host_request
        .signals
        .insert(SignalId::new("show_hint"), Value::Boolean(false));
    host_request
        .signals
        .insert(SignalId::new("fireworks_enabled"), Value::Boolean(false));
    host_request
        .signals
        .insert(SignalId::new("wave_speed"), Value::Number(4.0));
    let host_report = player().render_recipe(&recipe, &host_request);

    assert_eq!(source_start_report.status, PlayerStatus::Rendered);
    assert_eq!(loopback_report.status, PlayerStatus::Rendered);
    assert_eq!(host_report.status, PlayerStatus::Rendered);
    assert_eq!(
        loopback_report.rows.len(),
        24,
        "Madeira scene should preserve the source recipe's 80x24 fullscreen layout"
    );
    assert!(
        loopback_report
            .rows
            .iter()
            .all(|row| row.chars().count() == 80),
        "Madeira scene rows should preserve the source recipe's 80-column layout"
    );
    assert!(loopback_report.rows[0].ends_with("[LB]"));
    assert_eq!(
        braille_bbox(&source_start_report.rows),
        Some((20, 2, 59, 18, 548)),
        "source Madeira scene keeps the flag centered with the same braille bbox/count at t0"
    );
    for (x, y) in [(32, 2), (21, 3), (22, 3), (24, 3), (34, 3), (35, 3)] {
        assert_eq!(
            char_at(&source_start_report.rows, x, y),
            Some(' '),
            "source scene composition lets the flag layer's transparent cells occlude fireworks at ({x},{y})"
        );
    }
    assert!(
        loopback_report.rows[19].contains("Feliz Ano Novo"),
        "source sibling-relative placement puts greeting below the flag at row 19"
    );
    assert!(
        loopback_report.rows[20].contains("Happy New Year From"),
        "source sibling-relative placement puts subtext below the flag at row 20"
    );
    assert!(
        loopback_report.rows[21].contains("Funchal, Madeira"),
        "source sibling-relative placement puts location below the flag at row 21"
    );
    assert!(
        loopback_report.rows[22].contains("Press Esc to return"),
        "source sibling-relative placement puts hint below the flag at row 22"
    );
    assert!(!host_report.rows[0].contains("[LB]"));
    assert!(
        loopback_report
            .rows
            .iter()
            .any(|row| row.contains("Press") && row.contains("Esc")),
        "show_hint preview loopback should keep the hint text visible"
    );
    assert!(
        !host_report
            .rows
            .iter()
            .any(|row| row.contains("Press") && row.contains("Esc")),
        "host show_hint=false should hide the hint layer"
    );
    assert_ne!(
        loopback_report.rows, host_report.rows,
        "host wave/fireworks/hint signals should materially change the full scene"
    );
    let grid = loopback_report
        .styled_grid
        .expect("full Madeira scene should carry styled procedural evidence");
    let colored_scene_cells = grid
        .cells()
        .iter()
        .filter(|cell| {
            cell.foreground.starts_with("rgba(")
                && cell.foreground != "rgba(255,255,255,255)"
                && cell.role.as_deref() != Some("AuthoredLoopbackIndicator")
        })
        .count();
    assert!(
        colored_scene_cells > 20,
        "Madeira full scene should preserve flag/fireworks/backdrop color detail, not only the loopback indicator; colored cells={colored_scene_cells}"
    );
    assert_cell_style_at_text(
        &grid,
        "Feliz",
        "rgba(255,215,0,255)",
        &["bold"],
        "greeting text should preserve source gold/bold surface style",
    );
    assert_cell_style_at_text(
        &grid,
        "Funchal",
        "rgba(0,191,255,255)",
        &["bold"],
        "location text should preserve source cyan/bold surface style",
    );
    assert_cell_style_at_text(
        &grid,
        "Press",
        "rgba(150,150,150,255)",
        &["dim"],
        "hint text should preserve source grey/dim surface style",
    );
}

#[test]
fn madeira_flag_full_scene_uses_absolute_elapsed_time_for_wave_motion() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_madeira_flag_full_scene.json",
    ));
    let first = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase_t: 0.0,
            loop_t: Some(0.0),
            absolute_t_ms: Some(0.0),
            ..PlayerSampleRequest::default()
        },
    );
    let later = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase_t: 0.0,
            loop_t: Some(0.0),
            absolute_t_ms: Some(4_000.0),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(first.status, PlayerStatus::Rendered);
    assert_eq!(later.status, PlayerStatus::Rendered);
    assert_ne!(
        first.rows, later.rows,
        "Madeira flag/fireworks motion must advance from absolute elapsed time, not only normalized phase_t/loop_t"
    );
}

#[test]
fn scene_braille_flag_asset_token_preserves_source_centered_bbox() {
    let recipe = recipe(&v31_debug_recipe(
        "scene/scene_braille_flag_asset_token.json",
    ));

    let report = player().render_recipe(
        &recipe,
        &PlayerSampleRequest {
            phase_t: 0.0,
            loop_t: Some(0.0),
            absolute_t_ms: Some(0.0),
            ..PlayerSampleRequest::default()
        },
    );

    assert_eq!(report.status, PlayerStatus::Rendered);
    assert_eq!(braille_bbox(&report.rows), Some((1, 1, 40, 16, 549)));
    assert_eq!(report.rows.len(), 17);
    assert!(
        report.rows.iter().all(|row| row.chars().count() == 42),
        "asset-token scene must preserve the V3 fixture's 42x17 layout"
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
    assert_any_foreground(&grid, "rgba(222,6,6,255)");
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
    assert_any_foreground(&grid, "rgba(16,207,16,255)");
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
    serde_json::from_value(source_text_recipe_json("HELLO TEXT")).expect("source.text recipe")
}

fn source_text_recipe_json(text: &str) -> serde_json::Value {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("baseline.json")).expect("read baseline recipe"),
    )
    .expect("baseline json");
    value["id"] = serde_json::json!("debugTextSource");
    value["sources"]["mainCard"] = serde_json::json!({
        "source": "source.text",
        "inputs": {
            "text": { "kind": "literal", "value": { "kind": "text", "value": text } },
            "width": { "kind": "literal", "value": { "kind": "integer", "value": text.chars().count() } },
            "height": { "kind": "literal", "value": { "kind": "integer", "value": 1 } }
        },
        "assets": {}
    });
    value
}

fn source_card_chrome_recipe() -> RecipeDocument {
    let mut value: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(v31_debug_recipe("baseline.json")).expect("read baseline recipe"),
    )
    .expect("baseline json");
    value["id"] = serde_json::json!("debugCardChromeSource");
    value["sources"]["mainCard"]["inputs"] = serde_json::json!({
        "message": { "kind": "literal", "value": { "kind": "text", "value": "CARD" } },
        "width": { "kind": "literal", "value": { "kind": "integer", "value": 10 } },
        "height": { "kind": "literal", "value": { "kind": "integer", "value": 3 } },
        "foreground": color_source(255, 0, 255),
        "background": color_source(50, 20, 50),
        "borderStyle": enum_source("rounded"),
        "borderTrim": enum_source("none")
    });
    serde_json::from_value(value).expect("source.card chrome recipe")
}

fn assert_filter_phase_colors(
    recipe_relative: &str,
    expected_body_row: &str,
    phase_expectations: [(LifecyclePhase, &str, &str, usize); 3],
) {
    let player = player();
    let recipe = recipe(&v31_debug_recipe(recipe_relative));
    for (phase, expected_foreground, expected_background, expected_count) in phase_expectations {
        let report = player.render_recipe(
            &recipe,
            &PlayerSampleRequest {
                phase,
                phase_t: 1.0,
                ..PlayerSampleRequest::default()
            },
        );
        assert_eq!(report.status, PlayerStatus::Rendered);
        assert_eq!(report.rows[1].trim_end(), expected_body_row);
        assert_letter_color_count(
            &report.styled_grid.expect("phase styled grid"),
            expected_foreground,
            expected_background,
            expected_count,
        );
    }
}

fn assert_letter_color_count(
    grid: &PlayerStyledGrid,
    expected_foreground: &str,
    expected_background: &str,
    expected_count: usize,
) {
    let actual_count = grid
        .cells()
        .iter()
        .filter(|cell| cell.glyph.chars().next().is_some_and(char::is_alphanumeric))
        .filter(|cell| {
            cell.foreground == expected_foreground && cell.background == expected_background
        })
        .count();
    assert_eq!(
        actual_count,
        expected_count,
        "expected {expected_count} alphanumeric cells with fg={expected_foreground} bg={expected_background}; actual styled cells: {:?}",
        grid.cells()
            .iter()
            .filter(|cell| cell.glyph.chars().next().is_some_and(char::is_alphanumeric))
            .map(|cell| (&cell.glyph, &cell.foreground, &cell.background))
            .collect::<Vec<_>>()
    );
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

fn char_at(rows: &[String], x: usize, y: usize) -> Option<char> {
    rows.get(y)?.chars().nth(x)
}

fn braille_bbox(rows: &[String]) -> Option<(usize, usize, usize, usize, usize)> {
    let points = rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, ch)| {
                ('\u{2800}'..='\u{28ff}').contains(&ch).then_some((x, y))
            })
        })
        .collect::<Vec<_>>();
    Some((
        points.iter().map(|(x, _)| *x).min()?,
        points.iter().map(|(_, y)| *y).min()?,
        points.iter().map(|(x, _)| *x).max()?,
        points.iter().map(|(_, y)| *y).max()?,
        points.len(),
    ))
}

fn assert_cell_style_at_text(
    grid: &PlayerStyledGrid,
    text_fragment: &str,
    expected_foreground: &str,
    expected_modifiers: &[&str],
    message: &str,
) {
    let matching_cells = grid
        .cells()
        .iter()
        .filter(|cell| text_fragment.contains(cell.glyph.as_str()))
        .collect::<Vec<_>>();
    assert!(
        matching_cells
            .iter()
            .any(|cell| cell.foreground == expected_foreground
                && expected_modifiers
                    .iter()
                    .all(|modifier| cell.modifiers.iter().any(|known| known == modifier))),
        "{message}; matching styled cells: {:?}",
        matching_cells
            .iter()
            .map(|cell| (&cell.glyph, &cell.foreground, &cell.modifiers))
            .collect::<Vec<_>>()
    );
}

fn foregrounds_at_row(grid: &PlayerStyledGrid, y: usize) -> Vec<&str> {
    let mut foregrounds = grid
        .cells()
        .iter()
        .filter(|cell| cell.y == y)
        .map(|cell| cell.foreground.as_str())
        .collect::<Vec<_>>();
    foregrounds.sort_unstable();
    foregrounds.dedup();
    foregrounds
}

fn foregrounds_at_column(grid: &PlayerStyledGrid, x: usize) -> Vec<&str> {
    let mut foregrounds = grid
        .cells()
        .iter()
        .filter(|cell| cell.x == x)
        .map(|cell| cell.foreground.as_str())
        .collect::<Vec<_>>();
    foregrounds.sort_unstable();
    foregrounds.dedup();
    foregrounds
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
        .filter(|id| pack["effects"].get(**id).is_none())
        .map(|id| ((*id).to_string(), descriptor_fixture_effect(id)))
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
// <VERS>END OF VERSION: 0.6.5</VERS>
