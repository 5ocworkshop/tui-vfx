// <FILE>crates/tui-vfx-player-ui/src/fnc_player_ui_state_support.rs</FILE> - <DESC>Player UI state helpers</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>K2.13 schema decision burn-down: keep dwell trigger signal discovery exhaustive for sampled-field sources.</WCTX>
// <CLOG>0.1.1: PATCH — recurse into sampled-field coordinate sources during signal discovery.
// 0.1.0: INIT — factor recipe read, phase cycling, and signal-backed dwell trigger discovery.</CLOG>

use std::{fs, path::Path};

use tui_vfx_contract::{
    DwellPolicy, LifecyclePhase, PhaseTiming, RecipeDocument, SignalId, ValueSource,
};

/// Read a canonical v3.1 recipe document from disk.
pub(crate) fn read_recipe(path: &Path) -> Result<RecipeDocument, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("read recipe `{}` failed: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("parse recipe `{}` failed: {error}", path.display()))
}

/// Cycle among the v3.1 lifecycle phases used by the K0 player.
pub(crate) fn cycle_phase(phase: LifecyclePhase, delta: i32) -> LifecyclePhase {
    let phases = [
        LifecyclePhase::Enter,
        LifecyclePhase::Dwell,
        LifecyclePhase::Exit,
    ];
    let current = phases
        .iter()
        .position(|candidate| *candidate == phase)
        .unwrap_or(1);
    let next = (current as i32 + delta).rem_euclid(phases.len() as i32) as usize;
    phases[next]
}

/// Find the canonical signal source backing a dwell-until trigger.
pub(crate) fn dwell_trigger_signal(recipe: &RecipeDocument) -> Option<SignalId> {
    let policy = recipe
        .lifecycle
        .as_ref()?
        .phases
        .iter()
        .find(|phase| phase.phase == LifecyclePhase::Dwell)
        .and_then(|phase| match &phase.timing {
            PhaseTiming::Dwell { policy } => Some(policy),
            PhaseTiming::Fixed { .. } => None,
        })?;
    let DwellPolicy::Until { trigger, .. } = policy else {
        return None;
    };
    signal_from_source(&trigger.condition.source)
}

fn signal_from_source(source: &ValueSource) -> Option<SignalId> {
    match source {
        ValueSource::Signal { id, .. } => Some(id.clone()),
        ValueSource::Map { from, .. } => signal_from_source(from),
        ValueSource::SampledField { x, y, .. } => {
            signal_from_source(x).or_else(|| signal_from_source(y))
        }
        ValueSource::Literal { .. }
        | ValueSource::Parameter { .. }
        | ValueSource::GraphValue { .. } => None,
    }
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_player_ui_state_support.rs</FILE> - <DESC>Player UI state helpers</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
