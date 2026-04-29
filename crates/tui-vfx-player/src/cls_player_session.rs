// <FILE>crates/tui-vfx-player/src/cls_player_session.rs</FILE> - <DESC>Stateful player session for lifecycle triggers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: keep trigger latch state outside the recipe document.</WCTX>
// <CLOG>0.1.0: INIT — add dwell trigger sampling, latching, and reset behavior.</CLOG>

use std::collections::BTreeSet;

use tui_vfx_contract::{
    DwellPolicy, LifecyclePhase, PhaseTiming, RecipeDocument, TriggerLatchPolicy, Value,
    ValuePredicate,
};

use crate::{PlayerFrameReport, PlayerSampleRequest, RecipePlayer, resolve_value_source};

/// Stateful player session that owns lifecycle trigger latch state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlayerSession {
    latched_dwell_triggers: BTreeSet<String>,
}

impl PlayerSession {
    /// Create a fresh session with no trigger latch state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all sampled and latched lifecycle state.
    pub fn reset(&mut self) {
        self.latched_dwell_triggers.clear();
    }

    /// Render a recipe frame while updating session-owned trigger latch state.
    pub fn render(
        &mut self,
        player: &RecipePlayer,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> PlayerFrameReport {
        let dwell_terminated = self.sample_dwell_trigger(recipe, request);
        let mut report = player.render_recipe(recipe, request);
        report.dwell_terminated = dwell_terminated;
        report
    }

    fn sample_dwell_trigger(
        &mut self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> bool {
        if request.phase != LifecyclePhase::Dwell {
            return false;
        }
        let Some(policy) = dwell_policy(recipe) else {
            return false;
        };
        let DwellPolicy::Until { trigger, .. } = policy else {
            return false;
        };
        let latch_key = "recipe.dwell".to_string();
        if self.latched_dwell_triggers.contains(&latch_key) {
            return true;
        }
        let fired = predicate_matches(
            resolve_value_source(&trigger.condition.source, &request.signals).as_ref(),
            &trigger.condition.predicate,
        );
        if fired && trigger.latch != TriggerLatchPolicy::None {
            self.latched_dwell_triggers.insert(latch_key);
        }
        fired
    }
}

fn dwell_policy(recipe: &RecipeDocument) -> Option<&DwellPolicy> {
    recipe
        .lifecycle
        .as_ref()?
        .phases
        .iter()
        .find(|phase| phase.phase == LifecyclePhase::Dwell)
        .and_then(|phase| match &phase.timing {
            PhaseTiming::Dwell { policy } => Some(policy),
            PhaseTiming::Fixed { .. } => None,
        })
}

fn predicate_matches(value: Option<&Value>, predicate: &ValuePredicate) -> bool {
    match predicate {
        ValuePredicate::IsTrue => matches!(value, Some(Value::Boolean(true))),
        ValuePredicate::IsFalse => matches!(value, Some(Value::Boolean(false))),
        ValuePredicate::NonZero => numeric_value(value).is_some_and(|value| value != 0.0),
        ValuePredicate::NonEmpty => text_value(value).is_some_and(|value| !value.is_empty()),
        ValuePredicate::Equals { value: expected } => value == Some(expected),
        ValuePredicate::NotEquals { value: expected } => value != Some(expected),
        ValuePredicate::GreaterThan { value: expected } => numeric_value(value)
            .zip(numeric_value(Some(expected)))
            .is_some_and(|(actual, expected)| actual > expected),
        ValuePredicate::LessThan { value: expected } => numeric_value(value)
            .zip(numeric_value(Some(expected)))
            .is_some_and(|(actual, expected)| actual < expected),
        ValuePredicate::Truthy => truthy(value),
    }
}

fn numeric_value(value: Option<&Value>) -> Option<f64> {
    match value? {
        Value::Integer(value) => Some(*value as f64),
        Value::Number(value) | Value::Duration(value) => Some(*value),
        _ => None,
    }
}

fn text_value(value: Option<&Value>) -> Option<&str> {
    match value? {
        Value::Text(value) | Value::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn truthy(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Boolean(value)) => *value,
        Some(Value::Integer(value)) => *value != 0,
        Some(Value::Number(value) | Value::Duration(value)) => *value != 0.0,
        Some(Value::Text(value) | Value::String(value)) => !value.is_empty(),
        Some(Value::Color(_)) => true,
        _ => false,
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_session.rs</FILE> - <DESC>Stateful player session for lifecycle triggers</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
