// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs</FILE> - <DESC>Render player IR through the compositor backend</DESC>
// <VERS>VERSION: 0.22.0</VERS>
// <WCTX>Native compositor source isolation: render native requests from source-only IR, including backend-owned content/style/filter stages, and keep IR-resolved compatibility separate.</WCTX>
// <CLOG>0.22.0: PATCH — remove backend-owned radial mask source-stage rendering after mask.radial moved to compositor MaskSpec.
// 0.21.0: remove backend-owned cellular mask source-stage rendering after mask.cellular moved to compositor MaskSpec.
// 0.20.0: brighten cell-scoped focused-row gradients at the targeted cell.
// 0.19.0: MINOR — render cell-scoped style.spatial focused row gradients.
// 0.18.0: MINOR — render style.glitch native stage.
// 0.17.0: MINOR — render style.rainbow native stage.
// 0.16.0: MINOR — render shader highlighter/focusField applyTo targets.
// 0.15.0: MINOR — add horizontal center wipe rendering while preserving active filter/sampler patches.</CLOG>

use std::{borrow::Cow, collections::BTreeMap};

use serde_json::json;
use tui_vfx_compositor::pipeline::{CompositionSpec, render_pipeline_with_spec};
use tui_vfx_player::{
    PlayerRenderBackend, PlayerRenderBackendOutput, PlayerRenderBackendRequest, PlayerRenderCell,
    PlayerRenderCompositionMode, PlayerRenderIrReport,
};
use tui_vfx_types::{Grid, RoleTag};

use crate::fnc_lower_recipe_graph_to_composition_spec::lower_backend_request_to_composition_spec;
use crate::{
    fnc_lower_player_ir_to_semantic_scene::{
        lower_player_ir_to_semantic_scene, player_cell_from_compositor_cell,
    },
    fnc_lower_recipe_graph_to_composition_spec::{
        LoweredCompositionSpec, NativeContentStage, NativeStyleStage, TypewriterCursorWake,
        lower_player_ir_to_composition_spec,
    },
};

/// Player backend implementation that renders through `tui-vfx-compositor`.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompositorRenderBackend;

impl PlayerRenderBackend for CompositorRenderBackend {
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput {
        render_compositor_backend(input)
    }
}

