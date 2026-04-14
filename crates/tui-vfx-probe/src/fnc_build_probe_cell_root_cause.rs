// <FILE>crates/tui-vfx-probe/src/fnc_build_probe_cell_root_cause.rs</FILE> - <DESC>Build a cell-centric root-cause explainer from trace data</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Cell-centric root-cause explainers for probe debugging</WCTX>
// <CLOG>NEW: Synthesize a concise per-cell root-cause summary from trace events and runtime binding resolutions so users can debug one cell without replaying the whole pipeline manually</CLOG>

use std::collections::BTreeSet;

use tui_vfx_style::traits::ShaderRuntimeBindingResolution;

use crate::{ProbeCell, ProbeCellRootCause, ProbeCellStageCause, ProbeRuntimeContext, ProbeTraceEvent};

pub fn build_probe_cell_root_cause(
    cell: &ProbeCell,
    runtime: Option<&ProbeRuntimeContext>,
) -> Option<ProbeCellRootCause> {
    if cell.trace.is_empty() {
        return None;
    }

    let dominant_stage = cell.trace.last().map(|event| event.stage.clone());
    let changed_stages = cell
        .trace
        .iter()
        .map(|event| event.stage.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let hidden_by_masks = cell
        .trace
        .iter()
        .filter(|event| event.stage == "mask" && event.visible == Some(false))
        .filter_map(|event| event.effect.clone())
        .collect::<Vec<_>>();
    let sampled_from = cell
        .trace
        .iter()
        .rev()
        .find_map(|event| event.sampled_from);
    let bindings = relevant_bindings(&cell.trace, runtime);
    let stage_causes = cell
        .trace
        .iter()
        .map(|event| ProbeCellStageCause {
            stage: event.stage.clone(),
            effect: event.effect.clone(),
            summary: event_summary(event),
        })
        .collect::<Vec<_>>();

    Some(ProbeCellRootCause {
        summary: overall_summary(&cell.trace, &hidden_by_masks),
        dominant_stage,
        changed_stages,
        hidden_by_masks,
        sampled_from,
        bindings,
        stage_causes,
    })
}

fn relevant_bindings(
    trace: &[ProbeTraceEvent],
    runtime: Option<&ProbeRuntimeContext>,
) -> Vec<ShaderRuntimeBindingResolution> {
    let Some(runtime) = runtime else {
        return Vec::new();
    };
    let saw_shader = trace.iter().any(|event| event.stage == "shader");
    if !saw_shader {
        return Vec::new();
    }
    runtime.binding_resolutions.clone()
}

fn overall_summary(trace: &[ProbeTraceEvent], hidden_by_masks: &[String]) -> String {
    if !hidden_by_masks.is_empty() {
        return format!(
            "Cell was clipped or hidden by mask stage(s): {}",
            hidden_by_masks.join(", ")
        );
    }

    match trace.last() {
        Some(event) if event.stage == "filter" => "Final appearance was dominated by filter output".to_string(),
        Some(event) if event.stage == "shader" => "Final appearance was dominated by shader styling".to_string(),
        Some(event) if event.stage == "style" => "Final appearance was dominated by recipe style interpolation".to_string(),
        Some(event) if event.stage == "content" => "Rendered content differs because the recipe content effect transformed the text".to_string(),
        Some(event) if event.stage == "sampler" => "Rendered cell content was sampled from a different source coordinate".to_string(),
        Some(_) => "Cell was modified by one or more pipeline stages".to_string(),
        None => "Cell was not modified".to_string(),
    }
}

fn event_summary(event: &ProbeTraceEvent) -> String {
    match event.stage.as_str() {
        "sampler" => {
            if let Some(point) = event.sampled_from {
                format!("Sampler read from source cell ({}, {})", point.x, point.y)
            } else {
                "Sampler stage ran without a concrete source coordinate".to_string()
            }
        }
        "mask" => match event.visible {
            Some(true) => "Mask allowed the cell to remain visible".to_string(),
            Some(false) => "Mask hid the cell at this stage".to_string(),
            None => "Mask stage evaluated visibility".to_string(),
        },
        "content" | "style" | "shader" | "filter" => {
            let mut changes = Vec::new();
            if let (Some(before), Some(after)) = (&event.before, &event.after) {
                if before.ch != after.ch {
                    changes.push("character");
                }
                if before.fg != after.fg {
                    changes.push("foreground");
                }
                if before.bg != after.bg {
                    changes.push("background");
                }
                if before.modifiers != after.modifiers {
                    changes.push("modifiers");
                }
            }
            if changes.is_empty() {
                format!("{} stage touched the cell", event.stage)
            } else {
                format!("{} changed {}", event.stage, changes.join(", "))
            }
        }
        _ => format!("{} stage touched the cell", event.stage),
    }
}

// <FILE>crates/tui-vfx-probe/src/fnc_build_probe_cell_root_cause.rs</FILE> - <DESC>Build a cell-centric root-cause explainer from trace data</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
