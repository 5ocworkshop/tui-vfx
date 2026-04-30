// <FILE>crates/tui-vfx-player/src/fnc_apply_preview_loopbacks.rs</FILE> - <DESC>Apply signal preview loopbacks to player sample requests</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>v3.1 parity: let deterministic player samples honor host-signal loopback/demo providers when no host value is supplied.</WCTX>
// <CLOG>0.2.0: MINOR — adapt authored loopback static/signal evaluation through mixed-signals while preserving host-wins semantics.
// 0.1.0: INIT — hydrate missing request signals from previewLoopback or signal defaults.</CLOG>

use mixed_signals::{traits::SignalContext, types::SignalSpec};
use tui_vfx_contract::{
    PreviewLoopbackSpec, RecipeDocument, StructuredValue, Value, ValueKind, ValueSpec,
};

use crate::{PlayerSampleRequest, PlayerWarning};

/// Player-local result of applying authored preview loopbacks to one sample request.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AuthoredLoopbackResolution {
    /// Request with any allowed authored loopback values merged into missing host signals.
    pub request: PlayerSampleRequest,
    /// Signal ids whose values were supplied by authored loopback/default data.
    pub fired_keys: Vec<String>,
    /// Signal ids that would have used authored loopback/default data in strict modes.
    pub suppressed_keys: Vec<String>,
}

impl AuthoredLoopbackResolution {
    /// Non-fatal diagnostics for strict loopback modes that intentionally suppress merging.
    pub fn warnings(&self) -> Vec<PlayerWarning> {
        self.suppressed_keys
            .iter()
            .map(|key| {
                PlayerWarning::new(
                    "authoredLoopbackSuppressed",
                    format!("$.graph.signals.{key}.previewLoopback"),
                    format!("authored loopback for signal `{key}` was available but not merged"),
                    Some(
                        "Use permissive/warn loopback strictness for preview playback, or provide a host signal value.",
                    ),
                )
            })
            .collect()
    }
}

/// Resolve authored preview loopbacks while preserving fired/suppressed signal evidence.
pub(crate) fn resolve_preview_loopbacks(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
) -> AuthoredLoopbackResolution {
    let mut hydrated = request.clone();
    let mut fired_keys = Vec::new();
    let mut suppressed_keys = Vec::new();
    for (id, signal) in &recipe.graph.signals {
        if hydrated.signals.contains_key(id) {
            continue;
        }
        let preview_value = signal
            .preview_loopback
            .as_ref()
            .and_then(|loopback| sample_preview_loopback(loopback, &signal.value, request));
        let authored_value = preview_value
            .clone()
            .or_else(|| signal.value.default.clone());
        if request.loopback_strictness.suppresses_merge() {
            if preview_value.is_some() {
                suppressed_keys.push(id.as_str().to_string());
            }
            continue;
        }
        if let Some(value) = authored_value {
            hydrated.signals.insert(id.clone(), value);
            if preview_value.is_some() {
                fired_keys.push(id.as_str().to_string());
            }
        }
    }
    AuthoredLoopbackResolution {
        request: hydrated,
        fired_keys,
        suppressed_keys,
    }
}

fn sample_preview_loopback(
    loopback: &PreviewLoopbackSpec,
    value_spec: &ValueSpec,
    request: &PlayerSampleRequest,
) -> Option<Value> {
    match loopback {
        PreviewLoopbackSpec::Literal { value } => Some(value.clone()),
        PreviewLoopbackSpec::NumericStatic { value } => numeric_preview_value(*value, value_spec),
        PreviewLoopbackSpec::NumericSignal {
            expression,
            fallback,
        } => sample_authored_signal_expression(expression, value_spec, request)
            .or_else(|| fallback.clone()),
        PreviewLoopbackSpec::NumericRamp {
            start, end, repeat, ..
        } => numeric_preview_value(
            interpolate(*start, *end, preview_t(request, *repeat)),
            value_spec,
        ),
        PreviewLoopbackSpec::SignalExpression {
            expression,
            fallback,
        } => sample_authored_signal_expression(expression, value_spec, request)
            .or_else(|| sample_structured_signal_expression(expression, value_spec, request))
            .or_else(|| fallback.clone()),
    }
}

