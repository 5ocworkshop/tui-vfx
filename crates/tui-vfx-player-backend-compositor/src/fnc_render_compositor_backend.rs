// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs</FILE> - <DESC>Render player IR through the compositor backend</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Native compositor source isolation: render native requests from source-only IR and keep IR-resolved compatibility separate.</WCTX>
// <CLOG>0.3.1: PATCH — consolidate repeated composition metadata population without changing emitted keys.
// 0.3.0: MINOR — route native compositor requests through source-only IR unless auto mode explicitly falls back.
// 0.2.0: MINOR — add request-based render path for native/auto/irResolved composition modes.
// 0.1.0: INIT — implement PlayerRenderBackend over SemanticScene lowering and backend output collection.</CLOG>

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
        LoweredCompositionSpec, NativeContentStage, TypewriterCursorWake,
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
    if lowered_spec.content_stages.is_empty() || lowered_spec.evidence.fallback_used {
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
    let width = report
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
        );
    let height = report.height.max(report.rows.len()).max(
        report
            .styled_cells
            .iter()
            .map(|cell| cell.y + 1)
            .max()
            .unwrap_or(0),
    );
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
// <VERS>END OF VERSION: 0.3.1</VERS>
