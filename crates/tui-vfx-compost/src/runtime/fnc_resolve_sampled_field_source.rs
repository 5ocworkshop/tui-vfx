// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_sampled_field_source.rs</FILE> - <DESC>Resolve deterministic sampled-field value sources</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Only contract-declared deterministic substrate fields are resolved before primitive migration.</WCTX>
// <CLOG>0.1.0: INIT — split sampled-field resolution from value-source dispatch.</CLOG>

use tui_vfx_contract::{Value, ValueSource};

use crate::runtime::{
    ResolvedValue, RuntimeContext, RuntimeValueError, resolve_numeric_value_source,
};

pub(crate) fn resolve_sampled_field_source<'a>(
    field: &str,
    x: &'a ValueSource,
    y: &'a ValueSource,
    fallback: &Option<Value>,
    context: &RuntimeContext,
) -> Result<ResolvedValue<'a>, RuntimeValueError> {
    match field {
        "surfaceAngleFrom" => resolve_surface_angle_from(x, y, context),
        _ => fallback
            .clone()
            .map(ResolvedValue::owned)
            .ok_or_else(|| RuntimeValueError::unsupported("sampledField")),
    }
}

fn resolve_surface_angle_from<'a>(
    x: &'a ValueSource,
    y: &'a ValueSource,
    context: &RuntimeContext,
) -> Result<ResolvedValue<'a>, RuntimeValueError> {
    let x = resolve_numeric_value_source(x, context, "sampledField")?;
    let y = resolve_numeric_value_source(y, context, "sampledField")?;
    Ok(ResolvedValue::owned(Value::Number(y.atan2(x))))
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_sampled_field_source.rs</FILE> - <DESC>Resolve deterministic sampled-field value sources</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
