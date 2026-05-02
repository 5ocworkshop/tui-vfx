// <FILE>crates/tui-vfx-compost/src/render/fnc_resolve_element_graph_sample.rs</FILE> - <DESC>Derive element-local graph sample timing</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Element graph bindings consume timing envelopes as local progress without changing recipe-level clocks.</WCTX>
// <CLOG>0.1.0: INIT — derive element-local enter/exit graph progress from the parent sample context.</CLOG>

use std::f64::consts::PI;

use tui_vfx_contract::{LifecyclePhase, RecipeElementGraphTiming, RecipeSceneElement};

use crate::render::SampleContext;

pub(crate) fn resolve_element_graph_sample(
    element: &RecipeSceneElement,
    parent_sample: &SampleContext,
) -> SampleContext {
    let Some(timing) = element
        .graph_binding
        .as_ref()
        .and_then(|binding| binding.timing.as_ref())
    else {
        return *parent_sample;
    };

    let Some(local_progress) = local_graph_progress(timing, parent_sample) else {
        return *parent_sample;
    };

    let mut graph_sample = *parent_sample;
    graph_sample.phase_t = local_progress;
    graph_sample.loop_t = Some(local_progress);
    graph_sample
}

fn local_graph_progress(
    timing: &RecipeElementGraphTiming,
    parent_sample: &SampleContext,
) -> Option<f64> {
    match parent_sample.lifecycle_phase {
        Some(LifecyclePhase::Enter) => progress_from_elapsed(
            parent_sample
                .phase_time_ms
                .or(parent_sample.absolute_time_ms),
            timing.enter_offset_ms.unwrap_or(0),
            timing.enter_ms?,
            timing.enter_ease.as_deref(),
        ),
        Some(LifecyclePhase::Exit) => progress_from_elapsed(
            parent_sample
                .phase_time_ms
                .or(parent_sample.absolute_time_ms),
            timing.exit_offset_ms.unwrap_or(0),
            timing.exit_ms?,
            timing.exit_ease.as_deref(),
        ),
        Some(LifecyclePhase::Dwell) | None => None,
    }
}

fn progress_from_elapsed(
    elapsed_ms: Option<u64>,
    offset_ms: u64,
    duration_ms: u64,
    ease: Option<&str>,
) -> Option<f64> {
    let elapsed_ms = elapsed_ms?;
    let raw_progress = if duration_ms == 0 {
        f64::from(elapsed_ms >= offset_ms)
    } else {
        elapsed_ms.saturating_sub(offset_ms) as f64 / duration_ms as f64
    };
    Some(apply_ease(raw_progress.clamp(0.0, 1.0), ease))
}

fn apply_ease(t: f64, ease: Option<&str>) -> f64 {
    match ease.map(normalize_ease_name).as_deref() {
        Some("inquad") | Some("quadin") => t * t,
        Some("outquad") | Some("quadout") => 1.0 - (1.0 - t).powi(2),
        Some("inoutquad") | Some("quadinout") => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
            }
        }
        Some("incubic") | Some("cubicin") => t.powi(3),
        Some("outcubic") | Some("cubicout") => 1.0 - (1.0 - t).powi(3),
        Some("inoutcubic") | Some("cubicinout") => {
            if t < 0.5 {
                4.0 * t.powi(3)
            } else {
                1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
            }
        }
        Some("insine") | Some("sinein") => 1.0 - (t * PI / 2.0).cos(),
        Some("outsine") | Some("sineout") => (t * PI / 2.0).sin(),
        Some("inoutsine") | Some("sineinout") => -((PI * t).cos() - 1.0) / 2.0,
        _ => t,
    }
    .clamp(0.0, 1.0)
}

fn normalize_ease_name(name: &str) -> String {
    name.chars()
        .filter(|ch| !matches!(ch, '-' | '_' | ' '))
        .flat_map(char::to_lowercase)
        .collect()
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_resolve_element_graph_sample.rs</FILE> - <DESC>Derive element-local graph sample timing</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
