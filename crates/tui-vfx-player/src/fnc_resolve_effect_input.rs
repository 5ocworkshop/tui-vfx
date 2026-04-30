// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Player graph execution: resolve effect inputs against the graph value bus.</WCTX>
// <CLOG>0.5.0: MINOR — thread graph-local values through effect input resolution.
// 0.4.0: MINOR — add gradient input resolver.
// 0.3.1: PATCH — centralize effect input lookup and RGBA label formatting.</CLOG>

use tui_vfx_contract::{EffectInputId, GradientSpec, NodeSpec, Value};

pub(crate) use crate::cls_resolved_color::ResolvedColor;

use crate::{
    PlayerSampleRequest, fnc_resolve_value_source::resolve_value_source_with_graph_values,
};

pub(crate) fn resolve_effect_number(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: f64,
) -> f64 {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Number(value) | Value::Duration(value)) => value,
        Some(Value::Integer(value)) => value as f64,
        _ => fallback,
    }
}

pub(crate) fn resolve_effect_integer(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: i64,
) -> i64 {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Integer(value)) => value,
        Some(Value::Number(value) | Value::Duration(value)) => value.round() as i64,
        _ => fallback,
    }
}

pub(crate) fn resolve_effect_bool(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: bool,
) -> bool {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Boolean(value)) => value,
        _ => fallback,
    }
}

pub(crate) fn resolve_effect_enum(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: &str,
) -> String {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Enum(value) | Value::String(value)) => value,
        _ => fallback.to_string(),
    }
}

pub(crate) fn resolve_effect_text(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: &str,
) -> String {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Enum(value) | Value::String(value) | Value::Text(value)) => value,
        _ => fallback.to_string(),
    }
}

pub(crate) fn resolve_effect_color(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: ResolvedColor,
) -> ResolvedColor {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Color(value)) => ResolvedColor::new(value.r, value.g, value.b, value.a),
        _ => fallback,
    }
}

pub(crate) fn resolve_effect_gradient(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
) -> Option<GradientSpec> {
    match resolve_effect_value(node, request, input_id) {
        Some(Value::Gradient(value)) => Some(value),
        _ => None,
    }
}

fn resolve_effect_value(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
) -> Option<Value> {
    node.inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| {
            resolve_value_source_with_graph_values(source, &request.signals, &request.graph_values)
        })
}

// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
