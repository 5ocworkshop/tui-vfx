// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>VERSION: 0.33.0</VERS>
// <WCTX>v3.1 player CLI regressions for strict-native backend rendering, legacy-oracle evidence, studio evidence, and schema readiness.</WCTX>
// <CLOG>0.33.0: MINOR — lock vignette rejection for player-style-only fields after removing backend style-stage adapters.
// 0.32.0: MINOR — lock matrixRain compositor FilterSpec lowering and remove stale migrated-filter style-stage expectations.
// 0.31.0: MINOR — lock subPixelBar compositor FilterSpec lowering and adapter-only rejection.
// 0.30.0: MINOR — lock strict-native success for migrated filter parity recipes without claiming IR parity.</CLOG>

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::{Command, Output},
    thread,
    time::{Duration, Instant},
};

const RECURSIVE_DEBUG_FIXTURE_COUNT: i64 = 146;

#[test]
fn test_fnc_cli_capture_cells_writes_dense_sqlite_rows_for_small_recipe() {
    let temp_root = std::env::temp_dir().join("tui-vfx-player-capture-cells-small");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("temp root");
    let database_path = temp_root.join("capture.sqlite");

    let output = run_player_cli(
        vec![
            str_arg("capture-cells"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--sqlite-output"),
            database_path.display().to_string(),
            str_arg("--frames"),
            str_arg("2"),
            str_arg("--width"),
            str_arg("8"),
            str_arg("--height"),
            str_arg("4"),
        ],
        "capture-cells small recipe player cli",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("capture cells summary json");
    assert_eq!(report["schemaVersion"], "v3.1.player.cellCapture.1");
    assert_eq!(report["frameCount"], 2);

    let conn = rusqlite::Connection::open(&database_path).expect("capture sqlite");
    let cell_count: i64 = conn
        .query_row("select count(*) from player_capture_cells", [], |row| {
            row.get(0)
        })
        .expect("cell count");
    let expected_cell_count: i64 = conn
        .query_row(
            "select sum(width * height) from player_capture_frames",
            [],
            |row| row.get(0),
        )
        .expect("expected cell count");
    let styled_count: i64 = conn
        .query_row(
            "select count(*) from player_capture_cells where foreground != 'defaultForeground' or background != 'transparent' or role is not null",
            [],
            |row| row.get(0),
        )
        .expect("styled count");
    let glyphs: String = conn
        .query_row(
            "select group_concat(glyph, '') from player_capture_cells where frame_index = 0 order by row, col",
            [],
            |row| row.get(0),
        )
        .expect("glyph concat");

    assert_eq!(report["cellCount"], expected_cell_count);
    assert_eq!(cell_count, expected_cell_count);
    assert!(
        styled_count > 0,
        "styled cells should retain style/role evidence"
    );
    assert!(
        glyphs.contains("BASELIN"),
        "dense capture should include text-grid glyphs, got {glyphs:?}"
    );
}

#[test]
fn test_fnc_cli_capture_cells_writes_procedural_recipe_metadata() {
    let temp_root = std::env::temp_dir().join("tui-vfx-player-capture-cells-procedural");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("temp root");
    let database_path = temp_root.join("capture.sqlite");

    let output = run_player_cli(
        vec![
            str_arg("capture-cells"),
            str_arg("--recipe"),
            recipe_path("sources/source_procedural_checkerboard.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--sqlite-output"),
            database_path.display().to_string(),
            str_arg("--frames"),
            str_arg("1"),
        ],
        "capture-cells procedural recipe player cli",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));

    let conn = rusqlite::Connection::open(&database_path).expect("capture sqlite");
    let cell_count: i64 = conn
        .query_row("select count(*) from player_capture_cells", [], |row| {
            row.get(0)
        })
        .expect("cell count");
    let procedural_source_rows: i64 = conn
        .query_row(
            "select count(*) from player_capture_provenance where source_descriptor_id = 'source.procedural'",
            [],
            |row| row.get(0),
        )
        .expect("procedural provenance count");
    let run_schema: String = conn
        .query_row(
            "select schema_version from player_capture_runs",
            [],
            |row| row.get(0),
        )
        .expect("run schema");

    assert!(cell_count > 0);
    assert_eq!(procedural_source_rows, 1);
    assert_eq!(run_schema, "v3.1.player.cellCapture.sqlite.1");
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_target_shader_blockers_json() {
    for (recipe, recipe_id, effect_id) in [
        (
            "shaders/compositions/shader_focus_field_center_binding.json",
            "debugShaderFocusFieldCenterBinding",
            "shader.focusField",
        ),
        (
            "shaders/compositions/shader_glisten_band_direction_blend_binding.json",
            "debugShaderGlistenBandDirectionBlendBinding",
            "shader.glistenBand",
        ),
        (
            "shaders/compositions/shader_highlighter_runtime_bindings.json",
            "debugShaderHighlighterRuntimeBindings",
            "shader.highlighter",
        ),
        (
            "shaders/compositions/shader_wayfinding_node_current_index_binding.json",
            "debugShaderWayfindingNodeCurrentIndexBinding",
            "shader.wayfindingNode",
        ),
        (
            "shaders/primitives/shader_barber_pole.json",
            "debugShaderBarberPole",
            "shader.barberPole",
        ),
        (
            "shaders/primitives/shader_diffusion_background.json",
            "debugShaderDiffusionBackground",
            "shader.diffusion",
        ),
        (
            "shaders/primitives/shader_radar_sweep.json",
            "debugShaderRadarSweep",
            "shader.radar",
        ),
    ] {
        let report = assert_native_backend_matches_ir_resolved_at_phase(
            recipe_path(recipe),
            recipe_id,
            effect_id,
            "0.35",
            "render-backend native target shader blockers player cli",
        );

        assert_eq!(
            report["compositionSpecSummary"]["styleStages"], 1,
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_target_shader_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        (
            "focus_field",
            "shaders/compositions/shader_focus_field_center_binding.json",
            "centerX",
        ),
        (
            "glisten_band",
            "shaders/compositions/shader_glisten_band_direction_blend_binding.json",
            "direction",
        ),
        (
            "highlighter",
            "shaders/compositions/shader_highlighter_runtime_bindings.json",
            "color",
        ),
        (
            "wayfinding_node",
            "shaders/compositions/shader_wayfinding_node_current_index_binding.json",
            "currentIndex",
        ),
        (
            "barber_pole",
            "shaders/primitives/shader_barber_pole.json",
            "applyTo",
        ),
        (
            "diffusion",
            "shaders/primitives/shader_diffusion_background.json",
            "applyTo",
        ),
        (
            "radar",
            "shaders/primitives/shader_radar_sweep.json",
            "applyTo",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root).expect("create temp unsupported shader fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe).expect("serialize unsupported shader recipe"),
            )
            .expect("write unsupported shader recipe");

            let output = run_native_render_backend_with_fail_on_fallback(
                recipe_path.display().to_string(),
                "render-backend native unsupported shader player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_rejects_native_target_shader_invalid_enum_values_json() {
    for (effect_name, recipe_path_fragment, input_id, invalid_value) in [
        (
            "focus_field",
            "shaders/compositions/shader_focus_field_center_binding.json",
            "shape",
            "triangle",
        ),
        (
            "glisten_band",
            "shaders/compositions/shader_glisten_band_direction_blend_binding.json",
            "direction",
            "diagonal",
        ),
        (
            "highlighter_mode",
            "shaders/compositions/shader_highlighter_runtime_bindings.json",
            "mode",
            "sparkle",
        ),
        (
            "highlighter_direction",
            "shaders/compositions/shader_highlighter_runtime_bindings.json",
            "direction",
            "diagonal",
        ),
        (
            "barber_pole",
            "shaders/primitives/shader_barber_pole.json",
            "applyTo",
            "invalidChannel",
        ),
        (
            "diffusion",
            "shaders/primitives/shader_diffusion_background.json",
            "applyTo",
            "invalidChannel",
        ),
        (
            "radar",
            "shaders/primitives/shader_radar_sweep.json",
            "applyTo",
            "invalidChannel",
        ),
    ] {
        let temp_root = std::env::temp_dir().join(format!(
            "tui-vfx-native-{effect_name}-{input_id}-invalid-enum"
        ));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create temp invalid shader enum fixture root");
        let recipe = unsupported_native_effect_shape_recipe(
            recipe_path_fragment,
            Some((input_id, unsupported_native_enum_value(invalid_value))),
            None,
            None,
        );
        let recipe_path = temp_root.join(format!("{effect_name}_{input_id}_invalid_enum.json"));
        fs::write(
            &recipe_path,
            serde_json::to_string_pretty(&recipe).expect("serialize invalid shader enum recipe"),
        )
        .expect("write invalid shader enum recipe");

        let output = run_native_render_backend_with_fail_on_fallback(
            recipe_path.display().to_string(),
            "render-backend native invalid shader enum player cli",
        );

        assert!(
            !output.status.success(),
            "{effect_name}/{input_id} invalid enum unexpectedly succeeded"
        );
        assert!(
            stderr(&output).contains("unsupportedNativeEffect"),
            "{effect_name}/{input_id} stderr: {}",
            stderr(&output)
        );
    }
}

#[test]
fn test_fnc_cli_lowers_crt_samplers_to_compositor_samplers_not_source_stage_json() {
    for (recipe, recipe_id, effect_id) in [
        (
            "samplers/sampler_crt.json",
            "debugSamplerCrt",
            "sampler.crt",
        ),
        (
            "samplers/sampler_crt_jitter.json",
            "debugSamplerCrtJitter",
            "sampler.crtJitter",
        ),
    ] {
        let report = assert_native_backend_matches_ir_resolved_at_phase(
            recipe_path(recipe),
            recipe_id,
            effect_id,
            "0.35",
            "render-backend native CRT sampler blockers player cli",
        );

        assert_eq!(report["compositionSpecSummary"]["samplers"], 1, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["contentStages"], 0,
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_crt_sampler_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        ("crt", "samplers/sampler_crt.json", "curvature"),
        (
            "crt_jitter",
            "samplers/sampler_crt_jitter.json",
            "amplitude",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root).expect("create temp unsupported CRT fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe).expect("serialize unsupported CRT recipe"),
            )
            .expect("write unsupported CRT recipe");

            let output = run_native_render_backend_with_fail_on_fallback(
                recipe_path.display().to_string(),
                "render-backend native unsupported CRT sampler player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_native_crt_sampler_with_clamped_numeric_values_json() {
    let temp_root = std::env::temp_dir().join("tui-vfx-native-crt-clamped-values");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create temp clamped CRT fixture root");
    let mut recipe =
        unsupported_native_effect_shape_recipe("samplers/sampler_crt.json", None, None, None);
    for node in recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("graph nodes object")
        .values_mut()
    {
        node["inputs"]["curvature"] = literal_number_input(-1.0);
        node["inputs"]["scanlineStrength"] = literal_number_input(2.0);
        node["inputs"]["jitter"] = literal_number_input(-3.0);
    }
    let recipe_path = temp_root.join("crt_clamped_values.json");
    fs::write(
        &recipe_path,
        serde_json::to_string_pretty(&recipe).expect("serialize clamped CRT recipe"),
    )
    .expect("write clamped CRT recipe");

    assert_native_backend_matches_ir_resolved_at_phase(
        recipe_path.display().to_string(),
        "debugSamplerCrt",
        "sampler.crt",
        "0.35",
        "render-backend native clamped CRT sampler player cli",
    );
}

#[test]
fn test_fnc_cli_renders_native_crt_jitter_sampler_with_clamped_numeric_values_json() {
    let temp_root = std::env::temp_dir().join("tui-vfx-native-crt-jitter-clamped-values");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create temp clamped CRT jitter fixture root");
    let mut recipe = unsupported_native_effect_shape_recipe(
        "samplers/sampler_crt_jitter.json",
        None,
        None,
        None,
    );
    for node in recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("graph nodes object")
        .values_mut()
    {
        node["inputs"]["amplitude"] = literal_number_input(-2.0);
        node["inputs"]["frequency"] = literal_number_input(-3.0);
        node["inputs"]["seed"] = literal_integer_input(-7);
    }
    let recipe_path = temp_root.join("crt_jitter_clamped_values.json");
    fs::write(
        &recipe_path,
        serde_json::to_string_pretty(&recipe).expect("serialize clamped CRT jitter recipe"),
    )
    .expect("write clamped CRT jitter recipe");

    assert_native_backend_matches_ir_resolved_at_phase(
        recipe_path.display().to_string(),
        "debugSamplerCrtJitter",
        "sampler.crtJitter",
        "0.35",
        "render-backend native clamped CRT jitter sampler player cli",
    );
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_radial_wipe_corner_blockers_json() {
    for (recipe, recipe_id, effect_id, expected_mask_count, expected_content_stage_count) in [
        (
            "masks/mask_radial.json",
            "debugMaskRadial",
            "mask.radial",
            1,
            0,
        ),
        (
            "masks/mask_wipe_corner_out_from_top_left.json",
            "debugMaskWipeCornerOutFromTopLeft",
            "mask.wipeCorner",
            1,
            0,
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native radial wipe-corner blockers player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["recipeId"], recipe_id, "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
        assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["masks"], expected_mask_count,
            "{recipe}"
        );
        assert_eq!(
            report["compositionSpecSummary"]["contentStages"], expected_content_stage_count,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_lowers_cellular_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_cellular.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native cellular mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskCellular");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.cellular"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_lowers_diamond_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_diamond.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native diamond mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskDiamond");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.diamond"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_lowers_dissolve_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_dissolve.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native dissolve mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskDissolve");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.dissolve"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_lowers_iris_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_iris.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native iris mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskIris");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.iris"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_lowers_blinds_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_blinds.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native blinds mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskBlinds");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.blinds"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_lowers_wipe_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native wipe mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskWipe");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.wipe"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_lowers_path_reveal_mask_to_compositor_mask_not_source_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("masks/mask_path_reveal.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native path reveal mask compositor lowering player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugMaskPathReveal");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("mask.pathReveal"))
    );
    assert_eq!(report["compositionSpecSummary"]["masks"], 1);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 0);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
}

#[test]
fn test_fnc_cli_rejects_native_radial_wipe_corner_invalid_enum_values_json() {
    for (effect_name, recipe_path_fragment, input_id, invalid_value) in [
        ("radial", "masks/mask_radial.json", "origin", "topLeft"),
        (
            "wipe_corner",
            "masks/mask_wipe_corner_out_from_top_left.json",
            "direction",
            "spiral",
        ),
    ] {
        let temp_root = std::env::temp_dir().join(format!(
            "tui-vfx-native-{effect_name}-{input_id}-invalid-enum"
        ));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create temp radial/wipe-corner enum fixture root");
        let recipe = unsupported_native_effect_shape_recipe(
            recipe_path_fragment,
            Some((input_id, unsupported_native_enum_value(invalid_value))),
            None,
            None,
        );
        let recipe_path = temp_root.join(format!("{effect_name}_{input_id}_invalid_enum.json"));
        fs::write(
            &recipe_path,
            serde_json::to_string_pretty(&recipe)
                .expect("serialize invalid radial/wipe-corner enum recipe"),
        )
        .expect("write invalid radial/wipe-corner enum recipe");

        let output = run_native_render_backend_with_fail_on_fallback(
            recipe_path.display().to_string(),
            "render-backend native invalid radial/wipe-corner enum player cli",
        );

        assert!(
            !output.status.success(),
            "{effect_name}/{input_id} invalid enum unexpectedly succeeded"
        );
        assert!(
            stderr(&output).contains("unsupportedNativeEffect"),
            "{effect_name}/{input_id} stderr: {}",
            stderr(&output)
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_radial_wipe_corner_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        ("radial", "masks/mask_radial.json", "origin"),
        (
            "wipe_corner",
            "masks/mask_wipe_corner_out_from_top_left.json",
            "direction",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root)
                .expect("create temp unsupported radial/wipe-corner fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe)
                    .expect("serialize unsupported radial/wipe-corner recipe"),
            )
            .expect("write unsupported radial/wipe-corner recipe");

            let output = run_native_render_backend_with_fail_on_fallback(
                recipe_path.display().to_string(),
                "render-backend native unsupported radial/wipe-corner player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_single_recipe_frame_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-recipe"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "render-recipe player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.frame.1");
    assert_eq!(report["status"], "rendered");
    assert!(report["nonEmptyCells"].as_u64().expect("cell count") > 0);
}

#[test]
fn test_fnc_cli_renders_single_recipe_render_ir_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-ir"),
            str_arg("--recipe"),
            recipe_path("complex/graph_parallel_overlap_conflict_snapshot.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "render-ir player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.renderIr.1");
    assert_eq!(report["status"], "rendered");
    assert!(
        !report["styledCells"]
            .as_array()
            .expect("styled cells")
            .is_empty()
    );
    assert!(
        !report["provenance"]
            .as_array()
            .expect("provenance")
            .is_empty()
    );
    assert!(
        report["warnings"]
            .as_array()
            .expect("warnings")
            .iter()
            .any(|warning| warning["code"] == "parallelGraphValueConflict")
    );
}

#[test]
fn test_fnc_cli_renders_compositor_backend_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("shaders/primitives/shader_linear_gradient_apply_to_both.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("json"),
        ],
        "render-backend compositor player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.renderBackend.1");
    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugShaderLinearGradientApplyToBoth");
    assert!(report["backendHash"].as_u64().expect("backend hash") > 0);
    assert!(
        report["nonDefaultStyledCells"]
            .as_u64()
            .expect("styled cell count")
            > 0
    );
    assert!(
        report["styledCells"]
            .as_array()
            .expect("styled cells")
            .iter()
            .any(|cell| cell["foreground"]
                .as_str()
                .unwrap_or("")
                .starts_with("rgba("))
    );
    let letter_cell_evidence = &report["letterCellEvidence"];
    let letter_cell_count = letter_cell_evidence["letterCellCount"]
        .as_u64()
        .expect("letter cell count");
    assert!(letter_cell_count > 0);
    let background_count_sum = letter_cell_evidence["backgroundClassCounts"]
        .as_object()
        .expect("background class counts")
        .values()
        .map(|value| value.as_u64().expect("background class count"))
        .sum::<u64>();
    assert_eq!(background_count_sum, letter_cell_count);
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_metadata_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("shaders/primitives/shader_linear_gradient_apply_to_both.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
        ],
        "render-backend native compositor player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["compositionSpecNonEmpty"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert!(report["loweredNodeCount"].as_u64().unwrap() > 0);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("shader.linearGradient"))
    );
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["code"] == "nativeCompositionSpecApplied")
    );
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "playerIrAlreadyResolved")
    );
}

#[test]
fn test_fnc_cli_native_mask_primitives_match_v2_deprecated_letter_cell_oracle_json() {
    for (recipe, phase, phase_t, expected_letter_cells, expected_background) in [
        (
            "masks/mask_wipe.json",
            "enter",
            "0.5",
            16,
            Some("rgba(20,40,60,255)"),
        ),
        (
            "masks/mask_wipe.json",
            "dwell",
            "1.0",
            18,
            Some("rgba(20,40,60,255)"),
        ),
        (
            "masks/mask_dissolve.json",
            "enter",
            "0.5",
            10,
            Some("rgba(20,50,20,255)"),
        ),
        (
            "masks/mask_dissolve.json",
            "dwell",
            "1.0",
            22,
            Some("rgba(20,50,20,255)"),
        ),
        (
            "masks/mask_iris.json",
            "enter",
            "0.5",
            18,
            Some("rgba(60,30,10,255)"),
        ),
        (
            "masks/mask_iris.json",
            "dwell",
            "1.0",
            18,
            Some("rgba(60,30,10,255)"),
        ),
        (
            "masks/mask_diamond.json",
            "enter",
            "0.5",
            21,
            Some("rgba(20,30,60,255)"),
        ),
        (
            "masks/mask_diamond.json",
            "dwell",
            "1.0",
            21,
            Some("rgba(20,30,60,255)"),
        ),
        (
            "masks/mask_checkers.json",
            "enter",
            "0.5",
            11,
            Some("rgba(50,20,50,255)"),
        ),
        (
            "masks/mask_checkers.json",
            "dwell",
            "1.0",
            22,
            Some("rgba(50,20,50,255)"),
        ),
        ("masks/mask_blinds.json", "enter", "0.5", 0, None),
        (
            "masks/mask_blinds.json",
            "enter",
            "0.8",
            20,
            Some("rgba(50,50,20,255)"),
        ),
        (
            "masks/mask_blinds.json",
            "dwell",
            "1.0",
            20,
            Some("rgba(50,50,20,255)"),
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg(phase_t),
            ],
            "render-backend native V2 deprecated mask oracle player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}/{phase}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}/{phase}");
        assert_eq!(
            report["letterCellEvidence"]["letterCellCount"], expected_letter_cells,
            "{recipe}/{phase}@{phase_t}"
        );
        match expected_background {
            Some(expected_background) => assert_eq!(
                report["letterCellEvidence"]["backgroundClassCounts"][expected_background],
                expected_letter_cells,
                "{recipe}/{phase}@{phase_t}"
            ),
            None => assert_eq!(
                report["letterCellEvidence"]["backgroundClassCounts"]
                    .as_object()
                    .expect("background class counts object")
                    .len(),
                0,
                "{recipe}/{phase}@{phase_t}"
            ),
        }
    }
}

#[test]
fn test_fnc_cli_native_filter_primitives_match_v2_deprecated_letter_cell_oracle_json() {
    for (
        recipe,
        phase,
        phase_t,
        expected_filter_count,
        expected_letter_cells,
        expected_foreground_background,
    ) in [
        (
            "filters/filter_dim.json",
            "enter",
            "1.0",
            1,
            19,
            "fg=rgba(179,179,179,255) bg=rgba(42,42,56,255)",
        ),
        (
            "filters/filter_dim.json",
            "dwell",
            "1.0",
            0,
            19,
            "fg=rgba(255,255,255,255) bg=rgba(60,60,80,255)",
        ),
        (
            "filters/filter_dim.json",
            "exit",
            "1.0",
            1,
            19,
            "fg=rgba(128,128,128,255) bg=rgba(60,60,80,255)",
        ),
        (
            "filters/filter_invert.json",
            "enter",
            "1.0",
            1,
            22,
            "fg=rgba(20,40,60,255) bg=rgba(0,255,255,255)",
        ),
        (
            "filters/filter_invert.json",
            "dwell",
            "1.0",
            0,
            22,
            "fg=rgba(0,255,255,255) bg=rgba(20,40,60,255)",
        ),
        (
            "filters/filter_invert.json",
            "exit",
            "1.0",
            1,
            22,
            "fg=rgba(20,40,60,255) bg=rgba(20,40,60,255)",
        ),
        (
            "filters/filter_greyscale.json",
            "enter",
            "1.0",
            0,
            9,
            "fg=rgba(255,100,100,255) bg=rgba(20,60,180,255)",
        ),
        (
            "filters/filter_greyscale.json",
            "dwell",
            "1.0",
            1,
            9,
            "fg=rgba(168,137,137,255) bg=rgba(54,62,86,255)",
        ),
        (
            "filters/filter_greyscale.json",
            "exit",
            "1.0",
            0,
            9,
            "fg=rgba(255,100,100,255) bg=rgba(20,60,180,255)",
        ),
        (
            "filters/filter_tint.json",
            "enter",
            "1.0",
            1,
            20,
            "fg=rgba(255,193,173,255) bg=rgba(126,64,44,255)",
        ),
        (
            "filters/filter_tint.json",
            "dwell",
            "1.0",
            0,
            20,
            "fg=rgba(255,255,255,255) bg=rgba(40,40,40,255)",
        ),
        (
            "filters/filter_tint.json",
            "exit",
            "1.0",
            1,
            20,
            "fg=rgba(132,162,255,255) bg=rgba(46,76,169,255)",
        ),
        (
            "filters/filter_fade_to_canvas_canvas_color_binding.json",
            "exit",
            "0.5",
            1,
            19,
            "fg=rgba(15,15,20,255) bg=rgba(15,15,20,255)",
        ),
        (
            "filters/filter_fade_to_canvas.json",
            "exit",
            "0.5",
            1,
            12,
            "fg=rgba(240,240,245,255) bg=rgba(240,240,245,255)",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg(phase_t),
            ],
            "render-backend native V2 deprecated filter oracle player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}/{phase}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}/{phase}");
        assert_eq!(
            report["compositionSpecSummary"]["filters"], expected_filter_count,
            "{recipe}/{phase}@{phase_t}"
        );
        assert_eq!(
            report["letterCellEvidence"]["letterCellCount"], expected_letter_cells,
            "{recipe}/{phase}@{phase_t}"
        );
        assert_eq!(
            report["letterCellEvidence"]["foregroundBackgroundClassCounts"]
                [expected_foreground_background],
            expected_letter_cells,
            "{recipe}/{phase}@{phase_t}"
        );
    }
}

#[test]
fn test_fnc_cli_native_dot_indicator_matches_v2_deprecated_glyph_oracle_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("filters/filter_dot_indicator.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("dwell"),
            str_arg("--phase-t"),
            str_arg("1.0"),
        ],
        "render-backend native V2 deprecated dot-indicator oracle player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["compositionSpecSummary"]["filters"], 1);
    assert_eq!(
        styled_cell_count(&report, "•", "rgba(255,200,100,255)", "rgba(20,18,12,255)"),
        3
    );
}

