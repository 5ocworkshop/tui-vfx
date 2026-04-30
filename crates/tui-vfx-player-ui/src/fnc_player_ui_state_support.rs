// <FILE>crates/tui-vfx-player-ui/src/fnc_player_ui_state_support.rs</FILE> - <DESC>Player UI state helpers</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>K2.13 schema decision burn-down: keep dwell trigger signal discovery exhaustive for sampled-field sources.</WCTX>
// <CLOG>0.1.1: PATCH — recurse into sampled-field coordinate sources during signal discovery.
// 0.1.0: INIT — factor recipe read, phase cycling, and signal-backed dwell trigger discovery.</CLOG>

use std::{fs, path::Path};

use tui_vfx_contract::{
    DwellPolicy, LifecyclePhase, PhaseTiming, RecipeDocument, SignalId, Value, ValueKind,
    ValuePredicate, ValueSource,
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

/// Find the canonical signal and typed value that fires a signal-backed dwell-until trigger.
pub(crate) fn dwell_trigger_fire_value(recipe: &RecipeDocument) -> Option<(SignalId, Value)> {
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
    let signal = signal_from_source(&trigger.condition.source)?;
    let signal_kind = recipe.graph.signals.get(&signal)?.value.kind;
    let value = fire_value_for_predicate(&trigger.condition.predicate, signal_kind)?;
    Some((signal, value))
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

fn fire_value_for_predicate(predicate: &ValuePredicate, kind: ValueKind) -> Option<Value> {
    match predicate {
        ValuePredicate::IsTrue => (kind == ValueKind::Boolean).then_some(Value::Boolean(true)),
        ValuePredicate::IsFalse => (kind == ValueKind::Boolean).then_some(Value::Boolean(false)),
        ValuePredicate::NonZero | ValuePredicate::Truthy => truthy_value(kind),
        ValuePredicate::NonEmpty => match kind {
            ValueKind::String => Some(Value::String("trigger".to_string())),
            ValueKind::Text => Some(Value::Text("trigger".to_string())),
            _ => None,
        },
        ValuePredicate::Equals { value } => (value.kind() == kind).then(|| value.clone()),
        ValuePredicate::NotEquals { value } => not_equal_value(kind, value),
        ValuePredicate::GreaterThan { value } => offset_numeric_value(kind, value, 1.0),
        ValuePredicate::LessThan { value } => offset_numeric_value(kind, value, -1.0),
    }
}

fn truthy_value(kind: ValueKind) -> Option<Value> {
    match kind {
        ValueKind::Boolean => Some(Value::Boolean(true)),
        ValueKind::Integer => Some(Value::Integer(1)),
        ValueKind::Number => Some(Value::Number(1.0)),
        ValueKind::Duration => Some(Value::Duration(1.0)),
        ValueKind::String => Some(Value::String("trigger".to_string())),
        ValueKind::Text => Some(Value::Text("trigger".to_string())),
        _ => None,
    }
}

fn not_equal_value(kind: ValueKind, expected: &Value) -> Option<Value> {
    match (kind, expected) {
        (ValueKind::Boolean, Value::Boolean(value)) => Some(Value::Boolean(!value)),
        (ValueKind::Integer, Value::Integer(value)) => Some(Value::Integer(value + 1)),
        (ValueKind::Number, Value::Number(value)) => Some(Value::Number(value + 1.0)),
        (ValueKind::Duration, Value::Duration(value)) => Some(Value::Duration(value + 1.0)),
        (ValueKind::String, Value::String(value)) => {
            Some(Value::String(format!("{value}-trigger")))
        }
        (ValueKind::Text, Value::Text(value)) => Some(Value::Text(format!("{value}-trigger"))),
        _ => truthy_value(kind),
    }
}

fn offset_numeric_value(kind: ValueKind, expected: &Value, delta: f64) -> Option<Value> {
    match (kind, expected) {
        (ValueKind::Integer, Value::Integer(value)) => {
            Some(Value::Integer((*value as f64 + delta).round() as i64))
        }
        (ValueKind::Number, Value::Number(value)) => Some(Value::Number(value + delta)),
        (ValueKind::Duration, Value::Duration(value)) => {
            Some(Value::Duration((value + delta).max(0.0)))
        }
        _ => None,
    }
}

// <FILE>crates/tui-vfx-player-ui/src/fnc_player_ui_state_support.rs</FILE> - <DESC>Player UI state helpers</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