/// Render a player render IR report through the compositor backend adapter.
pub fn render_compositor_backend(input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput {
    let mut lowered = lower_player_ir_to_semantic_scene(input);
    let (spec, mut diagnostics) = lower_player_ir_to_composition_spec(input);
    diagnostics.append(&mut lowered.diagnostics);

    let width = lowered.source_grid.width();
    let height = lowered.source_grid.height();
    render_pipeline_with_spec(
        &lowered.source_grid,
        &lowered.source_roles,
        &mut lowered.destination_scene,
        width,
        height,
        0,
        0,
        &spec,
        None,
    );

    let rows = rows_from_scene(lowered.destination_scene.grid());
    let styled_cells = styled_cells_from_scene(&lowered.destination_scene);
    let mut backend_metadata = BTreeMap::new();
    backend_metadata.insert("compositorPipelineCalled".to_string(), json!(true));
    insert_common_spec_metadata(&mut backend_metadata, &spec);
    backend_metadata.insert(
        "sourceStyledCells".to_string(),
        json!(input.styled_cells.len()),
    );

    let mut output = PlayerRenderBackendOutput::from_ir(
        "compositor",
        input,
        rows,
        styled_cells,
        diagnostics,
        backend_metadata,
    );
    output.composition_spec_summary = composition_spec_summary(&spec);
    output
}

/// Render a full player backend request through the compositor adapter.
pub fn render_compositor_backend_request(
    request: &PlayerRenderBackendRequest,
) -> PlayerRenderBackendOutput {
    let lowered_spec = lower_backend_request_to_composition_spec(request);
    let scene_ir = scene_ir_for_request(request, lowered_spec.evidence.fallback_used);
    let staged_scene_ir = scene_ir_with_native_content_stages(scene_ir, &lowered_spec);
    let mut lowered_scene = lower_player_ir_to_semantic_scene(&staged_scene_ir);
    let mut diagnostics = lowered_spec.diagnostics;
    diagnostics.append(&mut lowered_scene.diagnostics);

    let width = lowered_scene.source_grid.width();
    let height = lowered_scene.source_grid.height();
    render_pipeline_with_spec(
        &lowered_scene.source_grid,
        &lowered_scene.source_roles,
        &mut lowered_scene.destination_scene,
        width,
        height,
        0,
        0,
        &lowered_spec.spec,
        None,
    );

    let rows = rows_from_scene(lowered_scene.destination_scene.grid());
    let styled_cells = styled_cells_from_scene(&lowered_scene.destination_scene);
    let mut backend_metadata = BTreeMap::new();
    backend_metadata.insert("compositorPipelineCalled".to_string(), json!(true));
    insert_common_spec_metadata(&mut backend_metadata, &lowered_spec.spec);
    backend_metadata.insert(
        "compositionSpecSamplers".to_string(),
        json!(lowered_spec.spec.effective_samplers().len()),
    );
    backend_metadata.insert(
        "sourceStyledCells".to_string(),
        json!(staged_scene_ir.styled_cells.len()),
    );
    backend_metadata.insert(
        "nativeContentStages".to_string(),
        json!(lowered_spec.content_stages.len()),
    );
    backend_metadata.insert(
        "nativeStyleStages".to_string(),
        json!(lowered_spec.style_stages.len()),
    );
    mirror_evidence_into_metadata(&mut backend_metadata, &lowered_spec.evidence);

    PlayerRenderBackendOutput::from_ir(
        "compositor",
        &staged_scene_ir,
        rows,
        styled_cells,
        diagnostics,
        backend_metadata,
    )
    .with_composition_evidence(lowered_spec.evidence)
}

fn scene_ir_with_native_content_stages(
    scene_ir: &PlayerRenderIrReport,
    lowered_spec: &LoweredCompositionSpec,
) -> PlayerRenderIrReport {
    if (lowered_spec.content_stages.is_empty() && lowered_spec.style_stages.is_empty())
        || lowered_spec.evidence.fallback_used
    {
        return scene_ir.clone();
    }

    let mut staged = scene_ir.clone();
    for stage in &lowered_spec.content_stages {
        match stage {
            NativeContentStage::Typewriter {
                speed,
                speed_variance,
                cursor_character,
                cursor_wake,
                wake_cells,
            } => apply_typewriter_content_stage(
                &mut staged,
                *speed,
                *speed_variance,
                *cursor_character,
                *cursor_wake,
                *wake_cells,
            ),
            NativeContentStage::SplitFlap {
                settle,
                cascade,
                speed,
                cycles,
                charset,
                tile_width,
                tile_height,
                jitter,
            } => apply_split_flap_content_stage(
                &mut staged,
                *settle,
                *cascade,
                *speed,
                *cycles,
                charset,
                *tile_width,
                *tile_height,
                *jitter,
            ),
            NativeContentStage::Odometer {
                direction,
                travel,
                from_message,
                tile_width,
                tile_height,
            } => apply_odometer_content_stage(
                &mut staged,
                direction,
                travel,
                from_message,
                *tile_width,
                *tile_height,
            ),
            NativeContentStage::CellMotion {
                route,
                stagger,
                affect,
            } => apply_cell_motion_content_stage(&mut staged, route, *stagger, affect),
            NativeContentStage::Marquee {
                direction,
                speed,
                width,
            } => apply_marquee_content_stage(&mut staged, direction, *speed, *width),
            NativeContentStage::Morph { target } => apply_morph_content_stage(&mut staged, target),
            NativeContentStage::Scramble { seed, charset } => {
                apply_scramble_content_stage(&mut staged, *seed, charset)
            }
            NativeContentStage::WrapIndicator { every } => {
                apply_wrap_indicator_content_stage(&mut staged, *every)
            }
            NativeContentStage::Redact { symbol, reveal } => {
                apply_redact_content_stage(&mut staged, *symbol, *reveal)
            }
            NativeContentStage::Mirror { axis } => apply_mirror_content_stage(&mut staged, axis),
            NativeContentStage::Numeric {
                value,
                decimals,
                prefix,
                suffix,
            } => apply_numeric_content_stage(&mut staged, *value, *decimals, prefix, suffix),
            NativeContentStage::Dissolve {
                replacement,
                direction,
                seed,
            } => apply_dissolve_content_stage(&mut staged, *replacement, direction, *seed),
            NativeContentStage::GlitchShift { amount, seed } => {
                apply_glitch_shift_content_stage(&mut staged, *amount, *seed)
            }
            NativeContentStage::SlideShift { start_col, end_col } => {
                apply_slide_shift_content_stage(&mut staged, *start_col, *end_col)
            }
            NativeContentStage::CrtSampler {
                curvature,
                scanline_strength,
                jitter,
            } => apply_crt_sampler_content_stage(
                &mut staged,
                *curvature,
                *scanline_strength,
                *jitter,
            ),
            NativeContentStage::CrtJitterSampler {
                amplitude,
                frequency,
                decay_ms,
                seed,
            } => apply_crt_jitter_sampler_content_stage(
                &mut staged,
                *amplitude,
                *frequency,
                *decay_ms,
                *seed,
            ),
            NativeContentStage::FaultLineSampler {
                seed,
                intensity,
                split_bias,
                width,
                height,
            } => apply_fault_line_sampler_content_stage(
                &mut staged,
                *seed,
                *intensity,
                *split_bias,
                *width,
                *height,
            ),
            NativeContentStage::BlindsMask { orientation, count } => {
                apply_blinds_mask_content_stage(&mut staged, orientation, *count)
            }
            NativeContentStage::DiamondMask { soft_edge } => {
                apply_shape_mask_content_stage(&mut staged, *soft_edge, SourceMaskShape::Diamond)
            }
            NativeContentStage::DissolveMask { seed, chunk_size } => {
                apply_dissolve_mask_content_stage(&mut staged, *seed, *chunk_size)
            }
            NativeContentStage::IrisMask { shape, soft_edge } => {
                apply_iris_mask_content_stage(&mut staged, shape, *soft_edge)
            }
            NativeContentStage::WipeMask {
                direction,
                soft_edge,
            } => apply_wipe_mask_content_stage(&mut staged, direction, *soft_edge),
        }
    }
    for stage in &lowered_spec.style_stages {
        match stage {
            NativeStyleStage::ModuloColumns {
                modulus,
                remainder,
                foreground,
                background,
            } => apply_modulo_columns_style_stage(
                &mut staged,
                *modulus,
                *remainder,
                foreground,
                background,
            ),
            NativeStyleStage::NeonFlicker {
                color,
                stability,
                dim_amount,
                italic_window,
            } => apply_neon_flicker_style_stage(
                &mut staged,
                color,
                *stability,
                *dim_amount,
                *italic_window,
            ),
            NativeStyleStage::Rainbow { rotation_speed } => {
                apply_rainbow_style_stage(&mut staged, *rotation_speed)
            }
            NativeStyleStage::Glitch {
                seed,
                intensity,
                italic_start,
                italic_end,
            } => {
                apply_glitch_style_stage(&mut staged, *seed, *intensity, *italic_start, *italic_end)
            }
            NativeStyleStage::SpatialFocusedRowGradient {
                x,
                y,
                bright_color,
                apply_to,
                ..
            } => apply_spatial_focused_row_gradient_style_stage(
                &mut staged,
                SpatialFocusedRowGradientInputs {
                    x: *x,
                    y: *y,
                    bright_color,
                    apply_to,
                },
            ),
            NativeStyleStage::ColorFade {
                target,
                color_space,
            } => apply_color_fade_style_stage(&mut staged, target, color_space),
            NativeStyleStage::ColorShift {
                hue_shift,
                saturation_shift,
                lightness_shift,
            } => apply_color_shift_style_stage(
                &mut staged,
                *hue_shift,
                *saturation_shift,
                *lightness_shift,
            ),
            NativeStyleStage::Pulse {
                color,
                frequency,
                apply_to,
            } => apply_pulse_style_stage(&mut staged, color, *frequency, apply_to),
            NativeStyleStage::ItalicWindow { start, end } => {
                apply_italic_window_style_stage(&mut staged, *start, *end)
            }
            NativeStyleStage::Vignette {
                strength,
                edge_color,
                apply_to,
            } => apply_vignette_style_stage(&mut staged, *strength, edge_color, apply_to),
            NativeStyleStage::BracketEmphasis {
                emphasis_color,
                background_color,
                progress,
                edge_width,
                apply_to,
            } => apply_bracket_emphasis_style_stage(
                &mut staged,
                emphasis_color,
                background_color,
                *progress,
                *edge_width,
                apply_to,
            ),
            NativeStyleStage::EdgeGrow {
                direction,
                progress,
                edge_color,
                background_color,
                margin_width,
                rest_eighths,
                peak_eighths,
                apply_to,
            } => apply_edge_grow_style_stage(
                &mut staged,
                EdgeGrowStyleInputs {
                    direction,
                    progress: *progress,
                    edge_color,
                    background_color,
                    margin_width: *margin_width,
                    rest_eighths: *rest_eighths,
                    peak_eighths: *peak_eighths,
                    apply_to,
                },
            ),
            NativeStyleStage::HoverBar {
                bar_color,
                thickness,
                position,
                apply_to,
            } => {
                apply_hover_bar_style_stage(&mut staged, bar_color, *thickness, *position, apply_to)
            }
            NativeStyleStage::MatrixRain {
                speed_multiplier,
                speed_min,
                speed_max,
                glyph_change_hz,
                density,
                seed,
                trail_min,
                trail_max,
                affect,
                chars,
                mode,
                preset,
                head_color,
                tail_color,
            } => apply_matrix_rain_style_stage(
                &mut staged,
                MatrixRainStyleInputs {
                    speed_multiplier: *speed_multiplier,
                    speed_min: *speed_min,
                    speed_max: *speed_max,
                    glyph_change_hz: *glyph_change_hz,
                    density: *density,
                    seed: *seed,
                    trail_min: *trail_min,
                    trail_max: *trail_max,
                    affect,
                    chars,
                    mode,
                    preset,
                    head_color,
                    tail_color,
                },
            ),
            NativeStyleStage::SubPixelBar {
                filled_color,
                unfilled_color,
                progress,
                direction,
            } => apply_sub_pixel_bar_style_stage(
                &mut staged,
                filled_color,
                unfilled_color,
                *progress,
                direction,
            ),
            NativeStyleStage::UnderlineWipe {
                underline_color,
                background_color,
                direction,
                line_char,
                row_offset,
                progress,
                gradient,
                glisten,
                apply_to,
            } => apply_underline_wipe_style_stage(
                &mut staged,
                UnderlineWipeStyleInputs {
                    underline_color,
                    background_color,
                    direction,
                    line_char: *line_char,
                    row_offset: *row_offset,
                    progress: *progress,
                    gradient: *gradient,
                    glisten: *glisten,
                    apply_to,
                },
            ),
            NativeStyleStage::Highlighter {
                color,
                apply_to,
                blend_strength,
                text_contrast,
                soft_edge,
                direction,
                mode: _,
                row_mask,
                band_width,
            } => apply_highlighter_style_stage(
                &mut staged,
                HighlighterStyleInputs {
                    color,
                    apply_to,
                    blend_strength: *blend_strength,
                    text_contrast: *text_contrast,
                    soft_edge: *soft_edge,
                    direction,
                    row_mask: *row_mask,
                    band_width: *band_width,
                },
            ),
            NativeStyleStage::FocusField {
                color,
                rect_x,
                rect_y,
                rect_width,
                rect_height,
                center_x,
                center_y,
                shape,
                radius_x,
                radius_y,
                feather,
                intensity,
                apply_to,
            } => apply_focus_field_style_stage(
                &mut staged,
                FocusFieldStyleInputs {
                    color,
                    rect_x: *rect_x,
                    rect_y: *rect_y,
                    rect_width: *rect_width,
                    rect_height: *rect_height,
                    center_x: *center_x,
                    center_y: *center_y,
                    shape,
                    radius_x: *radius_x,
                    radius_y: *radius_y,
                    feather: *feather,
                    intensity: *intensity,
                    apply_to,
                },
            ),
            NativeStyleStage::GlistenBand {
                color,
                blend_strength,
                angle_deg,
                speed,
                head,
                tail,
                band_width,
                direction,
            } => apply_glisten_band_style_stage(
                &mut staged,
                GlistenBandStyleInputs {
                    color,
                    blend_strength: *blend_strength,
                    angle_deg: *angle_deg,
                    speed: *speed,
                    head: *head,
                    tail: *tail,
                    band_width: *band_width,
                    direction,
                },
            ),
            NativeStyleStage::WayfindingNode {
                current_index,
                nodes,
                previous_strength,
                future_strength,
                intensity,
                radius,
                active_color,
            } => apply_wayfinding_node_style_stage(
                &mut staged,
                WayfindingNodeStyleInputs {
                    current_index: *current_index,
                    nodes: *nodes,
                    previous_strength: *previous_strength,
                    future_strength: *future_strength,
                    intensity: *intensity,
                    radius: *radius,
                    active_color,
                },
            ),
            NativeStyleStage::BarberPole {
                stripe_color,
                background_color,
                stripe_width,
                gap_width,
                angle_deg,
                speed,
                apply_to,
            } => apply_barber_pole_style_stage(
                &mut staged,
                BarberPoleStyleInputs {
                    stripe_color,
                    background_color,
                    stripe_width: *stripe_width,
                    gap_width: *gap_width,
                    angle_deg: *angle_deg,
                    speed: *speed,
                    apply_to,
                },
            ),
            NativeStyleStage::Diffusion {
                color,
                center_x,
                center_y,
                radius,
                intensity,
                apply_to,
            } => apply_diffusion_style_stage(
                &mut staged,
                DiffusionStyleInputs {
                    color,
                    center_x: *center_x,
                    center_y: *center_y,
                    radius: *radius,
                    intensity: *intensity,
                    apply_to,
                },
            ),
            NativeStyleStage::Radar {
                color,
                speed,
                tail_length,
                apply_to,
            } => apply_radar_style_stage(
                &mut staged,
                RadarStyleInputs {
                    color,
                    speed: *speed,
                    tail_length: *tail_length,
                    apply_to,
                },
            ),
        }
    }
    staged
}

fn apply_typewriter_content_stage(
    report: &mut PlayerRenderIrReport,
    speed: f64,
    speed_variance: f64,
    cursor_character: char,
    cursor_wake: TypewriterCursorWake,
    wake_cells: usize,
) {
    let visible_fraction = (report.phase_t
        * (speed.max(0.0) + speed_variance.clamp(0.0, 1.0) * report.phase_t))
        .clamp(0.0, 1.0);
    let width = report_width(report);
    let height = report_height(report);
    let mut rows = dense_rows(report, width, height);
    let total = rows.iter().map(|row| row.chars().count()).sum::<usize>();
    let visible = (total as f64 * visible_fraction).round() as usize;
    let mut seen = 0usize;

    for row in &mut rows {
        let mut wrote_cursor = false;
        let mut cursor_index = None;
        let mut chars = Vec::new();
        for (index, glyph) in row.chars().enumerate() {
            seen += 1;
            if seen <= visible || glyph == ' ' {
                chars.push(glyph);
            } else if !wrote_cursor {
                wrote_cursor = true;
                cursor_index = Some(index);
                chars.push(cursor_character);
            } else {
                chars.push(' ');
            }
        }
        if matches!(
            cursor_wake,
            TypewriterCursorWake::Ghost | TypewriterCursorWake::Tint
        ) && let Some(cursor_index) = cursor_index
        {
            let start = cursor_index.saturating_sub(wake_cells);
            for glyph in &mut chars[start..cursor_index] {
                if *glyph != ' ' {
                    *glyph = if cursor_wake == TypewriterCursorWake::Ghost {
                        '░'
                    } else {
                        '·'
                    };
                }
            }
        }
        *row = chars.into_iter().collect();
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_split_flap_content_stage(
    report: &mut PlayerRenderIrReport,
    settle: f64,
    cascade: f64,
    speed: f64,
    cycles: f64,
    charset: &str,
    tile_width: usize,
    tile_height: usize,
    jitter: f64,
) {
    let width = report_width(report);
    let height = report_height(report);
    let mut rows = dense_rows(report, width, height);
    let cascade = cascade.clamp(0.0, 1.0);
    let threshold =
        (report.phase_t * settle.clamp(0.0, 1.0) * speed.max(0.0) + cascade * 0.1).clamp(0.0, 1.0);
    let glyphs = split_flap_charset(charset);
    let tile_width = tile_width.max(1);
    let tile_height = tile_height.max(1);
    let cycles = cycles.max(0.0);
    let jitter = jitter.clamp(0.0, 1.0);

    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                let tile_offset = ((x / tile_width) + (y / tile_height)) as f64 * cascade * 0.08;
                let cell_progress = (threshold - tile_offset
                    + jitter * cell_threshold(x, y) * 0.05)
                    .clamp(0.0, 1.0);
                if glyph == ' ' || cell_progress >= 1.0 {
                    glyph
                } else if cycles > 0.0 {
                    let index =
                        ((cell_progress * cycles * glyphs.len() as f64).floor() as usize + x + y)
                            % glyphs.len();
                    glyphs[index]
                } else {
                    '▣'
                }
            })
            .collect();
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_odometer_content_stage(
    report: &mut PlayerRenderIrReport,
    direction: &str,
    travel: &str,
    from_message: &str,
    tile_width: usize,
    tile_height: usize,
) {
    let width = report_width(report);
    let height = report_height(report);
    let mut rows = dense_rows(report, width, height);
    let progress = report.phase_t.clamp(0.0, 1.0);
    if from_message.is_empty() {
        apply_glitch_shift_rows(&mut rows, 1, 3);
        report.rows = rows;
        sync_styled_cells_to_rows(report);
        return;
    }

    let from_rows = normalized_rows(from_message, rows.len());
    let row_count = rows.len();
    let tile_width = tile_width.max(1);
    let tile_height = tile_height.max(1);
    let reveal_threshold = odometer_reveal_threshold(travel);
    for (y, row) in rows.iter_mut().enumerate() {
        let target = row.chars().collect::<Vec<_>>();
        let source = from_rows
            .get(y)
            .map(|value| value.chars().collect::<Vec<_>>())
            .unwrap_or_default();
        let travel_span = odometer_travel_span(travel, target.len().max(source.len()), row_count);
        *row = target
            .iter()
            .enumerate()
            .map(|(x, target_glyph)| {
                let source_glyph = source.get(x).copied().unwrap_or(' ');
                let tile_delay = ((x / tile_width) + (y / tile_height)) as f64 * 0.04 * travel_span;
                let cell_progress = (progress - tile_delay).clamp(0.0, 1.0);
                if cell_progress >= reveal_threshold {
                    *target_glyph
                } else if direction == "down" || direction == "left" {
                    previous_digit(source_glyph)
                } else {
                    source_glyph
                }
            })
            .collect();
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_cell_motion_content_stage(
    report: &mut PlayerRenderIrReport,
    route: &str,
    stagger: usize,
    affect: &str,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let progress = report.phase_t.clamp(0.0, 1.0);
    let reveals_from_left = matches!(route, "fromLeft" | "left");
    let preserve_empty_cells = affect == "nonEmpty";

    for (y, row) in rows.iter_mut().enumerate() {
        let row_width = row.chars().count().max(1);
        let line_threshold = ((y * stagger) % 10) as f64 / 20.0;
        if progress < line_threshold {
            row.clear();
            continue;
        }
        if reveals_from_left {
            let visible = (row_width as f64 * progress).round() as usize;
            *row = row
                .chars()
                .enumerate()
                .map(|(x, glyph)| {
                    if x < visible && (!preserve_empty_cells || glyph != ' ') {
                        glyph
                    } else {
                        ' '
                    }
                })
                .collect();
        } else if progress <= 0.0 {
            row.clear();
        }
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_marquee_content_stage(
    report: &mut PlayerRenderIrReport,
    direction: &str,
    speed: f64,
    authored_width: usize,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let rotate_right = direction == "right";

    for row in &mut rows {
        let marquee_width = authored_width.max(row.chars().count());
        if marquee_width == 0 {
            continue;
        }
        let offset = ((report.phase_t * speed.max(0.0) * marquee_width as f64).round() as usize)
            % marquee_width;
        let rotation_offset = if rotate_right {
            marquee_width - offset
        } else {
            offset
        };
        *row = rotate_row(row, rotation_offset);
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_morph_content_stage(report: &mut PlayerRenderIrReport, target: &str) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let progress = report.phase_t.clamp(0.0, 1.0);
    let replacement_glyph = if target == "dots" { '·' } else { '█' };

    for row in &mut rows {
        *row = row
            .chars()
            .enumerate()
            .map(|(index, glyph)| {
                if glyph == ' ' || (index as f64 / row.len().max(1) as f64) > progress {
                    glyph
                } else {
                    replacement_glyph
                }
            })
            .collect();
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_scramble_content_stage(report: &mut PlayerRenderIrReport, seed: usize, charset: &str) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let scramble_charset = charset.chars().collect::<Vec<_>>();
    let resolved_progress = report.phase_t.clamp(0.0, 1.0);

    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || cell_threshold(x + seed, y) <= resolved_progress {
                    glyph
                } else {
                    scramble_glyph(x + y + seed, &scramble_charset)
                }
            })
            .collect();
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_wrap_indicator_content_stage(report: &mut PlayerRenderIrReport, every: usize) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let row_interval = every.max(1);

    for (index, row) in rows.iter_mut().enumerate() {
        if index % row_interval != 0 || row.is_empty() {
            continue;
        }

        let mut chars = row.chars().collect::<Vec<_>>();
        if let Some(last) = chars.last_mut() {
            *last = '↵';
        }
        *row = chars.into_iter().collect();
    }

    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_redact_content_stage(report: &mut PlayerRenderIrReport, symbol: char, reveal: f64) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let reveal = reveal.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || cell_threshold(x, y) < reveal {
                    glyph
                } else {
                    symbol
                }
            })
            .collect();
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_mirror_content_stage(report: &mut PlayerRenderIrReport, axis: &str) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    if axis == "vertical" {
        rows.reverse();
    } else {
        for row in &mut rows {
            *row = row.chars().rev().collect();
        }
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_numeric_content_stage(
    report: &mut PlayerRenderIrReport,
    value: f64,
    decimals: usize,
    prefix: &str,
    suffix: &str,
) {
    let report_rows = report_height(report).max(1);
    let mut rows = vec![String::new(); report_rows];
    rows[0] = format!("{prefix}{value:.decimals$}{suffix}");
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_dissolve_content_stage(
    report: &mut PlayerRenderIrReport,
    replacement: char,
    direction: &str,
    seed: usize,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let progress = report.phase_t.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || dissolve_threshold(x, y, width, seed, direction) > progress {
                    replacement
                } else {
                    glyph
                }
            })
            .collect();
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_glitch_shift_content_stage(report: &mut PlayerRenderIrReport, amount: usize, seed: usize) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    apply_glitch_shift_rows(&mut rows, amount, seed);
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_slide_shift_content_stage(
    report: &mut PlayerRenderIrReport,
    start_col: i64,
    end_col: i64,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let progress = report.phase_t.clamp(0.0, 1.0);
    let offset = (start_col as f64 + (end_col - start_col) as f64 * progress).round() as isize;
    for row in &mut rows {
        *row = shift_row(row, offset);
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_crt_sampler_content_stage(
    report: &mut PlayerRenderIrReport,
    curvature: f64,
    scanline_strength: f64,
    jitter: f64,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let center = rows.len() as f64 / 2.0;
    let curvature = curvature.clamp(0.0, 1.0);
    let scanline_strength = scanline_strength.clamp(0.0, 1.0);
    let jitter = jitter.max(0.0);
    let time = report.loop_t.unwrap_or(report.phase_t);
    for (y, row) in rows.iter_mut().enumerate() {
        let bow = ((y as f64 - center) * curvature * report.phase_t).round() as isize;
        let time_jitter = ((time * 10.0 + y as f64).sin() * jitter).round() as isize;
        let shifted = shift_row(row, bow + time_jitter);
        if scanline_strength > 0.0 && y % 2 == 1 {
            *row = drop_every_nth_glyph(&shifted, scanline_strength);
        } else {
            *row = shifted;
        }
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_crt_jitter_sampler_content_stage(
    report: &mut PlayerRenderIrReport,
    amplitude: f64,
    frequency: f64,
    decay_ms: f64,
    seed: usize,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let time = report.loop_t.unwrap_or(report.phase_t);
    let amplitude = decayed_crt_jitter_amplitude(amplitude.max(0.0), decay_ms, time);
    let frequency = frequency.max(0.0);
    let seed = seed as f64;
    for (y, row) in rows.iter_mut().enumerate() {
        let wave = ((time * frequency + y as f64 * 0.37 + seed * 0.01).sin() * amplitude).round();
        *row = shift_row(row, wave as isize);
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn decayed_crt_jitter_amplitude(amplitude: f64, decay_ms: f64, time: f64) -> f64 {
    if decay_ms <= 0.0 {
        amplitude
    } else {
        let decay = decay_ms / 1000.0;
        amplitude * (-decay * time * 5.0).exp()
    }
}

fn apply_fault_line_sampler_content_stage(
    report: &mut PlayerRenderIrReport,
    seed: u64,
    intensity: f64,
    split_bias: f64,
    width: usize,
    height: usize,
) {
    let report_columns = width;
    let report_rows = height;
    let split = fault_line_split(report_rows, seed, split_bias);
    let offset =
        ((1.0 - report.phase_t.clamp(0.0, 1.0)) * 20.0 * intensity.max(0.0)).round() as isize;
    report.rows = dense_rows(report, report_columns, report_rows)
        .into_iter()
        .enumerate()
        .map(|(row_index, row)| {
            if row_index < split {
                shift_row(&row, offset)
            } else {
                shift_row(&row, -offset)
            }
        })
        .map(|row| row.trim_end().to_string())
        .collect();
    sync_styled_cells_to_rows(report);
}

fn fault_line_split(row_count: usize, seed: u64, split_bias: f64) -> usize {
    if row_count < 3 {
        return row_count / 2;
    }
    let base_split = (seed.wrapping_mul(31) % row_count as u64) as f64;
    (base_split + split_bias.clamp(-1.0, 1.0) * row_count as f64 * 0.3)
        .clamp(1.0, (row_count - 1) as f64) as usize
}

fn apply_blinds_mask_content_stage(
    report: &mut PlayerRenderIrReport,
    orientation: &str,
    count: usize,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let count = count.max(1);
    let reveal = report.phase_t.clamp(0.0, 1.0);
    let height = rows.len().max(1);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if blinds_keeps_cell(x, y, width, height, count, orientation, reveal) {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_dissolve_mask_content_stage(
    report: &mut PlayerRenderIrReport,
    seed: u64,
    chunk_size: usize,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let chunk_size = chunk_size.max(1);
    let reveal_threshold = report.phase_t.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' '
                    || dissolve_cell_noise(seed, x / chunk_size, y / chunk_size) <= reveal_threshold
                {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_iris_mask_content_stage(report: &mut PlayerRenderIrReport, shape: &str, soft_edge: bool) {
    let mask_shape = if shape == "diamond" {
        SourceMaskShape::Diamond
    } else {
        SourceMaskShape::Circle
    };
    apply_shape_mask_content_stage(report, soft_edge, mask_shape);
}

fn apply_shape_mask_content_stage(
    report: &mut PlayerRenderIrReport,
    soft_edge: bool,
    shape: SourceMaskShape,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let height = rows.len().max(1);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        let reveal_radius = source_mask_spotlight_radius(report.phase_t, width, height, soft_edge);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if source_mask_spotlight_distance(x, y, width, height, shape) < reveal_radius {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

fn apply_wipe_mask_content_stage(
    report: &mut PlayerRenderIrReport,
    direction: &str,
    soft_edge: bool,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let Some(bounds) = non_blank_bounds(&rows) else {
        report.rows = rows;
        sync_styled_cells_to_rows(report);
        return;
    };
    let reveal = report.phase_t.clamp(0.0, 1.0);
    let width = bounds.width().max(1);
    let height = bounds.height().max(1);
    let horizontal_cutoff = wipe_cutoff(width, reveal, soft_edge);
    let vertical_cutoff = wipe_cutoff(height, reveal, soft_edge);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(index, glyph)| {
                if !bounds.contains(index, y) {
                    return ' ';
                }
                let local_x = index - bounds.min_x;
                let local_y = y - bounds.min_y;
                if wipe_keeps_cell(
                    local_x,
                    local_y,
                    width,
                    height,
                    horizontal_cutoff,
                    vertical_cutoff,
                    direction,
                ) {
                    glyph
                } else {
                    ' '
                }
            })
            .collect();
    }
    report.rows = rows;
    sync_styled_cells_to_rows(report);
}

#[derive(Clone, Copy, Debug)]
struct ContentBounds {
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
}

impl ContentBounds {
    fn width(self) -> usize {
        self.max_x.saturating_sub(self.min_x) + 1
    }

    fn height(self) -> usize {
        self.max_y.saturating_sub(self.min_y) + 1
    }

    fn contains(self, x: usize, y: usize) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

fn non_blank_bounds(rows: &[String]) -> Option<ContentBounds> {
    let mut bounds: Option<ContentBounds> = None;
    for (y, row) in rows.iter().enumerate() {
        for (x, glyph) in row.chars().enumerate() {
            if glyph == ' ' {
                continue;
            }
            bounds = Some(match bounds {
                Some(existing) => ContentBounds {
                    min_x: existing.min_x.min(x),
                    max_x: existing.max_x.max(x),
                    min_y: existing.min_y.min(y),
                    max_y: existing.max_y.max(y),
                },
                None => ContentBounds {
                    min_x: x,
                    max_x: x,
                    min_y: y,
                    max_y: y,
                },
            });
        }
    }
    bounds
}

fn apply_modulo_columns_style_stage(
    report: &mut PlayerRenderIrReport,
    modulus: usize,
    remainder: usize,
    foreground: &str,
    background: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let modulus = modulus.max(1);
    for y in 0..height {
        for x in 0..width {
            if x % modulus == remainder {
                set_report_cell_style(report, x, y, Some(foreground), Some(background), None);
            }
        }
    }
}

fn apply_neon_flicker_style_stage(
    report: &mut PlayerRenderIrReport,
    color: &str,
    stability: f64,
    dim_amount: f64,
    italic_window: bool,
) {
    let width = report_width(report);
    let height = report_height(report);
    let clock = report.loop_t.unwrap_or(report.phase_t);
    let flicker = (clock * 37.0).sin() * 0.5 + 0.5;
    let active_strength = if flicker <= stability {
        1.0
    } else {
        1.0 - dim_amount
    };
    let foreground = dimmed_rgba_label(color, active_strength);
    for y in 0..height {
        for x in 0..width {
            set_report_cell_style(
                report,
                x,
                y,
                Some(foreground.as_str()),
                None,
                italic_window.then_some("italic"),
            );
        }
    }
}

fn apply_rainbow_style_stage(report: &mut PlayerRenderIrReport, _rotation_speed: f64) {
    let width = report_width(report).max(1);
    let height = report_height(report);
    for y in 0..height {
        for x in 0..width {
            set_report_cell_style(
                report,
                x,
                y,
                Some(rgba_label(0, 255, 254, 255).as_str()),
                None,
                None,
            );
        }
    }
}

fn apply_glitch_style_stage(
    report: &mut PlayerRenderIrReport,
    _seed: usize,
    _intensity: f64,
    italic_start: f64,
    italic_end: f64,
) {
    let width = report_width(report);
    let height = report_height(report);
    let italic = (italic_start..=italic_end).contains(&report.phase_t);
    for y in 0..height {
        for x in 0..width {
            set_report_cell_style(
                report,
                x,
                y,
                Some(rgba_label(0, 255, 255, 255).as_str()),
                None,
                italic.then_some("italic"),
            );
        }
    }
}

struct SpatialFocusedRowGradientInputs<'a> {
    x: usize,
    y: usize,
    bright_color: &'a str,
    apply_to: &'a str,
}

fn apply_spatial_focused_row_gradient_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: SpatialFocusedRowGradientInputs<'_>,
) {
    if inputs.x >= report_width(report) || inputs.y >= report_height(report) {
        return;
    }
    set_report_shader_cell(
        report,
        inputs.x,
        inputs.y,
        inputs.apply_to,
        inputs.bright_color,
        "ShaderFocusedRowGradient",
    );
}

fn apply_color_fade_style_stage(
    report: &mut PlayerRenderIrReport,
    target: &str,
    color_space: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let progress = report.phase_t.clamp(0.0, 1.0) as f32;
    for y in 0..height {
        for x in 0..width {
            let (existing_foreground, existing_background) = report
                .styled_cells
                .iter()
                .find(|cell| cell.x == x && cell.y == y)
                .map(|cell| (cell.foreground.clone(), cell.background.clone()))
                .unwrap_or_else(|| (DEFAULT_FOREGROUND.to_string(), TRANSPARENT_RGBA.to_string()));
            let foreground = legacy_color_fade_label(
                existing_foreground.as_str(),
                target,
                progress,
                color_space,
            )
            .unwrap_or(existing_foreground);
            let background = legacy_color_fade_label(
                existing_background.as_str(),
                target,
                progress,
                color_space,
            )
            .unwrap_or(existing_background);
            set_report_cell_style(report, x, y, Some(&foreground), Some(&background), None);
        }
    }
}

fn apply_color_shift_style_stage(
    report: &mut PlayerRenderIrReport,
    hue_shift: f64,
    saturation_shift: f64,
    lightness_shift: f64,
) {
    let width = report_width(report);
    let height = report_height(report);
    let progress = report.phase_t.clamp(0.0, 1.0);
    let hue_shift = (hue_shift * progress) as f32;
    let saturation_shift = (saturation_shift * progress) as f32;
    let lightness_shift = (lightness_shift * progress) as f32;
    for y in 0..height {
        for x in 0..width {
            let (existing_foreground, existing_background) = report
                .styled_cells
                .iter()
                .find(|cell| cell.x == x && cell.y == y)
                .map(|cell| (cell.foreground.clone(), cell.background.clone()))
                .unwrap_or_else(|| (DEFAULT_FOREGROUND.to_string(), TRANSPARENT_RGBA.to_string()));
            let foreground = color_shift_label(
                existing_foreground.as_str(),
                hue_shift,
                saturation_shift,
                lightness_shift,
            )
            .unwrap_or(existing_foreground);
            let background = color_shift_label(
                existing_background.as_str(),
                hue_shift,
                saturation_shift,
                lightness_shift,
            )
            .unwrap_or(existing_background);
            set_report_cell_style(report, x, y, Some(&foreground), Some(&background), None);
        }
    }
}

fn apply_pulse_style_stage(
    report: &mut PlayerRenderIrReport,
    color: &str,
    frequency: f64,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let clock = report.loop_t.unwrap_or(report.phase_t);
    let strength = (clock * frequency.max(0.0) * std::f64::consts::TAU).sin() * 0.5 + 0.5;
    for y in 0..height {
        for x in 0..width {
            let (existing_foreground, existing_background) = report
                .styled_cells
                .iter()
                .find(|cell| cell.x == x && cell.y == y)
                .map(|cell| (cell.foreground.clone(), cell.background.clone()))
                .unwrap_or_else(|| (DEFAULT_FOREGROUND.to_string(), TRANSPARENT_RGBA.to_string()));
            let foreground = if matches!(apply_to, "foreground" | "both") {
                pulse_lerp_rgba_label(&existing_foreground, color, strength)
            } else {
                existing_foreground
            };
            let background = if matches!(apply_to, "background" | "both") {
                pulse_lerp_rgba_label(&existing_background, color, strength)
            } else {
                existing_background
            };
            set_report_cell_style(report, x, y, Some(&foreground), Some(&background), None);
        }
    }
}

fn apply_italic_window_style_stage(report: &mut PlayerRenderIrReport, start: f64, end: f64) {
    if !(start..=end).contains(&report.phase_t) {
        return;
    }
    let width = report_width(report);
    let height = report_height(report);
    for y in 0..height {
        for x in 0..width {
            set_report_cell_style(report, x, y, None, None, Some("italic"));
        }
    }
}

fn apply_vignette_style_stage(
    report: &mut PlayerRenderIrReport,
    strength: f64,
    edge_color: &str,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let strength = strength.clamp(0.0, 1.0) as f32;
    let max_distance = vignette_corner_distance(width, height).max(1.0);
    for y in 0..height {
        for x in 0..width {
            let distance = vignette_distance_from_center(x, y, width, height);
            let mix = ((distance / max_distance) as f32 * strength).clamp(0.0, 1.0);
            let foreground = lerp_rgba_label(WHITE_RGBA, edge_color, mix);
            let background = lerp_rgba_label(edge_color, BLACK_RGBA, 1.0 - mix);
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground.as_str(),
                background.as_str(),
                &[],
                "FilterVignette",
            );
        }
    }
}

fn apply_bracket_emphasis_style_stage(
    report: &mut PlayerRenderIrReport,
    emphasis_color: &str,
    background_color: &str,
    progress: f64,
    edge_width: usize,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let progress = progress.clamp(0.0, 1.0);
    let active_edge_width = ((edge_width.max(1) as f64) * progress).ceil() as usize;
    for y in 0..height {
        for x in 0..width {
            let on_edge = x < active_edge_width || x + active_edge_width >= width;
            let foreground = if on_edge {
                Cow::Borrowed(emphasis_color)
            } else {
                Cow::Owned(lerp_rgba_label(
                    emphasis_color,
                    WHITE_RGBA,
                    (0.7 + 0.3 * progress) as f32,
                ))
            };
            let background = if on_edge {
                Cow::Borrowed(background_color)
            } else {
                Cow::Owned(lerp_rgba_label(background_color, BLACK_RGBA, 0.7))
            };
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground.as_ref(),
                background.as_ref(),
                &[],
                "FilterBracketEmphasis",
            );
        }
    }
}

struct EdgeGrowStyleInputs<'a> {
    direction: &'a str,
    progress: f64,
    edge_color: &'a str,
    background_color: &'a str,
    margin_width: usize,
    rest_eighths: usize,
    peak_eighths: usize,
    apply_to: &'a str,
}

fn apply_edge_grow_style_stage(report: &mut PlayerRenderIrReport, inputs: EdgeGrowStyleInputs<'_>) {
    let width = report_width(report).max(1);
    let height = report_height(report).max(1);
    let eighth_span = inputs
        .peak_eighths
        .saturating_sub(inputs.rest_eighths)
        .max(1) as f64
        / 8.0;
    let progress = (inputs.rest_eighths as f64 / 8.0
        + inputs.progress.clamp(0.0, 1.0) * eighth_span)
        .clamp(0.0, 1.0);
    let limit = match inputs.direction {
        "top" | "bottom" => (height as f64 * progress).ceil() as usize,
        _ => (width as f64 * progress).ceil() as usize,
    };
    for y in 0..height {
        for x in 0..width {
            let coordinate = match inputs.direction {
                "right" => width.saturating_sub(1).saturating_sub(x),
                "top" => y,
                "bottom" => height.saturating_sub(1).saturating_sub(y),
                _ => x,
            };
            let in_margin = match inputs.direction {
                "top" | "bottom" => x < inputs.margin_width || x + inputs.margin_width >= width,
                _ => y < inputs.margin_width || y + inputs.margin_width >= height,
            };
            let mix = if coordinate < limit && !in_margin {
                0.0
            } else {
                0.75
            };
            let foreground = lerp_rgba_label(inputs.edge_color, WHITE_RGBA, mix);
            let background = lerp_rgba_label(inputs.background_color, BLACK_RGBA, mix * 0.35);
            set_report_filter_cell(
                report,
                x,
                y,
                inputs.apply_to,
                foreground.as_str(),
                background.as_str(),
                &[],
                "FilterEdgeGrow",
            );
        }
    }
}

fn apply_hover_bar_style_stage(
    report: &mut PlayerRenderIrReport,
    bar_color: &str,
    thickness: usize,
    position: f64,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let thickness = thickness.max(1);
    let position = position.clamp(0.0, 1.0);
    let center_y = ((height.saturating_sub(1)) as f64 * position).round() as usize;
    for y in 0..height {
        for x in 0..width {
            let distance = y.abs_diff(center_y);
            let mix = if distance < thickness { 0.0 } else { 0.8 };
            let foreground = lerp_rgba_label(bar_color, WHITE_RGBA, 0.4 + mix * 0.3);
            let background = lerp_rgba_label(bar_color, BLACK_RGBA, mix);
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground.as_str(),
                background.as_str(),
                &[],
                "FilterHoverBar",
            );
        }
    }
}

struct MatrixRainStyleInputs<'a> {
    speed_multiplier: f64,
    speed_min: f64,
    speed_max: f64,
    glyph_change_hz: f64,
    density: f64,
    seed: f64,
    trail_min: f64,
    trail_max: f64,
    affect: &'a str,
    chars: &'a str,
    mode: &'a str,
    preset: &'a str,
    head_color: &'a str,
    tail_color: &'a str,
}

fn apply_matrix_rain_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: MatrixRainStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    let speed = ((inputs.speed_min + inputs.speed_max.max(inputs.speed_min)) * 0.5)
        * inputs.speed_multiplier.max(0.0);
    let trail = ((inputs.trail_min + inputs.trail_max.max(inputs.trail_min)) * 0.5).max(1.0);
    let text_factor = (inputs.chars.chars().count() as f64
        + inputs.mode.len() as f64
        + inputs.preset.len() as f64
        + inputs.seed)
        % 7.0;
    let density = inputs.density.clamp(0.0, 1.0);
    let level = (100.0
        + ((report.phase_t * speed
            + inputs.glyph_change_hz.max(0.0) * 0.01
            + density
            + text_factor * 0.01)
            / trail)
            .fract()
            * 155.0) as u8;
    let color = lerp_rgba_label(
        lerp_rgba_label(
            inputs.head_color,
            inputs.tail_color,
            (1.0 - density as f32).clamp(0.0, 1.0),
        )
        .as_str(),
        rgba_label(40, level, 80, 255).as_str(),
        0.5,
    );
    let background = if inputs.affect == "background" || inputs.affect == "both" {
        rgba_label(0, level / 3, 0, 255)
    } else {
        TRANSPARENT_RGBA.to_string()
    };
    for y in 0..height {
        for x in 0..width {
            set_report_filter_cell(
                report,
                x,
                y,
                "foreground",
                color.as_str(),
                background.as_str(),
                &[],
                "FilterMatrixRain",
            );
        }
    }
}

struct HighlighterStyleInputs<'a> {
    color: &'a str,
    apply_to: &'a str,
    blend_strength: f64,
    text_contrast: f64,
    soft_edge: bool,
    direction: &'a str,
    row_mask: i64,
    band_width: usize,
}

fn apply_highlighter_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: HighlighterStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    let band_width = inputs.band_width;
    let span = if matches!(inputs.direction, "topToBottom" | "bottomToTop") {
        height.max(1)
    } else {
        width.max(1)
    };
    let center = (report.phase_t.clamp(0.0, 1.0) * span as f64).round() as isize;
    let active_color = lerp_rgba_label(
        inputs.color,
        WHITE_RGBA,
        (inputs.text_contrast * 0.25) as f32,
    );
    let color = lerp_rgba_label(
        active_color.as_str(),
        inputs.color,
        inputs.blend_strength as f32,
    );
    let role = if inputs.soft_edge {
        "ShaderHighlighterSoft"
    } else {
        "ShaderHighlighter"
    };
    for y in 0..height {
        if inputs.row_mask >= 0 && y as i64 != inputs.row_mask {
            continue;
        }
        for x in 0..width {
            let axis = if matches!(inputs.direction, "topToBottom" | "bottomToTop") {
                y
            } else {
                x
            };
            if (axis as isize - center).unsigned_abs() <= band_width {
                set_report_shader_cell(report, x, y, inputs.apply_to, color.as_str(), role);
            }
        }
    }
}

struct FocusFieldStyleInputs<'a> {
    color: &'a str,
    rect_x: f64,
    rect_y: f64,
    rect_width: f64,
    rect_height: f64,
    center_x: f64,
    center_y: f64,
    shape: &'a str,
    radius_x: f64,
    radius_y: f64,
    feather: f64,
    intensity: f64,
    apply_to: &'a str,
}

fn apply_focus_field_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: FocusFieldStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    let focus_color = lerp_rgba_label(
        inputs.color,
        WHITE_RGBA,
        ((1.0 - inputs.intensity) * 0.25) as f32,
    );
    for y in 0..height {
        for x in 0..width {
            let dx = (x as f64 - inputs.center_x).abs();
            let dy = (y as f64 - inputs.center_y).abs();
            let inside = if inputs.shape == "rect" {
                x as f64 >= inputs.rect_x
                    && y as f64 >= inputs.rect_y
                    && x as f64 <= inputs.rect_x + inputs.rect_width
                    && y as f64 <= inputs.rect_y + inputs.rect_height
            } else {
                let normalized = (dx / inputs.radius_x).mul_add(
                    dx / inputs.radius_x,
                    (dy / inputs.radius_y) * (dy / inputs.radius_y),
                );
                normalized.sqrt() <= 1.0 + inputs.feather
            };
            if inside {
                set_report_shader_cell(
                    report,
                    x,
                    y,
                    inputs.apply_to,
                    focus_color.as_str(),
                    "ShaderFocusField",
                );
            }
        }
    }
}

struct GlistenBandStyleInputs<'a> {
    color: &'a str,
    blend_strength: f64,
    angle_deg: f64,
    speed: f64,
    head: f64,
    tail: f64,
    band_width: f64,
    direction: &'a str,
}

fn apply_glisten_band_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: GlistenBandStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    let angle = inputs.angle_deg.to_radians();
    let band_width = inputs.band_width.max(1.0) * (inputs.tail - inputs.head).abs().max(0.25);
    let mut center = (report.phase_t * inputs.speed).fract();
    if inputs.direction == "rightToLeft" {
        center = 1.0 - center;
    }
    let max_x = width.saturating_sub(1).max(1) as f64;
    let max_y = height.max(1) as f64;
    let color = lerp_rgba_label(inputs.color, WHITE_RGBA, inputs.blend_strength as f32);
    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / max_x;
            let diagonal = (nx * angle.cos() + y as f64 * angle.sin() / max_y).fract();
            if (diagonal - center).abs() <= band_width / max_x {
                set_report_shader_cell(
                    report,
                    x,
                    y,
                    "foreground",
                    color.as_str(),
                    "ShaderGlistenBand",
                );
            }
        }
    }
}

struct WayfindingNodeStyleInputs<'a> {
    current_index: usize,
    nodes: usize,
    previous_strength: f64,
    future_strength: f64,
    intensity: f64,
    radius: usize,
    active_color: &'a str,
}