#[test]
fn test_fnc_cli_native_hover_bar_matches_v2_deprecated_glyph_oracle_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("filters/filter_hover_bar.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("dwell"),
            str_arg("--phase-t"),
            str_arg("1.0"),
        ],
        "render-backend native V2 deprecated hover-bar oracle player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["compositionSpecSummary"]["filters"], 1);
    assert_eq!(
        styled_cell_count(
            &report,
            "█",
            "rgba(100,200,255,255)",
            "rgba(100,200,255,255)"
        ),
        3
    );
    assert_eq!(report["rows"][1], " █OVER BAR                     │");
}

#[test]
fn test_fnc_cli_native_style_fades_preserve_canvas_color_endpoints_json() {
    for (recipe, phase, expected_foreground, expected_background) in [
        (
            "styles/style_fade_in_from_canvas.json",
            "enter",
            "rgba(27,31,37,255)",
            "rgba(194,196,196,255)",
        ),
        (
            "styles/style_fade_out_to_canvas_color.json",
            "exit",
            "rgba(102,187,201,255)",
            "rgba(11,24,49,255)",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.5"),
            ],
            "render-backend native canvas-aware style fade player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["filters"], 1, "{recipe}");
        assert!(
            styled_cell_color_count(&report, expected_foreground, expected_background) > 0,
            "{recipe} should preserve source foreground/background as distinct channels while fading to canvas"
        );
        assert_eq!(
            styled_cell_color_count(&report, "rgba(0,0,0,255)", "rgba(0,0,0,255)"),
            0,
            "{recipe} must not flash through hardcoded black"
        );
        assert_eq!(
            styled_cell_color_count(&report, "rgba(255,255,255,255)", "rgba(255,255,255,255)"),
            0,
            "{recipe} must not flash through hardcoded white"
        );
    }
}

#[test]
fn test_fnc_cli_native_style_fades_match_v2_deprecated_channel_oracle_json() {
    const V2_STYLE_FADE_CARD_CELL_COUNT: usize = 35 * 3;

    for (recipe, phase, expected_foreground, expected_background) in [
        (
            "styles/style_fade_in.json",
            "enter",
            "rgba(191,191,191,255)",
            "rgba(30,45,60,255)",
        ),
        (
            "styles/style_fade_out.json",
            "exit",
            "rgba(0,191,191,255)",
            "rgba(15,45,60,255)",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.5"),
            ],
            "render-backend native standard style fade V2 deprecated oracle player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["filters"], 1, "{recipe}");
        assert_eq!(
            report["rows"][0], "╭─────────────────────────────────╮",
            "{recipe}"
        );
        assert!(
            report["rows"][1]
                .as_str()
                .expect("row")
                .contains("STYLE TEST:"),
            "{recipe} should preserve the V2 debug message"
        );
        assert_eq!(
            styled_cell_color_count(&report, expected_foreground, expected_background),
            V2_STYLE_FADE_CARD_CELL_COUNT,
            "{recipe} should fade all 35x3 card cells while preserving distinct source foreground/background channels"
        );
    }
}

#[test]
fn test_fnc_cli_native_style_italic_window_matches_v2_deprecated_modifier_oracle_json() {
    const V2_ITALIC_WINDOW_CARD_CELL_COUNT: usize = 35 * 3;

    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("styles/style_italic_window.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.5"),
        ],
        "render-backend native V2 deprecated italic-window oracle player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["compositionSpecSummary"]["styleStages"], 1);
    assert_eq!(report["rows"][0], "╭─────────────────────────────────╮");
    assert_eq!(report["rows"][1], "│STYLE TEST: ItalicWindow         │");
    assert_eq!(
        styled_cell_modifier_count(
            &report,
            "rgba(255,255,0,255)",
            "rgba(50,40,20,255)",
            "italic"
        ),
        V2_ITALIC_WINDOW_CARD_CELL_COUNT
    );
}

#[test]
fn test_fnc_cli_native_style_pulse_matches_v2_deprecated_channel_oracle_json() {
    const V2_PULSE_CARD_CELL_COUNT: usize = 35 * 3;

    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("styles/style_pulse_runtime_frequency.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("dwell"),
            str_arg("--phase-t"),
            str_arg("0.25"),
        ],
        "render-backend native V2 deprecated pulse oracle player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["compositionSpecSummary"]["styleStages"], 1);
    assert_eq!(report["rows"][0], "╭─────────────────────────────────╮");
    assert_eq!(report["rows"][1], "│STYLE TEST: Pulse Effect         │");
    assert_eq!(
        styled_cell_color_count(&report, "rgba(230,50,50,255)", "rgba(152,55,55,255)"),
        V2_PULSE_CARD_CELL_COUNT
    );
}

#[test]
fn test_fnc_cli_native_style_color_fade_matches_v2_deprecated_letter_cell_oracle_json() {
    for (phase, phase_t, expected_foreground_background) in [
        (
            "enter",
            "0.5",
            "fg=rgba(255,227,152,255) bg=rgba(157,110,65,255)",
        ),
        (
            "exit",
            "0.5",
            "fg=rgba(180,150,180,255) bg=rgba(52,33,91,255)",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path("styles/style_color_fade.json"),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg(phase_t),
            ],
            "render-backend native V2 deprecated style color-fade oracle player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{phase}@{phase_t}");
        assert_eq!(report["fallbackUsed"], false, "{phase}@{phase_t}");
        assert_eq!(
            report["letterCellEvidence"]["letterCellCount"], 18,
            "{phase}@{phase_t}"
        );
        assert_eq!(
            report["letterCellEvidence"]["foregroundBackgroundClassCounts"]
                [expected_foreground_background],
            18,
            "{phase}@{phase_t}"
        );
    }
}

#[test]
fn test_fnc_cli_native_style_color_shift_matches_v2_deprecated_letter_cell_oracle_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("styles/style_color_shift.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.5"),
        ],
        "render-backend native V2 deprecated style color-shift oracle player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["compositionSpecSummary"]["styleStages"], 1);
    assert_eq!(report["letterCellEvidence"]["letterCellCount"], 19);
    assert_eq!(
        report["letterCellEvidence"]["foregroundBackgroundClassCounts"]["fg=rgba(0,57,230,255) bg=rgba(26,18,106,255)"],
        19
    );
}

