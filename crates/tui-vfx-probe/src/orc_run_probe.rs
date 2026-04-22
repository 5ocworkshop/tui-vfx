// <FILE>crates/tui-vfx-probe/src/orc_run_probe.rs</FILE> - <DESC>Run one structured pipeline probe</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Sub-plan A Phase A.2.3 — migrate to role-aware render_pipeline_with_spec signature</WCTX>
// <CLOG>0.7.0: MINOR — migrate call to the new `render_pipeline_with_spec` signature. Source roles default to `RoleMap::all_background(w, h)` (probe has no semantic info). Destination is lifted into a `SemanticScene` via `SemanticScene::from_grid_with_default_role`; after the pipeline runs, we extract the grid back via `grid_mut()` clone for downstream probe analysis (probe still speaks `Grid`, not `SemanticScene`).</CLOG>

use serde_json::{Value, json};
use tui_vfx_compositor::pipeline::CompositionSpec;
use tui_vfx_compositor::pipeline::render_pipeline_with_spec;
use tui_vfx_types::{Grid, RoleMap, RoleTag, SemanticScene};

use crate::cls_probe_cell::ProbeCell;
use crate::cls_probe_diff_report::ProbeDiffReport;
use crate::cls_probe_error::ProbeError;
use crate::cls_probe_inspector::ProbeInspector;
use crate::cls_probe_pipeline_inventory::ProbePipelineInventory;
use crate::cls_probe_report::{ProbeFrame, ProbePoint, ProbeReport, ProbeReportSource, ProbeSize};
use crate::cls_probe_request::{ProbeCellSelector, ProbePhase, ProbeRequest};
use crate::cls_probe_scene_spec::ProbeSceneSpec;
use crate::cls_probe_summary::ProbeSummary;
use crate::cls_probe_timing::ProbeTiming;
use crate::cls_probe_widget::ProbeWidget;
use crate::fnc_build_owned_grid::build_owned_grid;
use crate::fnc_build_probe_cell_root_cause::build_probe_cell_root_cause;
use crate::fnc_collect_basic_diagnostics::collect_basic_diagnostics;
use crate::fnc_diff_frames::diff_frames;
use crate::fnc_modifier_names::modifier_names;
use crate::fnc_normalize_color::normalize_color;
use crate::fnc_runtime_context_from_composition::runtime_context_from_composition;
use crate::fnc_select_cells::select_cells;
use crate::fnc_variant_name_from_debug::variant_name_from_debug;

