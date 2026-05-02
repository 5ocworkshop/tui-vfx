// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_mapped_value_source.rs</FILE> - <DESC>Resolve numeric mapped value sources</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>ValueSource::Map uses canonical inclusive ranges and optional input clamping.</WCTX>
// <CLOG>0.1.0: INIT — split map resolution from value-source dispatch.</CLOG>

use tui_vfx_contract::{NumericRange, Value, ValueSource};

use crate::runtime::{
    ResolvedValue, RuntimeContext, RuntimeValueError, resolve_numeric_value_source,
};

pub(crate) fn resolve_mapped_value_source<'a>(
    from: &'a ValueSource,
    input: NumericRange,
    output: NumericRange,
    clamp: bool,
    context: &RuntimeContext,
) -> Result<ResolvedValue<'a>, RuntimeValueError> {
    let mut numeric = resolve_numeric_value_source(from, context, "map")?;
    let bounds = map_bounds(input, output)?;
    if clamp {
        numeric = numeric.clamp(bounds.input_min, bounds.input_max);
    }
    let denominator = bounds.input_max - bounds.input_min;
    if denominator == 0.0 {
        return Err(RuntimeValueError::unavailable(
            "map",
            "input range cannot have equal bounds",
        ));
    }
    let unit = (numeric - bounds.input_min) / denominator;
    Ok(ResolvedValue::owned(Value::Number(
        bounds.output_min + unit * (bounds.output_max - bounds.output_min),
    )))
}

struct MapBounds {
    input_min: f64,
    input_max: f64,
    output_min: f64,
    output_max: f64,
}

fn map_bounds(input: NumericRange, output: NumericRange) -> Result<MapBounds, RuntimeValueError> {
    let (Some(input_min), Some(input_max), Some(output_min), Some(output_max)) =
        (input.min, input.max, output.min, output.max)
    else {
        return Err(RuntimeValueError::unavailable(
            "map",
            "input and output ranges must both have min and max bounds",
        ));
    };
    Ok(MapBounds {
        input_min,
        input_max,
        output_min,
        output_max,
    })
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_resolve_mapped_value_source.rs</FILE> - <DESC>Resolve numeric mapped value sources</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