fn apply_wayfinding_node_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: WayfindingNodeStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    let cell_count = width * height;
    if cell_count == 0 {
        return;
    }
    let active_index = inputs.current_index % cell_count.min(inputs.nodes).max(1);
    for offset in 0..=inputs.radius {
        for (index, strength) in [
            (
                active_index.saturating_sub(offset),
                inputs.previous_strength,
            ),
            (
                (active_index + offset).min(cell_count - 1),
                inputs.future_strength,
            ),
        ] {
            let x = index % width;
            let y = index / width;
            let color = lerp_rgba_label(
                inputs.active_color,
                WHITE_RGBA,
                (1.0 - strength * inputs.intensity).clamp(0.0, 1.0) as f32,
            );
            set_report_shader_cell(report, x, y, "both", color.as_str(), "ShaderWayfindingNode");
        }
    }
}

struct BarberPoleStyleInputs<'a> {
    stripe_color: &'a str,
    background_color: &'a str,
    stripe_width: usize,
    gap_width: usize,
    angle_deg: f64,
    speed: f64,
    apply_to: &'a str,
}

fn apply_barber_pole_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: BarberPoleStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    let period = inputs.stripe_width + inputs.gap_width;
    let angle = inputs.angle_deg.to_radians();
    let phase_offset =
        (report.loop_t.unwrap_or(report.phase_t) * inputs.speed * period as f64).round();
    for y in 0..height {
        for x in 0..width {
            let projection = x as f64 * angle.cos() + y as f64 * angle.sin() + phase_offset;
            let position = projection.rem_euclid(period as f64) as usize;
            let color = if position < inputs.stripe_width {
                inputs.stripe_color
            } else {
                inputs.background_color
            };
            set_report_shader_cell(report, x, y, inputs.apply_to, color, "ShaderBarberPole");
        }
    }
}