/// Runs one direct-engine probe and returns a structured frame dump.
///
/// The probe validates the input scene, renders the widget into the supplied destination
/// frame using `render_pipeline_with_spec`, then emits a [`ProbeReport`] describing:
///
/// - requested and effective timing
/// - frame and widget geometry
/// - configured pipeline inventory
/// - summary counts for total, non-empty, and modified cells
/// - the caller-selected set of emitted cells (`all`, `non-empty`, or `modified`)
/// - compositor-stage `last_touch` attribution, plus optional trace emission
///
/// Phase 1 compares each rendered widget-local cell against the original source cell to
/// determine whether it counts as `modified`.
pub fn run_probe(
    scene: &ProbeSceneSpec,
    request: &ProbeRequest,
) -> Result<ProbeReport, ProbeError> {
    if !(0.0..=1.0).contains(&request.sample_t) {
        return Err(ProbeError::InvalidRequest(format!(
            "sample_t must be within [0.0, 1.0], got {}",
            request.sample_t
        )));
    }

    let source = build_owned_grid(&scene.source)?;
    let destination_grid = build_owned_grid(&scene.destination)?;
    let widget_width = scene.source.width as usize;
    let widget_height = scene.source.height as usize;
    let abs_origin_x = scene.widget_offset.x as usize;
    let abs_origin_y = scene.widget_offset.y as usize;

    if abs_origin_x + widget_width > destination_grid.width()
        || abs_origin_y + widget_height > destination_grid.height()
    {
        return Err(ProbeError::InvalidScene(format!(
            "widget area {}x{} at ({}, {}) exceeds destination frame {}x{}",
            widget_width,
            widget_height,
            abs_origin_x,
            abs_origin_y,
            destination_grid.width(),
            destination_grid.height()
        )));
    }

    let mut composition = scene.composition.clone();
    composition.t = request.sample_t;
    composition.phase = Some(request.phase.to_mixed_phase());
    let runtime = runtime_context_from_composition(&composition);

    let mut inspector = ProbeInspector::default();
    // Probe has no semantic role information — pass all-Background source
    // roles and wrap the destination grid in a SemanticScene so the
    // role-aware pipeline signature is satisfied. A.2 uses
    // RoleMap::all_background everywhere as the no-info default; real
    // widget/recipe role-tagging is Sub-plan C's work.
    let source_w = source.width() as u16;
    let source_h = source.height() as u16;
    let source_roles = RoleMap::all_background(source_w, source_h);
    let mut destination_scene =
        SemanticScene::from_grid_with_default_role(destination_grid, RoleTag::Background);
    render_pipeline_with_spec(
        &source,
        &source_roles,
        &mut destination_scene,
        widget_width,
        widget_height,
        abs_origin_x,
        abs_origin_y,
        &composition,
        Some(&mut inspector),
    );
    // Recover the grid for downstream probe analysis.
    let destination = destination_scene.grid().clone();

    let mut all_cells = Vec::with_capacity(widget_width * widget_height);
    let mut non_empty_cells = 0usize;
    let mut modified_cells = 0usize;

    for y in 0..widget_height {
        for x in 0..widget_width {
            let source_cell = source.get(x, y).copied().ok_or_else(|| {
                ProbeError::InvalidScene(format!("missing source cell at ({x}, {y})"))
            })?;
            let final_cell = destination
                .get(abs_origin_x + x, abs_origin_y + y)
                .copied()
                .ok_or_else(|| {
                    ProbeError::InvalidScene(format!(
                        "missing destination cell at ({}, {})",
                        abs_origin_x + x,
                        abs_origin_y + y
                    ))
                })?;
            let is_non_empty = !final_cell.is_empty();
            let is_modified = final_cell != source_cell;
            if is_non_empty {
                non_empty_cells += 1;
            }
            if is_modified {
                modified_cells += 1;
            }

            let trace = if request.with_causation {
                let mut trace = inspector.trace_for(x as u16, y as u16);
                enrich_trace_events(&mut trace, &composition, runtime.as_ref());
                trace
            } else {
                Vec::new()
            };

            let mut probe_cell = ProbeCell {
                abs: ProbePoint {
                    x: (abs_origin_x + x) as u16,
                    y: (abs_origin_y + y) as u16,
                },
                widget_local: ProbePoint {
                    x: x as u16,
                    y: y as u16,
                },
                ch: final_cell.ch,
                fg: normalize_color(final_cell.fg),
                bg: normalize_color(final_cell.bg),
                modifiers: modifier_names(final_cell.mods),
                last_touch: inspector.last_touch_for(x as u16, y as u16),
                trace,
                root_cause: None,
            };
            probe_cell.root_cause = build_probe_cell_root_cause(&probe_cell, runtime.as_ref());

            all_cells.push((probe_cell, is_non_empty, is_modified));
        }
    }

    let diagnostics = collect_basic_diagnostics(&ProbeReport {
        schema_version: "0.1.0".to_string(),
        kind: "frame_dump".to_string(),
        source: ProbeReportSource {
            input_kind: "probe_scene_spec".to_string(),
        },
        request: request.clone(),
        timing: ProbeTiming {
            requested_phase: request.phase,
            requested_t: request.sample_t,
            effective_phase: request.phase,
            effective_t: request.sample_t,
            tick_ms: None,
        },
        frame: ProbeFrame {
            size: ProbeSize {
                width: scene.destination.width,
                height: scene.destination.height,
            },
        },
        widget: ProbeWidget {
            abs_origin: ProbePoint {
                x: scene.widget_offset.x,
                y: scene.widget_offset.y,
            },
            size: ProbeSize {
                width: scene.source.width,
                height: scene.source.height,
            },
        },
        pipeline: build_pipeline_inventory(&composition),
        runtime: runtime.clone(),
        summary: ProbeSummary {
            total_cells: widget_width * widget_height,
            non_empty_cells,
            modified_cells,
        },
        diagnostics: Vec::new(),
        cells: all_cells.iter().map(|(cell, _, _)| cell.clone()).collect(),
    });

    Ok(ProbeReport {
        schema_version: "0.1.0".to_string(),
        kind: "frame_dump".to_string(),
        source: ProbeReportSource {
            input_kind: "probe_scene_spec".to_string(),
        },
        request: request.clone(),
        timing: ProbeTiming {
            requested_phase: request.phase,
            requested_t: request.sample_t,
            effective_phase: request.phase,
            effective_t: request.sample_t,
            tick_ms: None,
        },
        frame: ProbeFrame {
            size: ProbeSize {
                width: scene.destination.width,
                height: scene.destination.height,
            },
        },
        widget: ProbeWidget {
            abs_origin: ProbePoint {
                x: scene.widget_offset.x,
                y: scene.widget_offset.y,
            },
            size: ProbeSize {
                width: scene.source.width,
                height: scene.source.height,
            },
        },
        pipeline: build_pipeline_inventory(&composition),
        runtime,
        summary: ProbeSummary {
            total_cells: widget_width * widget_height,
            non_empty_cells,
            modified_cells,
        },
        diagnostics,
        cells: select_cells(all_cells, request.cells),
    })
}