#[test]
fn test_fnc_cli_lowers_sub_pixel_bar_filter_to_compositor_filter_not_style_stage_json() {
    for recipe in [
        "filters/filter_sub_pixel_bar.json",
        "filters/filter_sub_pixel_bar_progress_binding.json",
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("dwell"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native sub-pixel-bar filter compositor lowering player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["filters"], 1, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["styleStages"], 0,
            "{recipe}"
        );
        assert_eq!(
            report["loweredEffectIds"],
            serde_json::json!(["filter.subPixelBar"]),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_sub_pixel_bar_adapter_only_semantics_json() {
    for (mutation_name, input_id, value) in [
        (
            "reverse_direction",
            "direction",
            unsupported_native_enum_value("rightToLeft"),
        ),
        (
            "legacy_color_alias",
            "color",
            serde_json::json!({
                "kind": "literal",
                "value": {
                    "kind": "color",
                    "value": {
                        "r": 255,
                        "g": 170,
                        "b": 40,
                        "a": 255
                    }
                }
            }),
        ),
        (
            "legacy_bar_color_alias",
            "barColor",
            serde_json::json!({
                "kind": "literal",
                "value": {
                    "kind": "color",
                    "value": {
                        "r": 255,
                        "g": 170,
                        "b": 40,
                        "a": 255
                    }
                }
            }),
        ),
        (
            "legacy_background_color_alias",
            "bgColor",
            serde_json::json!({
                "kind": "literal",
                "value": {
                    "kind": "color",
                    "value": {
                        "r": 20,
                        "g": 20,
                        "b": 20,
                        "a": 255
                    }
                }
            }),
        ),
    ] {
        let temp_root =
            std::env::temp_dir().join(format!("tui-vfx-native-sub-pixel-bar-{mutation_name}"));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create temp unsupported sub-pixel-bar fixture root");
        let recipe = unsupported_native_effect_shape_recipe(
            "filters/filter_sub_pixel_bar.json",
            Some((input_id, value)),
            None,
            None,
        );
        let recipe_path = temp_root.join(format!("{mutation_name}.json"));
        fs::write(
            &recipe_path,
            serde_json::to_string_pretty(&recipe)
                .expect("serialize unsupported sub-pixel-bar recipe"),
        )
        .expect("write unsupported sub-pixel-bar recipe");

        let output = run_player_cli(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path.display().to_string(),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("dwell"),
            ],
            "render-backend native unsupported sub-pixel-bar player cli",
        );

        assert!(
            !output.status.success(),
            "{mutation_name} unexpectedly succeeded"
        );
        assert!(
            stderr(&output).contains("unsupportedNativeEffect"),
            "{mutation_name} stderr: {}",
            stderr(&output)
        );
    }
}

#[test]
fn test_fnc_cli_lowers_matrix_rain_filter_to_compositor_filter_not_style_stage_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("filters/filter_matrix_rain_speed_profile.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native matrix-rain filter compositor lowering player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["compositionSpecSummary"]["filters"], 1);
    assert_eq!(report["compositionSpecSummary"]["styleStages"], 0);
    assert_eq!(
        report["loweredEffectIds"],
        serde_json::json!(["filter.matrixRain"])
    );
}

#[test]
fn test_fnc_cli_rejects_matrix_rain_adapter_only_semantics_json() {
    for (mutation_name, input_id, value) in [
        ("adapter_speed", "speed", literal_number_input(1.0)),
        (
            "background_affect",
            "affect",
            unsupported_native_enum_value("background"),
        ),
        (
            "unknown_mode",
            "mode",
            unsupported_native_enum_value("diagonal"),
        ),
        (
            "unknown_preset",
            "preset",
            serde_json::json!({
                "kind": "literal",
                "value": {
                    "kind": "string",
                    "value": "emoji"
                }
            }),
        ),
    ] {
        let temp_root =
            std::env::temp_dir().join(format!("tui-vfx-native-matrix-rain-{mutation_name}"));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create temp unsupported matrix-rain fixture root");
        let recipe = unsupported_native_effect_shape_recipe(
            "filters/filter_matrix_rain_speed_profile.json",
            Some((input_id, value)),
            None,
            None,
        );
        let recipe_path = temp_root.join(format!("{mutation_name}.json"));
        fs::write(
            &recipe_path,
            serde_json::to_string_pretty(&recipe)
                .expect("serialize unsupported matrix-rain recipe"),
        )
        .expect("write unsupported matrix-rain recipe");

        let output = run_player_cli(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path.display().to_string(),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
            ],
            "render-backend native unsupported matrix-rain player cli",
        );

        assert!(
            !output.status.success(),
            "{mutation_name} unexpectedly succeeded"
        );
        assert!(
            stderr(&output).contains("unsupportedNativeEffect"),
            "{mutation_name} stderr: {}",
            stderr(&output)
        );
    }
}

#[test]
fn test_fnc_cli_lowers_underline_wipe_filter_to_compositor_filter_not_style_stage_json() {
    for (recipe, phase) in [
        ("filters/filter_underline_wipe.json", "enter"),
        (
            "filters/filter_underline_wipe_progress_binding.json",
            "dwell",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native underline-wipe filter compositor lowering player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["filters"], 1, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["styleStages"], 0,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("filter.underlineWipe")),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_lowers_bracket_emphasis_filter_to_compositor_filter_not_style_stage_json() {
    for recipe in [
        "filters/filter_bracket_emphasis.json",
        "filters/filter_bracket_emphasis_progress_binding.json",
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("dwell"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native bracket-emphasis filter compositor lowering player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["filters"], 1, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["styleStages"], 0,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("filter.bracketEmphasis")),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_lowers_edge_grow_filter_to_compositor_filter_not_style_stage_json() {
    for (recipe, phase) in [
        ("filters/filter_edge_grow_left.json", "enter"),
        ("filters/filter_edge_grow_bottom_binding.json", "dwell"),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native edge-grow filter compositor lowering player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["filters"], 1, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["styleStages"], 0,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("filter.edgeGrow")),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_lowers_fault_line_sampler_to_compositor_sampler_not_source_stage_json() {
    for recipe in [
        "samplers/sampler_faultline.json",
        "samplers/sampler_faultline_offset_positive.json",
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native fault-line sampler compositor lowering player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["compositionSpecSummary"]["samplers"], 1, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["contentStages"], 0,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!("sampler.faultLine")),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_native_sampler_faultline_matches_v2_deprecated_rows_json() {
    for (phase, expected_sampler_count, expected_letter_cells, expected_rows) in [
        (
            "enter",
            1,
            26,
            [
                "        ╭───────────────────────────────",
                "        │SAMPLER TEST: FaultLine Effect",
                "                               │",
                "                               │",
                "───────────────────────────────╯",
            ],
        ),
        (
            "dwell",
            0,
            26,
            [
                "╭──────────────────────────────────────╮",
                "│SAMPLER TEST: FaultLine Effect        │",
                "│                                      │",
                "│                                      │",
                "╰──────────────────────────────────────╯",
            ],
        ),
        (
            "exit",
            1,
            23,
            [
                "            ╭───────────────────────────",
                "            │SAMPLER TEST: FaultLine Eff",
                "            │",
                "                           │",
                "───────────────────────────╯",
            ],
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path("samplers/sampler_faultline.json"),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.5"),
            ],
            "render-backend native V2 deprecated sampler fault-line oracle player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{phase}");
        assert_eq!(report["fallbackUsed"], false, "{phase}");
        assert_eq!(
            report["compositionSpecSummary"]["samplers"], expected_sampler_count,
            "{phase}"
        );
        assert_eq!(report["rows"].as_array().expect("rows").len(), 5, "{phase}");
        for (index, expected_row) in expected_rows.iter().enumerate() {
            assert_eq!(report["rows"][index], *expected_row, "{phase} row {index}");
        }
        assert_eq!(
            report["letterCellEvidence"]["foregroundBackgroundClassCounts"]["fg=rgba(255,255,0,255) bg=rgba(50,40,20,255)"],
            expected_letter_cells,
            "{phase}"
        );
    }
}

#[test]
fn test_fnc_cli_native_sampler_shredder_matches_v2_deprecated_rows_json() {
    for (phase, expected_sampler_count, expected_styled_color_count, expected_rows) in [
        (
            "enter",
            1,
            60,
            [
                "│",
                "│       ────    ────    ────",
                "╰───                            ────",
                "",
                "            ────    ────    ────",
            ],
        ),
        (
            "dwell",
            0,
            200,
            [
                "╭──────────────────────────────────────╮",
                "│SAMPLER TEST: Shredder Effect         │",
                "│                                      │",
                "│                                      │",
                "╰──────────────────────────────────────╯",
            ],
        ),
        (
            "exit",
            1,
            6,
            ["╰──                                 ───", "", "", "", ""],
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path("samplers/sampler_shredder.json"),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.5"),
            ],
            "render-backend native V2 deprecated sampler shredder oracle player cli",
        );

        assert_eq!(report["compositionMode"], "native", "{phase}");
        assert_eq!(report["fallbackUsed"], false, "{phase}");
        assert_eq!(
            report["compositionSpecSummary"]["samplers"], expected_sampler_count,
            "{phase}"
        );
        assert_eq!(report["rows"].as_array().expect("rows").len(), 5, "{phase}");
        for (index, expected_row) in expected_rows.iter().enumerate() {
            assert_eq!(report["rows"][index], *expected_row, "{phase} row {index}");
        }
        assert_eq!(
            styled_cell_color_count(&report, "rgba(255,0,0,255)", "rgba(50,15,15,255)"),
            expected_styled_color_count,
            "{phase}"
        );
    }
}

#[test]
fn test_fnc_cli_native_sampler_shredder_uses_descriptor_defaults_when_fields_omitted_json() {
    let mut recipe: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(debug_recipe_root().join("samplers/sampler_shredder.json"))
            .expect("read shredder recipe"),
    )
    .expect("parse shredder recipe");
    for field in ["stripeWidth", "oddSpeed", "evenSpeed"] {
        recipe["graph"]["nodes"]["shredderEnter"]["inputs"]
            .as_object_mut()
            .expect("enter inputs")
            .remove(field);
    }
    let temp_root = std::env::temp_dir().join("tui-vfx-shredder-descriptor-defaults");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create shredder defaults temp root");
    let temp_recipe_path = temp_root.join("sampler_shredder_defaults.json");
    fs::write(
        &temp_recipe_path,
        serde_json::to_string_pretty(&recipe).expect("serialize shredder defaults recipe"),
    )
    .expect("write shredder defaults recipe");

    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            temp_recipe_path.display().to_string(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.5"),
        ],
        "render-backend native shredder descriptor defaults player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(
        report["rows"],
        serde_json::json!([
            "│",
            "│       ────    ────    ────",
            "╰───                            ────",
            "",
            "            ────    ────    ────"
        ])
    );
    assert_eq!(
        styled_cell_color_count(&report, "rgba(255,0,0,255)", "rgba(50,15,15,255)"),
        60
    );
}

#[test]
fn test_fnc_cli_native_sub_pixel_bar_matches_v2_deprecated_glyph_oracle_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("filters/filter_sub_pixel_bar.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("dwell"),
            str_arg("--phase-t"),
            str_arg("1.0"),
        ],
        "render-backend native V2 deprecated sub-pixel-bar oracle player cli",
    );

    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["compositionSpecSummary"]["styleStages"], 1);
    assert_eq!(
        styled_cell_count(&report, "█", "rgba(100,220,255,255)", "rgba(30,40,50,255)"),
        66
    );
    assert_eq!(
        styled_cell_count(&report, "▏", "rgba(100,220,255,255)", "rgba(30,40,50,255)"),
        3
    );
    assert_eq!(
        styled_cell_count(&report, " ", "rgba(30,40,50,255)", "rgba(30,40,50,255)"),
        33
    );
    assert_eq!(report["rows"][1], "██████████████████████▏");
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_simple_filter_families_json() {
    for (recipe, effect_id, phase, expected_filter_count) in [
        ("filters/filter_invert.json", "filter.invert", "enter", 1),
        (
            "filters/filter_greyscale.json",
            "filter.greyscale",
            "dwell",
            1,
        ),
        (
            "filters/filter_fade_to_canvas_canvas_color_binding.json",
            "filter.fadeToCanvas",
            "exit",
            1,
        ),
        (
            "filters/filter_vignette_side_pair.json",
            "filter.vignette",
            "dwell",
            1,
        ),
        ("filters/filter_crt.json", "filter.crt", "dwell", 1),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
            ],
            "render-backend native simple filter family player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
        assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["filters"], expected_filter_count,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_content_typewriter_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("content/content_typewriter.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase-t"),
            str_arg("0.5"),
        ],
        "render-backend native content typewriter player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["recipeId"], "debugContentTypewriter");
    assert_eq!(report["compositionMode"], "native");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], true);
    assert_eq!(report["nativeLoweringSucceeded"], true);
    assert_eq!(report["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["nativeSourceIsolated"], true);
    assert_eq!(report["compositionSpecSummary"]["contentStages"], 1);
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("content.typewriter"))
    );
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect")
    );
    assert!(
        report["rows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row.as_str().unwrap_or("").contains('▌'))
    );
}