struct DiffusionStyleInputs<'a> {
    color: &'a str,
    center_x: f64,
    center_y: f64,
    radius: f64,
    intensity: f64,
    apply_to: &'a str,
}

fn apply_diffusion_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: DiffusionStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    for y in 0..height {
        for x in 0..width {
            let distance = ((x as f64 - inputs.center_x).powi(2)
                + (y as f64 - inputs.center_y).powi(2))
            .sqrt();
            let falloff = (1.0 - distance / inputs.radius).clamp(0.0, 1.0) as f32;
            if falloff > 0.0 {
                let color = lerp_rgba_label(
                    BLACK_RGBA,
                    inputs.color,
                    (falloff * inputs.intensity as f32).clamp(0.0, 1.0),
                );
                set_report_shader_cell(
                    report,
                    x,
                    y,
                    inputs.apply_to,
                    color.as_str(),
                    "ShaderDiffusion",
                );
            }
        }
    }
}

struct RadarStyleInputs<'a> {
    color: &'a str,
    speed: f64,
    tail_length: f64,
    apply_to: &'a str,
}

fn apply_radar_style_stage(report: &mut PlayerRenderIrReport, inputs: RadarStyleInputs<'_>) {
    let width = report_width(report);
    let height = report_height(report);
    let center_x = width.saturating_sub(1) as f64 / 2.0;
    let center_y = height.saturating_sub(1) as f64 / 2.0;
    let sweep = (report.loop_t.unwrap_or(report.phase_t) * inputs.speed).fract();
    for y in 0..height {
        for x in 0..width {
            let angle = ((y as f64 - center_y).atan2(x as f64 - center_x) + std::f64::consts::TAU)
                % std::f64::consts::TAU;
            let position = angle / std::f64::consts::TAU;
            let distance_behind = (sweep - position).rem_euclid(1.0);
            if distance_behind <= inputs.tail_length {
                let strength = (1.0 - distance_behind / inputs.tail_length) as f32;
                let color = lerp_rgba_label(BLACK_RGBA, inputs.color, strength);
                set_report_shader_cell(
                    report,
                    x,
                    y,
                    inputs.apply_to,
                    color.as_str(),
                    "ShaderRadar",
                );
            }
        }
    }
}

