// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs</FILE> - <DESC>Render player IR through the compositor backend</DESC>
// <VERS>VERSION: 0.38.0</VERS>
// <WCTX>Native compositor source isolation: render native requests from source-only IR, including backend-owned content/style/filter stages, and keep IR-resolved compatibility separate.</WCTX>
// <CLOG>0.38.0: MINOR — remove backend-owned focused-row-gradient style rendering after style.spatial moved to compositor ShaderLayerSpec.
// 0.37.0: MINOR — remove backend-owned diffusion shader rendering after diffusion moved to compositor ShaderLayerSpec.
// 0.36.0: MINOR — remove backend-owned barber-pole shader rendering after barberPole moved to compositor ShaderLayerSpec.
// 0.35.0: MINOR — remove backend-owned wayfinding-node shader rendering after wayfindingNode moved to compositor ShaderLayerSpec.
// 0.34.0: MINOR — remove backend-owned focus-field shader rendering after focusField moved to compositor ShaderLayerSpec.
// 0.33.0: MINOR — remove backend-owned glisten-band shader rendering after glistenBand moved to compositor ShaderLayerSpec.
// 0.32.0: MINOR — remove backend-owned highlighter shader rendering after highlighter moved to compositor ShaderLayerSpec.
// 0.31.0: MINOR — remove backend-owned radar shader rendering after radar moved to compositor ShaderLayerSpec.
// 0.30.0: MINOR — remove backend-owned vignette/hoverBar style rendering after those filters use compositor FilterSpec paths.
// 0.29.0: MINOR — remove backend-owned matrixRain style-stage rendering after matrixRain moved to compositor FilterSpec.
// 0.28.0: MINOR — remove backend-owned subPixelBar style-stage rendering after subPixelBar moved to compositor FilterSpec.
// 0.27.0: PATCH — remove final backend-owned wipe/pathReveal mask source-stage rendering after pathReveal moved to compositor MaskSpec.
// 0.26.0: remove backend-owned blinds mask source-stage rendering after mask.blinds moved to compositor MaskSpec.
// 0.25.0: remove backend-owned iris mask source-stage rendering after mask.iris moved to compositor MaskSpec.
// 0.24.0: remove backend-owned dissolve mask source-stage rendering after mask.dissolve moved to compositor MaskSpec.
// 0.23.0: remove backend-owned diamond mask source-stage rendering after mask.diamond moved to compositor MaskSpec.
// 0.22.0: remove backend-owned radial mask source-stage rendering after mask.radial moved to compositor MaskSpec.
// 0.21.0: remove backend-owned cellular mask source-stage rendering after mask.cellular moved to compositor MaskSpec.
// 0.20.0: brighten cell-scoped focused-row gradients at the targeted cell.
// 0.19.0: MINOR — render cell-scoped style.spatial focused row gradients.
// 0.18.0: MINOR — render style.glitch native stage.
// 0.17.0: MINOR — render style.rainbow native stage.
// 0.16.0: MINOR — render shader highlighter/focusField applyTo targets.
// 0.15.0: MINOR — add horizontal center wipe rendering while preserving active filter/sampler patches.</CLOG>

use std::collections::BTreeMap;

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
        }
    }
    for stage in &lowered_spec.style_stages {
        match stage {
            NativeStyleStage::Rainbow { rotation_speed } => {
                apply_rainbow_style_stage(&mut staged, *rotation_speed)
            }
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

const DEFAULT_FOREGROUND: &str = "defaultForeground";
const TRANSPARENT_RGBA: &str = "transparent";
fn rgba_label(r: u8, g: u8, b: u8, a: u8) -> String {
    format!("rgba({r},{g},{b},{a})")
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
// <VERS>END OF VERSION: 0.32.0</VERS>
