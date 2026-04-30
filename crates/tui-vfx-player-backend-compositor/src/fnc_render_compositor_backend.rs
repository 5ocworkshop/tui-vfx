// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs</FILE> - <DESC>Render player IR through the compositor backend</DESC>
// <VERS>VERSION: 0.8.0</VERS>
// <WCTX>Native compositor source isolation: render native requests from source-only IR, including backend-owned content/style/filter stages, and keep IR-resolved compatibility separate.</WCTX>
// <CLOG>0.8.0: MINOR — apply source-owned radial and wipe-corner mask stages for player-compatible parity.
// 0.7.0: MINOR — apply source-owned mask stages for player-compatible mask parity.
// 0.6.1: PATCH — hoist stable cellular reveal threshold calculation and sync metadata footer.
// 0.6.0: MINOR — apply source-owned vignette and cellular mask stages for non-isomorphic debug-recipe blockers.
// 0.5.1: PATCH — simplify one-off filter styling helpers without changing rendered cells.
// 0.5.0: MINOR — apply one-off content/filter native stages with player-compatible styled-cell parity.
// 0.4.0: MINOR — apply source-only native style stages and residual content stages before compositor rendering.
// 0.3.1: PATCH — consolidate repeated composition metadata population without changing emitted keys.
// 0.3.0: MINOR — route native compositor requests through source-only IR unless auto mode explicitly falls back.
// 0.2.0: MINOR — add request-based render path for native/auto/irResolved composition modes.
// 0.1.0: INIT — implement PlayerRenderBackend over SemanticScene lowering and backend output collection.</CLOG>

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
            NativeContentStage::CellularMask {
                cell_size,
                seed,
                threshold,
            } => apply_cellular_mask_content_stage(&mut staged, *cell_size, *seed, *threshold),
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
            NativeContentStage::RadialMask { soft_edge } => {
                apply_shape_mask_content_stage(&mut staged, *soft_edge, SourceMaskShape::Circle)
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
            NativeStyleStage::Vignette {
                strength,
                edge_color,
                apply_to,
            } => apply_vignette_style_stage(&mut staged, *strength, edge_color, apply_to),
            NativeStyleStage::BracketEmphasis {
                emphasis_color,
                edge_width,
                apply_to,
            } => apply_bracket_emphasis_style_stage(
                &mut staged,
                emphasis_color,
                *edge_width,
                apply_to,
            ),
            NativeStyleStage::DotIndicator {
                active_color,
                inactive_color,
                period,
                apply_to,
            } => apply_dot_indicator_style_stage(
                &mut staged,
                active_color,
                inactive_color,
                *period,
                apply_to,
            ),
            NativeStyleStage::EdgeGrow {
                direction,
                progress,
                edge_color,
                apply_to,
            } => {
                apply_edge_grow_style_stage(&mut staged, direction, *progress, edge_color, apply_to)
            }
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
                bar_color,
                offset,
                width,
                apply_to,
            } => apply_sub_pixel_bar_style_stage(&mut staged, bar_color, *offset, *width, apply_to),
            NativeStyleStage::UnderlineWipe {
                underline_color,
                progress,
                thickness,
                apply_to,
            } => apply_underline_wipe_style_stage(
                &mut staged,
                underline_color,
                *progress,
                *thickness,
                apply_to,
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

fn apply_cellular_mask_content_stage(
    report: &mut PlayerRenderIrReport,
    cell_size: usize,
    seed: usize,
    threshold: f64,
) {
    let report_columns = report_width(report);
    let report_rows = report_height(report);
    let mut rows = dense_rows(report, report_columns, report_rows);
    let cell_size = cell_size.max(1);
    let threshold = threshold.clamp(0.0, 1.0);
    let reveal = report.phase_t.clamp(0.0, 1.0);
    let visible_threshold = (threshold * 0.5 + reveal * 0.75).min(1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                let cell_x = x / cell_size;
                let cell_y = y / cell_size;
                let noise = deterministic_cell_noise(cell_x, cell_y, seed);
                if noise <= visible_threshold {
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
                let band = blinds_band(x, y, width, height, count, orientation);
                if (band + 1) as f64 / count as f64 <= reveal {
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
    let reveal = source_mask_reveal_threshold(report.phase_t, soft_edge);
    let height = rows.len().max(1);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if source_mask_normalized_distance(x, y, width, height, shape) <= reveal {
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
    let reveal = report.phase_t.clamp(0.0, 1.0);
    let height = rows.len().max(1);
    let vertical_cutoff = wipe_cutoff(height, reveal, soft_edge);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count();
        let cutoff = wipe_cutoff(width, reveal, soft_edge);
        *row = row
            .chars()
            .enumerate()
            .map(|(index, glyph)| {
                if wipe_keeps_cell(index, y, width, height, cutoff, vertical_cutoff, direction) {
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
    edge_width: usize,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    for y in 0..height {
        for x in 0..width {
            let on_edge = x < edge_width || x + edge_width >= width;
            let foreground = if on_edge {
                Cow::Borrowed(emphasis_color)
            } else {
                Cow::Owned(lerp_rgba_label(emphasis_color, WHITE_RGBA, 0.7))
            };
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground.as_ref(),
                TRANSPARENT_RGBA,
                &[],
                "FilterBracketEmphasis",
            );
        }
    }
}

fn apply_dot_indicator_style_stage(
    report: &mut PlayerRenderIrReport,
    active_color: &str,
    inactive_color: &str,
    period: usize,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let period = period.max(1);
    let phase_offset =
        ((report.loop_t.unwrap_or(report.phase_t) * period as f64).floor() as usize) % period;
    for y in 0..height {
        for x in 0..width {
            let foreground = if (x + y + phase_offset).is_multiple_of(period) {
                active_color
            } else {
                inactive_color
            };
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground,
                TRANSPARENT_RGBA,
                &[],
                "FilterDotIndicator",
            );
        }
    }
}

fn apply_edge_grow_style_stage(
    report: &mut PlayerRenderIrReport,
    direction: &str,
    progress: f64,
    edge_color: &str,
    apply_to: &str,
) {
    let width = report_width(report).max(1);
    let height = report_height(report).max(1);
    let progress = progress.clamp(0.0, 1.0);
    let limit = match direction {
        "top" | "bottom" => (height as f64 * progress).ceil() as usize,
        _ => (width as f64 * progress).ceil() as usize,
    };
    for y in 0..height {
        for x in 0..width {
            let coordinate = match direction {
                "right" => width.saturating_sub(1).saturating_sub(x),
                "top" => y,
                "bottom" => height.saturating_sub(1).saturating_sub(y),
                _ => x,
            };
            let mix = if coordinate < limit { 0.0 } else { 0.75 };
            let foreground = lerp_rgba_label(edge_color, WHITE_RGBA, mix);
            let background = lerp_rgba_label(edge_color, BLACK_RGBA, 0.7 + mix * 0.2);
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
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

fn apply_sub_pixel_bar_style_stage(
    report: &mut PlayerRenderIrReport,
    bar_color: &str,
    offset: f64,
    bar_width: usize,
    apply_to: &str,
) {
    let width = report_width(report).max(1);
    let height = report_height(report);
    let start = ((width.saturating_sub(1)) as f64 * offset.clamp(0.0, 1.0)).round() as usize;
    let bar_width = bar_width.max(1);
    for y in 0..height {
        for x in 0..width {
            let distance = x.abs_diff(start);
            let mix = (distance as f32 / bar_width as f32).clamp(0.0, 1.0);
            let foreground = lerp_rgba_label(bar_color, WHITE_RGBA, mix * 0.7);
            let background = lerp_rgba_label(bar_color, BLACK_RGBA, 0.5 + mix * 0.45);
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground.as_str(),
                background.as_str(),
                &[],
                "FilterSubPixelBar",
            );
        }
    }
}

fn apply_underline_wipe_style_stage(
    report: &mut PlayerRenderIrReport,
    underline_color: &str,
    progress: f64,
    thickness: usize,
    apply_to: &str,
) {
    let width = report_width(report);
    let height = report_height(report);
    let cutoff = (width as f64 * progress.clamp(0.0, 1.0)).ceil() as usize;
    let thickness = thickness.max(1);
    for y in 0..height {
        for x in 0..width {
            let underlined = x < cutoff && y + thickness >= height;
            let foreground = if underlined {
                Cow::Borrowed(underline_color)
            } else {
                Cow::Owned(lerp_rgba_label(underline_color, WHITE_RGBA, 0.75))
            };
            let modifiers: &[&str] = if underlined { &["underline"] } else { &[] };
            set_report_filter_cell(
                report,
                x,
                y,
                apply_to,
                foreground.as_ref(),
                TRANSPARENT_RGBA,
                modifiers,
                "FilterUnderlineWipe",
            );
        }
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

fn blinds_band(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    count: usize,
    orientation: &str,
) -> usize {
    if orientation == "vertical" {
        (x * count / width).min(count.saturating_sub(1))
    } else {
        (y * count / height).min(count.saturating_sub(1))
    }
}

#[derive(Clone, Copy)]
enum SourceMaskShape {
    Circle,
    Diamond,
}

fn source_mask_reveal_threshold(phase_t: f64, soft_edge: bool) -> f64 {
    let reveal = phase_t.clamp(0.0, 1.0);
    if soft_edge {
        (reveal + 0.06).min(1.0)
    } else {
        reveal
    }
}

fn source_mask_normalized_distance(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    shape: SourceMaskShape,
) -> f64 {
    let origin_x = width.saturating_sub(1) as f64 / 2.0;
    let origin_y = height.saturating_sub(1) as f64 / 2.0;
    let half_width = ((width.saturating_sub(1)) as f64 / 2.0).max(1.0);
    let half_height = ((height.saturating_sub(1)) as f64 / 2.0).max(1.0);
    let dx = (x as f64 - origin_x).abs() / half_width;
    let dy = (y as f64 - origin_y).abs() / half_height;
    match shape {
        SourceMaskShape::Circle => dx.mul_add(dx, dy * dy).sqrt(),
        SourceMaskShape::Diamond => dx + dy,
    }
}

fn deterministic_cell_noise(cell_x: usize, cell_y: usize, seed: usize) -> f64 {
    let mixed = cell_x
        .wrapping_mul(73_856_093)
        .wrapping_add(cell_y.wrapping_mul(19_349_663))
        .wrapping_add(seed.wrapping_mul(83_492_791));
    (mixed % 1000) as f64 / 999.0
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
    let scaled = width as f64 * reveal;
    if soft_edge {
        scaled.round() as usize
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
    use tui_vfx_player::{PlayerRenderIrReport, PlayerStatus};

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
// <VERS>END OF VERSION: 0.8.0</VERS>