fn apply_sub_pixel_bar_style_stage(
    report: &mut PlayerRenderIrReport,
    filled_color: &str,
    unfilled_color: &str,
    progress: f64,
    direction: &str,
) {
    let width = report
        .rows
        .first()
        .map(|row| row.chars().count())
        .unwrap_or(0)
        .max(1);
    let height = report.rows.len();
    let progress = progress.clamp(0.0, 1.0);
    let horizontal = !matches!(direction, "vertical" | "topToBottom" | "bottomToTop");
    let total_subcells = if horizontal { width } else { height }.saturating_mul(8);
    let filled_subcells = (total_subcells as f64 * progress).ceil() as usize;
    for y in 0..height {
        for x in 0..width {
            let coordinate = if horizontal { x } else { y };
            let filled = filled_subcells
                .saturating_sub(coordinate.saturating_mul(8))
                .min(8);
            let glyph = sub_pixel_bar_glyph(filled);
            let foreground = if filled == 0 {
                unfilled_color
            } else {
                filled_color
            };
            set_report_cell_glyph_exact(
                report,
                x,
                y,
                glyph,
                foreground,
                unfilled_color,
                &[],
                "FilterSubPixelBar",
            );
        }
    }
}

fn sub_pixel_bar_glyph(filled: usize) -> &'static str {
    match filled {
        0 => " ",
        1 => "▏",
        2 => "▎",
        3 => "▍",
        4 => "▌",
        5 => "▋",
        6 => "▊",
        7 => "▉",
        _ => "█",
    }
}