#[test]
fn test_fnc_cli_rejects_native_content_typewriter_with_unsupported_input_json() {
    let temp_root = std::env::temp_dir().join("tui-vfx-native-typewriter-unsupported");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create temp typewriter fixture root");
    let recipe_path = temp_root.join("content_typewriter_unsupported.json");
    fs::write(
        &recipe_path,
        serde_json::to_string_pretty(&unsupported_content_typewriter_recipe())
            .expect("serialize unsupported typewriter recipe"),
    )
    .expect("write unsupported typewriter recipe");

    let output = run_player_cli(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path.display().to_string(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
        ],
        "render-backend native unsupported content typewriter player cli",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("unsupportedNativeEffect"));
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_split_flap_and_odometer_json() {
    for (recipe, recipe_id, effect_id) in [
        (
            "content/content_split_flap_ambient_board.json",
            "debugContentSplitFlapAmbientBoard",
            "content.splitFlap",
        ),
        (
            "content/content_split_flap_cascade.json",
            "debugContentSplitFlapCascade",
            "content.splitFlap",
        ),
        (
            "content/content_split_flap_arrivals_board.json",
            "debugContentSplitFlapArrivalsBoard",
            "content.splitFlap",
        ),
        (
            "content/content_split_flap.json",
            "debugContentSplitFlap",
            "content.splitFlap",
        ),
        (
            "content/content_split_flap_authentic_timing.json",
            "debugContentSplitFlapAuthenticTiming",
            "content.splitFlap",
        ),
        (
            "content/content_odometer_cell_roll_diagonal.json",
            "debugContentOdometerCellRollDiagonal",
            "content.odometer",
        ),
        (
            "content/content_odometer.json",
            "debugContentOdometer",
            "content.odometer",
        ),
        (
            "content/content_odometer_3x3_count_bindable.json",
            "debugContentOdometer3x3CountBindable",
            "content.odometer",
        ),
        (
            "content/content_odometer_cell_roll_down.json",
            "debugContentOdometerCellRollDown",
            "content.odometer",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase-t"),
                str_arg("0.25"),
            ],
            "render-backend native content split-flap/odometer player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["recipeId"], recipe_id, "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
        assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["contentStages"], 1,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_split_flap_and_odometer_unsupported_content_shapes_json() {
    for (effect_name, mutation_name, recipe) in [
        (
            "split_flap",
            "unsupported_input",
            unsupported_content_recipe(
                "content/content_split_flap.json",
                Some(("unsupportedNativeField", unsupported_native_input())),
                None,
                None,
            ),
        ),
        (
            "split_flap",
            "unsupported_output",
            unsupported_content_recipe(
                "content/content_split_flap.json",
                None,
                Some(serde_json::json!({
                    "debugOutput": {
                        "source": {
                            "kind": "input",
                            "id": "settle"
                        }
                    }
                })),
                None,
            ),
        ),
        (
            "split_flap",
            "unsupported_scope",
            unsupported_content_recipe(
                "content/content_split_flap.json",
                None,
                None,
                Some(serde_json::json!({
                    "kind": "rowRange",
                    "start": 0,
                    "end": 1
                })),
            ),
        ),
        (
            "odometer",
            "unsupported_input",
            unsupported_content_recipe(
                "content/content_odometer.json",
                Some(("unsupportedNativeField", unsupported_native_input())),
                None,
                None,
            ),
        ),
        (
            "odometer",
            "unsupported_output",
            unsupported_content_recipe(
                "content/content_odometer.json",
                None,
                Some(serde_json::json!({
                    "debugOutput": {
                        "source": {
                            "kind": "input",
                            "id": "direction"
                        }
                    }
                })),
                None,
            ),
        ),
        (
            "odometer",
            "unsupported_scope",
            unsupported_content_recipe(
                "content/content_odometer.json",
                None,
                None,
                Some(serde_json::json!({
                    "kind": "rowRange",
                    "start": 0,
                    "end": 1
                })),
            ),
        ),
    ] {
        let temp_root = std::env::temp_dir().join(format!(
            "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
        ));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create temp unsupported content fixture root");
        let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
        fs::write(
            &recipe_path,
            serde_json::to_string_pretty(&recipe).expect("serialize unsupported content recipe"),
        )
        .expect("write unsupported content recipe");

        let output = run_player_cli(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path.display().to_string(),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
            ],
            "render-backend native unsupported split-flap/odometer player cli",
        );

        assert!(
            !output.status.success(),
            "{effect_name}/{mutation_name} unexpectedly succeeded"
        );
        assert!(
            stderr(&output).contains("unsupportedNativeEffect"),
            "{effect_name}/{mutation_name} stderr: {}",
            stderr(&output)
        );
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_remaining_content_transforms_json() {
    for (recipe, recipe_id, effect_id) in [
        (
            "content/content_cell_motion_middle_out.json",
            "debugContentCellMotionMiddleOut",
            "content.cellMotion",
        ),
        (
            "content/content_cell_motion_root_border_fixed.json",
            "debugContentCellMotionRootBorderFixed",
            "content.cellMotion",
        ),
        (
            "content/content_cell_motion_slice.json",
            "debugContentCellMotionSlice",
            "content.cellMotion",
        ),
        (
            "content/content_marquee.json",
            "debugContentMarquee",
            "content.marquee",
        ),
        (
            "content/content_marquee_direction_reverse.json",
            "debugContentMarqueeDirectionReverse",
            "content.marquee",
        ),
        (
            "content/content_morph.json",
            "debugContentMorph",
            "content.morph",
        ),
        (
            "content/content_morph_target_dots.json",
            "debugContentMorphTargetDots",
            "content.morph",
        ),
        (
            "content/content_scramble.json",
            "debugContentScramble",
            "content.scramble",
        ),
        (
            "content/content_scramble_seed_charset.json",
            "debugContentScrambleSeedCharset",
            "content.scramble",
        ),
        (
            "content/content_wrap_indicator.json",
            "debugContentWrapIndicator",
            "content.wrapIndicator",
        ),
        (
            "content/content_wrap_indicator_every_timing.json",
            "debugContentWrapIndicatorEveryTiming",
            "content.wrapIndicator",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native remaining content transforms player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["recipeId"], recipe_id, "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
        assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"]["contentStages"], 1,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_remaining_content_transform_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        (
            "cell_motion",
            "content/content_cell_motion_slice.json",
            "route",
        ),
        ("marquee", "content/content_marquee.json", "direction"),
        ("morph", "content/content_morph.json", "target"),
        ("scramble", "content/content_scramble.json", "seed"),
        (
            "wrap_indicator",
            "content/content_wrap_indicator.json",
            "every",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_content_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_content_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_content_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root).expect("create temp unsupported content fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe)
                    .expect("serialize unsupported content recipe"),
            )
            .expect("write unsupported content recipe");

            let output = run_player_cli(
                vec![
                    str_arg("render-backend"),
                    str_arg("--recipe"),
                    recipe_path.display().to_string(),
                    str_arg("--descriptor-pack"),
                    descriptor_pack_path(),
                    str_arg("--backend"),
                    str_arg("compositor"),
                    str_arg("--composition-mode"),
                    str_arg("native"),
                    str_arg("--fail-on-fallback"),
                    str_arg("--format"),
                    str_arg("json"),
                ],
                "render-backend native unsupported remaining content transform player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_exact_effect_blocker_subset_json() {
    for (recipe, recipe_id, effect_id, summary_key) in [
        (
            "shaders/primitives/shader_reveal_wipe.json",
            "debugShaderRevealWipe",
            "shader.revealWipe",
            "shaderLayers",
        ),
        (
            "shaders/primitives/shader_reveal_wipe_corner_out_top_left.json",
            "debugShaderRevealWipeCornerOutTopLeft",
            "shader.revealWipe",
            "shaderLayers",
        ),
        (
            "shaders/primitives/shader_reveal_wipe_right_to_left.json",
            "debugShaderRevealWipeRightToLeft",
            "shader.revealWipe",
            "shaderLayers",
        ),
        (
            "filters/filter_pattern_fill.json",
            "debugFilterPatternFill",
            "filter.patternFill",
            "filters",
        ),
        (
            "filters/filter_pattern_fill_density_anchors.json",
            "debugFilterPatternFillDensityAnchors",
            "filter.patternFill",
            "filters",
        ),
        (
            "filters/filter_kitt_scanner.json",
            "debugFilterKittScanner",
            "filter.kittScanner",
            "filters",
        ),
        (
            "filters/filter_kitt_scanner_vertical.json",
            "debugFilterKittScannerVertical",
            "filter.kittScanner",
            "filters",
        ),
        (
            "masks/mask_materialize_center.json",
            "debugMaskMaterializeCenter",
            "mask.materialize",
            "masks",
        ),
        (
            "masks/mask_materialize_corner.json",
            "debugMaskMaterializeCorner",
            "mask.materializeCorner",
            "masks",
        ),
        (
            "masks/mask_materialize_progress.json",
            "debugMaskMaterializeProgress",
            "mask.materialize",
            "masks",
        ),
        (
            "masks/mask_noise_dither.json",
            "debugMaskNoiseDither",
            "mask.noiseDither",
            "masks",
        ),
        (
            "masks/mask_noise_dither_seed_profile.json",
            "debugMaskNoiseDitherSeedProfile",
            "mask.noiseDither",
            "masks",
        ),
        (
            "samplers/sampler_faultline.json",
            "debugSamplerFaultLine",
            "sampler.faultLine",
            "samplers",
        ),
        (
            "samplers/sampler_faultline_offset_positive.json",
            "debugSamplerFaultlineOffsetPositive",
            "sampler.faultLine",
            "samplers",
        ),
        (
            "samplers/sampler_shredder.json",
            "debugSamplerShredder",
            "sampler.shredder",
            "samplers",
        ),
        (
            "samplers/sampler_shredder_slice_width_stride.json",
            "debugSamplerShredderSliceWidthStride",
            "sampler.shredder",
            "samplers",
        ),
        (
            "samplers/sampler_radial_twist_strength_extremes.json",
            "debugSamplerRadialTwistStrengthExtremes",
            "sampler.radialTwist",
            "samplers",
        ),
        (
            "samplers/sampler_radial_twist_v3.json",
            "debugSamplerRadialTwist",
            "sampler.radialTwist",
            "samplers",
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native exact effect blocker subset player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["recipeId"], recipe_id, "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
        assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
        assert_eq!(report["compositionSpecSummary"][summary_key], 1, "{recipe}");
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_exact_effect_blocker_subset_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        (
            "reveal_wipe",
            "shaders/primitives/shader_reveal_wipe.json",
            "direction",
        ),
        (
            "pattern_fill",
            "filters/filter_pattern_fill.json",
            "pattern",
        ),
        (
            "kitt_scanner",
            "filters/filter_kitt_scanner.json",
            "scanColor",
        ),
        (
            "materialize",
            "masks/mask_materialize_center.json",
            "origin",
        ),
        ("noise_dither", "masks/mask_noise_dither.json", "seed"),
        (
            "shredder",
            "samplers/sampler_shredder_slice_width_stride.json",
            "sliceWidth",
        ),
        (
            "radial_twist",
            "samplers/sampler_radial_twist_v3.json",
            "strength",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root)
                .expect("create temp unsupported exact effect fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe)
                    .expect("serialize unsupported exact effect recipe"),
            )
            .expect("write unsupported exact effect recipe");

            let output = run_player_cli(
                vec![
                    str_arg("render-backend"),
                    str_arg("--recipe"),
                    recipe_path.display().to_string(),
                    str_arg("--descriptor-pack"),
                    descriptor_pack_path(),
                    str_arg("--backend"),
                    str_arg("compositor"),
                    str_arg("--composition-mode"),
                    str_arg("native"),
                    str_arg("--fail-on-fallback"),
                    str_arg("--format"),
                    str_arg("json"),
                ],
                "render-backend native unsupported exact effect blocker subset player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_vignette_filter_json() {
    let (recipe, recipe_id, effect_id, summary_key, expected_stage_count) = (
        "filters/filter_vignette.json",
        "debugFilterVignette",
        "filter.vignette",
        "filters",
        1,
    );
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path(recipe),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native vignette mask blockers player cli",
    );

    assert_eq!(report["backend"], "compositor", "{recipe}");
    assert_eq!(report["recipeId"], recipe_id, "{recipe}");
    assert_eq!(report["compositionMode"], "native", "{recipe}");
    assert_eq!(report["fallbackUsed"], false, "{recipe}");
    assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
    assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
    assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
    assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
    assert_eq!(
        report["compositionSpecSummary"][summary_key], expected_stage_count,
        "{recipe}"
    );
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!(effect_id)),
        "{recipe}"
    );
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
        "{recipe}"
    );
}

#[test]
fn test_fnc_cli_rejects_vignette_player_style_only_semantics_json() {
    for (mutation_name, input_id, value) in [
        (
            "edge_color",
            "edgeColor",
            serde_json::json!({
                "kind": "literal",
                "value": {
                    "kind": "color",
                    "value": {
                        "r": 10,
                        "g": 20,
                        "b": 36,
                        "a": 255
                    }
                }
            }),
        ),
        (
            "apply_to_background",
            "applyTo",
            unsupported_native_enum_value("background"),
        ),
        (
            "apply_to_both",
            "applyTo",
            unsupported_native_enum_value("both"),
        ),
        (
            "apply_to_foreground",
            "applyTo",
            unsupported_native_enum_value("foreground"),
        ),
    ] {
        let temp_root =
            std::env::temp_dir().join(format!("tui-vfx-native-vignette-{mutation_name}"));
        let _ = fs::remove_dir_all(&temp_root);
        fs::create_dir_all(&temp_root).expect("create temp unsupported vignette fixture root");
        let recipe = unsupported_native_effect_shape_recipe(
            "filters/filter_vignette.json",
            Some((input_id, value)),
            None,
            None,
        );
        let recipe_path = temp_root.join(format!("{mutation_name}.json"));
        fs::write(
            &recipe_path,
            serde_json::to_string_pretty(&recipe).expect("serialize unsupported vignette recipe"),
        )
        .expect("write unsupported vignette recipe");

        let output = run_player_cli(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path.display().to_string(),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
            ],
            "render-backend native unsupported vignette player cli",
        );

        assert!(
            !output.status.success(),
            "{mutation_name} unexpectedly succeeded"
        );
        assert!(
            stderr(&output).contains("unsupportedNativeEffect"),
            "{mutation_name} stderr: {}",
            stderr(&output)
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_iris_mask_invalid_shape_json() {
    let (effect_name, recipe_path_fragment, input_id, invalid_value) =
        ("iris", "masks/mask_iris.json", "shape", "triangle");
    let temp_root = std::env::temp_dir().join(format!(
        "tui-vfx-native-{effect_name}-{input_id}-invalid-enum"
    ));
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create temp invalid enum fixture root");
    let recipe = unsupported_native_effect_shape_recipe(
        recipe_path_fragment,
        Some((input_id, unsupported_native_enum_value(invalid_value))),
        None,
        None,
    );
    let recipe_path = temp_root.join(format!("{effect_name}_{input_id}_invalid_enum.json"));
    fs::write(
        &recipe_path,
        serde_json::to_string_pretty(&recipe).expect("serialize invalid enum recipe"),
    )
    .expect("write invalid enum recipe");

    let output = run_player_cli(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path.display().to_string(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
        ],
        "render-backend native invalid enum vignette mask blocker player cli",
    );

    assert!(
        !output.status.success(),
        "{effect_name}/{input_id} invalid enum unexpectedly succeeded"
    );
    assert!(
        stderr(&output).contains("unsupportedNativeEffect"),
        "{effect_name}/{input_id} stderr: {}",
        stderr(&output)
    );
}

#[test]
fn test_fnc_cli_rejects_native_vignette_mask_blocker_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        ("vignette", "filters/filter_vignette.json", "strength"),
        ("cellular", "masks/mask_cellular.json", "pattern"),
        ("diamond", "masks/mask_diamond.json", "softEdge"),
        ("dissolve", "masks/mask_dissolve.json", "seed"),
        ("iris", "masks/mask_iris.json", "shape"),
        ("none", "masks/mask_none.json", "debugOutput"),
        ("path_reveal", "masks/mask_path_reveal.json", "path"),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root)
                .expect("create temp unsupported vignette mask fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe)
                    .expect("serialize unsupported vignette mask recipe"),
            )
            .expect("write unsupported vignette mask recipe");

            let output = run_player_cli(
                vec![
                    str_arg("render-backend"),
                    str_arg("--recipe"),
                    recipe_path.display().to_string(),
                    str_arg("--descriptor-pack"),
                    descriptor_pack_path(),
                    str_arg("--backend"),
                    str_arg("compositor"),
                    str_arg("--composition-mode"),
                    str_arg("native"),
                    str_arg("--fail-on-fallback"),
                    str_arg("--format"),
                    str_arg("json"),
                    str_arg("--phase"),
                    str_arg("enter"),
                ],
                "render-backend native unsupported vignette mask blocker player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_one_off_content_filter_blockers_json() {
    let (recipe, recipe_id, effect_id, summary_key, expected_stage_count) = (
        "content/content_slide_shift.json",
        "debugContentSlideShift",
        "content.slideShift",
        "contentStages",
        1,
    );
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path(recipe),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend native one-off content filter blockers player cli",
    );

    assert_eq!(report["backend"], "compositor", "{recipe}");
    assert_eq!(report["recipeId"], recipe_id, "{recipe}");
    assert_eq!(report["compositionMode"], "native", "{recipe}");
    assert_eq!(report["fallbackUsed"], false, "{recipe}");
    assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
    assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
    assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
    assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
    assert_eq!(
        report["compositionSpecSummary"][summary_key], expected_stage_count,
        "{recipe}"
    );
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!(effect_id)),
        "{recipe}"
    );
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
        "{recipe}"
    );

    let ir_resolved_report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path(recipe),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("ir-resolved"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--phase-t"),
            str_arg("0.35"),
        ],
        "render-backend ir-resolved one-off content filter parity player cli",
    );
    assert_eq!(report["rows"], ir_resolved_report["rows"], "{recipe}");
    assert_eq!(
        report["styledCells"], ir_resolved_report["styledCells"],
        "{recipe}"
    );
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_migrated_filter_parity_recipes_json() {
    for (recipe, recipe_id, effect_id, phase, summary_key, expected_stage_count) in [
        (
            "filters/filter_bracket_emphasis.json",
            "debugFilterBracketEmphasis",
            "filter.bracketEmphasis",
            "dwell",
            "filters",
            1,
        ),
        (
            "filters/filter_bracket_emphasis_progress_binding.json",
            "debugFilterBracketEmphasisProgressBinding",
            "filter.bracketEmphasis",
            "dwell",
            "filters",
            1,
        ),
        (
            "filters/filter_edge_grow_bottom_binding.json",
            "debugFilterEdgeGrowBottomBinding",
            "filter.edgeGrow",
            "dwell",
            "filters",
            1,
        ),
        (
            "filters/filter_matrix_rain_speed_profile.json",
            "debugFilterMatrixRainSpeedProfile",
            "filter.matrixRain",
            "enter",
            "filters",
            1,
        ),
        (
            "filters/filter_kitt_scanner_progress_binding.json",
            "debugFilterKittScannerProgressBinding",
            "filter.kittScanner",
            "dwell",
            "filters",
            1,
        ),
        (
            "filters/filter_pill_button.json",
            "debugFilterPillButton",
            "filter.pillButton",
            "dwell",
            "filters",
            1,
        ),
        (
            "filters/filter_underline_wipe_progress_binding.json",
            "debugFilterUnderlineWipeProgressBinding",
            "filter.underlineWipe",
            "dwell",
            "filters",
            1,
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg(phase),
                str_arg("--phase-t"),
                str_arg("0.5"),
            ],
            "render-backend native migrated filter parity recipe player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["recipeId"], recipe_id, "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"][summary_key], expected_stage_count,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
    }
}

#[test]
fn test_fnc_cli_rejects_native_one_off_content_filter_blocker_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        (
            "slide_shift",
            "content/content_slide_shift.json",
            "startCol",
        ),
        (
            "bracket_emphasis",
            "filters/filter_bracket_emphasis.json",
            "emphasisColor",
        ),
        (
            "dot_indicator",
            "filters/filter_dot_indicator.json",
            "activeColor",
        ),
        (
            "edge_grow",
            "filters/filter_edge_grow_left.json",
            "direction",
        ),
        ("hover_bar", "filters/filter_hover_bar.json", "barColor"),
        (
            "matrix_rain",
            "filters/filter_matrix_rain_speed_profile.json",
            "speedMultiplier",
        ),
        (
            "sub_pixel_bar",
            "filters/filter_sub_pixel_bar.json",
            "barColor",
        ),
        (
            "underline_wipe",
            "filters/filter_underline_wipe.json",
            "underlineColor",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root)
                .expect("create temp unsupported one-off content filter fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe)
                    .expect("serialize unsupported one-off content filter recipe"),
            )
            .expect("write unsupported one-off content filter recipe");

            let output = run_player_cli(
                vec![
                    str_arg("render-backend"),
                    str_arg("--recipe"),
                    recipe_path.display().to_string(),
                    str_arg("--descriptor-pack"),
                    descriptor_pack_path(),
                    str_arg("--backend"),
                    str_arg("compositor"),
                    str_arg("--composition-mode"),
                    str_arg("native"),
                    str_arg("--fail-on-fallback"),
                    str_arg("--format"),
                    str_arg("json"),
                ],
                "render-backend native unsupported one-off content filter blocker player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_native_residual_style_content_blockers_json() {
    for (recipe, recipe_id, effect_id, summary_key, expected_stage_count) in [
        (
            "styles/style_modulo_columns_period.json",
            "debugStyleModuloColumnsPeriod",
            "style.moduloColumns",
            "styleStages",
            1,
        ),
        (
            "styles/style_modulo_vertical_every_fourth_column_offset.json",
            "debugStyleModuloVerticalEveryFourthColumnOffset",
            "style.moduloColumns",
            "styleStages",
            1,
        ),
        (
            "styles/style_neon_flicker.json",
            "debugStyleNeonFlicker",
            "style.neonFlicker",
            "styleStages",
            1,
        ),
        (
            "styles/style_neon_flicker_modifier.json",
            "debugStyleNeonFlickerModifier",
            "style.neonFlicker",
            "styleStages",
            1,
        ),
        (
            "content/content_dissolve.json",
            "debugContentDissolve",
            "content.dissolve",
            "contentStages",
            1,
        ),
        (
            "content/content_glitch_shift.json",
            "debugContentGlitchShift",
            "content.glitchShift",
            "contentStages",
            1,
        ),
        (
            "content/content_mirror.json",
            "debugContentMirror",
            "content.mirror",
            "contentStages",
            1,
        ),
        (
            "content/content_numeric.json",
            "debugContentNumeric",
            "content.numeric",
            "contentStages",
            1,
        ),
        (
            "content/content_redact.json",
            "debugContentRedact",
            "content.redact",
            "contentStages",
            1,
        ),
        (
            "content/content_scramble_glitch_shift.json",
            "debugContentScrambleGlitchShift",
            "content.scrambleGlitchShift",
            "contentStages",
            2,
        ),
    ] {
        let report = player_cli_json(
            vec![
                str_arg("render-backend"),
                str_arg("--recipe"),
                recipe_path(recipe),
                str_arg("--descriptor-pack"),
                descriptor_pack_path(),
                str_arg("--backend"),
                str_arg("compositor"),
                str_arg("--composition-mode"),
                str_arg("native"),
                str_arg("--fail-on-fallback"),
                str_arg("--format"),
                str_arg("json"),
                str_arg("--phase"),
                str_arg("enter"),
                str_arg("--phase-t"),
                str_arg("0.35"),
            ],
            "render-backend native residual style content blockers player cli",
        );

        assert_eq!(report["backend"], "compositor", "{recipe}");
        assert_eq!(report["recipeId"], recipe_id, "{recipe}");
        assert_eq!(report["compositionMode"], "native", "{recipe}");
        assert_eq!(report["fallbackUsed"], false, "{recipe}");
        assert_eq!(report["nativeLoweringAttempted"], true, "{recipe}");
        assert_eq!(report["nativeLoweringSucceeded"], true, "{recipe}");
        assert_eq!(report["sourceRenderMode"], "sourceOnly", "{recipe}");
        assert_eq!(report["nativeSourceIsolated"], true, "{recipe}");
        assert_eq!(
            report["compositionSpecSummary"][summary_key], expected_stage_count,
            "{recipe}"
        );
        assert!(
            report["loweredEffectIds"]
                .as_array()
                .unwrap()
                .contains(&serde_json::json!(effect_id)),
            "{recipe}"
        );
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
            "{recipe}"
        );
    }
}
#[test]
fn test_fnc_cli_rejects_native_residual_style_content_blocker_unsupported_shapes_json() {
    for (effect_name, recipe_path_fragment, output_input_id) in [
        (
            "modulo_columns",
            "styles/style_modulo_columns_period.json",
            "foreground",
        ),
        (
            "neon_flicker",
            "styles/style_neon_flicker.json",
            "stability",
        ),
        ("dissolve", "content/content_dissolve.json", "replacement"),
        (
            "glitch_shift",
            "content/content_glitch_shift.json",
            "amount",
        ),
        ("mirror", "content/content_mirror.json", "axis"),
        ("numeric", "content/content_numeric.json", "value"),
        ("redact", "content/content_redact.json", "symbol"),
        (
            "scramble_glitch_shift",
            "content/content_scramble_glitch_shift.json",
            "charset",
        ),
    ] {
        for (mutation_name, recipe) in [
            (
                "unsupported_input",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    Some(("unsupportedNativeField", unsupported_native_input())),
                    None,
                    None,
                ),
            ),
            (
                "unsupported_output",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    Some(serde_json::json!({
                        "debugOutput": {
                            "source": {
                                "kind": "input",
                                "id": output_input_id
                            }
                        }
                    })),
                    None,
                ),
            ),
            (
                "unsupported_scope",
                unsupported_native_effect_shape_recipe(
                    recipe_path_fragment,
                    None,
                    None,
                    Some(serde_json::json!({
                        "kind": "rowRange",
                        "start": 0,
                        "end": 1
                    })),
                ),
            ),
        ] {
            let temp_root = std::env::temp_dir().join(format!(
                "tui-vfx-native-{effect_name}-{mutation_name}-unsupported"
            ));
            let _ = fs::remove_dir_all(&temp_root);
            fs::create_dir_all(&temp_root)
                .expect("create temp unsupported residual style content fixture root");
            let recipe_path = temp_root.join(format!("{effect_name}_{mutation_name}.json"));
            fs::write(
                &recipe_path,
                serde_json::to_string_pretty(&recipe)
                    .expect("serialize unsupported residual style content recipe"),
            )
            .expect("write unsupported residual style content recipe");

            let output = run_player_cli(
                vec![
                    str_arg("render-backend"),
                    str_arg("--recipe"),
                    recipe_path.display().to_string(),
                    str_arg("--descriptor-pack"),
                    descriptor_pack_path(),
                    str_arg("--backend"),
                    str_arg("compositor"),
                    str_arg("--composition-mode"),
                    str_arg("native"),
                    str_arg("--fail-on-fallback"),
                    str_arg("--format"),
                    str_arg("json"),
                ],
                "render-backend native unsupported residual style content blocker player cli",
            );

            assert!(
                !output.status.success(),
                "{effect_name}/{mutation_name} unexpectedly succeeded"
            );
            assert!(
                stderr(&output).contains("unsupportedNativeEffect"),
                "{effect_name}/{mutation_name} stderr: {}",
                stderr(&output)
            );
        }
    }
}