fn build_pipeline_inventory(composition: &CompositionSpec) -> ProbePipelineInventory {
    ProbePipelineInventory {
        sampler: composition
            .sampler_spec
            .as_ref()
            .map(|sampler| format!("{sampler:?}")),
        sampler_effects: composition
            .sampler_spec
            .iter()
            .map(|sampler| format!("{}#1", variant_name_from_debug(sampler)))
            .collect(),
        mask_count: composition.masks.len(),
        mask_effects: composition
            .masks
            .iter()
            .enumerate()
            .map(|(index, mask)| format!("{}#{}", variant_name_from_debug(mask), index + 1))
            .collect(),
        filter_count: composition.filters.len(),
        filter_effects: composition
            .filters
            .iter()
            .enumerate()
            .map(|(index, filter)| format!("{}#{}", variant_name_from_debug(filter), index + 1))
            .collect(),
        shader_count: composition.shader_layers.len(),
        shader_effects: composition
            .shader_layers
            .iter()
            .enumerate()
            .map(|(index, layer)| {
                format!("{}#{}", variant_name_from_debug(&layer.shader), index + 1)
            })
            .collect(),
        shader_families: composition
            .v3_shader_families()
            .into_iter()
            .map(|family| family.family_label().to_string())
            .collect(),
        style_count: 0,
        style_effects: Vec::new(),
        style_effect_families: Vec::new(),
        content_count: 0,
        content_effects: Vec::new(),
    }
}

fn enrich_trace_events(
    trace: &mut [crate::ProbeTraceEvent],
    composition: &CompositionSpec,
    runtime: Option<&crate::ProbeRuntimeContext>,
) {
    for event in trace {
        let (params, notes) =
            trace_event_details(&event.stage, event.effect.as_deref(), composition, runtime);
        event.params = params;
        if !notes.is_empty() {
            event.notes = notes;
        }
    }
}

fn trace_event_details(
    stage: &str,
    effect_name: Option<&str>,
    composition: &CompositionSpec,
    runtime: Option<&crate::ProbeRuntimeContext>,
) -> (Option<Value>, Vec<String>) {
    let Some(effect_name) = effect_name else {
        return (None, Vec::new());
    };
    let normalized_effect_name = effect_name.split('#').next().unwrap_or(effect_name);

    match stage {
        "sampler" => match composition.sampler_spec.as_ref() {
            Some(sampler) if variant_name_from_debug(sampler) == normalized_effect_name => {
                (serde_json::to_value(sampler).ok(), Vec::new())
            }
            _ => (None, Vec::new()),
        },
        "mask" => serialize_matches(
            composition
                .masks
                .iter()
                .filter(|mask| variant_name_from_debug(*mask) == normalized_effect_name)
                .map(|mask| serde_json::to_value(mask).unwrap_or(Value::Null))
                .collect(),
        ),
        "filter" => serialize_matches(
            composition
                .filters
                .iter()
                .filter(|filter| variant_name_from_debug(*filter) == normalized_effect_name)
                .map(|filter| serde_json::to_value(filter).unwrap_or(Value::Null))
                .collect(),
        ),
        "shader" => {
            let matches = composition
                .shader_layers
                .iter()
                .filter(|layer| variant_name_from_debug(&layer.shader) == normalized_effect_name)
                .map(|layer| {
                    let ctx = tui_vfx_style::traits::ShaderContext::new(
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        composition.loop_t.unwrap_or(composition.t),
                        composition.phase,
                        Some(composition.runtime_params.clone().into()),
                    );
                    json!({
                        "region": layer.region,
                        "shader": &layer.shader,
                        "binding_requests": layer.shader.runtime_binding_requests(),
                        "binding_resolutions": layer.shader.runtime_binding_resolutions(&ctx),
                    })
                })
                .collect::<Vec<_>>();
            serialize_matches(matches)
        }
        _ => {
            let _ = runtime;
            (None, Vec::new())
        }
    }
}

fn serialize_matches(matches: Vec<Value>) -> (Option<Value>, Vec<String>) {
    match matches.len() {
        0 => (None, Vec::new()),
        1 => (matches.into_iter().next(), Vec::new()),
        _ => (
            Some(Value::Array(matches)),
            vec!["multiple configured instances matched this effect name".to_string()],
        ),
    }
}

/// Compares two phase-local samples from the same scene and returns only the changed cells.
pub fn run_probe_diff(
    scene: &ProbeSceneSpec,
    phase: ProbePhase,
    from_t: f64,
    to_t: f64,
    with_causation: bool,
) -> Result<ProbeDiffReport, ProbeError> {
    let from_report = run_probe(
        scene,
        &ProbeRequest {
            phase,
            sample_t: from_t,
            cells: ProbeCellSelector::All,
            with_causation,
        },
    )?;
    let to_report = run_probe(
        scene,
        &ProbeRequest {
            phase,
            sample_t: to_t,
            cells: ProbeCellSelector::All,
            with_causation,
        },
    )?;
    let cells = diff_frames(&from_report, &to_report);

    Ok(ProbeDiffReport {
        schema_version: "0.1.0".to_string(),
        kind: "frame_diff".to_string(),
        source: ProbeReportSource {
            input_kind: "probe_scene_spec".to_string(),
        },
        phase,
        from_t,
        to_t,
        frame: to_report.frame.clone(),
        widget: to_report.widget.clone(),
        pipeline: to_report.pipeline.clone(),
        changed_cells_count: cells.len(),
        cells,
    })
}

// <FILE>crates/tui-vfx-probe/src/orc_run_probe.rs</FILE> - <DESC>Run one structured pipeline probe</DESC>
// <VERS>END OF VERSION: 0.7.0</VERS>