struct UnderlineWipeStyleInputs<'a> {
    underline_color: &'a str,
    background_color: &'a str,
    direction: &'a str,
    line_char: char,
    row_offset: usize,
    progress: f64,
    gradient: bool,
    glisten: f64,
    apply_to: &'a str,
}

fn apply_underline_wipe_style_stage(
    report: &mut PlayerRenderIrReport,
    inputs: UnderlineWipeStyleInputs<'_>,
) {
    let width = report_width(report);
    let height = report_height(report);
    if width == 0 || height == 0 {
        return;
    }
    let target_row = height
        .saturating_sub(1)
        .saturating_sub(inputs.row_offset.min(height.saturating_sub(1)));
    let progress = inputs.progress.clamp(0.0, 1.0);
    let revealed_width = (width as f64 * progress).ceil() as usize;
    if revealed_width == 0 {
        return;
    }
    for x in 0..width {
        let (should_draw, position_ratio) =
            underline_wipe_cell_position(x, width, revealed_width, inputs.direction);
        if !should_draw {
            continue;
        }
        let foreground =
            underline_wipe_color(report, &inputs, position_ratio.clamp(0.0, 1.0) as f32);
        set_report_cell_glyph_exact(
            report,
            x,
            target_row,
            &inputs.line_char.to_string(),
            foreground_for_apply_to(inputs.apply_to, &foreground),
            background_for_apply_to(inputs.apply_to, inputs.background_color),
            &["underline"],
            "FilterUnderlineWipe",
        );
    }
}

fn underline_wipe_cell_position(
    x: usize,
    width: usize,
    revealed_width: usize,
    direction: &str,
) -> (bool, f64) {
    match direction {
        "rightToLeft" | "right_to_left" => {
            let start = width.saturating_sub(revealed_width);
            let draw = x >= start;
            let ratio = width.saturating_sub(1).saturating_sub(x) as f64 / revealed_width as f64;
            (draw, ratio)
        }
        _ => {
            let draw = x < revealed_width;
            let ratio = x as f64 / revealed_width as f64;
            (draw, ratio)
        }
    }
}

fn underline_wipe_color(
    report: &PlayerRenderIrReport,
    inputs: &UnderlineWipeStyleInputs<'_>,
    position_ratio: f32,
) -> String {
    let base_color = if inputs.gradient {
        lerp_rgba_label(
            inputs.background_color,
            inputs.underline_color,
            1.0 - position_ratio,
        )
    } else {
        inputs.underline_color.to_string()
    };
    if inputs.glisten <= 0.0 {
        return base_color;
    }
    let clock = report.loop_t.unwrap_or(report.phase_t);
    let glisten_position = ((clock * 0.45).fract()) as f32;
    let distance = (position_ratio - glisten_position)
        .abs()
        .min((position_ratio - glisten_position + 1.0).abs());
    let intensity = ((1.0 - distance / 0.2).clamp(0.0, 1.0) * inputs.glisten as f32) * 0.25;
    if intensity <= 0.0 {
        base_color
    } else {
        lerp_rgba_label(&base_color, WHITE_RGBA, intensity)
    }
}

fn background_for_apply_to<'a>(apply_to: &str, background_color: &'a str) -> &'a str {
    if matches!(apply_to, "background" | "both") {
        background_color
    } else {
        TRANSPARENT_RGBA
    }
}

fn foreground_for_apply_to<'a>(apply_to: &str, foreground_color: &'a str) -> &'a str {
    if matches!(apply_to, "foreground" | "both") {
        foreground_color
    } else {
        DEFAULT_FOREGROUND
    }
}

fn set_report_cell_style(
    report: &mut PlayerRenderIrReport,
    x: usize,
    y: usize,
    foreground: Option<&str>,
    background: Option<&str>,
    modifier: Option<&str>,
) {
    if let Some(cell) = report
        .styled_cells
        .iter_mut()
        .find(|cell| cell.x == x && cell.y == y)
    {
        if let Some(foreground) = foreground {
            cell.foreground = foreground.to_string();
        }
        if let Some(background) = background {
            cell.background = background.to_string();
        }
        if let Some(modifier) = modifier
            && !cell.modifiers.iter().any(|existing| existing == modifier)
        {
            cell.modifiers.push(modifier.to_string());
        }
        return;
    }

    let glyph = report
        .rows
        .get(y)
        .and_then(|row| row.chars().nth(x))
        .unwrap_or(' ')
        .to_string();
    report.styled_cells.push(PlayerRenderCell {
        x,
        y,
        glyph,
        foreground: foreground.unwrap_or(DEFAULT_FOREGROUND).to_string(),
        background: background.unwrap_or(TRANSPARENT_RGBA).to_string(),
        modifiers: modifier.into_iter().map(str::to_string).collect(),
        role: None,
    });
}

fn set_report_filter_cell(
    report: &mut PlayerRenderIrReport,
    x: usize,
    y: usize,
    apply_to: &str,
    foreground: &str,
    background: &str,
    modifiers: &[&str],
    role: &str,
) {
    let foreground = if matches!(apply_to, "foreground" | "both") {
        foreground
    } else {
        DEFAULT_FOREGROUND
    };
    let background = if matches!(apply_to, "background" | "both") {
        background
    } else {
        TRANSPARENT_RGBA
    };
    set_report_cell_exact(report, x, y, foreground, background, modifiers, Some(role));
}

fn set_report_shader_cell(
    report: &mut PlayerRenderIrReport,
    x: usize,
    y: usize,
    apply_to: &str,
    color: &str,
    role: &str,
) {
    let (existing_foreground, existing_background) = report
        .styled_cells
        .iter()
        .find(|cell| cell.x == x && cell.y == y)
        .map(|cell| (cell.foreground.clone(), cell.background.clone()))
        .unwrap_or_else(|| (DEFAULT_FOREGROUND.to_string(), TRANSPARENT_RGBA.to_string()));
    let foreground = if matches!(apply_to, "foreground" | "both") {
        color.to_string()
    } else {
        existing_foreground
    };
    let background = if matches!(apply_to, "background" | "both") {
        color.to_string()
    } else {
        existing_background
    };
    set_report_cell_exact(
        report,
        x,
        y,
        foreground.as_str(),
        background.as_str(),
        &[],
        Some(role),
    );
}

fn set_report_cell_exact(
    report: &mut PlayerRenderIrReport,
    x: usize,
    y: usize,
    foreground: &str,
    background: &str,
    modifiers: &[&str],
    role: Option<&str>,
) {
    if let Some(cell) = report
        .styled_cells
        .iter_mut()
        .find(|cell| cell.x == x && cell.y == y)
    {
        cell.foreground = foreground.to_string();
        cell.background = background.to_string();
        cell.modifiers = modifier_labels(modifiers);
        cell.role = role.map(str::to_string);
        return;
    }

    let glyph = report
        .rows
        .get(y)
        .and_then(|row| row.chars().nth(x))
        .unwrap_or(' ')
        .to_string();
    report.styled_cells.push(PlayerRenderCell {
        x,
        y,
        glyph,
        foreground: foreground.to_string(),
        background: background.to_string(),
        modifiers: modifier_labels(modifiers),
        role: role.map(str::to_string),
    });
}

fn set_report_cell_glyph_exact(
    report: &mut PlayerRenderIrReport,
    x: usize,
    y: usize,
    glyph: &str,
    foreground: &str,
    background: &str,
    modifiers: &[&str],
    role: &str,
) {
    replace_row_glyph(report, x, y, glyph);
    if let Some(cell) = report
        .styled_cells
        .iter_mut()
        .find(|cell| cell.x == x && cell.y == y)
    {
        cell.glyph = glyph.to_string();
        cell.foreground = foreground.to_string();
        cell.background = background.to_string();
        cell.modifiers = modifier_labels(modifiers);
        cell.role = Some(role.to_string());
        return;
    }
    report.styled_cells.push(PlayerRenderCell {
        x,
        y,
        glyph: glyph.to_string(),
        foreground: foreground.to_string(),
        background: background.to_string(),
        modifiers: modifier_labels(modifiers),
        role: Some(role.to_string()),
    });
}

fn replace_row_glyph(report: &mut PlayerRenderIrReport, x: usize, y: usize, glyph: &str) {
    let Some(row) = report.rows.get_mut(y) else {
        return;
    };
    let mut glyphs = row.chars().collect::<Vec<_>>();
    let Some(replacement) = glyph.chars().next() else {
        return;
    };
    if let Some(cell) = glyphs.get_mut(x) {
        *cell = replacement;
        *row = glyphs.into_iter().collect();
    }
}

fn modifier_labels(modifiers: &[&str]) -> Vec<String> {
    modifiers
        .iter()
        .map(|modifier| (*modifier).to_string())
        .collect()
}