#[test]
fn test_fnc_cli_renders_compositor_backend_ir_resolved_metadata_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("shaders/primitives/shader_linear_gradient_apply_to_both.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("ir-resolved"),
            str_arg("--format"),
            str_arg("json"),
        ],
        "render-backend ir-resolved compositor player cli",
    );

    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["compositionMode"], "irResolved");
    assert_eq!(report["fallbackUsed"], false);
    assert_eq!(report["nativeLoweringAttempted"], false);
    assert_eq!(report["sourceRenderMode"], "postEffectIr");
    assert_eq!(report["nativeSourceIsolated"], false);
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["code"] == "playerIrAlreadyResolved")
    );
}

#[test]
fn test_fnc_cli_render_backend_timeline_native_hash_changes() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend-timeline"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--samples"),
            str_arg("5"),
        ],
        "render-backend-timeline native player cli",
    );

    let samples = report["samples"].as_array().expect("samples");
    assert_eq!(samples.len(), 5);
    assert!(
        samples
            .iter()
            .all(|sample| sample["compositionMode"] == "native"
                && sample["fallbackUsed"] == false
                && sample["sourceRenderMode"] == "sourceOnly"
                && sample["nativeSourceIsolated"] == true)
    );
    let hashes = samples
        .iter()
        .map(|sample| sample["backendHash"].as_u64().expect("backend hash"))
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        hashes.len() > 1,
        "native timeline should change backend hashes"
    );
}

#[test]
fn test_fnc_cli_studio_snapshot_native_mutation_changes_backend_hash() {
    let report = player_cli_json(
        vec![
            str_arg("studio-snapshot"),
            str_arg("--recipe"),
            recipe_path("filters/filter_pill_button_progress_binding.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--set"),
            str_arg("progress=0.25"),
            str_arg("--json"),
        ],
        "studio-snapshot native player cli",
    );

    assert_ne!(report["beforeBackendHash"], report["afterBackendHash"]);
    assert!(report["changedCells"].as_u64().unwrap() > 0);
    assert_eq!(report["before"]["compositionMode"], "native");
    assert_eq!(report["before"]["fallbackUsed"], false);
    assert_eq!(report["after"]["compositionMode"], "native");
    assert_eq!(report["after"]["fallbackUsed"], false);
    assert!(
        report["after"]["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("filter.pillButton"))
    );
}

#[test]
fn test_fnc_cli_studio_snapshot_descriptor_runtime_override_changes_backend_hash() {
    let report = player_cli_json(
        vec![
            str_arg("studio-snapshot"),
            str_arg("--recipe"),
            recipe_path("filters/filter_pill_button_progress_binding.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--set"),
            str_arg("effect:filter.pillButton:effectNode:activeColor=#ff0000"),
            str_arg("--json"),
        ],
        "studio-snapshot descriptor runtime override player cli",
    );

    assert_ne!(report["beforeBackendHash"], report["afterBackendHash"]);
    assert!(report["changedCells"].as_u64().unwrap() > 0);
    assert_eq!(report["after"]["sourceRenderMode"], "sourceOnly");
    assert_eq!(report["after"]["nativeSourceIsolated"], true);
    assert!(
        report["mutations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|mutation| mutation["targetKind"] == "runtimeInputOverride"
                && mutation["runtimeInput"] == "effect:filter.pillButton:effectNode:activeColor")
    );
    assert!(
        report["controls"]
            .as_array()
            .unwrap()
            .iter()
            .any(
                |control| control["id"] == "effect:filter.pillButton:effectNode:activeColor"
                    && control["controlKind"] == "colorPicker"
            )
    );
}

#[test]
fn test_fnc_cli_studio_snapshot_rejects_unknown_descriptor_runtime_override() {
    let output = run_player_cli(
        vec![
            str_arg("studio-snapshot"),
            str_arg("--recipe"),
            recipe_path("filters/filter_pill_button_progress_binding.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--set"),
            str_arg("bogus.control=1"),
            str_arg("--json"),
        ],
        "studio-snapshot bogus runtime override player cli",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("could not map studio control `bogus.control`"));
}

#[test]
fn test_fnc_cli_studio_snapshot_source_runtime_override_counts_row_changes() {
    let report = player_cli_json(
        vec![
            str_arg("studio-snapshot"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--set"),
            str_arg("source:source.card:mainCard:message=SOURCE OVERRIDE"),
            str_arg("--json"),
        ],
        "studio-snapshot source runtime override player cli",
    );

    assert_ne!(report["beforeBackendHash"], report["afterBackendHash"]);
    assert!(report["changedCells"].as_u64().unwrap() > 0);
    assert!(
        report["mutations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|mutation| mutation["runtimeInput"] == "source:source.card:mainCard:message")
    );
    assert!(
        report["after"]["rows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row.as_str().unwrap_or("").contains("SOURCE OVERRIDE"))
    );
}

#[test]
fn test_fnc_cli_studio_snapshot_valid_enum_override_reports_no_visual_change() {
    let report = player_cli_json(
        vec![
            str_arg("studio-snapshot"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--set"),
            str_arg("maskWipeEnter.direction=rightToLeft"),
            str_arg("--json"),
        ],
        "studio-snapshot enum runtime override player cli",
    );

    assert_eq!(report["changedCells"], 0);
    assert!(
        report["studioDiagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|diagnostic| diagnostic["code"] == "studioMutationNoVisualChange")
    );
    assert!(
        report["mutations"]
            .as_array()
            .unwrap()
            .iter()
            .any(|mutation| mutation["runtimeInput"] == "effect:mask.wipe:maskWipeEnter:direction")
    );
}

#[test]
fn test_fnc_cli_renders_compositor_backend_ansi() {
    let output = run_player_cli(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path("filters/filter_tint.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("ansi"),
        ],
        "render-backend ansi player cli",
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\x1b[38;2;") || stdout.contains("\x1b[48;2;"));
}

#[test]
fn test_fnc_cli_render_backend_timeline_preserves_sample_ms_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-backend-timeline"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--samples"),
            str_arg("5"),
            str_arg("--sample-ms"),
            str_arg("250"),
        ],
        "render-backend-timeline sample-ms player cli",
    );

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.renderBackendTimeline.1"
    );
    assert_eq!(report["sampleMs"], 250);
    let samples = report["samples"].as_array().expect("samples");
    assert_eq!(samples.len(), 5);
    assert_eq!(samples[0]["sample"]["phaseT"], 0.0);
    assert_eq!(samples[4]["sample"]["phaseT"], 1.0);
    let hashes = samples
        .iter()
        .map(|sample| sample["backendHash"].as_u64().expect("backend hash"))
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        hashes.len() > 1,
        "timeline should sample more than one output"
    );
}

#[test]
fn test_fnc_cli_studio_snapshot_mutation_changes_backend_hash() {
    let report = player_cli_json(
        vec![
            str_arg("studio-snapshot"),
            str_arg("--recipe"),
            recipe_path("shaders/compositions/shader_border_sweep_position_binding.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--set"),
            str_arg("sweep_progress=0.75"),
            str_arg("--json"),
        ],
        "studio-snapshot player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.studioSnapshot.1");
    assert_eq!(report["backend"], "compositor");
    assert_ne!(report["beforeBackendHash"], report["afterBackendHash"]);
    assert!(report["changedCells"].as_u64().expect("changed cells") > 0);
    assert!(
        report["controls"]
            .as_array()
            .expect("controls")
            .iter()
            .any(|control| control["inputName"] == "position"
                || control["id"].as_str().unwrap_or("").contains("position"))
    );
    assert!(
        report["mutations"]
            .as_array()
            .expect("mutations")
            .iter()
            .any(|mutation| mutation["signalId"] == "sweepPosition")
    );
}

#[test]
fn test_fnc_cli_play_backend_json_finishes_before_ci_timeout() {
    let output = run_player_cli_with_timeout(
        vec![
            str_arg("play-backend"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--frames"),
            str_arg("2"),
            str_arg("--fps"),
            str_arg("5"),
            str_arg("--duration-ms"),
            str_arg("1000"),
            str_arg("--no-clear"),
        ],
        "play-backend json player cli",
        Duration::from_secs(10),
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).expect("playback json");
    assert_eq!(report["schemaVersion"], "v3.1.player.backendPlayback.1");
    assert_eq!(report["backend"], "compositor");
    assert_eq!(report["format"], "json");
    assert_eq!(report["fps"], 5);
    assert_eq!(report["durationMs"], 1000);
    let frames = report["frames"].as_array().expect("frames");
    assert!(frames.len() >= 2);
    assert!(frames.iter().all(|frame| {
        frame["sample"]["sampleMs"].as_u64().is_some()
            && frame["output"]["schemaVersion"] == "v3.1.player.renderBackend.1"
            && frame["output"]["backend"] == "compositor"
            && frame["output"]["backendHash"].as_u64().unwrap_or_default() > 0
    }));
}

#[test]
fn test_fnc_cli_play_backend_ansi_emits_compositor_color_without_clear_when_no_clear() {
    let output = run_player_cli_with_timeout(
        vec![
            str_arg("play-backend"),
            str_arg("--recipe"),
            recipe_path("styles/style_fade_in_from_canvas.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("ansi"),
            str_arg("--fps"),
            str_arg("4"),
            str_arg("--duration-ms"),
            str_arg("500"),
            str_arg("--no-clear"),
        ],
        "play-backend ansi player cli",
        Duration::from_secs(3),
    );

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\x1b[38;2;") || stdout.contains("\x1b[48;2;"));
    assert!(stdout.contains("frame: 0"));
    assert!(stdout.contains("frame: 1"));
    assert!(stdout.contains("styled_cells=105"));
    assert!(!stdout.contains("\x1b[2J"));
    let hashes = stdout
        .lines()
        .filter_map(|line| line.split("backend_hash=").nth(1))
        .filter_map(|suffix| suffix.split_whitespace().next())
        .collect::<std::collections::BTreeSet<_>>();
    assert!(
        hashes.len() > 1,
        "playback ANSI should include changing colored frames"
    );
}

#[test]
fn test_fnc_cli_play_backend_rejects_zero_fps() {
    let output = run_player_cli(
        vec![
            str_arg("play-backend"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--fps"),
            str_arg("0"),
            str_arg("--duration-ms"),
            str_arg("500"),
        ],
        "play-backend zero fps",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("fps must be greater than 0"));
}

#[test]
fn test_fnc_cli_play_backend_rejects_zero_duration() {
    let output = run_player_cli(
        vec![
            str_arg("play-backend"),
            str_arg("--recipe"),
            recipe_path("baseline.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--fps"),
            str_arg("4"),
            str_arg("--duration-ms"),
            str_arg("0"),
        ],
        "play-backend zero duration",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("duration-ms must be greater than 0"));
}

#[test]
fn test_fnc_cli_renders_recursive_smoke_report_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-recipe"),
            str_arg("--json"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
        ],
        "recursive render-recipe player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.run.1");
    assert_eq!(report["summary"]["total"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["errors"], 0);
}

#[test]
fn test_fnc_cli_inventories_single_baseline_recipe_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("baseline.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.inventory.1");
    assert_eq!(report["summary"]["totalRecipes"], 1);
    assert_eq!(report["summary"]["rendered"], 1);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["recipes"][0]["status"], "rendered");
    assert_eq!(
        report["recipes"][0]["effectIds"].as_array().unwrap().len(),
        0
    );
    assert!(
        report["recipes"][0]["sourceIds"]
            .as_array()
            .expect("source ids")
            .iter()
            .any(|source| source == "source.card")
    );
}

#[test]
fn test_fnc_cli_inventories_visible_effect_adapter_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("masks/mask_wipe.json"),
    ]);

    assert_eq!(report["recipes"][0]["status"], "rendered");
    assert!(
        report["recipes"][0]["effectIds"]
            .as_array()
            .expect("effect ids")
            .iter()
            .any(|effect| effect == "mask.wipe")
    );
    let effect = find_effect(&report, "mask.wipe");
    assert_eq!(effect["descriptorCovered"], true);
    assert_eq!(effect["representedByRecipes"], true);
    assert_eq!(effect["adapterStatus"], "visible");
}

#[test]
fn test_fnc_cli_inventories_styled_effect_adapter_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["recipes"][0]["status"], "rendered");
    assert!(
        report["recipes"][0]["descriptorCoveredEffectIds"]
            .as_array()
            .expect("descriptor covered")
            .iter()
            .any(|effect| effect == "shader.linearGradient")
    );
    assert!(
        report["recipes"][0]["missingDescriptorEffectIds"]
            .as_array()
            .expect("missing descriptors")
            .is_empty()
    );
    assert!(
        report["recipes"][0]["unsupportedEffectIds"]
            .as_array()
            .expect("unsupported effects")
            .is_empty()
    );
    let effect = find_effect(&report, "shader.linearGradient");
    assert_eq!(effect["adapterStatus"], "styled");
}

