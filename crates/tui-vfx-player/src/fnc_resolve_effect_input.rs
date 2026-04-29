// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: share typed effect input resolution.</WCTX>
// <CLOG>0.1.0: INIT — add numeric and integer node input helpers.</CLOG>

use tui_vfx_contract::{EffectInputId, NodeSpec, Value};

use crate::{PlayerSampleRequest, fnc_resolve_value_source::resolve_value_source};

/// Resolve an effect input as a floating-point number.
pub(crate) fn resolve_effect_number(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: f64,
) -> f64 {
    match node
        .inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| resolve_value_source(source, &request.signals))
    {
        Some(Value::Number(value) | Value::Duration(value)) => value,
        Some(Value::Integer(value)) => value as f64,
        _ => fallback,
    }
}

/// Resolve an effect input as an integer.
pub(crate) fn resolve_effect_integer(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: i64,
) -> i64 {
    match node
        .inputs
        .get(&EffectInputId::new(input_id))
        .and_then(|source| resolve_value_source(source, &request.signals))
    {
        Some(Value::Integer(value)) => value,
        Some(Value::Number(value) | Value::Duration(value)) => value.round() as i64,
        _ => fallback,
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_resolve_effect_input.rs</FILE> - <DESC>Resolve graph node effect inputs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