fn dense_rows(report: &PlayerRenderIrReport, width: usize, height: usize) -> Vec<String> {
    let mut rows = (0..height)
        .map(|y| {
            let mut chars = report
                .rows
                .get(y)
                .map(|row| row.chars().collect::<Vec<_>>())
                .unwrap_or_default();
            chars.resize(width, ' ');
            chars
        })
        .collect::<Vec<_>>();
    for cell in &report.styled_cells {
        if cell.x < width && cell.y < height {
            rows[cell.y][cell.x] = cell.glyph.chars().next().unwrap_or(' ');
        }
    }
    rows.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect()
}

fn sync_styled_cells_to_rows(report: &mut PlayerRenderIrReport) {
    let row_chars = report
        .rows
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for cell in &mut report.styled_cells {
        cell.glyph = row_chars
            .get(cell.y)
            .and_then(|row| row.get(cell.x))
            .copied()
            .unwrap_or(' ')
            .to_string();
    }
}

fn report_width(report: &PlayerRenderIrReport) -> usize {
    report
        .width
        .max(
            report
                .rows
                .iter()
                .map(|row| row.chars().count())
                .max()
                .unwrap_or(0),
        )
        .max(
            report
                .styled_cells
                .iter()
                .map(|cell| cell.x + 1)
                .max()
                .unwrap_or(0),
        )
}

fn report_height(report: &PlayerRenderIrReport) -> usize {
    report.height.max(report.rows.len()).max(
        report
            .styled_cells
            .iter()
            .map(|cell| cell.y + 1)
            .max()
            .unwrap_or(0),
    )
}

fn split_flap_charset(charset_name: &str) -> Vec<char> {
    match charset_name {
        "digits" => "0123456789".chars().collect(),
        "binary" => "01".chars().collect(),
        "alphanumeric" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect(),
        _ => "▣▤▥▦▧▨".chars().collect(),
    }
}

fn normalized_rows(text: &str, expected: usize) -> Vec<String> {
    let mut rows = text.lines().map(str::to_string).collect::<Vec<_>>();
    rows.resize(expected, String::new());
    rows
}

fn odometer_travel_span(travel: &str, width: usize, height: usize) -> f64 {
    match travel {
        "fullClear" | "full_clear" => width.max(height).max(1) as f64,
        "cells" => 2.0,
        _ => 1.0,
    }
}

fn odometer_reveal_threshold(travel: &str) -> f64 {
    match travel {
        "fullClear" | "full_clear" => 0.75,
        "cells" => 0.6,
        _ => 0.5,
    }
}

fn previous_digit(glyph: char) -> char {
    match glyph {
        '0' => '9',
        '1'..='9' => char::from_u32(glyph as u32 - 1).unwrap_or(glyph),
        _ => glyph,
    }
}

fn apply_glitch_shift_rows(rows: &mut [String], amount: usize, seed: usize) {
    for (y, row) in rows.iter_mut().enumerate() {
        if row.is_empty() || !(y + seed).is_multiple_of(2) {
            continue;
        }
        *row = rotate_row(row, amount.min(row.chars().count().saturating_sub(1)));
    }
}

fn rotate_row(row: &str, offset: usize) -> String {
    let chars = row.chars().collect::<Vec<_>>();
    let width = chars.len();
    if width == 0 {
        return String::new();
    }
    chars[offset..]
        .iter()
        .chain(chars[..offset].iter())
        .collect::<String>()
}

fn shift_row(row: &str, offset: isize) -> String {
    let chars = row.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return String::new();
    }
    let width = chars.len() as isize;
    (0..width)
        .map(|x| {
            let source = x - offset;
            if (0..width).contains(&source) {
                chars[source as usize]
            } else {
                ' '
            }
        })
        .collect()
}

fn drop_every_nth_glyph(row: &str, scanline_strength: f64) -> String {
    let interval = if scanline_strength >= 0.66 {
        3
    } else if scanline_strength >= 0.33 {
        5
    } else {
        8
    };
    row.chars()
        .enumerate()
        .map(|(index, glyph)| if index % interval == 0 { ' ' } else { glyph })
        .collect()
}

fn cell_threshold(x: usize, y: usize) -> f64 {
    ((x * 37 + y * 17) % 100) as f64 / 99.0
}

fn dissolve_threshold(x: usize, y: usize, width: usize, seed: usize, direction: &str) -> f64 {
    match direction {
        "leftToRight" | "left_to_right" => x as f64 / width.max(1) as f64,
        "rightToLeft" | "right_to_left" => width.saturating_sub(x + 1) as f64 / width.max(1) as f64,
        _ => cell_threshold(x + seed, y),
    }
}

fn blinds_keeps_cell(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    count: usize,
    orientation: &str,
    reveal: f64,
) -> bool {
    let position = if orientation == "vertical" {
        x as f64
    } else {
        y as f64
    };
    let size = if orientation == "vertical" {
        width as f64
    } else {
        height as f64
    };
    let blind_size = (size / count.max(1) as f64).max(1.0);
    let blind_index = (position / blind_size).floor();
    let position_in_blind = position - blind_index * blind_size;
    position_in_blind < blind_size * reveal.clamp(0.0, 1.0)
}

#[derive(Clone, Copy)]
enum SourceMaskShape {
    Circle,
    Diamond,
}

fn source_mask_spotlight_radius(phase_t: f64, width: usize, height: usize, soft_edge: bool) -> f64 {
    let max_radius = width.max(height) as f64;
    let radius = max_radius * phase_t.clamp(0.0, 1.0);
    if soft_edge {
        radius + max_radius * 0.1
    } else {
        radius
    }
}

fn source_mask_spotlight_distance(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    shape: SourceMaskShape,
) -> f64 {
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;
    let dx = (x as f64 - center_x).abs();
    let dy = (y as f64 - center_y).abs();
    match shape {
        SourceMaskShape::Circle => dx.mul_add(dx, dy * dy).sqrt(),
        SourceMaskShape::Diamond => dx + dy,
    }
}