#[test]
fn test_fnc_cli_inventories_recursive_debug_fixture_gate_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.inventory.1");
    assert_eq!(
        report["summary"]["totalRecipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(report["summary"]["descriptorEffectIds"], 120);
    assert_eq!(report["summary"]["representedEffectIds"], 76);
    assert_eq!(report["summary"]["unrepresentedEffectIds"], 44);
    assert_eq!(report["summary"]["unsupportedEffectIds"], 0);
}

#[test]
fn test_fnc_cli_reports_primitive_adapter_gap_json() {
    let report = primitive_adapter_gap_report();

    assert_eq!(report["schemaVersion"], "v3.1.player.primitiveAdapterGap.1");
    assert_eq!(report["summary"]["totalEffects"], 76);
    assert_eq!(report["summary"]["rendered"], 76);
    assert_eq!(report["summary"]["stillUnsupported"], 0);
    assert_eq!(report["summary"]["blockedByStyledCellSubstrate"], 0);
    assert_eq!(report["summary"]["blockedBySemanticDecision"], 0);

    assert_gap_entry(&report, "mask.dissolve", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.blinds", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.radial", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.iris", "rendered", "textGrid");
    assert_gap_entry(&report, "mask.diamond", "rendered", "textGrid");
    assert_gap_entry(&report, "sampler.ripple", "rendered", "textGrid");
    assert_gap_entry(&report, "shader.borderSweep", "rendered", "styledCell");
    assert_gap_entry(&report, "shader.linearGradient", "rendered", "styledCell");
    assert_gap_entry(&report, "style.baseStyleOverride", "rendered", "styledCell");
    assert_gap_entry(&report, "style.colorFade", "rendered", "styledCell");
}

#[test]
fn test_fnc_cli_reports_source_text_descriptor_pilot_json() {
    let report = inventory_report(vec![
        str_arg("inventory-recipes"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    let source = find_source(&report, "source.text");
    assert_eq!(source["descriptorCovered"], true);
    assert_eq!(source["representedByRecipes"], true);
    assert_eq!(source["adapterStatus"], "visible");
}

#[test]
fn test_fnc_cli_reports_migration_gap_summary_json() {
    let report = migration_gap_report();

    assert_eq!(report["schemaVersion"], "v3.1.player.migrationGap.1");
    assert!(
        !report["descriptorPacks"]
            .as_array()
            .expect("descriptor packs")
            .is_empty()
    );
    assert_eq!(report["summary"]["legacyRecipes"], 603);
    assert_eq!(
        report["summary"]["v31Recipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["representedFamilies"], 12);
    assert_eq!(report["summary"]["unrepresentedFamilies"], 8);
    assert_eq!(report["summary"]["partiallyRepresentedFamilies"], 10);
    assert_eq!(report["recommendedQueue"][0]["family"], "complex");
}

#[test]
fn test_fnc_cli_rejects_migration_gap_recipe_paths() {
    let output = run_player_cli(
        vec![
            str_arg("migration-gap"),
            str_arg("accidental-path.json"),
            str_arg("--legacy-root"),
            legacy_debug_recipe_root_path(),
            str_arg("--v31-root"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
        ],
        "migration gap player cli",
    );

    assert!(!output.status.success());
    assert!(stderr(&output).contains("migration-gap does not accept recipe paths"));
}

#[test]
fn test_fnc_cli_reports_migration_gap_family_status_json() {
    let report = migration_gap_report();
    let filters = find_family(&report, "filters");
    let content = find_family(&report, "content");
    let complex = find_family(&report, "complex");

    assert_eq!(filters["legacyCount"], 98);
    assert_eq!(filters["v31Count"], 26);
    assert_eq!(filters["coverage"], "partial");
    assert_eq!(filters["status"], "adapterExpansionReady");
    assert!(
        filters["knownV31EffectIds"]
            .as_array()
            .expect("known effects")
            .iter()
            .any(|effect| effect == "filter.dim")
    );
    assert_eq!(content["coverage"], "partial");
    assert_eq!(content["status"], "notYetClassified");
    assert_eq!(complex["coverage"], "partial");
    assert_eq!(complex["status"], "notYetClassified");
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_masks_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--family"),
        str_arg("masks"),
    ]);

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.migrationMappingBatch.1"
    );
    assert_eq!(report["summary"]["families"], 1);
    assert!(report["summary"]["records"].as_u64().expect("records") > 0);
    assert_eq!(report["summary"]["candidateReady"], 9);
    assert_eq!(report["summary"]["duplicateOrVariant"], 3);
    assert_eq!(report["families"][0], "masks");

    let blinds = find_mapping_record(&report, "masks/mask_blinds.json");
    assert_eq!(blinds["legacyFamily"], "masks");
    assert_eq!(blinds["status"], "canonicalExists");
    assert_eq!(blinds["recommendation"], "skipAsDuplicateVariant");
    assert_eq!(blinds["requiredDescriptorIds"][0], "mask.blinds");
    assert!(
        blinds["missingDescriptorIds"]
            .as_array()
            .expect("missing descriptor ids")
            .is_empty()
    );
    assert_eq!(blinds["requiredInputFields"][0], "count");
    assert_eq!(blinds["requiredInputFields"][1], "orientation");

    let cellular = find_mapping_record(&report, "masks/mask_cellular.json");
    assert_eq!(cellular["status"], "canonicalExists");
    assert_eq!(cellular["recommendation"], "skipAsDuplicateVariant");

    let radial_square = find_mapping_record(&report, "masks/mask_radial_square.json");
    assert_eq!(radial_square["status"], "duplicateOrVariant");
    assert_eq!(radial_square["recommendation"], "skipAsDuplicateVariant");
}

#[test]
fn test_fnc_cli_reports_schema_readiness_recursive_json() {
    let report = schema_readiness_report(vec![str_arg("schema-readiness"), str_arg("--recursive")]);

    assert_eq!(report["schemaVersion"], "v3.1.player.schemaReadiness.1");
    assert_eq!(report["summary"]["totalLegacyRecords"], 603);
    assert_eq!(report["summary"]["schemaBlockedRecords"], 0);
    assert_eq!(report["summary"]["sourceBlockedRecords"], 0);
    assert_eq!(report["summary"]["descriptorBlockedRecords"], 0);
    assert_eq!(report["summary"]["fieldCoverageBlockedRecords"], 0);
    assert_eq!(report["summary"]["unknownRecords"], 0);
    assert_eq!(report["summary"]["canDeclareSchemaReady"], true);
    assert_eq!(report["summary"]["unresolvedSchemaBlockers"], 0);
    assert_eq!(report["summary"]["remainingOwnerDecisionCount"], 0);

    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["statusFromMigrationMapping"] != "schemaDecisionNeeded")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["statusFromMigrationMapping"] != "sourceDecisionNeeded")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "fieldCoverage")
    );
}

#[test]
fn test_fnc_cli_maps_schema_readiness_blockers_json() {
    let report = schema_readiness_report(vec![str_arg("schema-readiness"), str_arg("--recursive")]);

    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["statusFromMigrationMapping"] != "schemaDecisionNeeded")
    );

    let source = find_readiness_blocker(
        &report,
        "sourceDescriptor",
        "complex/complex_cellular_faultline.json",
    );
    assert_eq!(source["statusFromMigrationMapping"], "ownerAuditNeeded");
    assert_eq!(source["isSchemaReadinessBlocking"], false);
    assert!(
        source["notes"]
            .as_array()
            .expect("notes")
            .iter()
            .any(|note| note.as_str().unwrap_or("").contains("content.scramble"))
    );
    assert!(
        source["notes"]
            .as_array()
            .expect("notes")
            .iter()
            .any(|note| note.as_str().unwrap_or("").contains("source.text"))
    );

    assert_eq!(report["summary"]["fieldCoverageBlockedRecords"], 0);
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "fieldCoverage")
    );
}

#[test]
fn test_fnc_cli_reports_schema_readiness_offenders_json() {
    let report = schema_readiness_report(vec![
        str_arg("schema-readiness"),
        str_arg("--recursive"),
        str_arg("--include-offenders"),
    ]);

    let offenders = report["offenders"].as_array().expect("offenders");
    assert_eq!(offenders.len(), 87);
    assert_eq!(report["summary"]["unresolvedSchemaBlockers"], 0);
    assert_eq!(report["summary"]["explicitOwnerDecisionNeeded"], 0);
    assert_eq!(report["summary"]["remainingOwnerDecisionCount"], 0);
    assert_eq!(report["summary"]["canDeclareSchemaReady"], true);
    assert_eq!(
        report["summary"]["dispositionCounts"]["acceptedSchema"],
        343
    );
    assert_eq!(
        report["summary"]["dispositionCounts"]["descriptorBacklog"],
        46
    );
    assert_eq!(
        offender_kind_counts(&report),
        BTreeMap::from([
            ("backendRenderer", 15),
            ("descriptorPack", 37),
            ("guiHumanReview", 2),
            ("oracleOnly", 1),
            ("sceneSemantics", 14),
            ("sourceDescriptor", 9),
            ("valueSourceSemantics", 9),
        ])
    );
    assert!(
        offenders
            .iter()
            .all(|offender| offender["blockerKind"] != "ownerAudit")
    );
    assert!(
        offenders
            .iter()
            .all(|offender| offender["blockerKind"] != "unknown")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "ownerAudit")
    );
    assert!(
        report["blockers"]
            .as_array()
            .expect("blockers")
            .iter()
            .all(|blocker| blocker["blockerKind"] != "unknown")
    );
    assert!(offenders.iter().all(|offender| {
        offender
            .as_object()
            .expect("offender")
            .contains_key("disposition")
    }));
    assert!(offenders.iter().all(|offender| {
        offender
            .as_object()
            .expect("offender")
            .contains_key("schemaBlocking")
    }));

    let source = find_readiness_offender(&report, "complex/complex_cellular_faultline.json");
    assert_eq!(source["blockerKind"], "sourceDescriptor");
    assert_eq!(source["disposition"], "descriptorBacklog");
    assert_eq!(source["recommendedDisposition"], "descriptorBacklog");
    assert_eq!(source["schemaBlocking"], false);
    assert_eq!(source["schemaReadinessBlocking"], false);
    assert_json_array_contains(&source["requiredSourceIds"], "source.text");
    assert_json_array_contains(&source["requiredDescriptorIds"], "content.scramble");

    assert!(
        offenders
            .iter()
            .all(|offender| offender["blockerKind"] != "fieldCoverage")
    );

    let value_source =
        find_readiness_offender(&report, "complex/complex_field_hint_displace_shade.json");
    assert_eq!(value_source["blockerKind"], "valueSourceSemantics");
    assert_eq!(value_source["disposition"], "acceptedSchema");
    assert_eq!(value_source["schemaReadinessBlocking"], false);

    assert!(offenders.iter().all(|offender| {
        offender["legacyPath"] != "fixtures/command_capture_chain.capture.json"
    }));
}

#[test]
fn test_fnc_cli_classifies_complex_and_style_offenders_json() {
    let report = schema_readiness_report(vec![
        str_arg("schema-readiness"),
        str_arg("--recursive"),
        str_arg("--include-offenders"),
    ]);

    let complex = find_readiness_offender(&report, "complex/complex_full_pipeline.json");
    assert_eq!(complex["blockerKind"], "sourceDescriptor");
    assert_ne!(
        complex["recommendedDisposition"],
        "requiresArchitectDecision"
    );
    assert!(
        complex["holdbackReason"]
            .as_str()
            .unwrap_or("")
            .contains("composition")
    );

    let sequence =
        find_readiness_offender(&report, "complex/complex_nested_parallel_sequences.json");
    assert_eq!(sequence["blockerKind"], "sceneSemantics");
    assert_eq!(sequence["disposition"], "acceptedSchema");
    assert_eq!(sequence["schemaReadinessBlocking"], false);

    let visual_conflict = find_readiness_offender(
        &report,
        "complex/v3_scheduler_overlap_conflict_mixed_family.json",
    );
    assert_eq!(visual_conflict["blockerKind"], "guiHumanReview");
    assert_eq!(visual_conflict["disposition"], "guiHumanReviewHoldback");
    assert_eq!(visual_conflict["holdbackSignedOff"], true);

    let backend = find_readiness_offender(
        &report,
        "complex/complex_shadow_mask_sampler_shader_filter_native_mix.json",
    );
    assert_eq!(backend["blockerKind"], "backendRenderer");
    assert_eq!(backend["disposition"], "backendHoldback");
    assert_eq!(backend["holdbackSignedOff"], true);

    for style_path in [
        "styles/style_modulo_horizontal_every_third_row.json",
        "styles/style_modulo_vertical_every_fourth_column_offset.json",
        "styles/style_non_empty_scope.json",
        "styles/style_outer_scope_band.json",
        "styles/style_predicate_interior.json",
    ] {
        assert!(
            report["offenders"]
                .as_array()
                .expect("offenders")
                .iter()
                .all(|entry| entry["legacyPath"] != style_path),
            "style scope fixture should now be canonical rather than an offender: {style_path}"
        );
    }
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_recursive_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--recursive"),
    ]);

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.migrationMappingBatch.1"
    );
    assert!(report["summary"]["families"].as_u64().expect("families") > 1);
    assert!(
        report["recommendationQueue"]
            .as_array()
            .expect("recommendation queue")
            .iter()
            .any(|item| item["legacyFamily"] == "masks")
    );

    let families = report["families"].as_array().expect("families");
    for family in ["complex", "content", "filters", "masks", "samplers"] {
        assert!(
            families.iter().any(|entry| entry == family),
            "missing family {family}"
        );
    }
    assert_eq!(report["summary"]["records"], 603);
    assert_eq!(report["summary"]["candidateReady"], 219);
    assert_eq!(report["summary"]["schemaDecisionNeeded"], 0);
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_filter_records_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--family"),
        str_arg("filters"),
    ]);

    assert_eq!(report["families"][0], "filters");
    let dim = find_mapping_record(&report, "filters/filter_dim.json");
    assert_eq!(dim["legacyFamily"], "filters");
    assert_eq!(dim["requiredDescriptorIds"][0], "filter.dim");
    assert_eq!(dim["status"], "canonicalExists");
    assert_eq!(dim["recommendation"], "skipAsDuplicateVariant");

    let crt = find_mapping_record(&report, "filters/filter_crt.json");
    assert_eq!(crt["status"], "canonicalExists");
    assert_eq!(crt["recommendation"], "skipAsDuplicateVariant");

    let value_source_record =
        find_mapping_record(&report, "filters/filter_dim_sample_surface_radius.json");
    assert_eq!(value_source_record["status"], "candidateReady");
    assert_eq!(
        value_source_record["recommendation"],
        "createCanonicalFixture"
    );
    assert!(
        value_source_record["candidateBlockers"]
            .as_array()
            .expect("candidate blockers")
            .iter()
            .any(|blocker| blocker == "valueSourceOrSignalAccepted")
    );
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_content_source_decisions_json() {
    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--family"),
        str_arg("content"),
    ]);

    let marquee = find_mapping_record(&report, "content/content_marquee.json");
    assert_eq!(marquee["legacyFamily"], "content");
    assert_eq!(marquee["status"], "canonicalExists");
    assert_eq!(marquee["recommendation"], "skipAsDuplicateVariant");
    assert!(
        marquee["requiredSourceIds"]
            .as_array()
            .expect("required sources")
            .iter()
            .any(|source| source == "source.text")
    );

    let deprecated = find_mapping_record(&report, "content/_DEPRECATED_content_marquee.json");
    assert_ne!(deprecated["status"], "candidateReady");
    assert_eq!(deprecated["status"], "ownerAuditNeeded");
}

