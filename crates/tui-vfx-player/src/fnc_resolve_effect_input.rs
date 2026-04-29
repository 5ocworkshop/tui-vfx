// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Player adapter de-slop: keep typed input resolution and color labels centralized.</WCTX>
// <CLOG>0.3.1: PATCH — centralize effect input lookup and RGBA label formatting.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeSpec, Value};

pub(crate) use crate::cls_resolved_color::ResolvedColor;

use crate::{PlayerSampleRequest, fnc_resolve_value_source::resolve_value_source};

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

fn resolve_effect_value(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
) -> Option<Value> {
    node.inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| resolve_value_source(source, &request.signals))
}

// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>END OF VERSION: 0.3.1</VERS>