fn dissolve_cell_noise(seed: u64, x: usize, y: usize) -> f64 {
    let mut hash = seed ^ 0xcbf2_9ce4_8422_2325u64;
    for value in [x as u64, y as u64] {
        hash ^= value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    (hash % 10_000) as f64 / 9_999.0
}

fn wipe_cutoff(width: usize, reveal: f64, soft_edge: bool) -> usize {
    let mut scaled = width as f64 * reveal;
    if soft_edge {
        scaled += (width as f64 * 0.1).max(1.0);
        scaled.floor() as usize
    } else {
        scaled.floor() as usize
    }
}

fn wipe_keeps_cell(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    horizontal_cutoff: usize,
    vertical_cutoff: usize,
    direction: &str,
) -> bool {
    match direction {
        "rightToLeft" => x >= width.saturating_sub(horizontal_cutoff),
        "horizontalCenterOut" | "horizontal_center_out" => {
            horizontal_center_out_keeps_cell(x, width, horizontal_cutoff)
        }
        "horizontalEdgesIn" | "horizontal_edges_in" => {
            horizontal_edges_in_keeps_cell(x, width, horizontal_cutoff)
        }
        "topToBottom" => y < vertical_cutoff,
        "bottomToTop" => y >= height.saturating_sub(vertical_cutoff),
        "outFromTopLeft" => {
            x.saturating_add(y) <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "outFromTopRight" => {
            width.saturating_sub(1).saturating_sub(x).saturating_add(y)
                <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "outFromBottomLeft" => {
            x.saturating_add(height.saturating_sub(1).saturating_sub(y))
                <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "outFromBottomRight" => {
            width
                .saturating_sub(1)
                .saturating_sub(x)
                .saturating_add(height.saturating_sub(1).saturating_sub(y))
                <= horizontal_cutoff.saturating_add(vertical_cutoff)
        }
        "inToTopLeft" => {
            x.saturating_add(y)
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        "inToTopRight" => {
            width.saturating_sub(1).saturating_sub(x).saturating_add(y)
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        "inToBottomLeft" => {
            x.saturating_add(height.saturating_sub(1).saturating_sub(y))
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        "inToBottomRight" => {
            width
                .saturating_sub(1)
                .saturating_sub(x)
                .saturating_add(height.saturating_sub(1).saturating_sub(y))
                >= width.saturating_add(height).saturating_sub(
                    horizontal_cutoff
                        .saturating_add(vertical_cutoff)
                        .saturating_add(2),
                )
        }
        _ => x < horizontal_cutoff,
    }
}

fn horizontal_center_out_keeps_cell(x: usize, width: usize, cutoff: usize) -> bool {
    let center_twice = width.saturating_sub(1);
    let x_twice = x.saturating_mul(2);
    x_twice.abs_diff(center_twice) <= cutoff.saturating_mul(2).saturating_sub(1)
}

fn horizontal_edges_in_keeps_cell(x: usize, width: usize, cutoff: usize) -> bool {
    let edge_reveal = cutoff / 2;
    x < edge_reveal || x >= width.saturating_sub(edge_reveal)
}

fn vignette_distance_from_center(x: usize, y: usize, width: usize, height: usize) -> f64 {
    let center_x = (width.saturating_sub(1)) as f64 / 2.0;
    let center_y = (height.saturating_sub(1)) as f64 / 2.0;
    let dx = x as f64 - center_x;
    let dy = y as f64 - center_y;
    dx.mul_add(dx, dy * dy).sqrt()
}

fn vignette_corner_distance(width: usize, height: usize) -> f64 {
    vignette_distance_from_center(0, 0, width, height)
}

const DEFAULT_FOREGROUND: &str = "defaultForeground";
const TRANSPARENT_RGBA: &str = "transparent";
const WHITE_RGBA: &str = "rgba(255,255,255,255)";
const BLACK_RGBA: &str = "rgba(0,0,0,255)";

fn lerp_rgba_label(from: &str, to: &str, t: f32) -> String {
    let Some((from_r, from_g, from_b, from_a)) = parse_rgba_label(from) else {
        return from.to_string();
    };
    let Some((to_r, to_g, to_b, to_a)) = parse_rgba_label(to) else {
        return from.to_string();
    };
    let t = t.clamp(0.0, 1.0);
    let inv_t = 1.0 - t;
    rgba_label(
        lerp_channel(from_r, to_r, inv_t, t),
        lerp_channel(from_g, to_g, inv_t, t),
        lerp_channel(from_b, to_b, inv_t, t),
        lerp_channel(from_a, to_a, inv_t, t),
    )
}

fn pulse_lerp_rgba_label(from: &str, to: &str, t: f64) -> String {
    let Some((from_r, from_g, from_b, from_a)) = parse_rgba_label(from) else {
        return from.to_string();
    };
    let Some((to_r, to_g, to_b, to_a)) = parse_rgba_label(to) else {
        return from.to_string();
    };
    let t = t.clamp(0.0, 1.0);
    let inv_t = 1.0 - t;
    rgba_label(
        pulse_lerp_channel(from_r, to_r, inv_t, t),
        pulse_lerp_channel(from_g, to_g, inv_t, t),
        pulse_lerp_channel(from_b, to_b, inv_t, t),
        pulse_lerp_channel(from_a, to_a, inv_t, t),
    )
}

fn pulse_lerp_channel(start: u8, end: u8, inv_t: f64, t: f64) -> u8 {
    (start as f64 * inv_t + end as f64 * t) as u8
}

fn lerp_channel(start: u8, end: u8, inv_t: f32, t: f32) -> u8 {
    (start as f32 * inv_t + end as f32 * t + 0.5) as u8
}

fn rgba_label(r: u8, g: u8, b: u8, a: u8) -> String {
    format!("rgba({r},{g},{b},{a})")
}

fn dimmed_rgba_label(label: &str, strength: f64) -> String {
    let Some((r, g, b, a)) = parse_rgba_label(label) else {
        return label.to_string();
    };
    let strength = strength.clamp(0.0, 1.0);
    format!(
        "rgba({},{},{},{})",
        (r as f64 * strength).round() as u8,
        (g as f64 * strength).round() as u8,
        (b as f64 * strength).round() as u8,
        a
    )
}

fn parse_rgba_label(label: &str) -> Option<(u8, u8, u8, u8)> {
    let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(str::trim);
    Some((
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
        parts.next()?.parse().ok()?,
    ))
}

fn legacy_color_fade_label(from: &str, target: &str, t: f32, color_space: &str) -> Option<String> {
    let from = parse_rgba_label(from)?;
    let target = parse_rgba_label(target)?;
    let blended = if color_space.eq_ignore_ascii_case("hsl") {
        legacy_hsl_rgba(from, target, t)
    } else {
        legacy_rgb_rgba(from, target, t)
    };
    Some(rgba_label(blended.0, blended.1, blended.2, blended.3))
}

fn color_shift_label(
    label: &str,
    hue_shift: f32,
    saturation_shift: f32,
    lightness_shift: f32,
) -> Option<String> {
    let (r, g, b, a) = parse_rgba_label(label)?;
    let (hue, saturation, lightness) = rgb_to_hsl(r, g, b);
    let (r, g, b) = hsl_to_rgb(
        (hue + hue_shift).rem_euclid(360.0),
        (saturation + saturation_shift).clamp(0.0, 1.0),
        (lightness + lightness_shift).clamp(0.0, 1.0),
    );
    Some(rgba_label(r, g, b, a))
}

fn legacy_rgb_rgba(from: (u8, u8, u8, u8), target: (u8, u8, u8, u8), t: f32) -> (u8, u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        legacy_lerp_channel(from.0, target.0, t),
        legacy_lerp_channel(from.1, target.1, t),
        legacy_lerp_channel(from.2, target.2, t),
        legacy_lerp_channel(from.3, target.3, t),
    )
}

fn legacy_hsl_rgba(from: (u8, u8, u8, u8), target: (u8, u8, u8, u8), t: f32) -> (u8, u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    let (start_hue, start_saturation, start_lightness) = rgb_to_hsl(from.0, from.1, from.2);
    let (end_hue, end_saturation, end_lightness) = rgb_to_hsl(target.0, target.1, target.2);
    let hue_delta = if end_hue - start_hue > 180.0 {
        end_hue - start_hue - 360.0
    } else if end_hue - start_hue < -180.0 {
        end_hue - start_hue + 360.0
    } else {
        end_hue - start_hue
    };
    let hue = (start_hue + hue_delta * t).rem_euclid(360.0);
    let saturation = start_saturation + (end_saturation - start_saturation) * t;
    let lightness = start_lightness + (end_lightness - start_lightness) * t;
    let (r, g, b) = hsl_to_rgb(hue, saturation, lightness);
    (r, g, b, legacy_lerp_channel(from.3, target.3, t))
}

fn legacy_lerp_channel(start: u8, end: u8, t: f32) -> u8 {
    (start as f32 + (end as f32 - start as f32) * t) as u8
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let lightness = (max + min) / 2.0;
    if max == min {
        return (0.0, 0.0, lightness);
    }
    let delta = max - min;
    let saturation = if lightness > 0.5 {
        delta / (2.0 - max - min)
    } else {
        delta / (max + min)
    };
    let hue = if max == r {
        (g - b) / delta + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };
    (hue * 60.0, saturation, lightness)
}

fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (u8, u8, u8) {
    if saturation == 0.0 {
        let value = (lightness * 255.0) as u8;
        return (value, value, value);
    }
    let q = if lightness < 0.5 {
        lightness * (1.0 + saturation)
    } else {
        lightness + saturation - lightness * saturation
    };
    let p = 2.0 * lightness - q;
    let r = hue_to_rgb(p, q, hue / 360.0 + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, hue / 360.0);
    let b = hue_to_rgb(p, q, hue / 360.0 - 1.0 / 3.0);
    (
        (r * 255.0 + 0.0001) as u8,
        (g * 255.0 + 0.0001) as u8,
        (b * 255.0 + 0.0001) as u8,
    )
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

fn scramble_glyph(index: usize, charset: &[char]) -> char {
    const FALLBACK_GLYPHS: &[char] = &['#', '%', '&', '?', '+', '*'];
    let glyphs = if charset.is_empty() {
        FALLBACK_GLYPHS
    } else {
        charset
    };
    glyphs[index % glyphs.len()]
}

fn scene_ir_for_request(
    request: &PlayerRenderBackendRequest,
    fallback_used: bool,
) -> &PlayerRenderIrReport {
    match request.backend_options.composition_mode {
        PlayerRenderCompositionMode::Native => &request.source_ir,
        PlayerRenderCompositionMode::Auto if !fallback_used => &request.source_ir,
        PlayerRenderCompositionMode::Auto | PlayerRenderCompositionMode::IrResolved => &request.ir,
    }
}

fn mirror_evidence_into_metadata(
    metadata: &mut BTreeMap<String, serde_json::Value>,
    evidence: &tui_vfx_player::PlayerRenderBackendCompositionEvidence,
) {
    metadata.insert(
        "compositionMode".to_string(),
        json!(evidence.composition_mode),
    );
    metadata.insert("fallbackUsed".to_string(), json!(evidence.fallback_used));
    metadata.insert(
        "nativeLoweringAttempted".to_string(),
        json!(evidence.native_lowering_attempted),
    );
    metadata.insert(
        "nativeLoweringSucceeded".to_string(),
        json!(evidence.native_lowering_succeeded),
    );
    metadata.insert(
        "compositionSpecNonEmpty".to_string(),
        json!(evidence.composition_spec_non_empty),
    );
    metadata.insert(
        "loweredNodeCount".to_string(),
        json!(evidence.lowered_node_count),
    );
    metadata.insert(
        "unloweredNodeCount".to_string(),
        json!(evidence.unlowered_node_count),
    );
    metadata.insert(
        "loweredEffectIds".to_string(),
        json!(evidence.lowered_effect_ids),
    );
    metadata.insert(
        "unloweredEffectIds".to_string(),
        json!(evidence.unlowered_effect_ids),
    );
    metadata.insert(
        "compositionSpecSummary".to_string(),
        json!(evidence.composition_spec_summary),
    );
    metadata.insert(
        "sourceRenderMode".to_string(),
        json!(evidence.source_render_mode),
    );
    metadata.insert(
        "nativeSourceIsolated".to_string(),
        json!(evidence.native_source_isolated),
    );
}

fn insert_common_spec_metadata(
    metadata: &mut BTreeMap<String, serde_json::Value>,
    spec: &CompositionSpec,
) {
    metadata.insert(
        "compositionSpecFilters".to_string(),
        json!(spec.filters.len()),
    );
    metadata.insert("compositionSpecMasks".to_string(), json!(spec.masks.len()));
    metadata.insert(
        "compositionSpecShaderLayers".to_string(),
        json!(spec.shader_layers.len()),
    );
}

fn composition_spec_summary(spec: &CompositionSpec) -> BTreeMap<String, serde_json::Value> {
    BTreeMap::from([
        ("contentStages".to_string(), json!(0)),
        (
            "samplers".to_string(),
            json!(spec.effective_samplers().len()),
        ),
        ("masks".to_string(), json!(spec.masks.len())),
        ("filters".to_string(), json!(spec.filters.len())),
        ("shaderLayers".to_string(), json!(spec.shader_layers.len())),
        ("shadow".to_string(), json!(spec.shadow.is_some())),
        (
            "timing".to_string(),
            json!({"t": spec.t, "loopT": spec.loop_t}),
        ),
    ])
}

fn rows_from_scene(grid: &dyn Grid) -> Vec<String> {
    (0..grid.height())
        .map(|y| {
            (0..grid.width())
                .map(|x| grid.get(x, y).map(|cell| cell.ch).unwrap_or(' '))
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn styled_cells_from_scene(scene: &tui_vfx_types::SemanticScene) -> Vec<PlayerRenderCell> {
    let mut cells = Vec::new();
    for y in 0..scene.grid().height() {
        for x in 0..scene.grid().width() {
            let Some(cell) = scene.grid().get(x, y) else {
                continue;
            };
            let role = scene
                .role((x as u16, y as u16))
                .filter(|role| !matches!(role, RoleTag::Background));
            if cell.ch != ' '
                || cell.fg.a != 0
                || cell.bg.a != 0
                || !cell.mods.is_empty()
                || role.is_some()
            {
                cells.push(player_cell_from_compositor_cell(x, y, cell, role));
            }
        }
    }
    cells.sort_by_key(|cell| (cell.y, cell.x, cell.glyph.clone()));
    cells
}

#[cfg(test)]
mod tests {
    use tui_vfx_contract::LifecyclePhase;
    use tui_vfx_player::{PlayerRenderClockSample, PlayerRenderIrReport, PlayerStatus};

    use super::*;

    #[test]
    fn compositor_backend_preserves_styled_cells() {
        let report = PlayerRenderIrReport {
            schema_version: "v3.1.player.renderIr.1",
            recipe_id: "demo".to_string(),
            path: Some("demo.json".to_string()),
            status: PlayerStatus::Rendered,
            phase: LifecyclePhase::Dwell,
            phase_t: 0.5,
            loop_t: Some(0.5),
            clock: PlayerRenderClockSample {
                mode: "fixed".to_string(),
                period_ms: None,
                absolute_t_ms: None,
                loop_t: Some(0.5),
            },
            width: 1,
            height: 1,
            render_hash: 7,
            non_empty_cells: 1,
            rows: vec!["X".to_string()],
            styled_cells: vec![PlayerRenderCell {
                x: 0,
                y: 0,
                glyph: "X".to_string(),
                foreground: "rgba(10,20,30,255)".to_string(),
                background: "transparent".to_string(),
                modifiers: vec![],
                role: Some("text".to_string()),
            }],
            provenance: vec![],
            layers: vec![],
            graph_values: vec![],
            errors: vec![],
            warnings: vec![],
        };

        let output = render_compositor_backend(&report);
        assert_eq!(output.backend, "compositor");
        assert_eq!(output.render_hash, 7);
        assert_eq!(output.rows, vec!["X".to_string()]);
        assert_eq!(output.styled_cells[0].foreground, "rgba(10,20,30,255)");
        assert!(output.backend_hash > 0);
    }
}

// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs</FILE> - <DESC>Render player IR through the compositor backend</DESC>
// <VERS>END OF VERSION: 0.22.0</VERS>