#[test]
fn test_fnc_cli_reports_migration_mapping_batch_keeps_legacy_root_read_only() {
    let before = legacy_recipe_file_snapshot();

    let report = migration_mapping_batch_report(vec![
        str_arg("migration-mapping-batch"),
        str_arg("--recursive"),
    ]);

    assert_eq!(report["summary"]["records"], 603);
    assert_eq!(before, legacy_recipe_file_snapshot());
}

#[test]
fn test_fnc_cli_has_corpus_mapping_backlog_docs_checked_in() {
    for relative in [
        "docs/new_kernel/K2_10_DEBUG_RECIPE_CORPUS_MAPPING_REPORT.md",
        "docs/new_kernel/K2_10_MIGRATION_BACKLOG_BOARD.md",
        "docs/new_kernel/K2_10_RENDER_BACKEND_BOUNDARY_NOTE.md",
        "docs/new_kernel/PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md",
    ] {
        assert!(
            workspace_root().join(relative).is_file(),
            "missing checked-in doc {relative}"
        );
    }
}

#[test]
fn test_fnc_cli_renders_single_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("baseline.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.visualFrameReport.1");
    assert_eq!(report["summary"]["total"], 1);
    assert_eq!(report["summary"]["rendered"], 1);
    assert_eq!(report["frames"][0]["status"], "rendered");
    assert_eq!(report["frames"][0]["sampleT"], 1.0);
    assert!(report["frames"][0]["loopT"].is_null());
    assert_eq!(report["frames"][0]["absoluteTimeMs"], 0);
    assert_eq!(report["frames"][0]["substrate"], "textGrid");
    assert_eq!(report["frames"][0]["cellSource"], "rows");
    assert_eq!(report["frames"][0]["styleKnown"], false);
    assert!(
        !report["frames"][0]["rows"]
            .as_array()
            .expect("rows")
            .is_empty()
    );
    assert!(
        !report["frames"][0]["cells"]
            .as_array()
            .expect("cells")
            .is_empty()
    );
    let first_cell = &report["frames"][0]["cells"][0];
    assert_eq!(first_cell["foreground"], "defaultForeground");
    assert_eq!(first_cell["background"], "transparent");
    assert!(
        first_cell["modifiers"]
            .as_array()
            .expect("modifiers")
            .is_empty()
    );
    assert!(first_cell["role"].is_null());
}

#[test]
fn test_fnc_cli_renders_recursive_visual_frame_report_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recursive"),
        debug_recipe_root_path(),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.visualFrameReport.1");
    assert_eq!(report["summary"]["total"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["rendered"], RECURSIVE_DEBUG_FIXTURE_COUNT);
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["errors"], 0);
    assert_eq!(
        report["frames"].as_array().expect("frames").len() as i64,
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
}

#[test]
fn test_fnc_cli_renders_styled_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["frames"][0]["status"], "rendered");
    assert!(
        report["frames"][0]["unsupportedEffectIds"]
            .as_array()
            .expect("unsupported effect ids")
            .is_empty()
    );
    assert_eq!(report["frames"][0]["substrate"], "styledCell");
    assert_eq!(report["frames"][0]["cellSource"], "styledCells");
    assert_eq!(report["frames"][0]["styleKnown"], true);
    assert!(
        !report["frames"][0]["rows"]
            .as_array()
            .expect("rows")
            .is_empty()
    );
    assert!(
        report["frames"][0]["cells"]
            .as_array()
            .expect("cells")
            .iter()
            .any(|cell| cell["foreground"] != "defaultForeground"
                || cell["background"] != "transparent"
                || !cell["modifiers"].as_array().expect("modifiers").is_empty()
                || !cell["role"].is_null())
    );
    assert!(
        report["frames"][0]["errors"]
            .as_array()
            .expect("errors")
            .is_empty()
    );
}

#[test]
fn test_fnc_cli_renders_filter_field_handling_with_styled_visual_frame_json() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("filters/filter_tint.json"),
        str_arg("--phase"),
        str_arg("enter"),
    ]);

    let frame = &report["frames"][0];
    assert_eq!(frame["status"], "rendered");
    assert_eq!(frame["substrate"], "styledCell");
    assert_eq!(frame["cellSource"], "styledCells");
    assert_eq!(frame["styleKnown"], true);
    assert!(
        frame["cells"]
            .as_array()
            .expect("cells")
            .iter()
            .any(|cell| cell["role"] == "FilterTint")
    );
}

#[test]
fn test_fnc_cli_reports_primitive_field_coverage_for_fixture_corpus_json() {
    let report = primitive_field_coverage_report();

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.primitiveFieldCoverage.1"
    );
    assert_eq!(
        report["summary"]["totalRecipes"],
        RECURSIVE_DEBUG_FIXTURE_COUNT
    );
    assert_eq!(report["summary"]["usedButUnhandledInputFields"], 0);
    assert_eq!(report["summary"]["missingDescriptorInputFields"], 0);
    assert_eq!(report["summary"]["schemaDecisionNeededFields"], 0);
    assert!(
        report["summary"]["totalPrimitiveInstances"]
            .as_u64()
            .expect("instances")
            > RECURSIVE_DEBUG_FIXTURE_COUNT as u64
    );
}

#[test]
fn test_fnc_cli_reports_fixture_qc_for_fixture_corpus_json() {
    let report = player_cli_json(
        vec![
            str_arg("fixture-qc"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "fixture qc player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.fixtureQcReport.1");
    let total_recipes = report["summary"]["totalRecipes"]
        .as_u64()
        .expect("total recipe count");
    assert!(total_recipes >= RECURSIVE_DEBUG_FIXTURE_COUNT as u64);
    assert!(
        report["summary"]["validated"]
            .as_u64()
            .expect("validated count")
            < total_recipes,
        "fixtures without parity metadata should no longer be counted as validated"
    );
    assert!(
        report["summary"]["validationErrors"]
            .as_u64()
            .expect("validation error count")
            > 0,
        "fixture-qc should fail fixtures that lack parity metadata"
    );
    assert!(
        report["summary"]["rendered"]
            .as_u64()
            .expect("rendered count")
            <= total_recipes,
        "rendered count cannot exceed total recipes"
    );
    assert_eq!(report["summary"]["unsupported"], 0);
    assert_eq!(report["summary"]["playerErrors"], 0);
    assert_eq!(report["summary"]["fieldCoverageUnhandled"], 0);
    assert_eq!(report["summary"]["adapterGapUnresolved"], 0);
    assert_eq!(report["summary"]["timelineSmokePassed"], true);
    assert_eq!(report["summary"]["diffSmokePassed"], true);
    assert_eq!(report["summary"]["overallStatus"], "fail");
    assert!(
        report["errors"]
            .as_array()
            .expect("fixture-qc errors")
            .iter()
            .any(|error| error
                .as_str()
                .is_some_and(|message| message.contains("metadata.expectedVisual"))),
        "fixture-qc should report missing expectedVisual parity metadata"
    );
    assert_eq!(
        report["reports"]["render"]["schemaVersion"],
        "v3.1.player.run.1"
    );
    assert_eq!(
        report["reports"]["visualFrame"]["schemaVersion"],
        "v3.1.player.visualFrameReport.1"
    );
    assert_eq!(
        report["reports"]["fieldCoverage"]["schemaVersion"],
        "v3.1.player.primitiveFieldCoverage.1"
    );
    assert_eq!(
        report["reports"]["adapterGap"]["schemaVersion"],
        "v3.1.player.primitiveAdapterGap.1"
    );
}

#[test]
fn test_fnc_cli_fixture_qc_smoke_fields_fail_for_unrendered_recipe_json() {
    let temp_root = std::env::temp_dir().join("tui-vfx-fixture-qc-negative");
    let _ = fs::remove_dir_all(&temp_root);
    fs::create_dir_all(&temp_root).expect("create temp fixture root");
    let recipe = unsupported_effect_recipe();
    let recipe_path = temp_root.join("unsupported.json");
    fs::write(
        &recipe_path,
        serde_json::to_string_pretty(&recipe).expect("serialize negative recipe"),
    )
    .expect("write negative recipe");

    let report = player_cli_json(
        vec![
            str_arg("fixture-qc"),
            str_arg("--recursive"),
            temp_root.display().to_string(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "negative fixture qc player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.fixtureQcReport.1");
    assert_eq!(report["summary"]["totalRecipes"], 1);
    assert_eq!(report["summary"]["rendered"], 0);
    assert_eq!(report["summary"]["playerErrors"], 1);
    assert_eq!(report["summary"]["timelineSmokePassed"], false);
    assert_eq!(report["summary"]["diffSmokePassed"], false);
    assert_eq!(report["summary"]["overallStatus"], "fail");
}

#[test]
fn test_fnc_cli_reports_honest_primitive_field_coverage_shape_json() {
    let report = primitive_field_coverage_report();

    assert_eq!(
        report["summary"]["usedInputFields"],
        report["summary"]["handledInputFields"]
    );
    assert_eq!(report["summary"]["declaredButUnusedInputFields"], 635);

    let first_recipe = &report["recipes"].as_array().expect("recipes")[0];
    assert!(
        first_recipe["recipePath"]
            .as_str()
            .expect("recipe path")
            .ends_with(".json")
    );
    assert_eq!(first_recipe["status"], "scanned");
    assert!(
        first_recipe["errors"]
            .as_array()
            .expect("errors")
            .is_empty()
    );

    let instance = report["recipes"]
        .as_array()
        .expect("recipes")
        .iter()
        .flat_map(|recipe| recipe["primitiveInstances"].as_array().expect("instances"))
        .find(|instance| instance["classification"] == "usedAndHandled")
        .expect("used and handled primitive instance");
    assert!(instance["kind"] == "source" || instance["kind"] == "effect");
    assert!(
        instance["descriptorId"]
            .as_str()
            .expect("descriptor id")
            .contains('.')
    );
    assert!(
        instance["descriptorInputs"]
            .as_array()
            .expect("descriptor inputs")
            .len()
            >= instance["usedInputs"]
                .as_array()
                .expect("used inputs")
                .len()
    );
    assert_eq!(instance["classification"], "usedAndHandled");
    assert_eq!(instance["recommendation"], "none");
}

#[test]
fn test_fnc_cli_keeps_render_frame_schema_unchanged_after_report_commands() {
    let report = render_frame_report(vec![
        str_arg("render-frame"),
        str_arg("--recipe"),
        recipe_path("baseline.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.visualFrameReport.1");
    assert_eq!(report["frames"][0]["sampleT"], 1.0);
    assert_eq!(report["frames"][0]["absoluteTimeMs"], 0);
    assert!(report["frames"][0]["from"].is_null());
}

#[test]
fn test_fnc_cli_timeline_emits_deterministic_multiple_frames_json() {
    let first = timeline_report();
    let second = timeline_report();

    assert_eq!(first["schemaVersion"], "v3.1.player.frameTimeline.1");
    assert_eq!(first["frames"].as_array().expect("frames").len(), 3);
    assert_eq!(first["frames"][0]["sampleT"], 0.0);
    assert_eq!(first["frames"][2]["sampleT"], 1.0);
    assert_eq!(first["frames"][1]["absoluteTimeMs"], 500);
    assert_eq!(
        first["frames"][0]["renderHash"],
        second["frames"][0]["renderHash"]
    );
}

#[test]
fn test_fnc_cli_frame_diff_reports_changed_cells_when_sample_t_differs_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-frame-diff"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--phase"),
            str_arg("enter"),
            str_arg("--from-sample-t"),
            str_arg("0.0"),
            str_arg("--to-sample-t"),
            str_arg("1.0"),
            str_arg("--json"),
        ],
        "render-frame-diff player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.frameDiff.1");
    let report_object = report.as_object().expect("diff report object");
    assert!(report_object.contains_key("from"));
    assert!(report_object.contains_key("to"));
    assert!(!report_object.contains_key("fromFrame"));
    assert!(!report_object.contains_key("toFrame"));
    assert_eq!(report["hashChanged"], true);
    assert!(report["changedCellCount"].as_u64().expect("changed count") > 0);
    assert!(
        !report["changedCells"]
            .as_array()
            .expect("changed cells")
            .is_empty()
    );
    assert_ne!(report["nonEmptyDelta"], 0);
}

#[test]
fn test_fnc_cli_frame_diff_reports_styled_cell_changes_json() {
    let report = player_cli_json(
        vec![
            str_arg("render-frame-diff"),
            str_arg("--recipe"),
            recipe_path("shaders/compositions/shader_border_sweep.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--from-sample-t"),
            str_arg("0.0"),
            str_arg("--to-sample-t"),
            str_arg("0.5"),
            str_arg("--json"),
        ],
        "styled render-frame-diff player cli",
    );

    assert_eq!(report["schemaVersion"], "v3.1.player.frameDiff.1");
    assert_eq!(report["from"]["substrate"], "styledCell");
    assert_eq!(report["to"]["substrate"], "styledCell");
    assert_eq!(report["hashChanged"], true);
    assert!(report["changedCellCount"].as_u64().expect("changed count") > 0);
    assert!(
        report["changedCells"]
            .as_array()
            .expect("changed cells")
            .iter()
            .any(
                |cell| cell["from"].as_str().expect("from cell").contains("fg=")
                    || cell["to"].as_str().expect("to cell").contains("fg=")
            )
    );
}

#[test]
fn test_fnc_cli_reports_implementation_readiness_disposition_first_json() {
    let report = implementation_readiness_report(vec![
        str_arg("implementation-readiness"),
        str_arg("--recursive"),
    ]);

    assert_eq!(
        report["schemaVersion"],
        "v3.1.player.implementationReadiness.1"
    );
    assert!(
        report["summary"]
            .as_object()
            .expect("summary")
            .contains_key("dispositionCounts")
    );
    assert_eq!(report["summary"]["candidateReady"], 180);
    assert_eq!(report["summary"]["explicitOwnerDecisionNeeded"], 0);
    assert_eq!(report["summary"]["implementationBlocking"], 0);
    assert_eq!(report["summary"]["canonicalExists"], 167);
    assert_eq!(
        report["summary"]["dispositionCounts"]["backendHoldbackSignedOff"],
        91
    );
    assert_eq!(
        report["summary"]["dispositionCounts"]["candidateReady"],
        180
    );
    assert_eq!(
        report["summary"]["dispositionCounts"]["deprecatedLegacySignedOff"],
        124
    );
    assert_eq!(
        report["summary"]["dispositionCounts"]["duplicateVariantSignedOff"],
        38
    );
    assert_eq!(
        report["summary"]["dispositionCounts"]["oracleOnlySignedOff"],
        3
    );
    assert!(
        report["priorityQueues"]
            .as_array()
            .expect("priority queues")
            .is_empty()
    );
    assert!(
        report["records"].as_array().expect("records").is_empty(),
        "path-level records should require --include-blockers"
    );
    assert!(
        report["holdbacks"]
            .as_array()
            .expect("holdbacks")
            .is_empty(),
        "path-level holdbacks should require --include-blockers"
    );
}

#[test]
fn test_fnc_cli_reports_implementation_readiness_include_blockers_json() {
    let report = implementation_readiness_report(vec![
        str_arg("implementation-readiness"),
        str_arg("--recursive"),
        str_arg("--include-blockers"),
    ]);

    assert_eq!(report["summary"]["implementationBlocking"], 0);
    assert_eq!(
        report["records"].as_array().expect("records").len(),
        report["summary"]["records"].as_u64().expect("record count") as usize
    );
    assert!(
        report["priorityQueues"]
            .as_array()
            .expect("priority queues")
            .is_empty()
    );

    let animated_glyph = report["records"]
        .as_array()
        .expect("records")
        .iter()
        .find(|record| record["legacyPath"] == "filters/filter_animated_glyph_ramp.json")
        .expect("animated glyph ramp row");
    assert_eq!(animated_glyph["canonicalExists"], false);
    assert_eq!(animated_glyph["disposition"], "candidateReady");
    assert_eq!(animated_glyph["implementationBlocking"], false);
    assert_eq!(
        animated_glyph["recommendedNextAction"],
        "createCanonicalFixture"
    );
    assert_eq!(animated_glyph["playerAdapterStatus"], "covered");
    assert_eq!(animated_glyph["holdbackSignedOff"], false);
    assert!(
        animated_glyph["missingDescriptors"]
            .as_array()
            .expect("missing descriptors")
            .is_empty()
    );

    let serialized = serde_json::to_string(&report).expect("serialize readiness report");
    assert!(
        !serialized.contains("descriptorBacklogResolved"),
        "implementation readiness should not expose false-resolved descriptor dispositions"
    );
}

#[test]
fn test_fnc_cli_reports_implementation_readiness_uses_content_vocabulary_json() {
    let report = implementation_readiness_report(vec![
        str_arg("implementation-readiness"),
        str_arg("--recursive"),
        str_arg("--include-blockers"),
    ]);

    let serialized = serde_json::to_string(&report).expect("serialize readiness report");
    for stale in [
        "source.typewriterText",
        "source.odometer",
        "source.splitFlapText",
    ] {
        assert!(
            !serialized.contains(stale),
            "implementation readiness should not expose stale source/content label {stale}"
        );
    }

    assert!(
        report["records"]
            .as_array()
            .expect("records")
            .iter()
            .any(|record| record["requiredContentDescriptors"]
                .as_array()
                .expect("content descriptors")
                .iter()
                .any(|descriptor| descriptor == "content.odometer"
                    || descriptor == "content.splitFlap"
                    || descriptor == "content.typewriter"))
    );
}

#[test]
fn test_fnc_cli_reports_control_catalog_descriptor_controls_json() {
    let report = control_catalog_report(vec![str_arg("control-catalog")]);

    assert_eq!(report["schemaVersion"], "v3.1.player.controlCatalog.1");
    let angle = find_control(&report, "effect:shader.linearGradient:angleDeg");
    assert_eq!(angle["sourceKind"], "descriptorInput");
    assert_eq!(angle["descriptorId"], "shader.linearGradient");
    assert_eq!(angle["inputName"], "angleDeg");
    assert_eq!(angle["valueKind"], "number");
    assert_eq!(angle["controlKind"], "slider");
    assert_eq!(angle["range"]["min"], 0.0);
    assert_eq!(angle["range"]["max"], 360.0);
    assert_eq!(angle["unit"], "degrees");
    assert_eq!(angle["bindable"], true);

    let direction = find_control(&report, "effect:mask.wipe:direction");
    assert_eq!(direction["controlKind"], "select");
    assert_json_array_contains(&direction["allowedValues"], "leftToRight");

    let color = find_control(&report, "effect:shader.linearGradient:startColor");
    assert_eq!(color["controlKind"], "colorPicker");

    let gradient = find_control(&report, "effect:shader.linearGradient:gradient");
    assert_eq!(gradient["controlKind"], "gradientEditor");
    assert_eq!(gradient["optional"], true);
}

#[test]
fn test_fnc_cli_reports_recipe_aware_control_catalog_json() {
    let report = control_catalog_report(vec![
        str_arg("control-catalog"),
        str_arg("--recipe"),
        recipe_path("shaders/primitives/shader_linear_gradient.json"),
    ]);

    assert_eq!(report["schemaVersion"], "v3.1.player.controlCatalog.1");
    assert!(report["controls"].as_array().expect("controls").iter().any(
        |control| control["descriptorId"] == "shader.linearGradient"
            && !control["usedBy"].as_array().expect("usedBy").is_empty()
            && !control["nodeId"].is_null()
    ));
}

fn implementation_readiness_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--legacy-root"),
        legacy_debug_recipe_root_path(),
        str_arg("--v31-root"),
        debug_recipe_root_path(),
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "implementation readiness player cli")
}