fn sample_authored_signal_expression(
    expression: &StructuredValue,
    value_spec: &ValueSpec,
    request: &PlayerSampleRequest,
) -> Option<Value> {
    let signal_spec: SignalSpec = serde_json::from_value(structured_to_json(expression)).ok()?;
    let signal_or_float = mixed_signals::types::SignalOrFloat::from(signal_spec);
    let signal_context = signal_context_for_request(request);
    let raw = signal_or_float
        .evaluate(elapsed_t_seconds(request), &signal_context)
        .ok()? as f64;
    numeric_preview_value(raw, value_spec)
}

fn sample_structured_signal_expression(
    expression: &StructuredValue,
    value_spec: &ValueSpec,
    request: &PlayerSampleRequest,
) -> Option<Value> {
    let StructuredValue::Object(object) = expression else {
        return None;
    };
    if object.get("type").and_then(structured_str) != Some("ramp") {
        return None;
    }
    let start = object.get("start").and_then(structured_number)?;
    let end = object.get("end").and_then(structured_number)?;
    let duration = object
        .get("duration")
        .and_then(structured_number)
        .filter(|duration| duration.is_finite() && *duration > 0.0);
    let t = duration
        .map(|duration| (elapsed_t_seconds(request) / duration).clamp(0.0, 1.0))
        .unwrap_or_else(|| preview_t(request, false));
    numeric_preview_value(interpolate(start, end, t), value_spec)
}

fn signal_context_for_request(request: &PlayerSampleRequest) -> SignalContext {
    SignalContext {
        width: request.width.unwrap_or_default() as u16,
        height: request.height.unwrap_or_default() as u16,
        phase_t: Some(request.phase_t),
        loop_t: request.loop_t,
        absolute_t: request.absolute_t_ms,
        ..Default::default()
    }
}

fn elapsed_t_seconds(request: &PlayerSampleRequest) -> f64 {
    request
        .absolute_t_ms
        .map(|absolute_t_ms| absolute_t_ms.max(0.0) / 1000.0)
        .unwrap_or_else(|| preview_t(request, false))
}

fn preview_t(request: &PlayerSampleRequest, repeat: bool) -> f64 {
    let t = request.loop_t.unwrap_or(request.phase_t);
    if repeat {
        t.rem_euclid(1.0)
    } else {
        t.clamp(0.0, 1.0)
    }
}

fn interpolate(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn numeric_preview_value(value: f64, value_spec: &ValueSpec) -> Option<Value> {
    if !value.is_finite() {
        return None;
    }
    match value_spec.kind {
        ValueKind::Integer => {
            let clamped = clamp_numeric_to_spec(value.round(), value_spec, 0.0, u16::MAX as f64);
            Some(Value::Integer(clamped as i64))
        }
        ValueKind::Number => Some(Value::Number(clamp_numeric_to_spec(
            value,
            value_spec,
            f64::NEG_INFINITY,
            f64::INFINITY,
        ))),
        ValueKind::Duration => Some(Value::Duration(
            clamp_numeric_to_spec(value, value_spec, 0.0, f64::INFINITY).max(0.0),
        )),
        _ => None,
    }
}

fn clamp_numeric_to_spec(
    value: f64,
    value_spec: &ValueSpec,
    default_min: f64,
    default_max: f64,
) -> f64 {
    let min = value_spec
        .range
        .and_then(|range| range.min)
        .unwrap_or(default_min);
    let max = value_spec
        .range
        .and_then(|range| range.max)
        .unwrap_or(default_max);
    value.clamp(min, max)
}

fn structured_str(value: &StructuredValue) -> Option<&str> {
    match value {
        StructuredValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn structured_number(value: &StructuredValue) -> Option<f64> {
    match value {
        StructuredValue::Number(value) => Some(*value),
        _ => None,
    }
}

fn structured_to_json(value: &StructuredValue) -> serde_json::Value {
    match value {
        StructuredValue::Null => serde_json::Value::Null,
        StructuredValue::Boolean(value) => serde_json::Value::Bool(*value),
        StructuredValue::Number(value) => serde_json::json!(value),
        StructuredValue::String(value) => serde_json::Value::String(value.clone()),
        StructuredValue::Array(values) => {
            serde_json::Value::Array(values.iter().map(structured_to_json).collect())
        }
        StructuredValue::Object(values) => serde_json::Value::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), structured_to_json(value)))
                .collect(),
        ),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_preview_loopbacks.rs</FILE> - <DESC>preview loopbacks</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