fn control_catalog_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "control catalog player cli")
}

fn inventory_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "inventory player cli")
}

fn render_frame_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "render-frame player cli")
}

fn primitive_adapter_gap_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("primitive-adapter-gap"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "primitive adapter gap player cli",
    )
}

fn primitive_field_coverage_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("primitive-field-coverage"),
            str_arg("--recursive"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "primitive field coverage player cli",
    )
}

fn timeline_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("render-timeline"),
            str_arg("--recipe"),
            recipe_path("masks/mask_wipe.json"),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--frames"),
            str_arg("3"),
            str_arg("--json"),
        ],
        "render-timeline player cli",
    )
}

fn migration_gap_report() -> serde_json::Value {
    player_cli_json(
        vec![
            str_arg("migration-gap"),
            str_arg("--legacy-root"),
            legacy_debug_recipe_root_path(),
            str_arg("--v31-root"),
            debug_recipe_root_path(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--json"),
        ],
        "migration gap player cli",
    )
}

fn migration_mapping_batch_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--legacy-root"),
        legacy_debug_recipe_root_path(),
        str_arg("--v31-root"),
        debug_recipe_root_path(),
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "migration mapping batch player cli")
}

fn schema_readiness_report(mut args: Vec<String>) -> serde_json::Value {
    args.extend([
        str_arg("--legacy-root"),
        legacy_debug_recipe_root_path(),
        str_arg("--v31-root"),
        debug_recipe_root_path(),
        str_arg("--descriptor-pack"),
        descriptor_pack_path(),
        str_arg("--json"),
    ]);
    player_cli_json(args, "schema readiness player cli")
}

fn player_cli_json(args: Vec<String>, context: &str) -> serde_json::Value {
    let output = run_player_cli(args, context);

    assert!(output.status.success(), "stderr: {}", stderr(&output));
    serde_json::from_slice(&output.stdout).expect("json report")
}

fn assert_native_backend_matches_ir_resolved_at_phase(
    recipe_path: String,
    recipe_id: &str,
    effect_id: &str,
    phase_t: &str,
    context: &str,
) -> serde_json::Value {
    let report = player_cli_json(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path.clone(),
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase-t"),
            str_arg(phase_t),
        ],
        context,
    );

    assert_eq!(report["backend"], "compositor", "{context}");
    assert_eq!(report["recipeId"], recipe_id, "{context}");
    assert_eq!(report["compositionMode"], "native", "{context}");
    assert_eq!(report["fallbackUsed"], false, "{context}");
    assert_eq!(report["nativeLoweringAttempted"], true, "{context}");
    assert_eq!(report["nativeLoweringSucceeded"], true, "{context}");
    assert_eq!(report["sourceRenderMode"], "sourceOnly", "{context}");
    assert_eq!(report["nativeSourceIsolated"], true, "{context}");
    assert!(
        report["loweredEffectIds"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!(effect_id)),
        "{context}"
    );
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .all(|diagnostic| diagnostic["code"] != "unsupportedNativeEffect"),
        "{context}"
    );

    report
}

fn run_player_cli(args: Vec<String>, context: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {context}: {error}"))
}

fn run_native_render_backend_with_fail_on_fallback(recipe_path: String, context: &str) -> Output {
    run_player_cli(
        vec![
            str_arg("render-backend"),
            str_arg("--recipe"),
            recipe_path,
            str_arg("--descriptor-pack"),
            descriptor_pack_path(),
            str_arg("--backend"),
            str_arg("compositor"),
            str_arg("--composition-mode"),
            str_arg("native"),
            str_arg("--fail-on-fallback"),
            str_arg("--format"),
            str_arg("json"),
            str_arg("--phase"),
            str_arg("enter"),
        ],
        context,
    )
}

fn run_player_cli_with_timeout(args: Vec<String>, context: &str, timeout: Duration) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tui-vfx-player-cli"))
        .args(args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("spawn {context}: {error}"));
    let start = Instant::now();
    loop {
        if child
            .try_wait()
            .unwrap_or_else(|error| panic!("poll {context}: {error}"))
            .is_some()
        {
            return child
                .wait_with_output()
                .unwrap_or_else(|error| panic!("collect {context}: {error}"));
        }
        if start.elapsed() > timeout {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{context} timed out after {timeout:?}");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn assert_gap_entry(
    report: &serde_json::Value,
    effect_id: &str,
    expected_outcome: &str,
    expected_adapter_class: &str,
) {
    let entry = find_gap_entry(report, effect_id);

    assert_eq!(entry["outcome"], expected_outcome);
    assert_eq!(entry["adapterClass"], expected_adapter_class);
}

fn find_control<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["controls"]
        .as_array()
        .expect("controls")
        .iter()
        .find(|control| control["id"] == id)
        .expect("control entry")
}

fn find_source<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["sources"]
        .as_array()
        .expect("sources")
        .iter()
        .find(|source| source["id"] == id)
        .expect("source entry")
}

fn find_effect<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .find(|effect| effect["id"] == id)
        .expect("effect entry")
}

fn find_gap_entry<'a>(report: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    report["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .find(|effect| effect["effectId"] == id)
        .expect("adapter gap entry")
}

fn find_family<'a>(report: &'a serde_json::Value, family: &str) -> &'a serde_json::Value {
    report["families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|entry| entry["family"] == family)
        .expect("family entry")
}

fn find_readiness_blocker<'a>(
    report: &'a serde_json::Value,
    blocker_kind: &str,
    legacy_path: &str,
) -> &'a serde_json::Value {
    report["blockers"]
        .as_array()
        .expect("readiness blockers")
        .iter()
        .find(|entry| {
            entry["blockerKind"] == blocker_kind
                && entry["representativeLegacyPaths"]
                    .as_array()
                    .expect("representative paths")
                    .iter()
                    .any(|path| path == legacy_path)
        })
        .expect("schema readiness blocker")
}

fn find_readiness_offender<'a>(
    report: &'a serde_json::Value,
    legacy_path: &str,
) -> &'a serde_json::Value {
    report["offenders"]
        .as_array()
        .expect("readiness offenders")
        .iter()
        .find(|entry| entry["legacyPath"] == legacy_path)
        .expect("schema readiness offender")
}

fn offender_kind_counts(report: &serde_json::Value) -> BTreeMap<&str, usize> {
    let mut counts = BTreeMap::new();
    for offender in report["offenders"].as_array().expect("readiness offenders") {
        let kind = offender["blockerKind"].as_str().expect("blocker kind");
        *counts.entry(kind).or_insert(0) += 1;
    }
    counts
}

fn assert_json_array_contains(values: &serde_json::Value, expected: &str) {
    assert!(
        values
            .as_array()
            .expect("json array")
            .iter()
            .any(|value| value == expected),
        "missing {expected} in {values:?}"
    );
}

fn find_mapping_record<'a>(
    report: &'a serde_json::Value,
    legacy_path: &str,
) -> &'a serde_json::Value {
    report["records"]
        .as_array()
        .expect("mapping records")
        .iter()
        .find(|entry| entry["legacyPath"] == legacy_path)
        .expect("mapping record")
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn recipe_path(relative: &str) -> String {
    debug_recipe_root().join(relative).display().to_string()
}

fn debug_recipe_root_path() -> String {
    debug_recipe_root().display().to_string()
}

fn legacy_debug_recipe_root_path() -> String {
    recipe_repo_root()
        .join("recipes/debug_recipes")
        .display()
        .to_string()
}

fn descriptor_pack_path() -> String {
    workspace_root()
        .join("descriptors/v3.1/packs/primitive.json")
        .display()
        .to_string()
}

fn debug_recipe_root() -> PathBuf {
    recipe_repo_root().join("recipes/v3.1/debug_recipes")
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn str_arg(value: &str) -> String {
    value.to_owned()
}

fn legacy_recipe_file_snapshot() -> Vec<(String, u64, std::time::SystemTime)> {
    let root = recipe_repo_root().join("recipes/debug_recipes");
    let mut snapshot = Vec::new();
    collect_legacy_recipe_file_snapshot(&root, &root, &mut snapshot);
    snapshot.sort_by(|left, right| left.0.cmp(&right.0));
    snapshot
}

fn collect_legacy_recipe_file_snapshot(
    root: &std::path::Path,
    current: &std::path::Path,
    snapshot: &mut Vec<(String, u64, std::time::SystemTime)>,
) {
    for entry in fs::read_dir(current).expect("read legacy recipe dir") {
        let path = entry.expect("read legacy recipe entry").path();
        if path.is_dir() {
            collect_legacy_recipe_file_snapshot(root, &path, snapshot);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            let metadata = fs::metadata(&path).expect("legacy recipe metadata");
            snapshot.push((
                path.strip_prefix(root)
                    .expect("legacy relative path")
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/"),
                metadata.len(),
                metadata.modified().expect("legacy recipe mtime"),
            ));
        }
    }
}

fn unsupported_effect_recipe() -> serde_json::Value {
    let text = fs::read_to_string(debug_recipe_root().join("baseline.json"))
        .expect("read baseline fixture");
    let mut recipe: serde_json::Value =
        serde_json::from_str(&text).expect("baseline fixture parses");
    recipe["graph"]["nodes"]["missingAdapter"] = serde_json::json!({
        "id": "missingAdapter",
        "effect": "effect.notInPack",
        "inputs": {},
        "outputs": {},
        "scope": { "kind": "all" },
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" }
    });
    recipe["graph"]["order"]
        .as_array_mut()
        .expect("order array")
        .push(serde_json::Value::String("missingAdapter".to_string()));
    recipe
}

fn unsupported_content_typewriter_recipe() -> serde_json::Value {
    let text = fs::read_to_string(debug_recipe_root().join("content/content_typewriter.json"))
        .expect("read content typewriter fixture");
    let mut recipe: serde_json::Value =
        serde_json::from_str(&text).expect("content typewriter fixture parses");
    recipe["graph"]["nodes"]["effectNode"]["inputs"]["unsupportedNativeField"] = serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "string",
            "value": "must stay unsupported"
        }
    });
    recipe
}

fn unsupported_content_recipe(
    relative_recipe_path: &str,
    unsupported_input: Option<(&str, serde_json::Value)>,
    outputs: Option<serde_json::Value>,
    scope: Option<serde_json::Value>,
) -> serde_json::Value {
    let text = fs::read_to_string(debug_recipe_root().join(relative_recipe_path))
        .expect("read content fixture");
    let mut recipe: serde_json::Value =
        serde_json::from_str(&text).expect("content fixture parses");
    if let Some((key, value)) = unsupported_input {
        recipe["graph"]["nodes"]["effectNode"]["inputs"][key] = value;
    }
    if let Some(outputs) = outputs {
        recipe["graph"]["nodes"]["effectNode"]["outputs"] = outputs;
    }
    if let Some(scope) = scope {
        recipe["graph"]["nodes"]["effectNode"]["scope"] = scope;
    }
    recipe
}

fn unsupported_native_effect_shape_recipe(
    relative_recipe_path: &str,
    unsupported_input: Option<(&str, serde_json::Value)>,
    outputs: Option<serde_json::Value>,
    scope: Option<serde_json::Value>,
) -> serde_json::Value {
    let text = fs::read_to_string(debug_recipe_root().join(relative_recipe_path))
        .expect("read effect fixture");
    let mut recipe: serde_json::Value = serde_json::from_str(&text).expect("effect fixture parses");
    let graph_nodes = recipe["graph"]["nodes"]
        .as_object_mut()
        .expect("graph nodes object");
    let target_node_ids = graph_nodes.keys().cloned().collect::<Vec<_>>();
    for target_node_id in target_node_ids {
        let target_node = graph_nodes
            .get_mut(target_node_id.as_str())
            .expect("target node exists");
        if let Some((key, value)) = &unsupported_input {
            target_node["inputs"][key] = value.clone();
        }
        if let Some(outputs) = &outputs {
            target_node["outputs"] = outputs.clone();
        }
        if let Some(scope) = &scope {
            target_node["scope"] = scope.clone();
        }
    }
    recipe
}

fn unsupported_native_input() -> serde_json::Value {
    serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "string",
            "value": "must stay unsupported"
        }
    })
}

fn literal_number_input(value: f64) -> serde_json::Value {
    serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "number",
            "value": value
        }
    })
}

fn literal_integer_input(value: i64) -> serde_json::Value {
    serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "integer",
            "value": value
        }
    })
}

fn styled_cell_count(
    report: &serde_json::Value,
    glyph: &str,
    foreground: &str,
    background: &str,
) -> usize {
    report["styledCells"]
        .as_array()
        .expect("styled cells array")
        .iter()
        .filter(|cell| {
            cell["glyph"] == glyph
                && cell["foreground"] == foreground
                && cell["background"] == background
        })
        .count()
}

fn styled_cell_color_count(
    report: &serde_json::Value,
    foreground: &str,
    background: &str,
) -> usize {
    report["styledCells"]
        .as_array()
        .expect("styled cells array")
        .iter()
        .filter(|cell| cell["foreground"] == foreground && cell["background"] == background)
        .count()
}

fn styled_cell_modifier_count(
    report: &serde_json::Value,
    foreground: &str,
    background: &str,
    modifier: &str,
) -> usize {
    report["styledCells"]
        .as_array()
        .expect("styled cells array")
        .iter()
        .filter(|cell| {
            cell["foreground"] == foreground
                && cell["background"] == background
                && cell["modifiers"]
                    .as_array()
                    .expect("modifiers")
                    .iter()
                    .any(|value| value == modifier)
        })
        .count()
}

fn unsupported_native_enum_value(value: &str) -> serde_json::Value {
    serde_json::json!({
        "kind": "literal",
        "value": {
            "kind": "enum",
            "value": value
        }
    })
}

// <FILE>crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs</FILE> - <DESC>Player CLI regression tests</DESC>
// <VERS>END OF VERSION: 0.33.0</VERS>
